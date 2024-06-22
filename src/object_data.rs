//a Imports
use model3d_base::hierarchy::NodeEnumOp;
use model3d_base::{BufferAccessor, BufferData, ByteBuffer, Renderable};

use crate::try_buf_parse_base64;
use crate::Gltf;
use crate::Result;
use crate::{AccessorIndex, BufferIndex, BufferUsage, MeshIndex, NodeIndex};

//a ObjectData
//tp ObjectData
///
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
    /// For all meshes, if used Some(array of Vertices index for each
    /// primitive); same size as json_value.meshes
    meshes: Vec<Option<Vec<Option<usize>>>>,
    /// For each accessor, the index into the accessors array (if used and it
    /// worked)
    accessors: Vec<Option<usize>>,
}

impl ObjectData {
    //cp new
    pub fn new(gltf: &Gltf) -> Self {
        let num_buffers = gltf.buffers().len();
        let num_meshes = gltf.meshes().len();
        let num_accessors = gltf.accessors().len();

        let nodes = vec![];
        let joints = vec![];
        let buffers = vec![Default::default(); num_buffers];
        let meshes = vec![Default::default(); num_meshes];
        let accessors = vec![Default::default(); num_accessors];
        Self {
            nodes,
            joints,
            buffers,
            meshes,
            accessors,
        }
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
        for eo in gltf.node_hierarchy().iter_from(nh_index) {
            if let NodeEnumOp::Push((_, n), _) = eo {
                self.add_node(*n);
            }
        }
    }

    //mp derive_uses
    /// Fill out the meshes and buffer regions that are used
    pub fn derive_uses(&mut self, gltf: &Gltf) {
        for n in &self.nodes {
            let node = &gltf[*n];
            if let Some(node_mesh) = node.mesh() {
                let mesh = &mut self.meshes[node_mesh.as_usize()];
                if mesh.is_none() {
                    let num_primitives = gltf[node_mesh].primitives().len();
                    *mesh = Some(vec![None; num_primitives]);
                }
            }
        }
        let mut accessors = vec![];
        for m in 0..self.meshes.len() {
            if self.meshes[m].is_some() {
                let mesh = &gltf.meshes()[m];
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
        self.buffers[buffer.as_usize()].use_buffer(as_index, byte_start, byte_length);
    }

    //mp uses_buffer_zero
    /// Returns true if buffer index 0 is used by the objects
    ///
    /// This is only valid after *derive_uses* has been invoked
    pub fn uses_buffer_zero(&self) -> bool {
        if let Some(b) = self.buffers.get(0) {
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
        if self.buffers.len() > 0 && self.buffers[0].is_used() && opt_buffer_0.is_some() {
            result.push(opt_buffer_0.unwrap());
            used_opt_0 = true;
            self.buffers[0].set_buffer_index(0);
        }
        for i in 0..self.buffers.len() {
            let buffer = gltf.take_buffer_data(i.into());
            if !self.buffers[i].is_used() {
                continue;
            }
            if i > 0 || !used_opt_0 {
                self.buffers[0].set_buffer_index(result.len());
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
            if !self.buffers[i].is_used() {
                continue;
            }
            let buffer_index = self.buffers[i].buffer_index();
            let b = buffer(buffer_index);
            if self.buffers[i].has_index_data() {
                self.buffers[i].set_buffer_data(true, buffer_data.len());
                let byte_offset = self.buffers[i].index_data().start;
                let byte_length = self.buffers[i].index_data().end - byte_offset;
                let bd = model3d_base::BufferData::new(b, byte_offset as u32, byte_length as u32);
                buffer_data.push(bd);
            }
            if self.buffers[i].has_vertex_data() {
                self.buffers[i].set_buffer_data(false, buffer_data.len());
                let byte_offset = self.buffers[i].vertex_data().start;
                let byte_length = self.buffers[i].vertex_data().end - byte_offset;
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
        let buffer = bv.buffer();
        let buffer = &self.buffers[buffer.as_usize()];
        let data = {
            if is_index {
                buffer.index_bd()
            } else {
                buffer.vertex_bd()
            }
        };
        let data = buffer_data(data);
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
            if self.meshes[i].is_none() {
                continue;
            }
            let i: MeshIndex = i.into();
            let mesh = &gltf[i];
            for p in mesh.primitives() {
                if let Some(ia) = p.indices() {
                    if self.accessors[ia.as_usize()].is_some() {
                        continue;
                    }
                    let b = self.make_accessor(gltf, buffer_data, true, ia);
                    self.accessors[ia.as_usize()] = Some(buffer_accessors.len());
                    buffer_accessors.push(b);
                }
                for (_, va) in p.attributes() {
                    if self.accessors[va.as_usize()].is_some() {
                        continue;
                    }
                    let b = self.make_accessor(gltf, buffer_data, false, *va);
                    self.accessors[va.as_usize()] = Some(buffer_accessors.len());
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
            if self.meshes[i].is_none() {
                continue;
            }
            let i: MeshIndex = i.into();
            let mesh = &gltf[i];
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
                let Some(ia) = self.accessors[ia.as_usize()] else {
                    continue;
                };
                let Some(pa) = self.accessors[pa.as_usize()] else {
                    continue;
                };
                let indices = buffer_accessor(ia);
                let positions = buffer_accessor(pa);
                let mut v = model3d_base::Vertices::new(indices, positions);
                for (va, vpa) in p.attributes() {
                    if *va == model3d_base::VertexAttr::Position {
                        continue;
                    }
                    if let Some(vpa) = self.accessors[vpa.as_usize()] {
                        v.add_attr(*va, buffer_accessor(vpa));
                    }
                }
                let primitve_v = vertices.len();
                vertices.push(v);
                self.meshes[pi].as_mut().unwrap()[pi] = Some(primitve_v);
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
            let Some(mesh) = node.mesh() else {
                continue;
            };
            let gltf_mesh = &gltf[mesh];
            let Some(mp) = &self.meshes[mesh.as_usize()] else {
                continue;
            };
            let mut mesh = model3d_base::Mesh::new();
            for pi in 0..mp.len() {
                let Some(vertices_index) = mp[pi] else {
                    continue;
                };
                let gltf_prim = &gltf_mesh.primitives()[pi];
                let ia = gltf_prim.indices().unwrap();
                let index_count = gltf[ia].count() as u32;
                let mat_ind = 0;
                let primitive = model3d_base::Primitive::new(
                    gltf_prim.primitive_type(),
                    vertices_index,
                    0,
                    index_count,
                    mat_ind,
                );
                eprintln!("Add mesh {mesh:?} {pi}");
                mesh.add_primitive(primitive);
            }
            object.add_component(None, Some(*node.global_transformation()), mesh);
        }
        object
    }
}
