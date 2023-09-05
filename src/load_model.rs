use glium::{self, vertex::VertexBuffer};
use std::iter::zip;
use tobj;

fn load_model(model_name: &str, display: &dyn glium::backend::Facade) {
    #[derive(Copy, Clone)]
    struct Vertex {
        pos: [f32; 3],
        nor: [f32; 3],
        tex: [f32; 3],
    }
    implement_vertex!(Vertex, pos, nor, tex);

    let model = tobj::load_obj(model_name, &tobj::GPU_LOAD_OPTIONS);

    let (models, materials) = model.expect(&format!("Failed to import: {}", model_name));

    let materials = materials.expect("The given file is not a MTL file.");

    let mut vertexbuffers = Vec::<VertexBuffer<Vertex>>::new();
    vertexbuffers.reserve_exact(models.len());
    for model in &models {
        let mut min_pos = [f32::INFINITY; 3];
        let mut max_pos = [f32::INFINITY; 3];
        let mut vertexes = Vec::<Vertex>::new();
        vertexes.reserve_exact(model.mesh.positions.len() / 3);

        let mesh = &model.mesh;
        for ((pos, nor), tex) in zip(
            zip(mesh.positions.chunks(3), mesh.normals.chunks(3)),
            mesh.texcoords.chunks(3),
        ) {
            vertexes.push(Vertex {
                pos: Default::default(),
                nor: Default::default(),
                tex: Default::default(),
            });
            vertexes.last_mut().unwrap().pos.clone_from_slice(pos);
            vertexes.last_mut().unwrap().nor.clone_from_slice(nor);
            vertexes.last_mut().unwrap().tex.clone_from_slice(tex);

            min_pos = zip(min_pos, pos)
                .map(|(x, y)| f32::min(x, *y))
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap();
            max_pos = zip(max_pos, pos)
                .map(|(x, y)| f32::min(x, *y))
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap();
        }

        // Traverse the vertexes vector again, but this time rescaling the position to fit the -1~1 window.

        vertexbuffers.push(glium::VertexBuffer::<Vertex>::new(display, &vertexes).unwrap());
    }
}
