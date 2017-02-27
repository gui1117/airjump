extern crate fps_clock;
extern crate toml;
extern crate rustc_serialize;
extern crate svgparser;
extern crate fnv;
extern crate rusttype;
extern crate image;
extern crate vecmath;
#[macro_use] extern crate glium;
#[macro_use] extern crate conrod;
#[macro_use] extern crate lazy_static;

mod spatial_hashing;
mod configuration;
mod ui;
pub mod math;
mod app;
mod map;
mod physics;
pub mod graphics;

use configuration::CFG;

use glium::{glutin, DisplayBuild, Surface};

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

    let mut ui = ui::Ui::new(&window);
    let mut app = app::App::new(&window);

    // Main loop
    //
    // If running out of time then slow down the game

    let mut fps_clock = fps_clock::FpsClock::new(CFG.event_loop.max_fps);
    let dt = 1.0 / CFG.event_loop.max_fps as f64;

    'main_loop: loop {

        let events = window.poll_events().collect::<Vec<glium::glutin::Event>>();

        for event in &events {
            if let Some(event) = conrod::backend::winit::convert(event.clone(), &window) {
                ui.ui.handle_event(event);
            }
        }

        ui.update(&window);

        for event in events {
            use glium::glutin::Event::*;
            match event {
                Closed => break 'main_loop,
                Resized(w, h) => app.resize(w, h),
                _ => (),
            }
        }

        app.update(dt);

        let mut target = window.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        app.draw(&mut target);
        ui.draw(&window, &mut target);
        target.finish().unwrap();

        fps_clock.tick();
    }

    Ok(())
}
