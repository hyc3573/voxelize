#[macro_use]
extern crate glium;
use crate::load_model::load_model;
use glium::Surface;
use glam::{Mat4, Quat, Vec3};
use std::time::{SystemTime, UNIX_EPOCH};

mod load_model;
mod teapot;

fn main() {
    use glium::glutin;

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES,
    )
    .unwrap();

    let model = load_model("spoon.obj", &display);

    let frag = include_str!("fragshader.glsl");
    let vert = include_str!("vertexshader.glsl");
    let geom = include_str!("geomshader.glsl");

    let program = glium::Program::from_source(&display, vert, frag, Some(geom)).unwrap();

    let t = std::time::Instant::now();

    event_loop.run(move |ev, _, control_flow| {
        let m = Mat4::from_axis_angle(
            Vec3::new(f32::sqrt(2.0)/2., f32::sqrt(2.0)/2., 0.),
            t.elapsed().as_secs_f32()
        );
        let v = Mat4::from_scale(Vec3::new(0.1, 0.1, 0.1));
        let p = Mat4::IDENTITY;

        let mut target = display.draw();

        target.clear_color(0.0, 0.0, 0.0, 1.0);

        for i in 0..model.len() {
            target
                .draw(
                    &model[i].0,
                    &model[i].1,
                    &program,
                    &uniform! {M: m.to_cols_array_2d(), V: v.to_cols_array_2d(), P: p.to_cols_array_2d()},
                    &Default::default(),
                )
                .unwrap();
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
