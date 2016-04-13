#![cfg(feature = "nightly")]

#[macro_use] extern crate counted_array;

use std::time::{SystemTime, UNIX_EPOCH};

counted_array!(pub lazy_static PLAZY: [u64; _] = [3, 2, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()]);

#[test]
fn lazy_static_reexport() {
    // notice lazy_static was not imported separately in this file
    counted_array!(lazy_static LAZY: [u64; _] = [3, 2, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()]);

    assert_eq!(LAZY[0], 3);
    assert_eq!(LAZY[1], 2);
    assert!(LAZY[2] > 1400000000);
    assert_eq!(PLAZY[0], 3);
    assert_eq!(PLAZY[1], 2);
    assert!(PLAZY[2] > 1400000000);
}

