use super::cmd_arg::CmdArg;
use std::env;

pub fn get_command_line_arguments() -> Result<Vec<CmdArg>, &'static str> {
    let arguments: Vec<String> = env::args().collect();
    let mut output_args = vec![];
    output_args.reserve(arguments.len() - 1);
    if arguments.is_empty() {
        return Err("Not enough arguments");
    }
    for value in arguments.iter().skip(1) {
        let opt = value.split_once("--");
        if let Some(("", opt)) = opt {
            output_args.push(CmdArg::LongOpt(opt.to_string()));
            continue;
        }
        let opt = value.split_once('-');
        if let Some(("", opt)) = opt {
            output_args.push(CmdArg::ShortOpt(opt.to_string()));
            continue;
        }
        output_args.push(CmdArg::Seq(value.clone()));
    }
    Ok(output_args)
}
