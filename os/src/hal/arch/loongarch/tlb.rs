use crate::hal::arch::loongarch::config::PALEN;
use crate::mm::{PhysPageNum, VirtPageNum};
use core::arch::asm;
use loongArch64::register::{asid, tlbehi, tlbelo0, tlbelo1, tlbidx};

pub const USR_ASID: usize = 0;

pub const KERN_ASID: usize = (1 << 10) - 1;

#[inline(always)]
pub fn set_asid(asid: usize) {
    let mut id = asid::read();
    asid::set_asid(asid & (1 << id.asid_width() - 1));
}

pub fn tlb_addr_allow_write(vpn: VirtPageNum, ppn: PhysPageNum) -> Result<(), ()> {
    tlbehi::set_vppn(0, usize::from(vpn) >> 1);
    tlbsrch();
    let ret = tlbidx::read();
    if ret.ne() {
        return Err(());
    } else {
        if vpn.0 & 1 == 0 {
            tlbelo0::set_ppn(usize::from(ppn));
            tlbelo0::set_dirty(true);
        } else {
            tlbelo1::set_ppn(usize::from(ppn));
            tlbelo1::set_dirty(true);
        }
        Ok(())
    }
}

#[inline(always)]
pub fn tlb_invalidate() {
    unsafe {
        asm!("invtlb 0x3,$zero, $zero");
    }
}

#[inline(always)]
pub fn tlb_global_invalidate() {
    unsafe {
        asm!("invtlb 0x0,$zero, $zero");
    }
}

#[inline(always)]
fn get_bits(value: usize, start: usize, end: usize) -> usize {
    assert!(start < end, "start must be less than end");
    assert!(end <= usize::BITS as usize, "end exceeds usize bit width");

    let width = end - start;

    if width == usize::BITS as usize {
        // 全部位，直接返回
        value
    } else {
        let mask = (1usize << width) - 1;
        (value >> start) & mask
    }
}

pub fn tlb_read(idx: usize) -> Result<(PhysPageNum, PhysPageNum), ()> {
    tlbidx::set_index(idx);
    let ret = tlbidx::read();

    tlbrd();

    if ret.ne() {
        Err(())
    } else {
        let even_ppn: PhysPageNum = PhysPageNum(get_bits(tlbelo0::read().raw(), 12, PALEN));
        let odd_ppn: PhysPageNum = PhysPageNum(get_bits(tlbelo1::read().raw(), 12, PALEN));
        Ok((even_ppn, odd_ppn))
    }
}

pub fn tlb_serach(vpn: VirtPageNum) -> Result<PhysPageNum, ()> {
    tlbehi::set_vppn(0, usize::from(vpn) >> 1);

    tlbsrch();

    let ret = tlbidx::read();
    if ret.ne() {
        Err(())
    } else {
        if vpn.0 & 1 == 0 {
            Ok(tlb_read(get_bits(ret.raw(), 0, 16))?.0)
        } else {
            Ok(tlb_read(get_bits(ret.raw(), 0, 16))?.1)
        }
    }
}

fn tlbrd() {
    unsafe {
        asm!("tlbrd");
    }
}

fn tlbsrch() {
    unsafe {
        asm!("tlbsrch");
    }
}
