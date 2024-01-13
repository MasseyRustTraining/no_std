#![no_main]
#![no_std]

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}

#[panic_handler]
#[no_mangle]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
