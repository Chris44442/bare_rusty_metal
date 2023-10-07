#![no_std]
#![no_main]
// #![feature(asm)]

fn fast_sqrt (input: f64) -> f64 { // Famous Quake III "Fast inverse square root" algorithm implementation
  const MAGIC_U64: u64 = 0x5fe6ec85e7de30da;
  const THREEHALFS: f64 = 1.5;
  let x2 = input * 0.5;
  let value: u64 = unsafe { core::intrinsics::transmute(input) }; // evil floating point bit level hacking
  let i = MAGIC_U64 - (value >> 1); // what the fuck?
  let y: f64 = unsafe { core::intrinsics::transmute(i) };
  let mut inv_sqrt = y * ( THREEHALFS - ( x2 * y * y ) ); // Newton–Raphson method
  inv_sqrt = inv_sqrt * ( THREEHALFS - ( x2 * inv_sqrt * inv_sqrt ) ); // use for more precision
//   inv_sqrt = inv_sqrt * ( THREEHALFS - ( x2 * inv_sqrt * inv_sqrt ) ); // use for more precision
  inv_sqrt*input
}

// fn my_sqrt(x: f32) -> f32 {
//     let y;
//     unsafe {core::arch::asm!("VSQRT.F32 {0}, {0}", inout(reg) x => y)}
//     y
// }

mod boot {
  use core::arch::global_asm;
  global_asm!(".section .text._start");
}

#[no_mangle]
pub extern "C" fn _start() {
  // SQRT fun stuff
  let abc = unsafe {core::ptr::read_volatile(0x01100004 as *mut u32)};
  let bbb = libm::sqrt(abc as f64);
  let ccc = fast_sqrt(abc as f64);
  unsafe {core::ptr::write_volatile(0x01100008 as *mut u32, bbb as u32);}
  unsafe {core::ptr::write_volatile(0x0110000c as *mut u32, ccc as u32);}

  // let x: f32 = 16.0;
  // let _result = my_sqrt(x);

  //Hello World UART fun
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