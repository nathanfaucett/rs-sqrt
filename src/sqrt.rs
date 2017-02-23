use core::intrinsics::{sqrtf32, sqrtf64};
use core::{f32, f64};


pub trait Sqrt {
    fn sqrt(self) -> Self;
}

macro_rules! trait_sqrt {
    ($t:ident, $a:ident, $f:ident) => (
        impl Sqrt for $t {
            #[inline(always)]
            fn sqrt(self) -> Self {
                if self <= 0 as $t {
                    0 as $t
                } else {
                    unsafe {
                        $f(self as $a) as $t
                    }
                }
            }
        }
    );
}

macro_rules! trait_sqrt_no_cast {
    ($t:ident, $f:ident) => (
        impl Sqrt for $t {
            #[inline(always)]
            fn sqrt(self) -> Self {
                if self < 0.0 {
                    $t::NAN
                } else {
                    unsafe {
                        $f(self)
                    }
                }
            }
        }
    );
}

trait_sqrt!(usize, f32, sqrtf32);
trait_sqrt!(u8, f32, sqrtf32);
trait_sqrt!(u16, f32, sqrtf32);
trait_sqrt!(u32, f64, sqrtf64);
trait_sqrt!(u64, f64, sqrtf64);

trait_sqrt!(isize, f32, sqrtf32);
trait_sqrt!(i8, f32, sqrtf32);
trait_sqrt!(i16, f32, sqrtf32);
trait_sqrt!(i32, f64, sqrtf64);
trait_sqrt!(i64, f64, sqrtf64);

trait_sqrt_no_cast!(f32, sqrtf32);
trait_sqrt_no_cast!(f64, sqrtf64);

#[cfg(test)]
mod test {
    use ::Sqrt;

    fn test_sqrt<T: Sqrt>(x: T) -> T {
        x.sqrt()
    }

    #[test]
    fn sqrt() {
        assert_eq!(test_sqrt(4u8), 2u8);
        assert_eq!(test_sqrt(4u16), 2u16);
        assert_eq!(test_sqrt(4u32), 2u32);
        assert_eq!(test_sqrt(4u64), 2u64);

        assert_eq!(test_sqrt(4i8), 2i8);
        assert_eq!(test_sqrt(4i16), 2i16);
        assert_eq!(test_sqrt(4i32), 2i32);
        assert_eq!(test_sqrt(4i64), 2i64);

        assert_eq!(test_sqrt(4f32), 2f32);
        assert_eq!(test_sqrt(4f64), 2f64);
    }
}
