#[macro_use]
extern crate glium;
use std::rc::Rc;

use crate::load_model::load_model;
use glium::{
    uniforms::{ImageUnitAccess, ImageUnitFormat},
    Surface, texture::TextureAnyImage, backend::Facade, framebuffer::{self, SimpleFrameBuffer},
};
use nalgebra_glm as glm;
use itertools::iproduct;

mod load_model;

const GWIDTH: u16 = 64;

fn main() {
    use glium::glutin;

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let model = load_model("/home/yuchan/Projects/voxelize/src/models/spoon.obj", &display);

    let frag = include_str!("/home/yuchan/Projects/voxelize/src/shaders/voxelize.frag");
    let vert = include_str!("/home/yuchan/Projects/voxelize/src/shaders/voxelize.vert");
    let geom = include_str!("/home/yuchan/Projects/voxelize/src/shaders/voxelize.geom");

    let program = glium::Program::from_source(&display, vert, frag, Some(geom)).unwrap();

    #[derive(Copy, Clone)]
    struct P2 {
        pos: [f32; 2],
    }
    implement_vertex!(P2, pos);

    let fullscreen_rect = [
        P2 {pos: [-1., 1.]},
        P2 {pos: [1., 1.]},
        P2 {pos: [-1., -1.]},
        P2 {pos: [1., 1.]},
        P2 {pos: [1., -1.]},
        P2 {pos: [-1., -1.]},
    ];
    let fullscreen_rect = glium::VertexBuffer::new(&display, &fullscreen_rect).unwrap();
    let fullscreen_ind = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let gridvert = include_str!("/home/yuchan/Projects/voxelize/src/shaders/voxelgrid.vert");
    let gridfrag = include_str!("/home/yuchan/Projects/voxelize/src/shaders/voxelgrid.frag");
    let gridprog = glium::Program::from_source(&display, gridvert, gridfrag, None).unwrap();

    #[derive(Copy, Clone)]
    struct P3 {
        pos: [f32; 3]
    }
    implement_vertex!(P3, pos);

    let mut grid = iproduct!((0..64), (0..64), (0..64)).map(
        |(a, b, c)| P3 {pos: [a as f32, b as f32, c as f32]}
    ).collect::<Vec<P3>>();
    let grid = glium::VertexBuffer::new(&display, &grid).unwrap();
    let grid_ind = glium::index::NoIndices(glium::index::PrimitiveType::Points);
    let gvert = include_str!("/home/yuchan/Projects/voxelize/src/shaders/grid.vert");
    let ggeom = include_str!("/home/yuchan/Projects/voxelize/src/shaders/grid.geom");
    let gfrag = include_str!("/home/yuchan/Projects/voxelize/src/shaders/grid.frag");
    let gprog = glium::Program::from_source(&display, &gvert, &gfrag, Some(&ggeom)).unwrap();

    let clearvert = include_str!("/home/yuchan/Projects/voxelize/src/shaders/gridclear.vert");
    let clearfrag = include_str!("/home/yuchan/Projects/voxelize/src/shaders/gridclear.frag");
    let clearprog = glium::Program::from_source(
        &display, clearvert, clearfrag, None
    ).unwrap();
    
    let t = std::time::Instant::now();

    let mut imunit_behav: glium::uniforms::ImageUnitBehavior = Default::default();
    imunit_behav.access = ImageUnitAccess::ReadWrite;
    imunit_behav.level = 1;
    imunit_behav.format = ImageUnitFormat::RGBA32F;
    let imunit_behav = imunit_behav;

    let empty = vec![vec![vec![(0., 0., 0., 0.); GWIDTH.into()]; GWIDTH.into()]; GWIDTH.into()];
    let voxelgrid = glium::texture::texture3d::Texture3d::with_format(&display, empty, glium::texture::UncompressedFloatFormat::F32F32F32F32, glium::texture::MipmapsOption::NoMipmap).unwrap();

    let mut framebuffer = glium::framebuffer::EmptyFrameBuffer::new(
        &display,
        GWIDTH.into(),
        GWIDTH.into(),
        None, None, false
    ).unwrap();

    event_loop.run(move |ev, _, control_flow| {
        let m = glm::scale::<f32>(
            &glm::rotation(
                45., &glm::vec3(0., 1., 0.)
            ), &glm::vec3(1., 1., 1.),
        );
        let v = glm::translation::<f32>(&glm::vec3(0., 0., 0.));
        let p = glm::ortho::<f32>(-1., 1., -1., 1., 0., -3.);
        // let voxelgrid = glium::texture::Texture3d::empty_with_format(&display, glium::texture::UncompressedFloatFormat::F32F32F32F32, glium::texture::MipmapsOption::NoMipmap, GWIDTH.into(), GWIDTH.into(), GWIDTH.into()).unwrap();

        let matrix = glm::rotation::<f32>(
            t.elapsed().as_secs_f32(), &glm::vec3(0., 1., 0.)
        );

        for i in 0..i32::from(GWIDTH) {
            framebuffer.draw(
                &fullscreen_rect,
                fullscreen_ind,
                &clearprog,
                &uniform! {
                    grid: voxelgrid.image_unit(
                            glium::uniforms::ImageUnitFormat::RGBA32F
                        ).unwrap().set_access(
                            glium::uniforms::ImageUnitAccess::Write
                    ),
                    depth: i,
                },
                &Default::default()
            ).unwrap();
        }

        for i in 0..model.len() {
            framebuffer
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
                        ),
                        GWIDTH: GWIDTH
                    },
                    &Default::default(),
                ).unwrap();
        }

        let mut target = display.draw();

        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // for i in 0..i32::from(GWIDTH) {
        //     target.draw(
        //         &fullscreen_rect,
        //         fullscreen_ind,
        //         &gridprog,
        //         &uniform! {
        //             grid: glium::uniforms::Sampler::new(&voxelgrid)
        //                 .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
        //                 .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
        //                 .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp),
        //             depth: i,
        //             GWIDTH: GWIDTH,
        //             matrix: *matrix.as_ref()
        //         },
        //         &Default::default()
        //     ).unwrap();
        // }
        target.draw(
            &grid,
            grid_ind,
            &gprog,
            &uniform! {
                matrix: *matrix.as_ref(),
                GWIDTH: GWIDTH,
                grid: glium::uniforms::Sampler::new(&voxelgrid)
                        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                        .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
            },
            &Default::default()
        );

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
