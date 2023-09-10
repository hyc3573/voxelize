#[macro_use]
extern crate glium;
use crate::load_model::load_model;
use glam::{Mat4, Quat, Vec3};
use glium::{
    uniforms::{ImageUnitAccess, ImageUnitFormat},
    Surface, texture::TextureAnyImage,
};
use glm::normalize;
use nalgebra_glm as glm;
use std::{
    default,
    time::{SystemTime, UNIX_EPOCH},
};

mod load_model;
mod teapot;

fn main() {
    use glium::glutin;

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let model = load_model("spoon.obj", &display);

    let frag = include_str!("fragshader.glsl");
    let vert = include_str!("vertexshader.glsl");
    let geom = include_str!("geomshader.glsl");

    let program = glium::Program::from_source(&display, vert, frag, Some(geom)).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        pos: [f32; 2],
    }
    implement_vertex!(Vertex, pos);

    let fullscreen_rect = [
        Vertex {pos: [-1., 1.]},
        Vertex {pos: [1., 1.]},
        Vertex {pos: [-1., -1.]},
        Vertex {pos: [1., 1.]},
        Vertex {pos: [1., -1.]},
        Vertex {pos: [-1., -1.]},
    ];
    let fullscreen_rect = glium::VertexBuffer::new(&display, &fullscreen_rect).unwrap();
    let fullscreen_ind = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let gridvert = include_str!("voxelgrid.vert");
    let gridfrag = include_str!("voxelgrid.frag");
    
    let gridprog = glium::Program::from_source(&display, gridvert, gridfrag, None).unwrap();
    
    let t = std::time::Instant::now();

    let mut imunit_behav: glium::uniforms::ImageUnitBehavior = Default::default();
    imunit_behav.access = ImageUnitAccess::ReadWrite;
    imunit_behav.level = 1;
    imunit_behav.format = ImageUnitFormat::RGBA32F;
    let imunit_behav = imunit_behav;

    event_loop.run(move |ev, _, control_flow| {
        let m = glm::scale::<f32>(
            &glm::rotation(
                t.elapsed().as_secs_f32(),
                &glm::vec3(1., 1., 0.).normalize(),
            ),
            &glm::vec3(1., 1., 1.),
        );
        let v = glm::translation::<f32>(&glm::vec3(0., 0., 0.));
        let p = glm::ortho::<f32>(-1., 1., -1., 1., 1., -1.);
        // let p = glm::Mat4::identity();

        let empty = vec![vec![vec![(0., 0., 0., 0.); 64]; 64]; 64];
        let voxelgrid = glium::texture::texture3d::Texture3d::with_format(&display, empty, glium::texture::UncompressedFloatFormat::F32F32F32F32, glium::texture::MipmapsOption::NoMipmap).unwrap();

        let mut target = display.draw();

        target.clear_color(0.0, 0.0, 0.0, 1.0);

        for i in 0..model.len() {
            target
                .draw(
                    &model[i].0,
                    &model[i].1,
                    &program,
                    &uniform! {
                        M: *m.as_ref(),
                        V: *v.as_ref(),
                        P: *p.as_ref(),
                        grid: voxelgrid.image_unit(
                            glium::uniforms::ImageUnitFormat::RGBA32F
                        ).unwrap().set_access(
                            glium::uniforms::ImageUnitAccess::Write
                        )
                    },
                    &Default::default(),
                ).unwrap();
        }

        for i in 0..64 {
            target.draw(
                &fullscreen_rect,
                fullscreen_ind,
                &gridprog,
                &uniform! {
                    grid: &voxelgrid,
                    depth: i
                },
                &Default::default()
            ).unwrap();
        }

        target.finish().unwrap();

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
}
