use glium::{self, index::IndexBuffer, vertex::VertexBuffer};
use std::{iter::zip, f32::INFINITY};
use tobj;
use nalgebra_glm as glm;

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
) -> (std::vec::Vec<(glium::VertexBuffer<Vertex>, glium::IndexBuffer<u32>)>, glm::Mat4) {
    let model = tobj::load_obj(model_name, &tobj::GPU_LOAD_OPTIONS);

    let (models, materials) = model.expect(&format!("Failed to import: {}", model_name));

    let mut buffers = Vec::<(VertexBuffer<Vertex>, IndexBuffer<u32>)>::new();
    buffers.reserve_exact(models.len());

    let mut maxcoord = glm::vec3(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY);
    let mut mincoord = -maxcoord;
    for model in &models {
        let mut vertexes = Vec::<Vertex>::new(); // 꼭짓점 벡터 생성
        vertexes.reserve_exact(model.mesh.positions.len() / 3); // 미리 최대 크기 지정

        let mesh = &model.mesh;
        
        for ((pos, nor), tex) in zip(
            zip(mesh.positions.chunks(3), mesh.normals.chunks(3)),
            mesh.texcoords.chunks(2),
        ) /* 모델에 있는 위치, 법선벡터, 텍스쳐 좌표 데이터를 묶어서 반복 */ {
            vertexes.push(Vertex {
                pos: Default::default(),
                nor: Default::default(),
                tex: Default::default(),
            });
            vertexes.last_mut().unwrap().pos.clone_from_slice(pos);
            vertexes.last_mut().unwrap().nor.clone_from_slice(nor);
            vertexes.last_mut().unwrap().tex.clone_from_slice(tex);
            // 꼭짓점 벡터에 저장

            let vec = glm::vec3(pos[0], pos[1], pos[2]);
            maxcoord = glm::max2(&maxcoord, &vec);
            mincoord = glm::min2(&mincoord, &vec);
        }

        buffers.push((
            glium::VertexBuffer::<Vertex>::new(display, &vertexes).unwrap(),
            glium::IndexBuffer::<u32>::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                mesh.indices.as_slice(),
            )
            .unwrap(),
        )); // OpenGL Vertex Buffer Object 생성
    }

    println!("{} {}", mincoord, maxcoord);

    let size = (maxcoord - mincoord)/2.;
    let sizesc = size.max();
    let sizevec = glm::vec3(1./size.max(), 1./size.max(), 1./size.max());
    let position = (mincoord + maxcoord)/2.;

    let modelmat = glm::translate(
        &glm::scaling(&sizevec),
        &(-position)
    );

    (buffers, modelmat)
}
