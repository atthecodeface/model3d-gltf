//a Imports
use std::ops::Range;

use crate::{ODBufDataIndex, ODBufIndex};

//a BufferUsage
//tp BufferUsage
/// This type monitors the usage of a buffer - which range of bytes
/// are used for indices, and which range for vertex data
///
/// Once a buffer is exposed to the client of the gltf (in a Vec<B>)
/// this also maps to the index of that array; it also holds the
/// indices into the arrays of BufferData that the ranges themselves
/// refer to for vertex and index data (separately)
#[derive(Debug, Default, Clone)]
pub struct BufferUsage {
    /// The range of the buffer used for vertex data
    vertex_data: Range<usize>,
    /// The range of the buffer used for index data
    index_data: Range<usize>,
    /// The index into the user buffer Vec that this refers to
    buffer_index: ODBufIndex,
    /// The index into the user BufferData Vec that the vertex data range uses
    vertex_bd: Option<ODBufDataIndex>,
    /// The index into the user BufferData Vec that the index data range uses
    index_bd: Option<ODBufDataIndex>,
}

//ip BufferUsage
impl BufferUsage {
    //ap has_vertex_data
    /// Return true if this uses vertex data
    pub fn has_vertex_data(&self) -> bool {
        !self.vertex_data.is_empty()
    }

    //ap has_index_data
    /// Return true if this uses index data
    pub fn has_index_data(&self) -> bool {
        !self.index_data.is_empty()
    }

    //ap is_used
    /// Return true if this buffer is used at all
    pub fn is_used(&self) -> bool {
        self.has_vertex_data() || self.has_index_data()
    }

    //ap buffer_index
    /// Return the buffer index
    pub fn buffer_index(&self) -> ODBufIndex {
        self.buffer_index
    }

    //ap vertex_data
    /// Return the range of the buffer used for vertex data
    pub fn vertex_data(&self) -> &Range<usize> {
        &self.vertex_data
    }

    //ap index_data
    /// Return the range of the buffer used for index data
    pub fn index_data(&self) -> &Range<usize> {
        &self.index_data
    }

    //ap vertex_bd
    /// Return the buffer data index used by the range
    #[track_caller]
    pub fn vertex_bd(&self) -> ODBufDataIndex {
        self.vertex_bd.unwrap()
    }

    //ap index_bd
    /// Return the buffer data index used by the range
    #[track_caller]
    pub fn index_bd(&self) -> ODBufDataIndex {
        self.index_bd.unwrap()
    }

    //mp use_buffer
    /// Record the use of a portion of a buffer in its Usage
    ///
    /// The buffer records the extents of its usage for both index and vertex
    /// data, so that a client can map that data onto (e.g.) a GPU at a later
    /// date
    pub fn use_buffer(&mut self, as_index: bool, byte_start: usize, byte_length: usize) {
        let range = {
            if as_index {
                &mut self.index_data
            } else {
                &mut self.vertex_data
            }
        };
        if std::ops::Range::<usize>::is_empty(range) {
            *range = byte_start..(byte_start + byte_length)
        } else {
            *range = byte_start.min(range.start)..(byte_start + byte_length).max(range.end)
        };
    }

    //mp set_buffer_index
    pub fn set_buffer_index(&mut self, buffer_index: ODBufIndex) {
        self.buffer_index = buffer_index;
    }

    //mp set_buffer_data
    pub fn set_buffer_data(&mut self, as_index: bool, buffer_data_index: ODBufDataIndex) {
        if as_index {
            self.index_bd = Some(buffer_data_index);
        } else {
            self.vertex_bd = Some(buffer_data_index);
        }
    }
}
