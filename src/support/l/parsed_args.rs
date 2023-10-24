#[derive(Default)]
pub struct ParsedArgs {
    pub debug_enabled: bool,
    pub gdb_delay_loop: bool,
    pub run_options: Vec<String>,
}
