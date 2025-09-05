// Task Scheduler Practice
//
// Learning Objectives:
// - Use BinaryHeap to implement a task scheduler with priorities
// - Practice with custom Ord implementations for priority ordering
// - Implement priority-based scheduling algorithms
// - Understand max-heap behavior with task priorities
//
// Run with: cargo test --bin task_scheduler

/// Use BinaryHeap to implement a task scheduler with priorities.
/// Higher priority tasks (larger numbers) should be executed first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: u32,
    pub priority: u32,
    pub description: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!("Implement task ordering by priority (higher priority first)")
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct TaskScheduler {
    heap: std::collections::BinaryHeap<Task>,
}

impl TaskScheduler {
    pub fn new() -> Self {
        todo!("Create new task scheduler")
    }

    pub fn add_task(&mut self, task: Task) {
        todo!("Add task to scheduler")
    }

    pub fn get_next_task(&mut self) -> Option<Task> {
        todo!("Get highest priority task")
    }

    pub fn peek_next_task(&self) -> Option<&Task> {
        todo!("Peek at highest priority task without removing")
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BinaryHeap;

    #[test]
    fn test_task_scheduler() {
        let mut scheduler = TaskScheduler::new();
        
        let task1 = Task { id: 1, priority: 3, description: "Low priority".to_string() };
        let task2 = Task { id: 2, priority: 5, description: "High priority".to_string() };
        let task3 = Task { id: 3, priority: 1, description: "Lowest priority".to_string() };
        
        scheduler.add_task(task1);
        scheduler.add_task(task2.clone());
        scheduler.add_task(task3);
        
        assert_eq!(scheduler.len(), 3);
        
        // Should get highest priority task first
        let next = scheduler.get_next_task().unwrap();
        assert_eq!(next.id, 2);
        assert_eq!(next.priority, 5);
        
        assert_eq!(scheduler.len(), 2);
        
        // Peek should not remove task
        let peeked = scheduler.peek_next_task().unwrap();
        assert_eq!(peeked.priority, 3); // Next highest
        assert_eq!(scheduler.len(), 2); // Still 2 tasks
    }

    #[test]
    fn test_task_ordering() {
        let mut heap = BinaryHeap::new();
        
        let task1 = Task { id: 1, priority: 3, description: "Medium".to_string() };
        let task2 = Task { id: 2, priority: 5, description: "High".to_string() };
        let task3 = Task { id: 3, priority: 1, description: "Low".to_string() };
        
        heap.push(task1);
        heap.push(task2.clone());
        heap.push(task3);
        
        // Should come out in priority order: 5, 3, 1
        assert_eq!(heap.pop().unwrap().priority, 5);
        assert_eq!(heap.pop().unwrap().priority, 3);
        assert_eq!(heap.pop().unwrap().priority, 1);
    }
}