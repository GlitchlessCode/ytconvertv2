use super::*;

#[derive(Lens)]
pub struct TaskQueue {
    pub tasks: Vec<Task>,
}

impl TaskQueue {
    pub fn push(&mut self, task: Task) {
        self.tasks.insert(0, task);
    }

    pub fn pull(&mut self) -> Option<Task> {
        self.tasks.pop()
    }

    pub fn remove(&mut self, index: usize) {
        if self.tasks.get(index).is_some() {
            self.tasks.remove(index);
        }
    }
}

impl Default for TaskQueue {
    fn default() -> Self {
        Self { tasks: Vec::new() }
    }
}
