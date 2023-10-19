#![no_std]
#![no_main]

fn inv_sqrt (x: f64) -> f64 { // Famous Quake III "Fast inverse square root" algorithm implementation
  const MAGIC_U64: u64 = 0x5fe6ec85e7de30da;
  let x2 = x * 0.5;
  let value: u64 = unsafe {core::intrinsics::transmute(x)}; // evil floating point bit level hacking
  let i = MAGIC_U64 - (value >> 1); // what the fuck?
  let mut inv_sqrt: f64 = unsafe {core::intrinsics::transmute(i)};
  inv_sqrt *= 1.5 - (x2 * inv_sqrt * inv_sqrt); // Newton–Raphson method
  inv_sqrt *= 1.5 - (x2 * inv_sqrt * inv_sqrt); // repeat for more precision
  inv_sqrt
}

mod boot {
  use core::arch::global_asm;
  global_asm!(".section .text._start");
}

#[no_mangle]
pub extern "C" fn _start() {
  // SQRT fun stuff
  let abc = unsafe {core::ptr::read_volatile(0x01100004 as *mut u32)};
  let bbb = libm::sqrt(abc as f64);
  let ccc = abc as f64 * inv_sqrt(abc as f64);
  unsafe {core::ptr::write_volatile(0x01100008 as *mut u32, bbb as u32);}
  unsafe {core::ptr::write_volatile(0x0110000c as *mut u32, ccc as u32);}

  // Hello World UART fun
  let my_string = *b"Hello World!\n";
  for &letter in &my_string {
    while unsafe { core::ptr::read_volatile(0xFFC02014 as *const u32) } & 0x20 == 0 {}
    let data_to_send = letter as u32;
    unsafe {core::ptr::write_volatile(0xFFC02000 as *mut u32, data_to_send);}
  }
}

#[panic_handler]
fn panic (_info: &core::panic::PanicInfo) -> ! {
  loop {}
}
