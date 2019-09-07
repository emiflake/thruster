pub struct StackQueue {
    queue: [usize; 64],
    start: usize,
    end: usize,
}

impl Default for StackQueue {
    fn default() -> Self {
        Self {
            queue: [0; 64],
            start: 0,
            end: 0,
        }
    }
}

impl StackQueue {
    pub fn new() -> StackQueue {
        Self::default()
    }

    pub fn push(&mut self, n: usize) {
        self.queue[self.end] = n;
        self.end = (self.end + 1) % self.queue.len();
    }

    pub fn pop(&mut self) -> Option<usize> {
        if self.start == self.end {
            None
        } else {
            let r = self.queue[self.start];
            self.start = (self.start + 1) % self.queue.len();
            Some(r)
        }
    }
}

impl std::ops::Index<usize> for StackQueue {
    type Output = usize;
    fn index(&self, i: usize) -> &Self::Output {
        &self.queue[i]
    }
}

pub struct FastStack {
    stack: [usize; 64],
    end: usize,
}

impl Default for FastStack {
    fn default() -> Self {
        Self {
            stack: [0; 64],
            end: 0,
        }
    }
}

impl FastStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, n: usize) {
        self.stack[self.end] = n;
        self.end += 1;
    }

    pub fn pop(&mut self) -> Option<usize> {
        if self.end == 0 {
            None
        } else {
            self.end -= 1;
            let ret = self.stack[self.end];
            Some(ret)
        }
    }
}
