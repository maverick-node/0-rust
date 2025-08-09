use std::cell::{ Cell, RefCell };
use std::borrow::Borrow;
#[derive(Debug)]
pub struct ThreadPool {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl ThreadPool {
    pub fn new() -> Self {
        Self {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    pub fn new_thread(&self, c: String) -> (usize, Thread) {
        let pid = self.thread_len();
        self.states.borrow_mut().push(true);
        let thread = Thread::new(pid, c, self);
        (pid, thread)
    }

    pub fn thread_len(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states
            .borrow()
            .get(id)
            .map_or(true, |&active| !active)
    }

    pub fn drop_thread(&self, id: usize) {
        let mut binding = self.states.borrow_mut();
        let res = binding.get_mut(id).unwrap_or_else(|| {
            panic!("Thread id {} is out of bounds", id);
        });
        if !*res {
            panic!("{} is already dropped", id);
        }
        *res = false;
        self.drops.set(self.drops.get() + 1);
    }
}

#[derive(Debug)]
pub struct Thread<'a> {
    pub pid: i64,
    pub cmd: String,
    pub parent: &'a ThreadPool,
}

impl<'a> Thread<'a> {
    pub fn new(p: usize, c: String, t: &'a ThreadPool) -> Self {
        let ee = p as i64;
        Self {
            pid: ee,
            cmd: c,
            parent: t,
        }
    }

    pub fn skill(self) {
        drop(self);
    }
}

impl Drop for Thread<'_> {
    fn drop(&mut self) {
        self.parent.drop_thread(self.pid.try_into().unwrap());
    }
}
