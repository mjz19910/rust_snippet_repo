use crate::support::constants::FORCE_DEBUG_FLAG;
use crate::support::constants::SKIP_DEBUG_FLAG;

pub fn get_debug_flag_state() -> bool {
    (cfg!(feature = "debug") && unsafe { !SKIP_DEBUG_FLAG }) || unsafe { FORCE_DEBUG_FLAG }
}
