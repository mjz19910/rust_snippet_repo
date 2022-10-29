use std::fmt::Debug;
use std::{cell::RefCell, rc::Rc};

use crate::disabled;

use super::ptr_iter::PtrIter;

pub fn iter_type<T, U>(
    per_line: usize,
    state: &PtrIter,
    step_count: &Rc<RefCell<usize>>,
    end: usize,
) -> *const *const ()
where
    U: Debug + Copy,
{
    let mut fns_arr = state.fns_arr as *const U;
    for x in 0..end {
        if x % per_line == 0 {
            disabled!(print!("{} vtable_next: ", p_dbg(state)));
        }
        disabled!(print!("{:02x?}, ", unsafe { *fns_arr }));
        use super::ptr_math::add;
        add(&mut fns_arr, 1);
        if x % per_line == (per_line - 1) {
            disabled!(println!());
        }
    }
    if end % per_line != 0 {
        println!();
    }
    let c = Rc::clone(step_count);
    *c.borrow_mut() += 1;
    use crate::support::p_dbg::p_dbg;
    disabled!(println!(
        "{} iter_type: ({}, {})",
        p_dbg(state),
        c.borrow(),
        end,
    ));
    fns_arr as *const *const ()
}
