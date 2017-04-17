extern crate svgparser;

use self::svgparser::svg;
use physics::{Body, Shape};
use OkOrExit;

pub struct Map {
    pub bodies: Vec<Body>,
    pub start: [f64; 2],
}

const MAP_FILE: &'static str = "map.svg";

enum Error {
    Io(::std::io::Error),
    Svg(svgparser::Error),
}
impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Error {
        Error::Io(err)
    }
}
impl From<svgparser::Error> for Error {
    fn from(err: svgparser::Error) -> Error {
        Error::Svg(err)
    }
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        use self::Error::*;
        match *self {
            Io(ref e) => write!(fmt, "file `{}`: io error: {}", MAP_FILE, e),
            Svg(ref e) => write!(fmt, "file `{}`: svg parser error: {:?}", MAP_FILE, e),
        }
    }
}

#[cfg(not(feature = "include_all"))]
fn read_map_file() -> Result<Vec<u8>, Error> {
    use std::fs::File;
    use std::io::Read;
    let mut map = Vec::new();
    File::open(MAP_FILE)?.read_to_end(&mut map)?;
    Ok(map)
}

#[cfg(feature = "include_all")]
fn read_map_file() -> Result<&'static [u8], Error> {
    Ok(include_bytes!("../map.svg"))
}

fn load_map() -> Result<Map, Error> {
    let text = read_map_file()?;

    let mut parser = svg::Tokenizer::new(&text);

    let mut bodies = Vec::new();

    let mut start = None;

    // bool is whereas it is start and f64 are cx, cy, r
    let mut circle: Option<(bool, Option<f64>,Option<f64>,Option<f64>)> = None;

    // f64 are x, y, width, height
    let mut rect: Option<(Option<f64>,Option<f64>,Option<f64>,Option<f64>)> = None;

    loop {
        match parser.parse_next()? {
            svg::Token::ElementStart(name) => {
                if name == b"circle" {
                    circle = Some((false, None, None, None));
                } else if name == b"rect" {
                    rect = Some((None, None, None, None));
                }
            },
            svg::Token::ElementEnd(_) => {
                if let Some(circle) = circle.take() {
                    match circle {
                        (true, Some(x), Some(y), _) => {
                            if start.is_some() {
                                println!("WARGNING: svg map redefinition of start");
                            }
                            start = Some([x, y]);
                        }
                        (false, Some(x), Some(y), Some(r)) => bodies.push(Body {
                            pos: [x, y],
                            shape: Shape::Circle(r),
                        }),
                        _ => println!("WARGNING: svg map incomplete circle definition"),
                    }
                } else if let Some(rect) = rect.take() {
                    match rect {
                        (Some(x), Some(y), Some(w), Some(h)) => bodies.push(Body {
                            pos: [x+w/2., y-h/2.],
                            shape: Shape::Rectangle(w, h),
                        }),
                        _ => println!("WARGNING: svg map incomplete circle definition"),
                    }
                }
            },
            svg::Token::Attribute(b"id", value) => {
                if let Some(ref mut circle) = circle {
                    let d = b"start";
                    circle.0 = value.slice_next_raw(d.len()) == d;
                }
            },
            svg::Token::Attribute(b"cx", mut value) => {
                if let Some(ref mut circle) = circle {
                    circle.1 = Some(value.parse_number().unwrap());
                }
            },
            svg::Token::Attribute(b"cy", mut value) => {
                if let Some(ref mut circle) = circle {
                    circle.2 = Some(-value.parse_number().unwrap());
                }
            },
            svg::Token::Attribute(b"r", mut value) => {
                if let Some(ref mut circle) = circle {
                    circle.3 = Some(value.parse_number().unwrap());
                }
            },
            svg::Token::Attribute(b"x", mut value) => {
                if let Some(ref mut rect) = rect {
                    rect.0 = Some(value.parse_number().unwrap());
                }
            },
            svg::Token::Attribute(b"y", mut value) => {
                if let Some(ref mut rect) = rect {
                    rect.1 = Some(-value.parse_number().unwrap());
                }
            },
            svg::Token::Attribute(b"width", mut value) => {
                if let Some(ref mut rect) = rect {
                    rect.2 = Some(value.parse_number().unwrap());
                }
            },
            svg::Token::Attribute(b"height", mut value) => {
                if let Some(ref mut rect) = rect {
                    rect.3 = Some(value.parse_number().unwrap());
                }
            },
            svg::Token::EndOfStream => break,
            _ => (),
        }
    }

    Ok(Map {
        bodies: bodies,
        start: start.unwrap_or_else(|| {
            println!("WARGNING: svg map incomplete circle definition");
            [0., 0.]
        }),
    })
}

lazy_static! {
    pub static ref MAP: Map = load_map().ok_or_exit();
}
