// extern "C" { fn emscripten_asm_const(code: *const ::std::os::raw::c_char); }

pub struct Audio {
}

#[derive(Debug)]
pub enum Error {
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, _fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        Ok(())
    }
}

impl Audio {
    pub fn new() -> Result<Audio, Error> {
        Ok(Audio {})
    }

    pub fn play_jump(&self) {
    }

    pub fn play_wall(&self, _vol: f32) {
    }
}
