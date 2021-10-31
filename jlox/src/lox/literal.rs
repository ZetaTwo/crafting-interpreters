use std::fmt;

pub struct Literal {

}

impl fmt::Display for Literal {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!("Literal isn't displayable");
    }
}
