// Work-Stealing Deque Practice
//
// Learning Objectives:
// - Implement a work-stealing deque using VecDeque
// - Workers can steal work from both ends for load balancing
// - Practice with concurrent algorithm patterns using deque operations
// - Understand LIFO vs FIFO access patterns for work distribution
//
// Run with: cargo test --bin work_stealing_deque

/// Implement a work-stealing deque using VecDeque.
/// Workers can steal work from both ends for load balancing.
pub struct WorkStealingDeque<T> {
    deque: std::collections::VecDeque<T>,
}

impl<T> WorkStealingDeque<T> {
    pub fn new() -> Self {
        todo!("Create new work-stealing deque")
    }

    pub fn push_task(&mut self, task: T) {
        todo!("Add task to own end (back)")
    }

    pub fn pop_task(&mut self) -> Option<T> {
        todo!("Remove task from own end (back)")
    }

    pub fn steal_task(&mut self) -> Option<T> {
        todo!("Steal task from other end (front)")
    }

    pub fn len(&self) -> usize {
        self.deque.len()
    }

    pub fn is_empty(&self) -> bool {
        self.deque.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_work_stealing_deque() {
        let mut deque = WorkStealingDeque::new();
        assert!(deque.is_empty());
        
        // Add some tasks
        deque.push_task("task1");
        deque.push_task("task2");
        deque.push_task("task3");
        assert_eq!(deque.len(), 3);
        
        // Owner pops from back (LIFO)
        let task = deque.pop_task();
        assert_eq!(task, Some("task3"));
        assert_eq!(deque.len(), 2);
        
        // Thief steals from front (FIFO)
        let stolen = deque.steal_task();
        assert_eq!(stolen, Some("task1"));
        assert_eq!(deque.len(), 1);
        
        // Remaining task
        let last = deque.pop_task();
        assert_eq!(last, Some("task2"));
        assert!(deque.is_empty());
        
        // Stealing from empty deque
        assert_eq!(deque.steal_task(), None);
    }
}