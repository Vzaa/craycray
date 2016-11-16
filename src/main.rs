extern crate sdl2;
extern crate vecmath;
extern crate crossbeam;

mod vec3d;
mod light;
mod scene;
mod material;
mod color;
mod shape;
mod sphere;
mod plane;

use scene::Scene;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread;

// Return true for quit
fn handle_events(scene: &mut Scene,
                 resolution: u32,
                 event_pump: &mut EventPump,
                 x: i32,
                 y: i32)
                 -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return true,
            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                scene.mv_camera_fwd();
            }
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                scene.mv_camera_back();
            }
            _ => {}
        }
    }

    // TODO: Y rotation is incorrect
    let x_dist = ((resolution as i32) / 2) - x;
    let y_dist = ((resolution as i32) / 2) - y;
    let x_rot = x_dist as f64 * (-0.001);
    let y_rot = y_dist as f64 * (-0.001);
    scene.rot_camera(x_rot, y_rot);
    false
}

// Test scene
fn test_scene() -> Scene {
    use light::Light;
    use vec3d::Vec3d;
    use sphere::Sphere;
    use plane::Plane;

    let camera_pos: Vec3d = [0.0, 0.0, -100.0];

    let camera_dir: Vec3d = [0.0, 0.0, 1.0];
    let camera_up: Vec3d = [0.0, 1.0, 0.0];
    let light_pos: Vec3d = [50.0, 5.0, -100.0];

    let light = Light::new(light_pos, color::Color(0.6, 0.6, 0.6));
    let mut scene = Scene::new(light, camera_pos, camera_dir, camera_up);
    scene.add_shape(Box::new(Sphere::new([-20.0, -15.0, 45.0], 5.0, color::Color(1.0, 0.5, 0.5))));
    scene.add_shape(Box::new(Sphere::new([15.0, -10.0, 35.0], 5.0, color::Color(0.4, 0.4, 0.4))));
    scene.add_shape(Box::new(Sphere::new([0.0, -10.0, 25.0], 5.0, color::Color(0.5, 0.5, 1.0))));
    scene.add_shape(Box::new(Sphere::new([0.0, -120.0, 55.0], 100.0, color::WHITE)));
    scene.add_shape(Box::new(Sphere::new([0.0, 120.0, 55.0], 100.0, color::Color(1.0, 1.0, 0.0))));
    scene.add_shape(Box::new(Sphere::new([0.0, 0.0, 200.0], 100.0, color::Color(1.0, 0.5, 1.0))));
    scene.add_shape(Box::new(Sphere::from_material([15.0, 0.0, 40.0], 5.0, material::MIRROR)));
    scene.add_shape(Box::new(Sphere::new([-45.0, 0.0, 45.0], 20.0, color::Color(0.1, 0.1, 0.1))));

    scene.add_shape(Box::new(Plane::new([0.0, -200.0, 0.0], [0.0, 1.0, 0.0], color::Color(0.4, 0.4, 0.4))));
    scene.add_shape(Box::new(Plane::new([0.0, 200.0, 0.0], [0.0, -1.0, 0.0], color::Color(0.4, 0.4, 0.4))));
    scene.add_shape(Box::new(Plane::from_material([200.0, 0.0, 0.0], [-1.0, 0.0, 0.0], material::MIRROR)));
    scene.add_shape(Box::new(Plane::new([-200.0, 0.0, 0.0], [1.0, 0.0, 0.0], color::Color(0.8, 0.4, 0.8))));
    scene.add_shape(Box::new(Plane::new([0.0, 0.0, 200.0], [0.0, 0.0, -1.0], color::Color(0.4, 0.4, 0.4))));
    scene.add_shape(Box::new(Plane::new([0.0, 0.0, -200.0], [0.0, 0.0, 1.0], color::Color(0.4, 0.4, 0.4))));
    scene
}

// A simple test code that uses SDL for rendering
fn main() {
    let resolution: u32 = 512;
    let mut scene = test_scene();
    let thread_cnt = 4;

    // Stuff from sdl2-rust example
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Ray", resolution, resolution)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let mut texture =
        renderer.create_texture_streaming(PixelFormatEnum::RGB24, resolution, resolution)
            .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    loop {
        let (_, x, y) = sdl_context.mouse().mouse_state();
        let quit = handle_events(&mut scene, resolution, &mut event_pump, x, y);
        if quit {
            break;
        }

        scene.step();

        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                let sc = &scene;
                let res_u = resolution as usize;
                for y in (0..(res_u / thread_cnt) + 1).map(|x| x * thread_cnt) {
                    let threads = if y + thread_cnt <= res_u {
                        thread_cnt
                    } else {
                        res_u % thread_cnt
                    };

                    if threads == 0 {
                        break;
                    }

                    let mut lines : Vec<Vec<(u8, u8, u8)>> = Vec::new();
                    for _ in (0..threads) {
                        lines.push(Vec::new());
                    }

                    crossbeam::scope(|scope| {
                        for (thread_num, vec) in lines.iter_mut().enumerate() {
                            scope.spawn(move || {
                                sc.line_iter_u8(res_u, res_u, y + thread_num).fold((), |(), x| vec.push(x));
                            });
                        }
                    });

                    for (thread_num, vec) in lines.iter_mut().enumerate() {
                        let line_start = (thread_num + y) * pitch;
                        let line_end = line_start + res_u * 3;
                        let line_buf = &mut buffer[line_start..line_end];
                        for (c, offset) in vec.iter().zip((0..).map(|x| 3 * x)) {
                            line_buf[offset + 0] = c.0;
                            line_buf[offset + 1] = c.1;
                            line_buf[offset + 2] = c.2;
                        }
                    }
                }
            })
            .unwrap();
        renderer.clear();
        renderer.copy(&texture,
                  None,
                  Some(Rect::new(0, 0, resolution, resolution)))
            .unwrap();
        renderer.present();
    }
}
