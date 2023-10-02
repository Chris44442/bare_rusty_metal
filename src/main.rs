#![no_std]
#![no_main]

mod boot {
  use core::arch::global_asm;
  global_asm!(".section .text._start");
}

#[no_mangle]
fn _start() {
  let abc = unsafe {core::ptr::read_volatile(0x01100004 as *mut u32)};

  let bbb = abc as f64;
  let ccc = libm::sqrt(bbb) as u32;

  unsafe {core::ptr::write_volatile(0x01100008 as *mut u32, ccc as u32);}
}

#[panic_handler]
fn panic (_info: &core::panic::PanicInfo) -> ! {
  loop {}
}