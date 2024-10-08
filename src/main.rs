#[macro_use]
extern crate glium;
use std::{rc::Rc, default};

use crate::load_model::load_model;
use glium::{
    uniforms::{ImageUnitAccess, ImageUnitFormat, ImageUnit},
    Surface, texture::TextureAnyImage, backend::Facade, framebuffer::{self, SimpleFrameBuffer}
};
use winit::{self, event::{self, ElementState}};
use nalgebra_glm as glm;
use itertools::{iproduct, Itertools};
use image;
use egui;
use std::path::Path;
use egui_glium;
use const_format::concatcp;
use std::env;
use load_file::load_file_str;
use egui::ViewportId;

mod load_model;

#[cfg(debug_assertions)]
const RUN_DIR: &str = env!("CARGO_MANIFEST_DIR");
#[cfg(debug_assertions)]
const SRC_DIR: &str = concatcp!(env!("CARGO_MANIFEST_DIR"), "/src/");

#[cfg(not(debug_assertions))]
const RUN_DIR: &str = ".";
#[cfg(not(debug_assertions))]
const SRC_DIR: &str = "./";

fn itmat(input: &glm::Mat4) -> glm::Mat3 {
    glm::transpose(&glm::inverse(&glm::mat4_to_mat3(input)))
}

fn generate_mipmap(grid: &glium::texture::Texture3d, mipmapprog: &glium::program::ComputeShader, gwidth: u16, miplvl: u32, frame_counter: i32) {
    let mut size = gwidth/2;
    let mut factor = 1;
    for i in 1..(miplvl-1) {
        if frame_counter % factor == 0 {
            mipmapprog.execute(
                uniform! {
                    lod: (i as i32)-1,
                    resultimg: grid.image_unit(
                        glium::uniforms::ImageUnitFormat::RGBA16F
                    ).unwrap().set_access(
                        glium::uniforms::ImageUnitAccess::Write
                    ).set_level(i.into()).unwrap(),
                    sampler: grid
                },
                size.into(), size.into(), size.into()
            );
        }
        size /= 2;
        factor *= 4;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    use glium::backend::glutin;

    // let mut event_loop = glutin::event_loop::EventLoop::new();
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    //  let wb = glutin::window::WindowBuilder::new();
    // let cb = glutin::ContextBuilder::new();
    // let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    let mut egui_glium = egui_glium::EguiGlium::new(ViewportId::ROOT, &display, &window, &event_loop);

    let (models, m, voxelmatrix) = load_model(Path::new(concatcp!(RUN_DIR, "/models/sponza.obj")),
                                              Path::new(RUN_DIR), &display);

    let frag = load_file_str(Path::new(concatcp!(SRC_DIR, "shaders/voxelize.frag"))).unwrap();
    let vert = load_file_str(Path::new(concatcp!(SRC_DIR, "shaders/voxelize.vert"))).unwrap();
    let geom = load_file_str(Path::new(concatcp!(SRC_DIR, "shaders/voxelize.geom"))).unwrap();

    let program = glium::Program::from_source(&display, vert, frag, Some(geom)).unwrap();

    let mut gwidth: u16 = 256;
    if args.len() >= 2 {
        gwidth = args[1].parse().unwrap();
    }
    let gwidth = gwidth;
    let miplvl = gwidth.ilog2();

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
    let gridvert = load_file_str(Path::new(concatcp!(SRC_DIR, "shaders/voxelgrid.vert"))).unwrap();
    let gridfrag = load_file_str(Path::new(concatcp!(SRC_DIR, "shaders/voxelgrid.frag"))).unwrap();
    let gridprog = glium::Program::from_source(&display, gridvert, gridfrag, None).unwrap();

    #[derive(Copy, Clone)]
    struct P3 {
        pos: [f32; 3]
    }
    implement_vertex!(P3, pos);

    let mut grid = iproduct!((0..gwidth), (0..gwidth), (0..gwidth)).map(
        |(a, b, c)| P3 {pos: [a as f32, b as f32, c as f32]}
    ).collect::<Vec<P3>>();
    let grid = glium::VertexBuffer::new(&display, &grid).unwrap();
    let grid_ind = glium::index::NoIndices(glium::index::PrimitiveType::Points);
    let gvert = load_file_str(Path::new(concatcp!(SRC_DIR, "shaders/grid.vert"))).unwrap();
    let ggeom = load_file_str(Path::new(concatcp!(SRC_DIR, "shaders/grid.geom"))).unwrap();
    let gfrag = load_file_str(Path::new(concatcp!(SRC_DIR, "shaders/grid.frag"))).unwrap();
    let gprog = glium::Program::from_source(&display, &gvert, &gfrag, Some(&ggeom)).unwrap();

    let clearvert = load_file_str(Path::new(concatcp!(SRC_DIR, "shaders/gridclear.vert"))).unwrap();
    let clearfrag = load_file_str(Path::new(concatcp!(SRC_DIR, "shaders/gridclear.frag"))).unwrap();
    let clearprog = glium::Program::from_source(
        &display, clearvert, clearfrag, None
    ).unwrap();

    let vxgi1vert = load_file_str(Path::new(concatcp!(SRC_DIR, "shaders/vxgi1.vert"))).unwrap();
    let vxgi1frag = load_file_str(Path::new(concatcp!(SRC_DIR, "shaders/vxgi1.frag"))).unwrap();
    let vxgi1geom = load_file_str(Path::new(concatcp!(SRC_DIR, "shaders/vxgi1.geom"))).unwrap();
    let vxgi1prog = glium::Program::from_source(
        &display, &vxgi1vert, &vxgi1frag, None
    ).unwrap();
    let vxgi1prog_geom = glium::Program::from_source(
        &display, &vxgi1vert, &vxgi1frag, Some(&vxgi1geom)
    ).unwrap();

    let mipmapcomp = load_file_str(Path::new(concatcp!(SRC_DIR, "shaders/mipmap.glsl"))).unwrap();
    let mipmapprog = glium::program::ComputeShader::from_source(&display, mipmapcomp).unwrap();
    
    let t = std::time::Instant::now();
    let mut dtclock = std::time::Instant::now();

    let mut imunit_behav: glium::uniforms::ImageUnitBehavior = Default::default();
    imunit_behav.access = ImageUnitAccess::ReadWrite;
    imunit_behav.level = 1;
    imunit_behav.format = ImageUnitFormat::RGBA16F;
    let imunit_behav = imunit_behav;

    let voxelgrid1 = glium::texture::texture3d::Texture3d::empty_with_format(&display, glium::texture::UncompressedFloatFormat::F16F16F16F16, glium::texture::MipmapsOption::EmptyMipmapsMax(miplvl.into()), gwidth.into(), gwidth.into(), gwidth.into()).unwrap();
    let voxelgrid2 = glium::texture::texture3d::Texture3d::empty_with_format(&display, glium::texture::UncompressedFloatFormat::F16F16F16F16, glium::texture::MipmapsOption::EmptyMipmapsMax(miplvl.into()), gwidth.into(), gwidth.into(), gwidth.into()).unwrap();

    let mut framebuffer = glium::framebuffer::EmptyFrameBuffer::new(
        &display,
        gwidth.into(),
        gwidth.into(),
        None, None, false
    ).unwrap();

    let mut camera_pos = glm::vec3(0., 0., 0.);
    let mut camera_dir = glm::vec3(0., 0., -1.);
    let camera_up = glm::vec3(0., 1., 0.);
    let mut lpos = glm::vec3::<f32>(0., 0., 0.);

    let mut kd: f32 = 1.0;
    let mut kid: f32 = 1.0;
    let mut ks: f32 = 0.3;
    let mut kis: f32 = 0.3;

    let mut model_rot = 0.0;

    let mut draw_grid = false;
    let mut draw_voxelization_camera = false;

    let mut key_w = false;
    let mut key_a = false;
    let mut key_s = false;
    let mut key_d = false;
    let mut key_shift = false;
    let mut key_space = false;
    let mut key_up = false;
    let mut key_down = false;
    let mut key_left = false;
    let mut key_right = false;
    let mut key_i = false;
    let mut key_j = false;
    let mut key_k = false;
    let mut key_l = false;
    let mut key_n = false;
    let mut key_m = false;

    let mut speed = 0.5;

    let mut enabled = false;
    let mut only_occ = false;
    let mut eid = true;
    let mut eis = true;
    let mut ed = true;

    let mut frame_counter = 0;

    event_loop.run(move |event, target| {
        
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        // *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        // *control_flow = glutin::event_loop::ControlFlow::Poll;
        match event {
            winit::event::Event::WindowEvent { event, .. } => {
                use event::WindowEvent;
                match event {
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                        target.exit();
                    },
                    WindowEvent::RedrawRequested => {
                        
                        let dt = dtclock.elapsed().as_secs_f32();
                        dtclock = std::time::Instant::now();

                        let cwpos = ((voxelmatrix*glm::inverse(&m)*glm::vec4(camera_pos.x, camera_pos.y, camera_pos.z, 1.0)).xyz() + 
                                     glm::vec3(1., 1., 1.))/2.;
                        
                        let repaint_after = egui_glium.run(&window, |egui_ctx| {
                            egui::SidePanel::left("Controls!").show(egui_ctx, |ui| {
                                ui.heading(format!("FPS: {}", 1./dt));
                                ui.label(format!("Light pos:"));
                                ui.label(format!("x: {x}, y: {y}, z: {z}",
                                                 x = lpos.x,
                                                 y = lpos.y,
                                                 z = lpos.z)
                                );
                                ui.label(format!("Camera pos:"));
                                ui.label(format!("x: {x}, y: {y}, z: {z}",
                                                 x = cwpos.x,
                                                 y = cwpos.y,
                                                 z = cwpos.z)
                                );
                                ui.add(egui::Slider::new(&mut kd, 0.0..=2.0).text("kd"));
                                ui.add(egui::Slider::new(&mut ks, 0.0..=2.0).text("ks"));
                                ui.add(egui::Slider::new(&mut kid, 0.0..=2.0).text("kid"));
                                ui.add(egui::Slider::new(&mut kis, 0.0..=2.0).text("kis"));
                                ui.toggle_value(&mut only_occ, "Only occlusion");
                                ui.toggle_value(&mut eid, "Indirect Diffuse");
                                ui.toggle_value(&mut eis, "Indirect Specular");
                                ui.toggle_value(&mut ed, "Direct");
                                ui.add(egui::Slider::new(&mut speed, 0.0..=5.0).text("speed"));
                            });
                        });
                        
                        let voxelview = glm::Mat4::identity(); 
                        let voxelproj = glm::Mat4::identity(); 
                        // let voxelgrid = glium::texture::Texture3d::empty_with_format(&display, glium::texture::UncompressedFloatFormat::F32F32F32F32, glium::texture::MipmapsOption::NoMipmap, GWIDTH.into(), GWIDTH.into(), GWIDTH.into()).unwrap();
                        // let normalmat = glm::inverse_transpose(m.ad_solve_upper_triangular_mut);

                        let matrix = glm::translation::<f32>(
                            &glm::vec3(0., 0., -3.)
                        )*glm::rotation::<f32>(
                            t.elapsed().as_secs_f32(), &glm::vec3(0., 1., 0.),
                        );
                        let matrix = glm::perspective::<f32>(
                            1.,
                            glm::pi::<f32>()/4.0,
                            0.1,
                            100.
                        )*voxelview*m;
                        let matrix = voxelproj*voxelview*m;
                        let matrix = glm::Mat4::identity();

                        // clear voxel
                        for i in 0..i32::from(gwidth) {
                            framebuffer.draw(
                                &fullscreen_rect,
                                fullscreen_ind,
                                &clearprog,
                                &uniform! {
                                    grid: voxelgrid1.image_unit(
                                        glium::uniforms::ImageUnitFormat::RGBA16F
                                    ).unwrap().set_access(
                                        glium::uniforms::ImageUnitAccess::Write
                                    ),
                                    depth: i,
                                },
                                &Default::default()
                            ).unwrap();

                            framebuffer.draw(
                                &fullscreen_rect,
                                fullscreen_ind,
                                &clearprog,
                                &uniform! {
                                    grid: voxelgrid2.image_unit(
                                        glium::uniforms::ImageUnitFormat::RGBA16F
                                    ).unwrap().set_access(
                                        glium::uniforms::ImageUnitAccess::Write
                                    ),
                                    depth: 1,
                                },
                                &Default::default()
                            );
                        }


                        // voxelize
                        let normalmat = itmat(&(voxelview*m));
                        for model in &models {
                            framebuffer
                                .draw(
                                    &model.vbo,
                                    &model.ibo,
                                    &program,
                                    &uniform! {
                                        M: *voxelmatrix.as_ref(),
                                        VNM: *(normalmat).as_ref(),
                                        grid: voxelgrid1.image_unit(
                                            glium::uniforms::ImageUnitFormat::RGBA16F
                                        ).unwrap().set_access(
                                            glium::uniforms::ImageUnitAccess::Write
                                        ),
                                        GWIDTH: gwidth,
                                        cameraworldpos: *cwpos.as_ref(),
                                        image: glium::uniforms::Sampler::new(&model.material.kd),
                                        lpos: *lpos.as_ref()
                                    },
                                    &Default::default(),
                                ).unwrap();
                        }

                        generate_mipmap(&voxelgrid1, &mipmapprog, gwidth, miplvl, frame_counter);

                        // voxelize direct illumination
                        {
                            let vnormalmat = itmat(&(voxelview*voxelmatrix));
                            let rnormalmat = itmat(&(voxelview*m));
                            for model in &models {
                                framebuffer.draw(
                                    &model.vbo,
                                    &model.ibo,
                                    &vxgi1prog_geom,
                                    &uniform! {
                                        M: *voxelmatrix.as_ref(),
                                        V: *(voxelview).as_ref(),
                                        P: *voxelproj.as_ref(),
                                        VP: *voxelproj.as_ref(),
                                        VV: *voxelview.as_ref(),
                                        VM: *voxelmatrix.as_ref(),
                                        VNM: *vnormalmat.as_ref(),
                                        RNM: *rnormalmat.as_ref(),
                                        grid: glium::uniforms::Sampler::new(&voxelgrid1)
                                            .minify_filter(glium::uniforms::MinifySamplerFilter::LinearMipmapLinear)
                                            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear)
                                            .wrap_function(glium::uniforms::SamplerWrapFunction::BorderClamp),
                                        GWIDTH: gwidth,
                                        cameraworldpos: *(camera_pos).as_ref(),
                                        enabled: true,
                                        lpos: *lpos.as_ref(),
                                        kd: glium::uniforms::Sampler::new(&model.material.kd),
                                        ks: glium::uniforms::Sampler::new(&model.material.ks),
                                        shininess: model.shininess,
                                        only_occ: true,
                                        enable_indspec: false,
                                        enable_inddiff: false,
                                        enable_dir: true,
                                        write_vox: true,
                                        wgrid: voxelgrid2.image_unit(
                                            glium::uniforms::ImageUnitFormat::RGBA16F
                                        ).unwrap().set_access(
                                            glium::uniforms::ImageUnitAccess::Write
                                        )
                                    },
                                    &Default::default()
                                ).unwrap();
                            }
                        }

                        generate_mipmap(&voxelgrid2, &mipmapprog, gwidth, miplvl, frame_counter);

                        let mut target = display.draw();

                        target.clear_color(0.0, 0.0, 0.0, 1.0);
                        target.clear_depth(1.0);

                        let view = glm::look_at::<f32>(
                            &camera_pos, &(camera_pos+camera_dir), &glm::vec3(0., 1., 0.)
                        );
                        let pers = glm::perspective(1., glm::pi::<f32>()/4., 0.01, 100.);
                        let model_rot_mat = glm::rotation::<f32>(model_rot, &glm::vec3(0., 1., 0.));

                        // draw scene
                        if !draw_grid && !draw_voxelization_camera {
                            let vnormalmat = itmat(&(voxelview*voxelmatrix));
                            let rnormalmat = itmat(&(voxelview*m));

                            for model in &models {
                                target.draw(
                                    &model.vbo,
                                    &model.ibo,
                                    &vxgi1prog,
                                    &uniform! {
                                        M: *m.as_ref(),
                                        V: *view.as_ref(),
                                        P: *pers.as_ref(),
                                        VP: *voxelproj.as_ref(),
                                        VV: *voxelview.as_ref(),
                                        VM: *voxelmatrix.as_ref(),
                                        VNM: *vnormalmat.as_ref(),
                                        RNM: *rnormalmat.as_ref(),
                                        grid: glium::uniforms::Sampler::new(&voxelgrid2)
                                            .minify_filter(glium::uniforms::MinifySamplerFilter::LinearMipmapLinear)
                                            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear)
                                            .wrap_function(glium::uniforms::SamplerWrapFunction::BorderClamp),
                                        GWIDTH: gwidth,
                                        cameraworldpos: *cwpos.as_ref(),
                                        enabled: enabled,
                                        lpos: *lpos.as_ref(),
                                        kd: glium::uniforms::Sampler::new(&model.material.kd),
                                        ks: glium::uniforms::Sampler::new(&model.material.ks),
                                        shininess: model.shininess,
                                        only_occ: only_occ,
                                        enable_indspec: eis,
                                        enable_inddiff: eid,
                                        enable_dir: ed,
                                        write_vox: false
                                    },
                                    &glium::DrawParameters {
                                        depth: glium::Depth {
                                            test: glium::draw_parameters::DepthTest::IfLess,
                                            write: true,
                                            ..Default::default()
                                        },
                                        blend: glium::Blend::alpha_blending(),
                                        ..Default::default()
                                    }
                                ).unwrap();
                            }
                        }

                        egui_glium.paint(&display, &mut target);

                        target.finish().unwrap();               

                        let angspeed = glm::pi::<f32>()/2.;
                        if key_w {
                            camera_pos += camera_dir*speed*dt;
                        }
                        if key_s {
                            camera_pos -= camera_dir*speed*dt;
                        }
                        if key_a {
                            camera_pos -= glm::cross(&camera_dir, &camera_up)*speed*dt;
                        }
                        if key_d {
                            camera_pos += glm::cross(&camera_dir, &camera_up)*speed*dt;
                        }
                        if key_shift {
                            camera_pos -= camera_up*speed*dt;
                        }
                        if key_space {
                            camera_pos += camera_up*speed*dt;
                        }
                        if key_up {
                            camera_dir = glm::rotate_vec3(
                                &camera_dir,
                                angspeed*dt,
                                &glm::cross(&camera_dir, &camera_up)
                            )
                        }
                        if key_down {
                            camera_dir = glm::rotate_vec3(
                                &camera_dir,
                                -angspeed*dt,
                                &glm::cross(&camera_dir, &camera_up)
                            )
                        }
                        if key_left {
                            camera_dir = glm::rotate_vec3(
                                &camera_dir,
                                angspeed*dt,
                                &camera_up
                            )
                        }
                        if key_right {
                            camera_dir = glm::rotate_vec3(
                                &camera_dir,
                                -angspeed*dt,
                                &camera_up
                            )
                        }
                        if key_i {
                            lpos.y += 0.1*dt;
                        }
                        if key_j {
                            lpos.x += 0.1*dt;
                        }
                        if key_k {
                            lpos.y -= 0.1*dt;
                        }
                        if key_l {
                            lpos.x -= 0.1*dt;
                        }
                        if key_n {
                            lpos.z += 0.1*dt;
                        }
                        if key_m {
                            lpos.z -= 0.1*dt;
                        }

                        frame_counter += 1;
                    }
                    WindowEvent::Resized(new_size) => {
                        display.resize(new_size.into());
                    }
                    WindowEvent::KeyboardInput {ref event, ..} => {
                        use winit::keyboard::{Key, NamedKey};
                        let pressed = event.state == ElementState::Pressed;

                        if let Key::Character(ref c) = event.logical_key {
                            if c == "w" {
                                key_w = pressed
                            }
                            if c == "s" {
                                key_s = pressed;
                            }
                            if c == "a" {
                                key_a = pressed;
                            }
                            if c == "d" {
                                key_d = pressed;
                            }
                            if c == "i" {
                                key_i = pressed;
                            }
                            if c == "j" {
                                key_j = pressed;
                            }
                            if c == "k" {
                                key_k = pressed;
                            }
                            if c == "l" {
                                key_l = pressed;
                            }
                            if c == "n" {
                                key_n = pressed;
                            }
                            if c == "m" {
                                key_m = pressed;
                            }
                            if c == "t" {
                                if pressed {
                                    draw_grid = !draw_grid;
                                }
                            }
                            if c == "v" {
                                if pressed {
                                    draw_voxelization_camera = !draw_voxelization_camera
                                }
                            }
                            if c == "c" {
                                if pressed {
                                    enabled = !enabled;
                                }
                            }
                            if c == "p" {
                                model_rot += 0.1;
                            }
                            if c == "o" {
                                model_rot += -0.1;
                            }
                        }
                        match event.logical_key {
                            Key::Named(NamedKey::Space) => {
                                key_space = pressed;
                            }
                            Key::Named(NamedKey::Shift) => {
                                key_shift = pressed;
                            }
                            Key::Named(NamedKey::ArrowUp) => {
                                key_up = pressed;
                            }
                            Key::Named(NamedKey::ArrowDown) => {
                                key_down = pressed;
                            }
                            Key::Named(NamedKey::ArrowLeft) => {
                                key_left = pressed;
                            }
                            Key::Named(NamedKey::ArrowRight) => {
                                key_right = pressed;
                            }
                            _ => ()
                        }
                        
                    }
                    _ => {},
                }

                let event_response = egui_glium.on_event(&window, &event);
                if event_response.repaint {
                    window.request_redraw();
                }
            },
            _ => (),
        }

        

        // println!("{} {}", draw_grid, draw_voxelization_camera);
    });
}
