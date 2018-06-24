// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// This is an equivalent of issue #50504, but for declarative macros.

#![feature(decl_macro, rustc_attrs)]

#[rustc_transparent_macro]
macro genmod() {
    mod m {
        type A = S; //~ ERROR cannot find type `S` in this scope
    }
}

struct S;

genmod!();
