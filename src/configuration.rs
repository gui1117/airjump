extern crate toml;

use OkOrExit;

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct Control {
    pub mouse_sensibility: f64,
}
/// Those setting are not taking into account for emscripten backend
#[derive(Deserialize)]
pub struct Window {
    pub samples: u8,
    pub vsync: bool,
}
#[derive(Deserialize)]
pub struct Gameplay {
    pub gravity: f64,
    pub ball_radius: f64,
    pub damping: f64,
    pub impulse: f64,
    pub reset: bool,
}
#[derive(Deserialize)]
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
#[derive(Deserialize)]
pub struct Camera {
    pub zoom: f64,
}
#[derive(Deserialize)]
pub struct EventLoop {
    pub max_fps: u32,
}
#[derive(Deserialize)]
pub struct Physics {
    pub unit: f64,
}
#[derive(Deserialize)]
pub struct Audio {
    pub jump_volume: f32,
    pub wall_volume: f32,

    pub wall_max_intensity: f64,
    pub wall_min_intensity: f64,
}

const CONFIG_FILE: &'static str = "config.toml";

enum Error {
    Io(::std::io::Error),
    Toml(toml::de::Error),
}
impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Error {
        Error::Io(err)
    }
}
impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Error {
        Error::Toml(err)
    }
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        use self::Error::*;
        match *self {
            Io(ref e) => write!(fmt, "file `{}`: io error: {}", CONFIG_FILE, e),
            Toml(ref e) => write!(fmt, "file `{}`: toml decode error: {}", CONFIG_FILE, e),
        }
    }
}

#[cfg(feature = "exclude_all")]
fn read_configuration_file() -> Result<String, Error> {
    use std::fs::File;
    use std::io::Read;
    let mut config = String::new();
    File::open(CONFIG_FILE)?.read_to_string(&mut config)?;
    Ok(config)
}

#[cfg(not(feature = "exclude_all"))]
fn read_configuration_file() -> Result<&'static str, Error> {
    Ok(include_str!("../config.toml"))
}

fn load_configuration() -> Result<Configuration, Error> {
    let config = read_configuration_file()?;
    Ok(toml::from_str(&config)?)
}

lazy_static! {
    pub static ref CFG: Configuration = load_configuration().ok_or_exit();
}
