use crate::drivers::Ns16550a;
use crate::hal::platform::UART_BASE;
use embedded_hal::serial::nb::{Read, Write};

pub static mut UART: Ns16550a = Ns16550a { base: UART_BASE };

pub fn console_putchar(c: usize) {
    let mut retry = 0;
    unsafe {
        UART.write(c as u8).expect("console_putchar failed");
    }
}

pub fn console_getchar() -> usize {
    unsafe {
        if let Ok(i) = UART.read() {
            i as usize
        } else {
            1usize.wrapping_neg()
        }
    }
}

pub fn console_flush() {
    unsafe { while UART.flush().is_err() {} }
}

pub fn shutdown() -> ! {
    {
        unsafe {
            (0x100E_001C as *mut u8).write_volatile(0x34);
        }
    }
    loop {}
}
