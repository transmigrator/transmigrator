// browser/tab.rs

pub struct Tab {
    pub id: usize,
    pub content: String,
}

impl Tab {
    pub fn new(id: usize) -> Self {
        Tab {
            id,
            content: String::new(),
        }
    }
}