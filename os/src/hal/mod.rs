pub mod arch;
mod paltform;

pub use arch::{bootstrap_init, machine_init};
pub use arch::{console_putchar, console_getchar, console_flush, shutdown};
pub use arch::{get_time, get_clock_freq};
pub use arch::{kstack_alloc};
pub use arch::{USER_STACK_SIZE, KERNEL_HEAP_SIZE, KERNEL_STACK_SIZE, PAGE_SIZE, PAGE_SIZE_BITS, TRAMPOLINE, TRAP_CONTEXT_BASE, MEMORY_END};
pub use arch::{PageTableImpl, PageTableEntryImpl, KernelStack};

#[cfg(feature = "loongarch")]
pub use arch::{HIGH_BASE_EIGHT,MEMORY_HIGH_BASE, MEMORY_HIGH_BASE_VPN, MEMORY_SIZE, PALEN, VA_MASK, VPN_SEG_MASK};

#[cfg(feature = "board_laqemu")]
pub use paltform::{MMIO, MEM_SIZE};

#[cfg(feature = "board_rvqemu")]
pub use paltform::{MMIO, CLOCK_FREQ};

#[cfg(feature = "board_la2k1000")]
pub use paltform::{MMIO, MEM_SIZE};