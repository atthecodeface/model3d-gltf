//a Imports
use model3d_base::hierarchy::NodeEnumOp;
use model3d_base::{BufferAccessor, BufferData, ByteBuffer, Renderable};

use crate::try_buf_parse_base64;
use crate::Gltf;
use crate::Result;
use crate::{AccessorIndex, BufferIndex, BufferUsage, MeshIndex, NodeIndex, PrimitiveIndex};
use crate::{ODAccIndex, ODVerticesIndex};

//a ObjectData
//tp ObjectData
/// The type that is used to construct model3d_base from a Gltf
///
/// The objects from the Gltf that are required by the client must be added to this, so that
/// the node hierarchy can be interrogated to determine which buffers,
/// views, accessors, meshes, images, samples, and textures are
/// required
///
/// Once the ojbects are added the uses must be derived (no more objects can be added at this point).
///
/// Once derived the model3d_base::Buffer required must be generated;
/// then the model3d_base::BufferData; then the
/// model3d_base::BufferAccessor. These generate Vec of the relevant
/// types, which must remain live until any object is made into an
/// Instantiable; the [ObjectData] maintains indiices into these Vec
/// for the Gltf buffers (etc) that are used by the required objects.
///
/// Then the Vertices are created; these borrow from the previous Vec
/// of Buffer-related structures
///
/// Finally a model3d_base::Object can be created, from which the
/// client can create a model3d_base::Instantiable; at this point the
/// buffer data and vertices can be dropped (if the
/// model3d_base::Renderable permits it)
#[derive(Debug)]
pub struct ObjectData {
    /// Nodes used in the object; this must contain all the mesh nodes (but not
    /// the joint nodes)
    nodes: Vec<NodeIndex>,
    /// Nodes used in skeletons for the object
    ///
    /// There is no support (as yet) for unskinned meshes that are children of
    /// joints; if a joint appears anywhere in the ancestors of an unskinned
    /// mesh node then *unlike* the GLTF specification (which says the joint
    /// pose should be used) the mesh is not posed, but placed at its static
    /// positioning given by the hierarchy of nodes from its root node
    #[allow(dead_code)]
    joints: Vec<NodeIndex>,
    /// A vec the same size as
    /// json_value. which maps a Json buffer to
    /// the range of it that is used
    buffers: Vec<BufferUsage>,
    /// For each accessor, the index into the Vec<BufferAccessor> (if used and it
    /// worked); same size as gltf.buffer_views
    accessors: Vec<Option<ODAccIndex>>,
    /// For all meshes, if used Some(array of possible Vertices index for each
    /// primitive); same size as gltf.meshes
    ///
    /// This maps each primitive that is used in each mesh that is
    /// used (in the order of the Gltf fiile itself) to a client
    /// Vertices index (which must be the same index as in the model3d_base::Object
    meshes: Vec<Option<Vec<Option<ODVerticesIndex>>>>,
    /// For each image in the Fltf, the index into the Vec<> array (if used and it
    /// worked)
    images: Vec<Option<usize>>,
}

//ip Index<BufferIndex> for ObjectData
impl std::ops::Index<BufferIndex> for ObjectData {
    type Output = BufferUsage;
    fn index(&self, index: BufferIndex) -> &Self::Output {
        &self.buffers[index.as_usize()]
    }
}

//ip IndexMut<BufferIndex> for ObjectData
impl std::ops::IndexMut<BufferIndex> for ObjectData {
    fn index_mut(&mut self, index: BufferIndex) -> &mut Self::Output {
        &mut self.buffers[index.as_usize()]
    }
}

//ip Index<AccessorIndex> for ObjectData
impl std::ops::Index<AccessorIndex> for ObjectData {
    type Output = Option<ODAccIndex>;
    fn index(&self, index: AccessorIndex) -> &Self::Output {
        &self.accessors[index.as_usize()]
    }
}

//ip IndexMut<AccessorIndex> for ObjectData
impl std::ops::IndexMut<AccessorIndex> for ObjectData {
    fn index_mut(&mut self, index: AccessorIndex) -> &mut Self::Output {
        &mut self.accessors[index.as_usize()]
    }
}

//ip Index<MeshIndex> for ObjectData
impl std::ops::Index<MeshIndex> for ObjectData {
    type Output = Option<Vec<Option<ODVerticesIndex>>>;
    fn index(&self, index: MeshIndex) -> &Self::Output {
        &self.meshes[index.as_usize()]
    }
}

//ip IndexMut<MeshIndex> for ObjectData
impl std::ops::IndexMut<MeshIndex> for ObjectData {
    fn index_mut(&mut self, index: MeshIndex) -> &mut Self::Output {
        &mut self.meshes[index.as_usize()]
    }
}

//ip ObjectData
impl ObjectData {
    //cp new
    /// Create a new [ObjectData]
    pub fn new(gltf: &Gltf) -> Self {
        let num_buffers = gltf.buffers().len();
        let num_meshes = gltf.meshes().len();
        let num_accessors = gltf.accessors().len();

        let nodes = vec![];
        let joints = vec![];
        let buffers = vec![Default::default(); num_buffers];
        let meshes = vec![Default::default(); num_meshes];
        let accessors = vec![Default::default(); num_accessors];
        let images = vec![];
        Self {
            nodes,
            joints,
            buffers,
            meshes,
            accessors,
            images,
        }
    }

    //ap first_buffer
    fn first_buffer(&mut self) -> Option<&mut BufferUsage> {
        self.buffers.get_mut(0)
    }

    //mi add_joint_node
    /// Add a joint node to the set of nodes used by this Object
    #[allow(dead_code)]
    fn add_joint_node(&mut self, node: NodeIndex) {
        if !self.joints.contains(&node) {
            self.joints.push(node);
        }
    }

    //mi add_node
    /// Add a node index to the set of nodes used by this Object
    fn add_node(&mut self, node: NodeIndex) {
        if !self.nodes.contains(&node) {
            self.nodes.push(node);
        }
    }

    //mp add_object
    /// Add an object to the ObjectData; adds all the nodes in the hierarchy of
    /// the specified node
    pub fn add_object(&mut self, gltf: &Gltf, node: NodeIndex) {
        let nh_index = gltf.nh_index(node);
        for eo in gltf.node_hierarchy().iter_from(nh_index.as_usize()) {
            if let NodeEnumOp::Push((_, n), _) = eo {
                self.add_node(*n);
            }
        }
    }

    //mp derive_uses
    /// Fill out the meshes and buffer regions that are used
    pub fn derive_uses(&mut self, gltf: &Gltf) {
        for n in 0..self.nodes.len() {
            let ni: NodeIndex = n.into();
            let node = &gltf[ni];
            if let Some(node_mesh) = node.mesh() {
                let mesh = &mut self[node_mesh];
                if mesh.is_none() {
                    let num_primitives = gltf[node_mesh].primitives().len();
                    *mesh = Some(vec![None; num_primitives]);
                }
            }
        }
        let mut accessors = vec![];
        for m in 0..self.meshes.len() {
            let mi: MeshIndex = m.into();
            if self[mi].is_some() {
                let mesh = &gltf[mi];
                for p in mesh.primitives() {
                    if let Some(a) = p.indices() {
                        accessors.push((true, a));
                    }
                    for (_, a) in p.attributes() {
                        accessors.push((false, *a));
                    }
                }
            }
        }
        for (as_index, a) in accessors {
            if let Some(bv) = gltf[a].buffer_view() {
                let buffer = gltf[bv].buffer();
                let byte_start = gltf[bv].byte_offset();
                let byte_length = gltf[bv].byte_length();
                self.use_buffer(as_index, buffer, byte_start, byte_length);
            }
        }
    }

    //mi use_buffer
    /// Record the use of a portion of a buffer in its Usage
    ///
    /// The buffer records the extents of its usage for both index and vertex
    /// data, so that a client can map that data onto (e.g.) a GPU at a later
    /// date
    fn use_buffer(
        &mut self,
        as_index: bool,
        buffer: BufferIndex,
        byte_start: usize,
        byte_length: usize,
    ) {
        self[buffer].use_buffer(as_index, byte_start, byte_length);
    }

    //mp uses_buffer_zero
    /// Returns true if buffer index 0 is used by the objects
    ///
    /// This is only valid after *derive_uses* has been invoked
    pub fn uses_buffer_zero(&self) -> bool {
        if let Some(b) = self.buffers.first() {
            b.is_used()
        } else {
            false
        }
    }

    //mp gen_buffers
    /// Generate a Vec of all the buffers required for the objects used in Gltf
    ///
    /// Drop the buffer descriptors in the GltfJsonValue as we go
    ///
    /// The first comes from opt_buffer_0 if Some; this may come from
    /// a Glb file binary chunk, for example.
    ///
    /// The rest are created by invoking buf_parse on the Uri and
    /// byte_length specified in the [GltfJsonValue]
    pub fn gen_buffers<B, BP>(
        &mut self,
        gltf: &mut Gltf,
        buf_parse: &BP,
        opt_buffer_0: Option<B>,
    ) -> Result<Vec<B>>
    where
        BP: Fn(&str, usize) -> Result<B>,
    {
        let mut result = vec![];
        let mut used_opt_0 = false;
        if let Some(b) = self.first_buffer() {
            if b.is_used() && opt_buffer_0.is_some() {
                result.push(opt_buffer_0.unwrap());
                used_opt_0 = true;
                b.set_buffer_index(0.into());
            }
        }
        for i in 0..self.buffers.len() {
            let bi: BufferIndex = i.into();
            let buffer = gltf.take_buffer_data(bi);
            if !self[bi].is_used() {
                continue;
            }
            if i > 0 || !used_opt_0 {
                self[bi].set_buffer_index(result.len().into());
                result.push(buf_parse(buffer.uri(), buffer.byte_length())?);
            }
        }
        Ok(result)
    }

    //mp gen_byte_buffers
    /// Generate a Vec of all the Vec<u8> buffers required for the objects used
    /// in the Gltf
    ///
    /// This is the same as [gen_buffers] except that it requires the buffer
    /// type by Vec<u8>, and it also implicitly supports base64 decode of data:
    /// URIs
    pub fn gen_byte_buffers<BP>(
        &mut self,
        gltf: &mut Gltf,
        buf_parse: &BP,
        opt_buffer_0: Option<Vec<u8>>,
    ) -> Result<Vec<Vec<u8>>>
    where
        BP: Fn(&str, usize) -> Result<Vec<u8>>,
    {
        let bp = |uri: &str, byte_length: usize| {
            if let Some(buf) = try_buf_parse_base64(uri, byte_length)? {
                Ok(buf)
            } else {
                buf_parse(uri, byte_length)
            }
        };
        self.gen_buffers(gltf, &bp, opt_buffer_0)
    }

    //mp gen_buffer_data
    /// Generate [BufferData] from all of the buffer views (one BufferData per
    /// view)
    ///
    /// Should be invoked after gen_buffers has returned a Vec<> of the buffers
    /// used by the data
    pub fn gen_buffer_data<'buffers, B, F, R>(&mut self, buffer: &F) -> Vec<BufferData<'buffers, R>>
    where
        B: ByteBuffer + ?Sized + 'buffers,
        F: Fn(usize) -> &'buffers B,
        R: Renderable + ?Sized,
    {
        let mut buffer_data = vec![];
        for i in 0..self.buffers.len() {
            let bi: BufferIndex = i.into();
            if !self[bi].is_used() {
                continue;
            }
            let b = buffer(self[bi].buffer_index().as_usize());
            if self[bi].has_index_data() {
                self[bi].set_buffer_data(true, buffer_data.len().into());
                let byte_offset = self[bi].index_data().start;
                let byte_length = self[bi].index_data().end - byte_offset;
                let bd = model3d_base::BufferData::new(b, byte_offset as u32, byte_length as u32);
                buffer_data.push(bd);
            }
            if self[bi].has_vertex_data() {
                self[bi].set_buffer_data(false, buffer_data.len().into());
                let byte_offset = self[bi].vertex_data().start;
                let byte_length = self[bi].vertex_data().end - byte_offset;
                let bd = model3d_base::BufferData::new(b, byte_offset as u32, byte_length as u32);
                buffer_data.push(bd);
            }
        }
        buffer_data
    }

    //mi make_accessor
    fn make_accessor<'buffers, F, R>(
        &self,
        gltf: &Gltf,
        buffer_data: &F,
        is_index: bool,
        acc: AccessorIndex,
    ) -> BufferAccessor<'buffers, R>
    where
        F: Fn(usize) -> &'buffers BufferData<'buffers, R>,
        R: Renderable + ?Sized,
    {
        let ba = &gltf[acc];
        let bv = ba.buffer_view().unwrap();
        let bv = &gltf[bv];
        let buffer = &self[bv.buffer()];
        let data = {
            if is_index {
                buffer.index_bd()
            } else {
                buffer.vertex_bd()
            }
        };
        let data = buffer_data(data.as_usize());
        let byte_offset = ba.byte_offset() + bv.byte_offset() - (data.byte_offset as usize);
        let byte_stride = bv.byte_stride(ba.ele_byte_size());
        let count = {
            if is_index {
                ba.count()
            } else {
                ba.elements_per_data()
            }
        };
        eprintln!("make_accessor ba:? {data:?}, {count}, {byte_offset}, {byte_stride}");
        BufferAccessor::new(
            data,
            count as u32,
            ba.component_type(),
            byte_offset as u32,
            byte_stride as u32,
        )
    }

    //mp gen_accessors
    /// Generate [BufferAccessor] from all of the accessors used by the mesh
    /// primitives used in the objects used in the Gltf
    ///
    /// Should be invoked after gen_buffer_data has returned a Vec<> of the
    /// BufferData
    pub fn gen_accessors<'buffers, F, R>(
        &mut self,
        gltf: &Gltf,
        buffer_data: &F,
    ) -> Vec<BufferAccessor<'buffers, R>>
    where
        F: Fn(usize) -> &'buffers BufferData<'buffers, R>,
        R: Renderable + ?Sized,
    {
        let mut buffer_accessors = vec![];
        for i in 0..self.meshes.len() {
            let mi: MeshIndex = i.into();
            if self[mi].is_none() {
                continue;
            }
            let mesh = &gltf[mi];
            for p in mesh.primitives() {
                if let Some(ia) = p.indices() {
                    if self[ia].is_some() {
                        continue;
                    }
                    let b = self.make_accessor(gltf, buffer_data, true, ia);
                    self[ia] = Some(buffer_accessors.len().into());
                    buffer_accessors.push(b);
                }
                for (_, va) in p.attributes() {
                    if self[*va].is_some() {
                        continue;
                    }
                    let b = self.make_accessor(gltf, buffer_data, false, *va);
                    self[*va] = Some(buffer_accessors.len().into());
                    buffer_accessors.push(b);
                }
            }
        }
        buffer_accessors
    }

    //mp gen_vertices
    /// Generate vertices from the objects in the Gltf, given buffer accessors
    /// that have been generated already
    pub fn gen_vertices<'vertices, F, R>(
        &mut self,
        gltf: &Gltf,
        buffer_accessor: &F,
    ) -> Vec<model3d_base::Vertices<'vertices, R>>
    where
        F: Fn(usize) -> &'vertices BufferAccessor<'vertices, R>,
        R: Renderable + ?Sized,
    {
        let mut vertices = vec![];
        for i in 0..self.meshes.len() {
            let mi: MeshIndex = i.into();
            if self[mi].is_none() {
                continue;
            }
            let mesh = &gltf[mi];
            for (pi, p) in mesh.primitives().iter().enumerate() {
                let Some(ia) = p.indices() else {
                    continue;
                };
                let mut pa = None;
                for (va, vpa) in p.attributes() {
                    if *va == model3d_base::VertexAttr::Position {
                        pa = Some(vpa);
                        break;
                    }
                }
                let Some(pa) = pa else {
                    continue;
                };
                let Some(ia) = self[ia] else {
                    continue;
                };
                let Some(pa) = self[*pa] else {
                    continue;
                };
                let indices = buffer_accessor(ia.as_usize());
                let positions = buffer_accessor(pa.as_usize());
                let mut v = model3d_base::Vertices::new(indices, positions);
                for (va, vpa) in p.attributes() {
                    if *va == model3d_base::VertexAttr::Position {
                        continue;
                    }
                    if let Some(vpa) = self[*vpa] {
                        v.add_attr(*va, buffer_accessor(vpa.as_usize()));
                    }
                }
                let primitve_v = vertices.len();
                vertices.push(v);
                self[mi].as_mut().unwrap()[pi] = Some(primitve_v.into());
            }
        }

        vertices
    }

    //mp gen_materials
    // Create materials

    //mp gen_object
    /// Create object
    pub fn gen_object<'object, R>(
        &mut self,
        gltf: &Gltf,
        vertices: &'object [model3d_base::Vertices<'object, R>],
    ) -> model3d_base::Object<'object, R>
    where
        R: Renderable + ?Sized,
    {
        let mut object = model3d_base::Object::new();
        for v in vertices {
            object.add_vertices(v);
        }
        for n in &self.nodes {
            let node = &gltf[*n];
            let Some(mi) = node.mesh() else {
                continue;
            };
            let gltf_mesh = &gltf[mi];
            let Some(od_mesh_prims) = &self[mi] else {
                continue;
            };
            let mut mesh = model3d_base::Mesh::new();
            for (m_pi, opt_od_vi) in od_mesh_prims.iter().enumerate() {
                let m_pi: PrimitiveIndex = m_pi.into();
                let Some(od_vi) = *opt_od_vi else {
                    continue;
                };
                let gltf_prim = &gltf_mesh[m_pi];
                let ia = gltf_prim.indices().unwrap();
                let index_count = gltf[ia].count() as u32;
                let mat_ind = 0;
                let primitive = model3d_base::Primitive::new(
                    gltf_prim.primitive_type(),
                    od_vi.as_usize(),
                    0,
                    index_count,
                    mat_ind,
                );
                eprintln!("Add mesh {mesh:?} {m_pi}");
                mesh.add_primitive(primitive);
            }
            object.add_component(None, Some(*node.global_transformation()), mesh);
        }
        object
    }
}
