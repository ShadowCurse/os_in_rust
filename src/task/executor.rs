use alloc::{collections::BTreeMap, sync::Arc, task::Wake};
use core::task::{Context, Poll, Waker};

use crossbeam::queue::ArrayQueue;
use x86_64::instructions::interrupts::{self, enable_and_hlt};

use super::{Task, TaskId};

struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    pub fn new_waker(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(Self {
            task_id,
            task_queue,
        }))
    }

    fn wake_task(&self) {
        self.task_queue
            .push(self.task_id)
            .expect("Task queue is full");
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}

pub struct Executor {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl Default for Executor {
    fn default() -> Self {
        Self {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(100)),
            waker_cache: BTreeMap::new(),
        }
    }
}

impl Executor {
    pub fn spawn(&mut self, task: Task) {
        let task_id = task.id;
        if self.tasks.insert(task_id, task).is_some() {
            panic!("Task with same ID already exists");
        }
        self.task_queue.push(task_id).expect("Task queue is full");
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.sleep_if_no_tasks();
        }
    }

    fn sleep_if_no_tasks(&self) {
        interrupts::disable();
        if self.task_queue.is_empty() {
            enable_and_hlt();
        } else {
            interrupts::enable();
        }
    }

    fn run_ready_tasks(&mut self) {
        while let Some(task_id) = self.task_queue.pop() {
            let task = match self.tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue,
            };
            let waker = self
                .waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new_waker(task_id, self.task_queue.clone()));
            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    self.tasks.remove(&task_id);
                    self.waker_cache.remove(&task_id);
                }
                Poll::Pending => {}
            }
        }
    }
}
