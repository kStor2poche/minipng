use crate::parser::{Header, Comment};
use std::fmt;

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Comment : {}", self.get_content())
    }
}
impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let content = self.get_content();
                let tmp_err_format;
                let mode = match content.2 {
                    0 => "black and white",
                    1 => "grey level",
                    2 => "palette",
                    3 => "24 bit color",
                    n => {
                        tmp_err_format = format!("found invalid mode \"{}\"", n);
                        &tmp_err_format
                    },
                };
    write!(f, "Image info :\n{}x{}, {}.", content.0, content.1, mode)
    }
}
