#[macro_export]
macro_rules! disabled {
    ($body:expr) => {{
        use $crate::support::get_debug_flag_state::get_debug_flag_state;
        if get_debug_flag_state() {
            $body;
        }
    }};
}

#[macro_export]
macro_rules! enabled {
    (use_enabled_macro) => {};
    ($body:expr) => {
        $body;
    };
}

enabled!(use_enabled_macro);
