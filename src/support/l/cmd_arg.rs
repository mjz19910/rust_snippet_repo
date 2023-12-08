use std::fmt;

#[derive(Clone, Debug)]
pub enum CmdArg {
    LongOpt(String),
    ShortOpt(String),
    Seq(String),
}
impl fmt::Display for CmdArg {
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
