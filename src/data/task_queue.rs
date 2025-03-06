use super::*;

#[derive(Lens)]
pub struct TaskQueue {
    pub tasks: Vec<()>,
}

impl Default for TaskQueue {
    fn default() -> Self {
        Self {
            tasks: vec![
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
            ],
        }
    }
}
