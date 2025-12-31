use super::config;
use core::arch::asm;

pub const TICKS_PER_SEC: usize = 100;

pub fn get_time() -> usize {
    let mut counter: usize;
    unsafe {
        asm!(
        "rdtime.d {}, {}",
        out(reg) counter,
        out(reg)_,
        );
    }
    counter
}

#[inline]
pub fn get_clock_freq() -> usize {
    unsafe { config::CLOCK_FREQ }
}

pub fn get_timer_freq_first_time() {
    // 获取时钟晶振频率
    // 配置信息字index:4
    let base_freq = config::CPUCfg4::read().get_bits(0, 31);
    // 获取时钟倍频因子
    // 配置信息字index:5 位:0-15
    let cfg5 = config::CPUCfg5::read();
    let mul = cfg5.get_bits(0, 15);
    let div = cfg5.get_bits(16, 31);
    // 计算时钟频率
    let cc_freq = base_freq * mul / div;
    println!(
        "[get_timer_freq_first_time] clk freq: {}(from CPUCFG)",
        cc_freq
    );
    unsafe { super::config::CLOCK_FREQ = cc_freq as usize }
}
