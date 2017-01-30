use toml;
use rustc_serialize::Decodable;
use std::fs::File;
use std::io::Read;
use error::*;

#[derive(RustcDecodable)]
pub struct Configuration {
    pub window: Window,
    pub gameplay: Gameplay,
    pub graphics: Graphics,
    pub camera: Camera,
    pub event_loop: EventLoop,
    pub control: Control,
}

#[derive(RustcDecodable)]
pub struct Control {
    pub mouse_sensibility: f64,
}
#[derive(RustcDecodable)]
pub struct Window {
    pub samples: u8,
    pub fullscreen: bool,
    pub vsync: bool,
    pub dimensions: [u32; 2],
}
#[derive(RustcDecodable)]
pub struct Gameplay {
    pub gravity: f64,
    pub ball_radius: f64,
    pub damping: f64,
    pub impulse: f64,
    pub reset: bool,
}
#[derive(RustcDecodable)]
pub struct Graphics {
    pub ball_color: [f32; 4],
    pub wall_color: [f32; 4],
    pub background_color: [f32; 4],
    pub cursor_color: [f32; 4],
    pub cursor_inner_radius: f64,
    pub cursor_outer_radius: f64,
    pub cursor_thickness: f64,
    pub effect_timer: f64,
    pub effect_color: [f32; 4],
    pub effect_thickness: f64,
}
#[derive(RustcDecodable)]
pub struct Camera {
    pub zoom: f64,
}
#[derive(RustcDecodable)]
pub struct EventLoop {
    pub ups: u64,
    pub max_fps: u64,
}

fn read_configuration_file() -> AirjumpResult<String> {
    let mut config = String::new();
    File::open("Config.toml")?.read_to_string(&mut config)?;
    Ok(config)
}

fn load_configuration() -> AirjumpResult<Configuration> {
    let config = read_configuration_file()?;
    let mut parser = toml::Parser::new(&config);
    let value = match parser.parse() {
        Some(p) => p,
        None => {
            let mut string = String::new();
            for error in parser.errors.iter() {
                let lo = parser.to_linecol(error.lo);
                let hi = parser.to_linecol(error.hi);
                string.push_str(&format!("\tline {} col {} to line {} col {}: {}", lo.0, lo.1, hi.0, hi.1, error.desc));
            }
            return Err(AirjumpError::TomlParser(string));
        },
    };
    Ok(Configuration::decode(&mut toml::Decoder::new(toml::Value::Table(value)))?)
}

lazy_static! {
    pub static ref CFG: Configuration = load_configuration().ok_or_exit();
}
