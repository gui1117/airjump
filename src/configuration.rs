extern crate toml;

use rustc_serialize::Decodable;
use OkOrExit;

#[derive(RustcDecodable)]
pub struct Configuration {
    pub window: Window,
    pub gameplay: Gameplay,
    pub graphics: Graphics,
    pub camera: Camera,
    pub event_loop: EventLoop,
    pub control: Control,
    pub physics: Physics,
    pub audio: Audio,
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
    pub cursor_inner_radius: f32,
    pub cursor_outer_radius: f32,
    pub cursor_thickness: f32,
    pub effect_timer: f64,
    pub effect_color: [f32; 4],
    pub effect_thickness: f32,
}
#[derive(RustcDecodable)]
pub struct Camera {
    pub zoom: f64,
}
#[derive(RustcDecodable)]
pub struct EventLoop {
    pub max_fps: u32,
}
#[derive(RustcDecodable)]
pub struct Physics {
    pub unit: f64,
}
#[derive(RustcDecodable)]
pub struct Audio {
    pub jump_volume: f32,
    pub wall_volume: f32,

    pub wall_max_intensity: f64,
    pub wall_min_intensity: f64,
}

const CONFIG_FILE: &'static str = "config.toml";

enum Error {
    Io(::std::io::Error),
    TomlParser(String),
    TomlDecode(toml::DecodeError),
}
impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Error {
        Error::Io(err)
    }
}
impl From<toml::DecodeError> for Error {
    fn from(err: toml::DecodeError) -> Error {
        Error::TomlDecode(err)
    }
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        use self::Error::*;
        match *self {
            Io(ref e) => write!(fmt, "file `{}`: io error: {}", CONFIG_FILE, e),
            TomlParser(ref e) => write!(fmt, "file `{}`: toml parser error:\n{}", CONFIG_FILE, e),
            TomlDecode(ref e) => write!(fmt, "file `{}`: toml decode error: {}", CONFIG_FILE, e),
        }
    }
}

#[cfg(not(feature = "include_all"))]
fn read_configuration_file() -> Result<String, Error> {
    use std::fs::File;
    use std::io::Read;
    let mut config = String::new();
    File::open(CONFIG_FILE)?.read_to_string(&mut config)?;
    Ok(config)
}

#[cfg(feature = "include_all")]
fn read_configuration_file() -> Result<&'static str, Error> {
    Ok(include_str!("../config.toml"))
}

fn load_configuration() -> Result<Configuration, Error> {
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
            return Err(Error::TomlParser(string));
        },
    };
    Ok(Configuration::decode(&mut toml::Decoder::new(toml::Value::Table(value)))?)
}

lazy_static! {
    pub static ref CFG: Configuration = load_configuration().ok_or_exit();
}
