use bytemuck::{Zeroable, Pod};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Zeroable, Pod)]
struct Vertex {
    position: [f32; 3],
}

vulkano::impl_vertex!(Vertex, position);
