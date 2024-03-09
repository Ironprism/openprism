pub trait To<Dst> {
    fn to(self) -> Dst;
}

impl<T> To<T> for T {
    #[inline(always)]
    fn to(self) -> T {
        self
    }
}

macro_rules! impl_to {
    ($head:ty, $($tail:ty,)*) => {$(
        impl To<$tail> for $head {
            #[inline(always)]
            fn to(self) -> $tail {
                self as $tail
            }
        }
        impl To<$head> for $tail {
            #[inline(always)]
            fn to(self) -> $head {
                self as $head
            }
        }
        impl To<$tail> for &$head {
            #[inline(always)]
            fn to(self) -> $tail {
                *self as $tail
            }
        }
        impl To<$head> for &$tail {
            #[inline(always)]
            fn to(self) -> $head {
                *self as $head
            }
        }
        impl To<$tail> for &mut $head {
            #[inline(always)]
            fn to(self) -> $tail {
                *self as $tail
            }
        }
        impl To<$head> for &mut $tail {
            #[inline(always)]
            fn to(self) -> $head {
                *self as $head
            }
        }
        impl To<$tail> for Box<$head> {
            #[inline(always)]
            fn to(self) -> $tail {
                *self as $tail
            }
        }
        impl To<$head> for Box<$tail> {
            #[inline(always)]
            fn to(self) -> $head {
                *self as $head
            }
        }
    )*
    impl_to!( $($tail,)* );
    };
    () => {};
}

impl_to!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64,);
