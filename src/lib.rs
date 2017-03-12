#![feature(core_intrinsics)]
#![no_std]


mod sqrt;


pub use sqrt::Sqrt;


#[inline(always)]
pub fn sqrtf32(x: f32) -> f32 {
    unsafe {
        core::intrinsics::sqrtf32(x)
    }
}
#[inline(always)]
pub fn sqrtf64(x: f64) -> f64 {
    unsafe {
        core::intrinsics::sqrtf64(x)
    }
}
