use configuration::CFG;

extern "C" { fn emscripten_asm_const(code: *const ::std::os::raw::c_char); }

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
        unsafe {
            let vol = CFG.audio.jump_volume
            emscripten_asm_const(format!(b"play_jump({})", vol) as *const u8);
        }
    }

    pub fn play_wall(&self, vol: f32) {
        unsafe {
            let vol = vol*CFG.audio.wall_volume
            emscripten_asm_const(format!(b"play_wall({})", vol) as *const u8);
        }
    }
}
