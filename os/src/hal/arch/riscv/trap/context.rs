use riscv::register::sstatus::{read, Sstatus, SPP};

// INFO: 因为实现浮点寄存器需要修改整个汇编代码，所以暂时注释掉

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct GeneralRegs {
    pub pc: usize,
    pub ra: usize,
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub s0: usize,
    pub s1: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
}
//
// #[repr(C)]
// #[derive(Debug, Default, Clone, Copy)]
// pub struct FloatRegs {
//     pub f: [usize; 32],
//     pub fcsr: usize,
// }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TrapContext {
    pub general_regs: GeneralRegs,
    // pub float_regs: FloatRegs,
    pub sstatus: Sstatus,
    pub sepc: usize,
    pub kernel_satp: usize,
    pub kernel_sp: usize,
    pub trap_handler: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.general_regs.sp = sp;
    }

    pub fn app_init_context(
        entry: usize,
        sp: usize,
        kernel_satp: usize,
        kernel_sp: usize,
        trap_handler: usize,
    ) -> Self {
        let mut sstatus = read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            general_regs: GeneralRegs::default(),
            // float_regs: FloatRegs::default(),
            sstatus,
            sepc: entry,
            kernel_satp,
            kernel_sp,
            trap_handler,
        };
        cx.set_sp(sp);
        cx
    }
}
