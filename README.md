What it is
==========

This crate exports a macro which can be used to declare a fixed-size array without counting the elements by hand. This is achieved by using macro repetition to make rustc count the elements at compile time.

The macro can declare local variables, consts, statics, and lazy statics (if you import that crate, or enable this crate's "nightly" feature).

For examples, see the tests directory.

