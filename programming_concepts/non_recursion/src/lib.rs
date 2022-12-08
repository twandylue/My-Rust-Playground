pub fn fab_recur(f: usize) -> usize {
    if f < 2 {
        return f;
    }

    return self::fab_recur(f - 1) + self::fab_recur(f - 2);
}

#[derive(Debug)]
enum Action<T, U> {
    Call(T),
    Handle(U),
}

pub fn fab_nonrecur(f: usize) -> usize {
    let mut stack = Vec::<Action<usize, ()>>::new();
    let mut ret_stack = Vec::<usize>::new();
    stack.push(Action::Call(f));
    while let Some(action) = stack.pop() {
        println!("stack: {:?}", stack);
        println!("ret_stack: {:?}", ret_stack);
        match action {
            Action::Call(number) => {
                if number < 2 {
                    ret_stack.push(number);
                } else {
                    stack.push(Action::Handle(()));
                    stack.push(Action::Call(number - 2));
                    stack.push(Action::Call(number - 1));
                }
            }
            Action::Handle(()) => {
                let f: usize = ret_stack.pop().unwrap();
                let s: usize = ret_stack.pop().unwrap();
                ret_stack.push(f + s);
            }
        }
    }

    ret_stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn fab_recur_case_1() {
        let expected: usize = 0;
        let actual = fab_recur(0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn fab_recur_case_2() {
        let expected: usize = 55;
        let actual = fab_recur(10);

        assert_eq!(expected, actual);
    }

    #[test]
    fn fab_nonrecur_case_1() {
        let expected: usize = 0;
        let actual = fab_nonrecur(0);

        assert_eq!(expected, actual);
    }

    #[test]
    fn fab_nonrecur_case_2() {
        let expected: usize = 55;
        let actual = fab_nonrecur(10);

        assert_eq!(expected, actual);
    }
}
