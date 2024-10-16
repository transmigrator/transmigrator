// browser/tab.rs

pub struct Tab {
    pub id: usize,
    pub url: String,
    // Add other fields as needed
}

impl Tab {
    pub fn new(id: usize, url: String) -> Self {
        Tab { id, url }
    }
}
