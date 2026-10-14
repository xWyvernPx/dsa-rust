
mod datastructure;
use datastructure::heap_queue::MinHeapQueue;
use datastructure::heap_queue::QueueADT;
fn main() {
    let mut queue = MinHeapQueue::new(3);
    queue.enqueue(5);
    queue.enqueue(3);
    queue.enqueue(7);
    queue.enqueue(10);
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(1);
    queue.enqueue(0);
    queue.dequeue();
    queue.print_heap_tree();
    queue.dequeue();
    queue.print_heap_tree();
    queue.dequeue();
    queue.print_heap_tree();
    queue.dequeue();
    queue.print_heap_tree();
}
