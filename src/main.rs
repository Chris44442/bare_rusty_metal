#![no_std]
#![no_main]

use core::panic::PanicInfo;
// use core::arch::asm;

mod boot {
    use core::arch::global_asm;
        global_asm!(".section .text._start");

}


#[no_mangle]
fn _start() {
    unsafe {core::ptr::write_volatile(0x01100000 as *mut u32, 0xaffe1266 as u32);}

    // for _ in 1..100000 {
    //   unsafe {asm!("nop");}
    // }


}

#[panic_handler]
fn panic (_info: &PanicInfo) -> ! {
    loop {}
}

// fatload mmc 0:1 01000000 ke
// bootelf 01000000
// go 0x01000000