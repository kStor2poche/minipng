use crate::parser::Block;
use std::fmt;

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.get_kind() {
            'C' => {
                write!(f, "Comment : {}", self.get_content().iter().map(|n| *n as char).collect::<String>())
            },
            'H' => {
                let width = u32::from_be_bytes(self.get_content()[0..3].try_into().unwrap());
                let height = u32::from_be_bytes(self.get_content()[4..8].try_into().unwrap());
                let mode_digit = self.get_content().get(8).unwrap();
                let tmp_err_format;
                let mode = match mode_digit {
                    0 => "black and white",
                    1 => "grey level",
                    2 => "palette",
                    3 => "24 bit color",
                    n => {
                        tmp_err_format = format!("found invalid mode {}", n);
                        &tmp_err_format
                    },
                };
                write!(f, "Image info :\n{}x{}, {}", width, height, mode)
            },
            _ => {
                Ok(())
            },
        }
    }
}
