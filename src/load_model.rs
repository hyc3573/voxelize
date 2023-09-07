use glium::{self, index::IndexBuffer, vertex::VertexBuffer};
use std::iter::zip;
use tobj;

#[derive(Copy, Clone)]
pub struct Vertex {
    pos: [f32; 3],
    nor: [f32; 3],
    tex: [f32; 2],
}
implement_vertex!(Vertex, pos, nor, tex);

pub fn load_model(
    model_name: &str,
    display: &dyn glium::backend::Facade,
) -> std::vec::Vec<(glium::VertexBuffer<Vertex>, glium::IndexBuffer<u32>)> {
    let model = tobj::load_obj(model_name, &tobj::GPU_LOAD_OPTIONS);

    let (models, materials) = model.expect(&format!("Failed to import: {}", model_name));

    // let materials = materials.expect("The given file is not a MTL file.");

    let mut buffers = Vec::<(VertexBuffer<Vertex>, IndexBuffer<u32>)>::new();
    buffers.reserve_exact(models.len());
    for model in &models {
        let mut vertexes = Vec::<Vertex>::new();
        vertexes.reserve_exact(model.mesh.positions.len() / 3);

        let mesh = &model.mesh;
        
        for ((pos, nor), tex) in zip(
            zip(mesh.positions.chunks(3), mesh.normals.chunks(3)),
            mesh.texcoords.chunks(2),
        ) {
            vertexes.push(Vertex {
                pos: Default::default(),
                nor: Default::default(),
                tex: Default::default(),
            });
            vertexes.last_mut().unwrap().pos.clone_from_slice(pos);
            vertexes.last_mut().unwrap().nor.clone_from_slice(nor);
            vertexes.last_mut().unwrap().tex.clone_from_slice(tex);
        }

        buffers.push((
            glium::VertexBuffer::<Vertex>::new(display, &vertexes).unwrap(),
            glium::IndexBuffer::<u32>::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                mesh.indices.as_slice(),
            )
            .unwrap(),
        ));
    }

    buffers
}
