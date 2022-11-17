use crate::childs::prac_Box;
use crate::childs::prac_Rc;
use crate::childs::prac_RefCell;
use crate::childs::prac_ref_cyc_leak;
use crate::childs::prac_ref_cyc_leak2;

mod childs;

fn main() {
    // prac_Box::run();
    // prac_Rc::run();
    // prac_RefCell::run();
    // prac_ref_cyc_leak::run();
    prac_ref_cyc_leak2::run();
}
