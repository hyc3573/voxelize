use glium::{self, index::IndexBuffer, vertex::VertexBuffer};
use image::{io::Reader, GenericImageView};
use std::{iter::zip, f32::INFINITY, path::Path};
use std::rc::Rc;
use tobj;
use nalgebra_glm as glm;

#[derive(Copy, Clone)]
pub struct Vertex {
    pos: [f32; 3],
    nor: [f32; 3],
    tex: [f32; 2],
}
implement_vertex!(Vertex, pos, nor, tex);

pub struct Material {
    pub kd: glium::Texture2d,
    pub ks: glium::Texture2d
}

pub struct Model {
    pub vbo: glium::VertexBuffer<Vertex>,
    pub ibo: glium::IndexBuffer<u32>,
    pub material: Rc::<Material>
}

fn get_texture_or_value_or_default(
    display: &dyn glium::backend::Facade,
    texturepath: &Option<String>,
    value: &Option<[f32; 3]>,
    default: [f32; 3]
) -> glium::Texture2d {
    match texturepath {
        Some(texture) => {
            let image = Reader::open(texture).unwrap().decode().unwrap().to_rgba8();
            let img_dim = image.dimensions();
            return glium::Texture2d::new(display, glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), img_dim)).unwrap();
        }
        None => {
            let kd_value = value.unwrap_or(default);
            return glium::Texture2d::new(display, glium::texture::RawImage2d::from_raw_rgba(vec!(kd_value[0], kd_value[1], kd_value[2], 1.0), (1, 1,))).unwrap();
        }
    }
}

pub fn load_model(
    model_objpath: &Path,
    display: &dyn glium::backend::Facade,
) -> (Vec<Model>, glm::Mat4) {
    let model = tobj::load_obj(model_objpath, &tobj::GPU_LOAD_OPTIONS);

    let (models, materials) = model.expect(&format!("Failed to import: {}", model_objpath.to_str().unwrap()));

    let materials = materials.expect("Failed to load matching MTL file");

    let mut preload_textures = Vec::<Rc<Material>>::new();
    preload_textures.reserve_exact(materials.len()+1);

    for (i, mat) in materials.iter().enumerate() {
        println!("Loading texture: {}", mat.name);
        let kd = get_texture_or_value_or_default(display, &mat.diffuse_texture, &mat.diffuse, [1., 1., 1.]);
        let ks = get_texture_or_value_or_default(display, &mat.specular_texture, &mat.specular, [1., 1., 1.]);

        preload_textures.push(Rc::new(Material {kd, ks}));
    }
    preload_textures.push(Rc::new(Material {
        kd: get_texture_or_value_or_default(display, &None, &None, [1., 1., 1.,]),
        ks: get_texture_or_value_or_default(display, &None, &None, [1., 1., 1.,]),
    }));

    let mut buffers = Vec::<Model>::new();
    buffers.reserve_exact(models.len());

    let mut maxcoord = glm::vec3(-f32::INFINITY, -f32::INFINITY, -f32::INFINITY);
    let mut mincoord = -maxcoord;
;
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

        let id = mesh.material_id.unwrap_or(materials.len());

        buffers.push(Model {
            vbo: glium::VertexBuffer::<Vertex>::new(display, &vertexes).unwrap(),
            ibo: glium::IndexBuffer::<u32>::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                mesh.indices.as_slice(),
            ).unwrap(),
            material: Rc::clone(&preload_textures[id])
        }); // OpenGL Vertex Buffer Object 생성
    }

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
