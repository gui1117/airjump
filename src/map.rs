use svgparser::{self, svg};
use physics::{Body, Shape};

pub struct Map {
    pub bodies: Vec<Body>,
    pub start: [f64; 2],
}

pub fn load_map(text: &[u8]) -> Result<Map, svgparser::Error> {
    //TODO assert and unwrap....
    let mut parser = svg::Tokenizer::new(&text);

    let mut bodies = Vec::new();

    let mut start = None;

    // bool is whereas it is start and array is cx, cy, rx, ry
    let mut circle: Option<(bool, [Option<f64>; 3])> = None;

    // array is x, y, width, height
    let mut rect: Option<[Option<f64>; 4]> = None;

    loop {
        match parser.parse_next()? {
            svg::Token::ElementStart(name) => {
                if name == b"circle" {
                    circle = Some((false, [None, None, None]));
                } else if name == b"rect" {
                    rect = Some([None, None, None, None]);
                }
            },
            svg::Token::ElementEnd(name) => {
                if circle.is_some() {
                    assert_eq!(name, svg::ElementEnd::Empty);
                    if circle.unwrap().0 {
                        assert!(start.is_none());
                        start = Some([circle.unwrap().1[0].unwrap(), circle.unwrap().1[1].unwrap()]);
                    } else {
                        bodies.push(Body {
                            pos: [circle.unwrap().1[0].unwrap(), circle.unwrap().1[1].unwrap()],
                            shape: Shape::Circle(circle.unwrap().1[2].unwrap()),
                        });
                    }
                    circle = None;
                } else if rect.is_some() {
                    assert_eq!(name, svg::ElementEnd::Empty);
                    bodies.push(Body {
                        pos: [
                            rect.unwrap()[0].unwrap()-rect.unwrap()[2].unwrap()/2.,
                            rect.unwrap()[1].unwrap()-rect.unwrap()[3].unwrap()/2.,
                        ],
                        shape: Shape::Rectangle(rect.unwrap()[2].unwrap(), rect.unwrap()[3].unwrap()),
                    });
                    rect = None;
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
                    circle.1[0] = Some(-value.parse_number().unwrap());
                }
            },
            svg::Token::Attribute(b"cy", mut value) => {
                if let Some(ref mut circle) = circle {
                    circle.1[1] = Some(-value.parse_number().unwrap());
                }
            },
            svg::Token::Attribute(b"r", mut value) => {
                if let Some(ref mut circle) = circle {
                    circle.1[2] = Some(value.parse_number().unwrap());
                }
            },
            svg::Token::Attribute(b"x", mut value) => {
                if let Some(ref mut rect) = rect {
                    rect[0] = Some(-value.parse_number().unwrap());
                }
            },
            svg::Token::Attribute(b"y", mut value) => {
                if let Some(ref mut rect) = rect {
                    rect[1] = Some(-value.parse_number().unwrap());
                }
            },
            svg::Token::Attribute(b"width", mut value) => {
                if let Some(ref mut rect) = rect {
                    rect[2] = Some(value.parse_number().unwrap());
                }
            },
            svg::Token::Attribute(b"height", mut value) => {
                if let Some(ref mut rect) = rect {
                    rect[3] = Some(value.parse_number().unwrap());
                }
            },
            svg::Token::EndOfStream => break,
            _ => (),
        }
    }

    Ok(Map {
        bodies: bodies,
        start: start.unwrap(),
    })
}

