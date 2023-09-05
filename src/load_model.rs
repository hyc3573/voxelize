use glium;
use tobj;

use crate::teapot::INDICES;

pub struct Mesh<T>
{
    positions: glium::VertexBuffer<f32>,
    normals: glium::VertexBuffer<f32>,
    texcoords: glium::VertexBuffer<f32>,
    indices: glium::IndexBuffer<i8>
}

fn load_model<F>(model_name: &str, display: &F)
{
    let mut model = tobj::load_obj(model_name, &tobj::GPU_LOAD_OPTIONS);

    let (models, materials) = model.expect(
        &format!("Failed to import: {}", model_name)
    );

    let materials = materials.expect("The given file is not a MTL file.");

    let mut meshes = vec![];
    meshes.reserve_exact(models.len());
    let mut materials = vec![1, 2, 3];
    materials.reserve_exact(materials.len());

    for (i, m) in models.iter().enumerate() {
        meshes[i] = Mesh {
            positions: glium::VertexBuffer::new(
                &display,
                &m.mesh.positions
            ),
            normals: glium::VertexBuffer::new(
                &display,
                &m.mesh.normals
            ),
            texcoords: glium::VertexBuffer::new(
                &display,
                &m.mesh.texcoords
            ),
            indices: glium::IndexBuffer::new(
                &display,
                &m.mesh.indices
            )
        };
    }
}
