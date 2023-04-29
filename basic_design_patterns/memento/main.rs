// Memento
// ref:
// 1. https://refactoring.guru/design-patterns/memento
// NOTE: main is like a command
fn main() {
    let mut history = Vec::<OriginatorBackup>::new();
    let mut originator = Originator { state: 0 };

    originator.state = 1;
    history.push(originator.save());
    originator.state = 2;
    history.push(originator.save());

    for moment in history.iter() {
        moment.print();
    }

    let originator = history.pop().unwrap().restore();
    println!("Restored to state: {state}", state = originator.state);
    let originator = history.pop().unwrap().restore();
    println!("Restored to state: {state}", state = originator.state);
}

trait Memento<T> {
    fn restore(&self) -> T;
    fn print(&self);
}

struct Originator {
    state: u32,
}

impl Originator {
    fn save(&self) -> OriginatorBackup {
        OriginatorBackup {
            state: self.state.to_string(),
        }
    }
}

struct OriginatorBackup {
    state: String,
}

impl Memento<Originator> for OriginatorBackup {
    // TODO: Use the mutable reference to modify the state
    fn restore(&self) -> Originator {
        Originator {
            state: self.state.parse().unwrap(),
        }
    }

    fn print(&self) {
        println!("Originator backup: {state}", state = self.state);
    }
}
