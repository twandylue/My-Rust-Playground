use queue::Queue;

fn main() {
    let mut q: Queue<String> = Queue::new();
    q.enqueue(String::from("first"));
    q.enqueue(String::from("second"));
    q.dequeue();
    println!("length: {}", q.size())
}
