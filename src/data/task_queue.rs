use super::*;

#[derive(Lens)]
pub struct TaskQueue {
    pub tasks: Vec<Task>,
}

impl TaskQueue {
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }
}

impl Default for TaskQueue {
    fn default() -> Self {
        Self { tasks: Vec::new() }
    }
}
