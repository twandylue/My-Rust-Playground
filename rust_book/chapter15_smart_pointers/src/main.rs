use crate::childs::prac_Box;
use crate::childs::prac_Rc;
use crate::childs::prac_RefCell;

mod childs;

fn main() {
    prac_Box::run();
    prac_Rc::run();
}
