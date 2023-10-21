use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum CmdArg<'a> {
    LongOpt(&'a str),
    ShortOpt(&'a str),
    Seq(&'a str),
}
impl<'a> fmt::Display for CmdArg<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LongOpt(v) => {
                write!(f, "--{}", v)
            }
            Self::ShortOpt(v) => {
                write!(f, "-{}", v)
            }
            Self::Seq(v) => {
                write!(f, "{}", v)
            }
        }
    }
}
