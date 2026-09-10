pub mod flags;
pub use flags::Flags;

pub mod registers;
pub use registers::RegisterFile;

pub struct Cpu {
    registers: RegisterFile,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: RegisterFile::new(),
        }
    }
}
