use crate::parser::Block;
use std::fmt;

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.get_kind() {
            'C' => {
                write!(f, "{}", self.get_content().iter().map(|n| n.to_owned() as char).collect::<String>())
                },
            _ => {
                write!(f, "todo")
                },
        }
    }
}
