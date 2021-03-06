// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(target_word_size = "32")]
pub fn bswap(x: uint) -> uint {
    super::i32::bswap(x as i32) as uint
}

#[cfg(target_word_size = "64")]
pub fn bswap(x: uint) -> uint {
    super::i64::bswap(x as i64) as uint
}

#[cfg(target = "big")]
pub fn to_be(x: uint) -> uint {
    x
}

#[cfg(target = "little")]
pub fn to_be(x: uint) -> uint {
    bswap(x)
}

#[cfg(target = "big")]
pub fn to_le(x: uint) -> uint {
    bswap(x)
}

#[cfg(target = "little")]
pub fn to_le(x: uint) -> uint {
    x
}
