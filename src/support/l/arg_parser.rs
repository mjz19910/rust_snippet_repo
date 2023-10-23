use super::{CmdArg, ParsedArgs};

pub(crate) struct ArgParser<'a> {
    pub(crate) args: Vec<CmdArg<'a>>,
}
impl<'a> ArgParser<'a> {
    pub(crate) fn parse_args(&self) -> Result<ParsedArgs<'a>, String> {
        let mut args = self.args.iter();
        let mut ret = ParsedArgs::default();
        loop {
            let arg = args.next();
            let Some(&arg) = arg else {break};
            match arg {
                CmdArg::LongOpt("run") | CmdArg::ShortOpt("r") => {
                    let arg2 = *args
                        .next()
                        .ok_or_else(|| format!("Missing option for `{}`", arg))?;
                    if let CmdArg::Seq(arg2) = arg2 {
                        ret.run_options.push(arg2);
                    } else {
                        return Err(format!("Unknown option `{}` for `{}`", arg2, arg));
                    }
                }
                CmdArg::LongOpt(value) => match value {
                    "gdb" => ret.gdb_delay_loop = true,
                    "code-gen" => ret.code_gen_enabled = true,
                    "no-code-gen" => ret.code_gen_enabled = false,
                    "debug" => ret.debug_enabled = true,
                    "no-debug" => ret.debug_enabled = false,
                    _ => return Err(format!("Invalid option `{}`", arg)),
                },
                _ => return Err(format!("Unknown option `{}`", arg)),
            }
        }
        Ok(ret)
    }
}
