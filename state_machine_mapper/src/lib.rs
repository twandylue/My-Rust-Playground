struct StateMachine<'a> {
    states: Vec<&'a str>,
    commands: Vec<&'a str>,
}

// // TODO:
// impl StateMachine {
//     pub fn build(&sefl, StateMachine) {
//
//     }
// }

// fn state_mapper(pre_state: &str, command: &str, after_state: &str) -> Vec<Vec<u32>> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {}
}
