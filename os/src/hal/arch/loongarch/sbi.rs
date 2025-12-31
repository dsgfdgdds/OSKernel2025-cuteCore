pub fn console_putchar(c: usize) {
    todo!()
}

pub fn console_getchar() -> isize {
    todo!()
}

pub fn console_flush() {
    todo!()
}

pub fn shutdown() -> ! {
    {
        unsafe {
            (0x100E_001C as *mut u8).write_volatile(0x34);
        }
    }
    loop {}
}