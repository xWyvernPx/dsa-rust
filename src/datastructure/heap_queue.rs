
pub struct MinHeapQueue {
    pub heap: Vec<i32>,
    pub size: usize
}

#[allow(dead_code)]
pub struct MaxHeapQueue {
    heap: Vec<i32>,
    size: usize,
}

pub trait QueueADT {
    fn enqueue(&mut self, value: i32);
    fn dequeue(&mut self) -> Option<i32>;
    fn peek(&self) -> Option<&i32>;
    fn print_heap_tree(&mut self);
}

impl MinHeapQueue { 
    pub fn new(capacity: usize) -> Self {
        let mut heap = Vec::with_capacity(capacity + 1);
        heap.push(0);
        MinHeapQueue { heap, size: 0 }
    }
    
    pub fn size(&self) -> usize {
        self.size
    }
}

impl QueueADT for MinHeapQueue {
    fn enqueue(&mut self, value: i32) {
        self.heap.push(value);
        self.size += 1;
        let mut current_pos = self.size;
        while current_pos > 1 {
            let parent_pos = current_pos / 2;
            if self.heap[current_pos] >= self.heap[parent_pos] { 
                break;
            }   

            self.heap.swap(current_pos, parent_pos);
            current_pos = parent_pos;
        }
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        if self.size == 1 {
            self.size -= 1;
            return Some(self.heap.pop().unwrap());
        }
        
        let min_value = self.heap[1];
        self.heap[1] = self.heap.pop().unwrap();
        self.size -= 1;

        let mut current_pos = 1;
          while current_pos * 2 <= self.size {
            let left_child_pos = current_pos * 2;
            let right_child_pos = current_pos * 2 + 1;
            let current_child = self.heap[current_pos];

            let mut smallest_child_pos = current_pos;

            if right_child_pos <= self.size && self.heap[left_child_pos] < self.heap[smallest_child_pos] {
                smallest_child_pos = left_child_pos
            }
            if right_child_pos <= self.size && self.heap[right_child_pos] < self.heap[smallest_child_pos] {
                smallest_child_pos = right_child_pos;
            }

            if smallest_child_pos == current_pos {
                break;
            }

            self.heap.swap(current_pos, smallest_child_pos);
            current_pos = smallest_child_pos;
        } 

        Some(min_value)
    }

    fn peek(&self) -> Option<&i32> {
        if self.size > 0 {
            Some(&self.heap[1])
        } else {
            None
        }
    }

fn print_heap_tree(&mut self) {
    // Check if the heap has any actual data (ignoring index 0)
    if self.heap.len() <= 1 {
        println!("(Heap is empty)");
        return;
    }

    println!("\n=== Heap Tree Structure (1-Indexed) ===");

    // The effective number of elements in the heap
    let count = self.heap.len() - 1;

    // Calculate total levels required for `count` elements
    let total_levels = ((count as f64).log2().floor() as usize) + 1;

    // Track the 1-based index in the heap array
    let mut current_idx = 1;

    for level in 0..total_levels {
        let nodes_in_level = 1 << level; // 2^level potential nodes on this level

        // Dynamic spacing calculations to align the tree nicely
        let spacing_factor = 1 << (total_levels - level);
        let leading_spaces = spacing_factor - 2;
        let between_spaces = (spacing_factor * 2) - 2;

        // --- 1. Print Node Values ---
        print!("{:width$}", "", width = leading_spaces);
        for i in 0..nodes_in_level {
            let node_idx = current_idx + i;
            if node_idx < self.heap.len() {
                // Print formatted 2-digit number
                print!("{:02}", self.heap[node_idx]);
            } else {
                // Empty placeholder if the node does not exist
                print!("  ");
            }
            print!("{:width$}", "", width = between_spaces);
        }
        println!();

        // --- 2. Print Branch Connectors ---
        if level + 1 < total_levels {
            let branch_leading = if leading_spaces > 0 { leading_spaces - 1 } else { 0 };
            let branch_gap = if spacing_factor > 2 { spacing_factor - 2 } else { 0 };

            print!("{:width$}", "", width = branch_leading);
            for i in 0..nodes_in_level {
                let node_idx = current_idx + i;
                let left_child_idx = 2 * node_idx;
                let right_child_idx = 2 * node_idx + 1;

                // Left branch '/'
                if left_child_idx < self.heap.len() {
                    print!("/");
                } else {
                    print!(" ");
                }

                print!("{:width$}", "", width = branch_gap);

                // Right branch '\'
                if right_child_idx < self.heap.len() {
                    print!("\\");
                } else {
                    print!(" ");
                }

                let after_branch_gap = if between_spaces > branch_gap {
                    between_spaces - branch_gap - 1
                } else {
                    1
                };
                print!("{:width$}", "", width = after_branch_gap);
            }
            println!();
        }

        // Advance to the start of the next level
        current_idx += nodes_in_level;
        if current_idx >= self.heap.len() {
            break;
        }
    }
    println!("=======================================\n");
}
}