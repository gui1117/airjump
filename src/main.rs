extern crate piston;
extern crate glium_graphics;
extern crate toml;
extern crate graphics;
extern crate rustc_serialize;
extern crate glium;
extern crate glutin;
extern crate svgparser;
extern crate fnv;
#[macro_use] extern crate lazy_static;

mod spatial_hashing;
mod configuration;
pub mod math;
mod app;
mod map;
mod physics;

use configuration::CFG;

use piston::window::{WindowSettings, AdvancedWindow};
use piston::event_loop::EventLoop;
use piston::input::Input;
use glium_graphics::{GliumWindow, OpenGL, Glium2d};

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
    //TODO maybe let that being configuration
    let opengl = OpenGL::V3_2;

    // set monitor dimension instead of configuration in fullscreen
    let dimensions = if CFG.window.fullscreen {
        let (w, h) = glutin::get_primary_monitor().get_dimensions();
        [w, h]
    } else {
        CFG.window.dimensions
    };

    let mut window: GliumWindow = WindowSettings::new(
        "airjump", dimensions)
        .exit_on_esc(true)
        .opengl(opengl)
        .vsync(CFG.window.vsync)
        .fullscreen(CFG.window.fullscreen)
        .samples(CFG.window.samples)
        .build()?;

    window = window.capture_cursor(true)
        .ups(CFG.event_loop.ups)
        .max_fps(CFG.event_loop.max_fps);


    let mut app = app::App::new();
    let mut gl = Glium2d::new(opengl, &window);
    while let Some(e) = window.next() {
        match e {
            Input::Render(args) => {
                let mut target = window.draw();
                gl.draw(&mut target, args.viewport(), |context, frame| {
                    app.render(context, frame);
                });
                target.finish().unwrap();
            },
            Input::AfterRender(_args) => (),
            Input::Update(args) => app.update(args.dt),
            Input::Idle(_args) => (),
            Input::Press(button) => app.press(button),
            Input::Release(button) => app.release(button),
            Input::Move(motion) => app.do_move(motion),
            Input::Resize(w, h) => app.resize(w, h),
            Input::Close(..) => break,
            Input::Text(..) | Input::Cursor(..) | Input::Focus(..) | Input::Custom(..) => (),
        }
    }
    Ok(())
}
