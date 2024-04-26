struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    fn length(&self) -> usize {
        self.queue.len()
    }

    fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.queue.first()
    }

}

fn main(){
    let mut queue: Queue<isize> = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    queue.enqueue(4);
    queue.enqueue(5);
    queue.enqueue(6);
    queue.enqueue(7);
    println!("Queue length is {}", queue.length());
    let item = queue.dequeue();
    assert_eq!(item, 1);
    let item = queue.dequeue();
    assert_eq!(item, 2);
    println!("Queue length is {}", queue.length());
    println!("The next element is {:?}", queue.peek());
    println!("Is the queue empty? {}", queue.is_empty());
}
