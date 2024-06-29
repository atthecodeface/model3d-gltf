/// A GLB file contains a 12-byte header; a chunk 0 (JSON); an optional
/// chunk 1 (binary data)
///
/// The 12-byte header is 0x46546C67_u32 ; 0x00000002_u32; byte_length_u32
///
/// byte_length_u32 must equal the byte size of the GLB file content
///
/// Chunk 0 has an 8-byte header that is byte_length_u32; 0x4E4F534A_u32,
/// followed by data
///
/// byte_length_u32 must equal the byte size of the data - must be a
/// multiple of 4 bytes
///
/// Chunk 1 (optional) has an 8-byte header that is byte_length_u32;
/// 0x004E4942_32, followed by data
///
/// byte_length_u32 must equal the byte size of the data - must be a
/// multiple of 4 bytes
///
///
/// A function is required that takes a GLB file and returns a Gltf Json
/// Value and invokes a callback on the chunk 1 file data to get an
/// Option<Vec<u8>> of chunk 1
///
/// A function is required that takes a Json file and returns a Gltf Json
/// Value
///
/// A method on the Gltf Json Value is required that takes invokes
/// callbacks for each buffer data in the file
///
/// A method on the Gltf Json Value is required that turns it into a Gltf
/// descriptor
///
/// Methods are required on the Gltf descriptor that access the
/// scenes; the default scene; the nodes by name; the skeletons; etc.
use serde_json::json;
use serde_json::Value as JsonValue;

use crate::{Error, Gltf, Result};

struct GlbLoader<'file, F: std::io::Read, B> {
    file: &'file mut F,
    max_json_length: usize,
    byte_length: u32,
    json_value: JsonValue,
    buffer_0: Option<B>,
}
impl<'file, F: std::io::Read, B> GlbLoader<'file, F, B> {
    //cp new
    /// Create a new GlbLoader given a file and maximum length
    /// expected of the Json within it
    fn new(file: &'file mut F, max_json_length: usize) -> Self {
        let byte_length = 0;
        let json_value = json!(null);
        let buffer_0 = None;
        Self {
            file,
            max_json_length,
            byte_length,
            json_value,
            buffer_0,
        }
    }

    //mp read_glb_hdr
    /// Read the GLB header from the file for the loader and validate
    /// it
    fn read_glb_hdr(&mut self) -> Result<()> {
        let mut hdr = [0; 12];
        self.file.read_exact(&mut hdr)?;
        if hdr[0..8] != [0x67, 0x6c, 0x54, 0x46, 0x02, 0x00, 0x00, 0x00] {
            return Err(Error::GlbHdr);
        }
        let byte_length = (hdr[8] as u32)
            | ((hdr[9] as u32) << 8)
            | ((hdr[10] as u32) << 16)
            | ((hdr[11] as u32) << 24);
        self.byte_length = byte_length;
        Ok(())
    }

    //mp read_json
    /// Read the Json chunk of the GLB file (assuming the header has
    /// been read), and validate the lengtht of it, and parse the Json
    ///
    /// Do not at this point attempt to validate the Json to be Gltf Json
    fn read_json(&mut self) -> Result<()> {
        let mut hdr = [0; 8];
        self.file.read_exact(&mut hdr).map_err(Error::GlbJsonIo)?;
        if hdr[4..8] != [0x4a, 0x53, 0x4f, 0x4e] {
            return Err(Error::GlbJsonHdr);
        }
        let json_byte_length = (hdr[0] as u32)
            | ((hdr[1] as u32) << 8)
            | ((hdr[2] as u32) << 16)
            | ((hdr[3] as u32) << 24);
        let json_byte_length = json_byte_length as usize;
        if json_byte_length > self.max_json_length {
            return Err(Error::GlbJsonHdr);
        }
        let mut buffer = vec![0; json_byte_length];
        let bytes_read = self.file.read(&mut buffer)?;
        if bytes_read < json_byte_length {
            return Err(Error::GlbJsonLength);
        }
        self.json_value = serde_json::from_str(std::str::from_utf8(&buffer)?)?;
        Ok(())
    }

    //mp read_buffer
    /// Read the 'binary' buffer  chunk of the GLB if it is present
    ///
    /// This does *NOT* support GLB files with a Json chunk and an
    /// extension chunk without a binary chunk. It consumes the header
    /// of the next Glb chunk and checks that it is a binary chunk; if
    /// no header is there (no binary) then all is okay, but if there
    /// is no binary chunk but there is an extension chunk then its
    /// header will have been consumed
    fn read_buffer<BR>(&mut self, buf_reader: &BR) -> Result<()>
    where
        BR: Fn(&mut F, usize) -> std::result::Result<Option<B>, std::io::Error>,
    {
        let mut hdr = [0; 8];
        let n = self.file.read(&mut hdr).map_err(Error::GlbJsonIo)?;
        if n == 0 {
            return Ok(());
        }
        if hdr[4..8] != [0x42, 0x49, 0x4e, 0x00] {
            return Err(Error::GlbBinHdr);
        }
        let bin_byte_length = (hdr[0] as u32)
            | ((hdr[1] as u32) << 8)
            | ((hdr[2] as u32) << 16)
            | ((hdr[3] as u32) << 24);
        let bin_byte_length = bin_byte_length as usize;
        self.buffer_0 = buf_reader(self.file, bin_byte_length).map_err(Error::GlbBinIo)?;
        Ok(())
    }

    //dp into_gltf
    /// Drop the borrow of the file and return a GltfJsonValue and the
    /// binary buffer loaded from the Glb file (if any)
    fn into_gltf(self) -> Result<(Gltf, Option<B>)> {
        let gltf_json_value = Gltf::of_json_value(self.json_value)?;
        Ok((gltf_json_value, self.buffer_0))
    }
}

pub fn glb_load<F, B, BR>(file: &mut F, b: &BR, max_json_length: usize) -> Result<(Gltf, Option<B>)>
where
    F: std::io::Read,
    BR: Fn(&mut F, usize) -> std::result::Result<Option<B>, std::io::Error>,
{
    let mut loader = GlbLoader::new(file, max_json_length);
    loader.read_glb_hdr()?;
    loader.read_json()?;
    loader.read_buffer(b)?;
    loader.into_gltf()
}
