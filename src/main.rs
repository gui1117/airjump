extern crate fps_clock;
extern crate rustc_serialize;
#[macro_use] extern crate glium;
#[macro_use] extern crate lazy_static;

mod spatial_hashing;
mod configuration;
mod audio;
pub mod math;
#[cfg(target_os = "emscritpen")]
#[path="emscripten_audio.rs"]
mod audio;
#[cfg(all(not(target_os = "emscripten")))]
#[path="rodio_audio.rs"]
mod audio;
mod app;
mod map;
mod physics;
pub mod backtrace_hack;
pub mod graphics;

use configuration::CFG;

use glium::{glutin, DisplayBuild};

pub trait OkOrExit {
    type OkType;
    fn ok_or_exit(self) -> Self::OkType;
}
impl<T, E: ::std::fmt::Display> OkOrExit for Result<T,E> {
    type OkType = T;
    fn ok_or_exit(self) -> T {
        match self {
            Ok(t) => t,
            Err(err) => {
                println!("ERROR: {}", err);
                ::std::process::exit(1);
            },
        }
    }
}

fn main() {
    safe_main().ok_or_exit();
}

fn safe_main() -> Result<(), String> {
    let mut builder = glutin::WindowBuilder::new()
        .with_multitouch()
        .with_title("airjump");

    if CFG.window.fullscreen {
        builder = builder.with_fullscreen(glutin::get_primary_monitor());
    } else {
        let width = CFG.window.dimensions[0];
        let height = CFG.window.dimensions[1];
        builder = builder.with_dimensions(width, height);
    }
    if CFG.window.vsync {
        builder = builder.with_vsync();
    }
    if CFG.window.samples > 0 && CFG.window.samples.is_power_of_two() {
        builder = builder.with_multisampling(CFG.window.samples as u16);
    } else {
        panic!("multisampling invalid");
    }

    let window = builder.build_glium().unwrap();

    let (w, h) = window.get_window().unwrap().get_inner_size_points().unwrap();
    let mut cursor = [w as f64/2., h as f64/2.];
    window.get_window().unwrap().set_cursor_position(cursor[0] as i32, cursor[1] as i32).unwrap();
    window.get_window().unwrap().set_cursor_state(glutin::CursorState::Hide).unwrap();
    let mut graphics = graphics::Graphics::new(&window).unwrap();

    let audio = audio::Audio::new().unwrap();

    let mut app = app::App::new(audio);

    // Main loop
    //
    // If running out of time then slow down the game

    let mut fps_clock = fps_clock::FpsClock::new(CFG.event_loop.max_fps);
    let dt = 1.0 / CFG.event_loop.max_fps as f64;


    'main_loop: loop {


        for event in window.poll_events() {
            use glium::glutin::Event::*;
            use glium::glutin::ElementState;
            use glium::glutin::MouseButton;
            use glium::glutin::TouchPhase;
            match event {
                Closed => break 'main_loop,
                MouseMoved(x, y) => {
                    let (w, h) = window.get_window().unwrap().get_inner_size_points().unwrap();
                    if let Ok(()) = window.get_window().unwrap().set_cursor_position((w/2) as i32, (h/2) as i32) {

                        let dx = x - (w/2) as i32;
                        let dy = y - (h/2) as i32;

                        if dx != 0 || dy != 0 {
                            cursor[0] += dx as f64 * CFG.control.mouse_sensibility;
                            cursor[1] += -dy as f64 * CFG.control.mouse_sensibility;

                            let ratio = w as f64 / h as f64;
                            cursor[0] = f64::max(-1., f64::min(cursor[0], 1.));
                            cursor[1] = f64::max(-1./ratio, f64::min(cursor[1], 1./ratio));
                            app.set_jump_angle(cursor[1].atan2(cursor[0]) + ::std::f64::consts::PI);
                        }
                    }
                },
                Touch(touch) => {
                    if touch.phase == TouchPhase::Started {
                        println!("x: {}, y: {}", touch.location.0, touch.location.1);
                        // let (w, h) = window.get_window().unwrap().get_inner_size_points().unwrap();
                    }
                },
                MouseInput(ElementState::Pressed, MouseButton::Left) => app.do_jump(),
                Refresh => {
                    let mut target = window.draw();
                    {
                        let camera = app.camera();
                        let mut frame = graphics::Frame::new(&mut graphics, &mut target, &camera);
                        frame.clear();
                        app.draw(&mut frame);
                        draw_cursor(cursor, &mut frame);
                    }
                    target.finish().unwrap();
                }
                _ => (),
            }
        }

        app.update(dt);

        let mut target = window.draw();
        {
            let camera = app.camera();
            let mut frame = graphics::Frame::new(&mut graphics, &mut target, &camera);
            frame.clear();
            app.draw(&mut frame);
            draw_cursor(cursor, &mut frame);
        }
        target.finish().unwrap();

        if app.must_quit {
            break 'main_loop;
        }

        fps_clock.tick();
    }

    Ok(())
}

fn draw_cursor(cursor: [f64; 2], frame: &mut graphics::Frame) {
    use graphics::{self, Layer, Transformed};
    let (w, h) = frame.size();

    let unit = f32::min(w as f32, h as f32);

    let color = CFG.graphics.cursor_color;
    let half_width = (CFG.graphics.cursor_outer_radius - CFG.graphics.cursor_inner_radius)/2. * unit;
    let half_height = CFG.graphics.cursor_thickness/2. * unit;
    let delta = CFG.graphics.cursor_inner_radius * unit + half_width;

    let transform = graphics::Transformation::identity()
        .translate(cursor[0] as f32, cursor[1] as f32)
        .translate(delta, 0.)
        .scale(half_width, half_height);
    frame.draw_quad(transform, Layer::Billboard, color);

    let transform = graphics::Transformation::identity()
        .translate(cursor[0] as f32, cursor[1] as f32)
        .translate(-delta, 0.)
        .scale(half_width, half_height);
    frame.draw_quad(transform, Layer::Billboard, color);

    let transform = graphics::Transformation::identity()
        .translate(cursor[0] as f32, cursor[1] as f32)
        .translate(0., delta)
        .scale(half_height, half_width);
    frame.draw_quad(transform, Layer::Billboard, color);

    let transform = graphics::Transformation::identity()
        .translate(cursor[0] as f32, cursor[1] as f32)
        .translate(0., -delta)
        .scale(half_height, half_width);
    frame.draw_quad(transform, Layer::Billboard, color);
}
