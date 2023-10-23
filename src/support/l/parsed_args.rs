#[derive(Default)]
pub struct ParsedArgs<'a> {
    pub code_gen_enabled: bool,
    pub debug_enabled: bool,
    pub gdb_delay_loop: bool,
    pub run_options: Vec<&'a str>,
}
