
pub struct MerrEra {
    bits: usize,
}

impl MerrEra {
    pub fn raw(&self) -> usize {
        self.bits
    }

    pub fn pc(&self) -> usize {
        self.bits
    }
}

impl From<usize> for MerrEra {
    fn from(value: usize) -> Self {
        Self { bits: value }
    }
}

#[inline(always)]
pub fn read() -> MerrEra {
    MerrEra {
        bits: unsafe {
            let bits: usize;
            core::arch::asm!("csrrd {}, {}", out(reg) bits, const 0x94);
            bits
        }
    }
}

pub fn write(pc: usize) {
    unsafe {
        core::arch::asm!("csrwr {}, {}", in(reg) pc, const 0x94);
    }
}