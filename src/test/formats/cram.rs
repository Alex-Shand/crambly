use std::fmt;

pub(crate) use self::{read::read, render::render};
use crate::test::{Test, tc::TestCase};

mod magic;
mod read;
mod render;
