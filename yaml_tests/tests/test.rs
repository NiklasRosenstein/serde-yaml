// Copyright 2016 Serde YAML Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(plugin)]
#![plugin(indoc)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_yaml;

mod test_de;
mod test_serde;
mod test_error;
