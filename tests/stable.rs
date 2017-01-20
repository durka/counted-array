#[cfg(test)] #[macro_use] extern crate lazy_static;
#[macro_use] extern crate counted_array;

use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn private_arrays() {
    counted_array!(const LOCAL_CONST_ARR: [i32; _] = [1, 2, 3]);
    counted_array!(let local_arr: [i32; _] = [4, 5, 6]);
    counted_array!(static LOCAL_STATIC_ARR: [i32; _] = [7, 8, 9, 10]);

    assert_eq!(LOCAL_CONST_ARR, [1, 2, 3]);
    assert_eq!(local_arr, [4, 5, 6]);
    assert_eq!(LOCAL_STATIC_ARR, [7, 8, 9, 10]);
}

counted_array!(pub const CONST_ARR: [i32; _] = [1, 2, 3]);
counted_array!(pub static STATIC_ARR: [i32; _] = [7, 8, 9, 10]);

#[test]
fn public_arrays() {

    assert_eq!(CONST_ARR, [1, 2, 3]);
    assert_eq!(STATIC_ARR, [7, 8, 9, 10]);
}

#[cfg(test)]
counted_array!(pub lazy_static PLAZY: [u64; _] = [3, 2, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()]);

#[cfg(test)]
fn lazy_static() {
    counted_array!(lazy_static LAZY: [u64; _] = [3, 2, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()]);

    assert_eq!(LAZY[0], 3);
    assert_eq!(LAZY[1], 2);
    assert!(LAZY[2] > 1400000000);
    assert_eq!(PLAZY[0], 3);
    assert_eq!(PLAZY[1], 2);
    assert!(PLAZY[2] > 1400000000);
}

#[test] fn test_private_arrays() { private_arrays(); }
#[test] fn test_public_arrays() { public_arrays(); }
#[test] fn test_lazy_static() { lazy_static(); }

