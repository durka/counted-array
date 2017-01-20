#![cfg_attr(feature = "nightly", feature(allow_internal_unstable, macro_reexport))]

#[cfg(feature = "nightly")]
#[macro_reexport(lazy_static, __lazy_static_create)]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "nightly")]
pub use lazy_static::lazy;

/// Declare a fixed-size array with an autogenerated length.
///
/// ```
/// # #[macro_use] extern crate counted_array;
/// # fn main() {
/// counted_array!(let arr: [i32; _] = [1, 2, 3]);
/// assert_eq!(arr.len(), 3);
/// # }
/// ```
#[macro_export]
macro_rules! counted_array {
    // INTERNAL

    // last element (proceed to output)
    (@parse $size:expr, ($val:expr) -> [$($accs:expr),*] $thru:tt) => {
        counted_array!(@output $size + 1usize, [$($accs,)* $val] $thru);
    };
    // more elements (keep parsing)
    (@parse $size:expr, ($val:expr, $($vals:expr),*) -> [$($accs:expr),*] $thru:tt) => {
        counted_array!(@parse $size + 1usize, ($($vals),*) -> [$($accs,)* $val] $thru);
    };
    
    // output a local variable
    (@output $size:expr, $acc:tt (() let $n:ident $t:ty)) => {
        let $n: [$t; $size] = $acc;
    };
    // output a lazy static
    (@output $size:expr, $acc:tt (($($p:tt)*) lazy_static $n:ident $t:ty)) => {
        lazy_static!{ $($p)* static ref $n: [$t; $size] = $acc; }
    };
    // output a static or const item
    (@output $size:expr, $acc:tt (($($p:tt)*) $s:ident $n:ident $t:ty)) => {
        $($p)* $s $n: [$t; $size] = $acc;
    };
    
    // EXTERNAL
    
    // entry point
    (pub $storage:ident $n:ident: [$t:ty; _] = [$($vals:expr),* $(,)*]) => {
        counted_array!(@parse 0usize, ($($vals),*) -> [] ((pub) $storage $n $t));
    };
    ($storage:ident $n:ident: [$t:ty; _] = [$($vals:expr),* $(,)*]) => {
        counted_array!(@parse 0usize, ($($vals),*) -> [] (() $storage $n $t));
    };
}

