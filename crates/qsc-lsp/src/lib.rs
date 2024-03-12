#![feature(box_into_inner)]

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate serde;

pub mod backend;
pub mod completion;
pub mod data;
pub mod hint;
pub mod refs;
pub mod util;
