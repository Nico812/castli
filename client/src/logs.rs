use std::collections::VecDeque;

pub struct Logs {
    pub content: VecDeque<String>,
    max_len: usize,
}

impl Logs {
    pub fn new(max_len: usize) -> Self {
        Self {
            content: VecDeque::with_capacity(max_len),
            max_len,
        }
    }

    pub fn add(&mut self, item: String) {
        if self.content.len() >= self.max_len {
            let _ = self.content.pop_front();
        }
        self.content.push_back(item);
    }
}
