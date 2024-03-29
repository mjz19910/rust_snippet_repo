use std::slice::Iter;

use super::{CmdArg, ParsedArgs};

pub struct ArgParser {
    pub args: Vec<CmdArg>,
}
impl ArgParser {
    pub fn parse_args(&self) -> Result<ParsedArgs, String> {
        let mut args = self.args.iter();
        let mut ret = ParsedArgs::default();
        loop {
            let arg = args.next();
            let Some(arg) = arg else {break};
            let on_run_opt = |arg: &CmdArg, args: &mut Iter<'_, CmdArg>, ret: &mut ParsedArgs| {
                let arg2 = args
                    .next()
                    .ok_or_else(|| format!("Missing option for `{}`", arg))?;
                if let CmdArg::Seq(arg2) = arg2 {
                    ret.run_options.push(arg2.clone());
                } else {
                    return Err(format!("Unknown option `{}` for `{}`", arg2, arg));
                }
                Ok(())
            };
            match arg {
                CmdArg::ShortOpt(value) => match value.as_str() {
                    "r" => on_run_opt(arg, &mut args, &mut ret)?,
                    _ => return Err(format!("Invalid option `{}`", arg)),
                },
                CmdArg::LongOpt(value) => match value.as_str() {
                    "run" => on_run_opt(arg, &mut args, &mut ret)?,
                    "gdb" => ret.gdb_delay_loop = true,
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
