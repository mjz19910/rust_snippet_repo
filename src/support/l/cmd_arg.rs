#[derive(Clone, Debug)]
pub enum CmdArg<'a> {
    LongOpt(&'a str),
    ShortOpt(&'a str),
    Seq(&'a str)
}
