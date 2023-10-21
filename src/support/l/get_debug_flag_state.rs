use crate::support::constants::DEBUG_ENABLED;

pub fn get_debug_flag_state() -> bool {
    unsafe { DEBUG_ENABLED }
}
