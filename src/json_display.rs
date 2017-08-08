use std::io::{Result, Write};

pub trait JsonDisplay {
    fn json_item(&self, f: &mut Write) -> Result<()>;
}
