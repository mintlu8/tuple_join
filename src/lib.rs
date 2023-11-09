#![no_std]
#![allow(nonstandard_style)]
//! A crate for joining tuples at the type level.
//! 
//! Supports up to tuple length 13.
//! 
//! # Examples
//! 
//! ```
//! use tuple_join::*;
//! 
//! assert_eq!((1,2).join((3,4,5,6)), (1,2,3,4,5,6));
//! assert_eq!((1,2,3,4,5,6).split(), ((1,2), (3,4,5,6)));
//! 
//! assert_eq!((1,2,3).push("hello"), (1,2,3,"hello"));
//! assert_eq!(("ferris", "the", "rustacean").pop(), (("ferris", "the"), "rustacean"));
//! ```

/// Append a regular type to a tuple type.
pub trait Append<A>: Join<(A,)> {
    fn push(self, other: A) -> Self::Out where Self: Sized, A: Sized;
    fn pop(tuple: Self::Out) -> (Self, A) where Self: Sized, A: Sized;
}

impl<A, T> Append<A> for T where T: Join<(A,)>{
    fn push(self, other: A) -> Self::Out where Self: Sized, A: Sized {
        self.join((other,))
    }

    fn pop(tuple: Self::Out) -> (Self, A) where Self: Sized, A: Sized {
        let (a, (b, )) = Self::split(tuple);
        return (a, b);
    }
}

/// Join 2 tuple types as the associated type.
pub trait Join<A> {
    type Out;

    fn join(self, other: A) -> Self::Out where Self: Sized, A: Sized;
    fn split(tuple: Self::Out) -> (Self, A) where Self: Sized, A: Sized;
}

/// Split a regular from a tuple type.
pub trait Appended<A, B> {
    fn pop(self) -> (A, B);
}

/// Split two tuple types from a tuple type.
pub trait Joined<A, B> {
    fn split(self) -> (A, B);
}

impl<A, B, T> Appended<A, B> for T where A: Append<B, Out = T>{
    fn pop(self) -> (A, B) {
        A::pop(self)
    }
}

impl<A, B, T> Joined<A, B> for T where A: Join<B, Out = T>{
    fn split(self) -> (A, B) {
        A::split(self)
    }
}


macro_rules! tuple_join_y {
    ($($x: ident)* ,) => {
        impl<$($x,)*> Join<()> for ($($x,)*) {
            type Out = ($($x,)*);

            fn join(self, _: ()) -> Self::Out {
                self
            }
            fn split(out: Self::Out) -> (Self, ()) {
                (out, ())
            }
        }
    };
    ($($x: ident)*, $y0:ident $($y: ident)*) => {
        impl<$($x,)* $y0,$($y,)*> Join<($y0,$($y,)*)> for ($($x,)*) {
            type Out = ($($x,)* $y0,$($y,)*);

            fn join(self, ($y0,$($y,)*): ($y0,$($y,)*)) -> Self::Out {
                let ($($x,)*) = self;
                ($($x,)* $y0,$($y,)*)
            }

            fn split(($($x,)* $y0,$($y,)*): Self::Out) -> (Self, ($y0,$($y,)*)) {
                (($($x,)*), ($y0,$($y,)*))
            }
        }
        tuple_join_y!($($x)*, $($y)*);
    };
}

macro_rules! tuple_join {
    (, $y0:ident $($y: ident)*) => {
        impl<$y0,$($y,)*> Join<($y0,$($y),*)> for () {
            type Out = ($y0,$($y,)*);
            
            fn join(self, other: ($y0,$($y,)*)) -> Self::Out {
                other
            }

            fn split(out: Self::Out) -> (Self, ($y0,$($y,)*)) {
                ((), out)
            }
        }
        tuple_join_y!(, $($y)*);
    };

    ($x0: ident $($x: ident)*, $y0:ident $($y: ident)*) => {
        impl<$x0,$($x,)* $y0,$($y,)*> Join<($y0,$($y,)*)> for ($x0,$($x,)*) {
            type Out = ($x0,$($x,)* $y0,$($y,)*);

            fn join(self, ($y0,$($y,)*): ($y0,$($y,)*)) -> Self::Out {
                let ($x0,$($x,)*) = self;
                ($x0,$($x,)* $y0,$($y,)*)
            }
            fn split(($x0,$($x,)* $y0,$($y,)*): Self::Out) -> (Self, ($y0,$($y,)*)) {
                (($x0,$($x,)*), ($y0,$($y,)*))
            }
        }
        tuple_join!($($x)*, $y0 $($y)*);
        tuple_join_y!($x0 $($x)*, $($y)*);
    };
}


tuple_join!(
    A B C D E F G H I J K L M,
    N O P Q R S T U V W X Y Z
);
