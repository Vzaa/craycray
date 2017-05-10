#[macro_use]
extern crate clap;
extern crate sdl2;
extern crate rayon;
extern crate craycray;

use std::time::Instant;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use rayon::prelude::*;

use craycray::scene::Scene;

// Return true for quit
fn handle_events(scene: &mut Scene, event_pump: &mut EventPump) -> bool {
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

    let rel_mouse = event_pump.relative_mouse_state();

    let x_rot = rel_mouse.x() as f64 * (0.010);
    let y_rot = rel_mouse.y() as f64 * (0.010);
    scene.rot_camera(x_rot, y_rot);
    false
}

// A simple test code that uses SDL for rendering
fn main() {
    let matches = clap_app!(craycray =>
        (version: "0.1")
        (about: "craycray")
        (@arg RESOLUTION: -r --resolution +takes_value "Render resolution")
        (@arg FULLSCREEN: -f --fullscreen "Fullscreen")
    ).get_matches();


    let fullscreen = matches.is_present("FULLSCREEN");
    let resolution: u32 = matches
        .value_of("RESOLUTION")
        .unwrap_or("512")
        .parse()
        .unwrap_or(512);
    let res_u = resolution as usize;
    let mut scene = Scene::from_file("scene.json").unwrap();

    // Stuff from sdl2-rust example
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = if fullscreen {
        let display_mode = video_subsystem.desktop_display_mode(0).unwrap();
        video_subsystem
            .window("craycray", display_mode.w as u32, display_mode.h as u32)
            .fullscreen_desktop()
            .build()
            .unwrap()
    } else {
        video_subsystem
            .window("craycray", resolution, resolution)
            .position_centered()
            .build()
            .unwrap()
    };

    let (_, window_h) = window.size();

    let mut renderer = window.renderer().build().unwrap();

    let mut texture = renderer
        .create_texture_streaming(PixelFormatEnum::RGB24, resolution, resolution)
        .unwrap();
    sdl_context.mouse().show_cursor(false);
    sdl_context.mouse().set_relative_mouse_mode(true);

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut frame_cnt: u32 = 0;
    let mut time_stamp = Instant::now();
    loop {
        let quit = handle_events(&mut scene, &mut event_pump);
        if quit {
            break;
        }

        if frame_cnt == 10 {
            let elapsed = time_stamp.elapsed();
            let msecs = (elapsed.as_secs() * 1000) as u64 +
                        (elapsed.subsec_nanos() as u64 / 1000000);
            time_stamp = Instant::now();
            println!("FPS {}", (frame_cnt as f32) / (msecs as f32 / 1000.0));
            frame_cnt = 0;
        }

        scene.step();

        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                let pchunks = buffer.par_chunks_mut(pitch);
                pchunks
                    .enumerate()
                    .for_each(|(line_no, chunk)| {
                        for (idx, c) in scene
                            .line_iter_u8(res_u, res_u, line_no).enumerate() {
                                    chunk[idx * 3] = c.0;
                                    chunk[idx * 3 + 1] = c.1;
                                    chunk[idx * 3 + 2] = c.2;
                                }
                    });
            })
            .unwrap();
        renderer.clear();
        renderer
            .copy(&texture, None, Some(Rect::new(0, 0, window_h, window_h)))
            .unwrap();
        renderer.present();
        frame_cnt += 1;
    }
}
