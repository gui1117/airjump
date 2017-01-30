extern crate piston;
extern crate glium_graphics;
extern crate toml;
extern crate graphics;
extern crate rustc_serialize;
extern crate glium;
extern crate svgparser;
#[macro_use] extern crate lazy_static;

mod configuration;
pub mod math;
mod error;
mod app;
mod map;
mod physics;

use error::*;
use configuration::CFG;

use piston::window::{WindowSettings, AdvancedWindow};
use piston::event_loop::EventLoop;
use piston::input::Event::*;
use piston::input::Input;
use glium_graphics::{GliumWindow, OpenGL, Glium2d};

fn main() {
    safe_main().ok_or_exit();
}

fn safe_main() -> AirjumpResult<()> {
    //TODO maybe let that being configuration
    let opengl = OpenGL::V3_2;

    let mut window: GliumWindow = WindowSettings::new(
        "airjump", CFG.window.dimensions)
        .exit_on_esc(true)
        .opengl(opengl)
        .vsync(CFG.window.vsync)
        .fullscreen(CFG.window.fullscreen)
        .samples(CFG.window.samples)
        .build()
        .map_err(|e| AirjumpError::GliumWindow(e))?;

    window = window.capture_cursor(true)
        .ups(CFG.event_loop.ups)
        .max_fps(CFG.event_loop.max_fps);


    let mut app = app::App::new()?;
    let mut gl = Glium2d::new(opengl, &window);
    while let Some(e) = window.next() {
        match e {
            Render(args) => {
                let mut target = window.draw();
                gl.draw(&mut target, args.viewport(), |context, frame| {
                    app.render(context, frame);
                });
                target.finish().unwrap();
            },
            AfterRender(_args) => (),
            Update(args) => app.update(args.dt),
            Idle(_args) => (),
            Input(Input::Press(button)) => app.press(button),
            Input(Input::Release(button)) => app.release(button),
            Input(Input::Move(motion)) => app.do_move(motion),
            Input(Input::Close) => break,
            Input(Input::Resize(w, h)) => app.resize(w, h),
            Input(_) => (),
        }
    }
    Ok(())
}
