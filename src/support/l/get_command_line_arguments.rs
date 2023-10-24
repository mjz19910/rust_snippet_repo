use super::cmd_arg::CmdArg;

pub fn get_command_line_arguments<'a>(
    arguments: &'a Vec<String>,
) -> Result<Vec<CmdArg<'a>>, &'static str> {
    let mut output_args = vec![];
    output_args.reserve(arguments.len() - 1);
    if arguments.len() < 1 {
        return Err("Not enough arguments");
    }
    for value in arguments.iter().skip(1) {
        let opt = value.split_once("--");
        if let Some(("", opt)) = opt {
            output_args.push(CmdArg::LongOpt(opt));
            continue;
        }
        let opt = value.split_once('-');
        if let Some(("", opt)) = opt {
            output_args.push(CmdArg::ShortOpt(opt));
            continue;
        }
        output_args.push(CmdArg::Seq(value));
    }
    Ok(output_args)
}
