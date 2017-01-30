#[derive(Debug)]
pub enum AirjumpError {
    Io(::std::io::Error),
    TomlParser(String),
    TomlDecode(::toml::DecodeError),
    GliumWindow(String),
    Svg(::svgparser::Error),
}
impl ::std::fmt::Display for AirjumpError {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        use self::AirjumpError::*;
        match *self {
            Io(ref e) => write!(fmt, "Io error: {}", e),
            TomlParser(ref e) => write!(fmt, "Toml parser error:\n{}", e),
            TomlDecode(ref e) => write!(fmt, "Toml decode error: {}", e),
            GliumWindow(ref e) => write!(fmt, "Glium window error: {}", e),
            Svg(ref e) => write!(fmt, "Svg parser error: {:?}", e),
        }
    }
}
impl From<::svgparser::Error> for AirjumpError {
    fn from(err: ::svgparser::Error) -> AirjumpError {
        AirjumpError::Svg(err)
    }
}
impl From<::toml::DecodeError> for AirjumpError {
    fn from(err: ::toml::DecodeError) -> AirjumpError {
        AirjumpError::TomlDecode(err)
    }
}
impl From<::std::io::Error> for AirjumpError {
    fn from(err: ::std::io::Error) -> AirjumpError {
        AirjumpError::Io(err)
    }
}
pub type AirjumpResult<T> = Result<T, AirjumpError>;

pub trait OkOrExit {
    type OkType;
    fn ok_or_exit(self) -> Self::OkType;
}
impl<T> OkOrExit for AirjumpResult<T> {
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
