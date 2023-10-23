#[derive(Default)]
pub(crate) struct ParsedArgs<'a> {
    pub(crate) code_gen_enabled: bool,
    pub(crate) debug_enabled: bool,
    pub(crate) gdb_delay_loop: bool,
    pub(crate) run_options: Vec<&'a str>,
}
