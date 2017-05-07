extern crate fps_clock;
#[macro_use] extern crate glium;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate serde;

mod spatial_hashing;
mod configuration;
pub mod math;
#[cfg(target_os = "emscripten")]
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
#[cfg(target_os = "emscripten")]
pub mod emscripten;

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
    configure_fullscreen_strategy();
    let mut builder = glutin::WindowBuilder::new()
        .with_multitouch()
        .with_title("airjump");

    let (fullscreen, dimensions, vsync, samples) = if cfg!(target_os = "emscripten") {
        (
            false,
            None,
            true,
            2,
        )
    } else {
        (
            CFG.window.fullscreen,
            Some(CFG.window.dimensions),
            CFG.window.vsync,
            CFG.window.samples,
        )
    };

    if fullscreen {
        builder = builder.with_fullscreen(glutin::get_primary_monitor());
    } else if let Some(dimensions) = dimensions {
        let width = dimensions[0];
        let height = dimensions[1];
        builder = builder.with_dimensions(width, height);
    }
    if vsync {
        builder = builder.with_vsync();
    }
    if samples > 0 && samples.is_power_of_two() {
        builder = builder.with_multisampling(samples as u16);
    } else {
        panic!("multisampling invalid");
    }

    let window = builder.build_glium().map_err(|e| format!("build glium: {}", e))?;

    let (w, h) = window.get_window().unwrap().get_inner_size_points().unwrap();
    let mut cursor = [w as f64/2., h as f64/2.];
    window.get_window().unwrap().set_cursor_position(cursor[0] as i32, cursor[1] as i32).unwrap();
    window.get_window().unwrap().set_cursor_state(glutin::CursorState::Hide).unwrap();
    let mut graphics = graphics::Graphics::new(&window).map_err(|e| format!("graphics: {}", e))?;

    let audio = audio::Audio::new().map_err(|e| format!("audio: {}", e))?;

    let mut app = app::App::new(audio);

    // return whereas main loop breaks
    set_main_loop(|dt| -> bool {
        for event in window.poll_events() {
            use glium::glutin::Event::*;
            use glium::glutin::ElementState;
            use glium::glutin::MouseButton;
            use glium::glutin::TouchPhase;
            match event {
                Closed => return true,
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
                        let (w, h) = window.get_window().unwrap().get_inner_size_points().unwrap();
                        let x = touch.location.0 - (w/2) as f64;
                        let y = touch.location.1 - (h/2) as f64;
                        app.set_jump_angle(y.atan2(x) + ::std::f64::consts::PI);
                        app.do_jump();
                    }
                },
                MouseInput(ElementState::Pressed, MouseButton::Left) => app.do_jump(),
                MouseInput(ElementState::Pressed, MouseButton::Right) => app.do_unlimited_jump(),
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

        return app.must_quit
    });

    Ok(())
}

#[cfg(target_os = "emscripten")]
fn configure_fullscreen_strategy() {
    emscripten::request_soft_fullscreen_strategy();
}

#[cfg(not(target_os = "emscripten"))]
fn configure_fullscreen_strategy() {
}

#[cfg(target_os = "emscripten")]
fn set_main_loop<F: FnMut(f64) -> bool>(mut main_loop: F) {
    let dt = 1.0 / 60f64;
    emscripten::set_main_loop_callback(|| {
        if main_loop(dt) {
            emscripten::cancel_main_loop();
        }
    });
}

// behavior differ from emscripten as it doesn't return
// as long as the main loop doesn't end
#[cfg(all(not(target_os = "emscripten")))]
fn set_main_loop<F: FnMut(f64) -> bool>(mut main_loop: F) {
    // If running out of time then slow down the game
    let mut fps_clock = fps_clock::FpsClock::new(CFG.event_loop.max_fps);
    let dt = 1.0 / CFG.event_loop.max_fps as f64;
    loop {
        if main_loop(dt) {
            break
        }
        fps_clock.tick();
    }
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
