use std::fmt::Debug;
use std::{cell::RefCell, rc::Rc};

use crate::disabled;

use super::{ptr_iter::PtrIter, ptr_math::add};

pub fn iter_type<T>(
    state: &PtrIter,
    per_line: usize,
    step_count: &Rc<RefCell<usize>>,
    end: usize,
) -> *const *const ()
where
    T: Debug + Copy,
{
    let mut fns_arr = state.fns_arr as *const T;
    for x in 0..end {
        if x % per_line == 0 {
            disabled!(print!("{} vtable_next: ", state.p_dbg()));
        }
        disabled!(print!("{:02x?}, ", unsafe { *fns_arr }));
        add(&mut fns_arr, 1);
        if x % per_line == (per_line - 1) {
            disabled!(println!());
        }
    }
    if end % per_line != 0 {
        disabled!(println!());
    }
    let c = Rc::clone(step_count);
    *c.borrow_mut() += 1;
    disabled!(println!(
        "{} iter_type: ({}, {})",
        state.p_dbg(),
        c.borrow(),
        end,
    ));
    fns_arr as *const *const ()
}
