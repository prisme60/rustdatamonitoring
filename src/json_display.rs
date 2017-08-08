use std::fmt;

pub trait JsonDisplay {
    fn json_item(&self, f: &mut fmt::Formatter) -> fmt::Result;
}
