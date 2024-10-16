// browser/tab.rs

pub struct Tab {
    pub id: usize,
    pub url: String,
}

impl Tab {
    pub fn new(id: usize, url: String) -> Self {
        Tab { id, url }
    }
}

pub struct Browser {
    tabs: Vec<Tab>,
    current_tab: usize,
}

impl Browser {
    pub fn new() -> Self {
        Browser {
            tabs: Vec::new(),
            current_tab: 0,
        }
    }

    pub fn open_tab(&mut self, url: String) {
        let id = self.tabs.len();
        let tab = Tab::new(id, url);
        self.tabs.push(tab);
        self.current_tab = id;
    }

    pub fn close_tab(&mut self, id: usize) {
        self.tabs.retain(|tab| tab.id != id);
        if self.current_tab == id {
            self.current_tab = if self.tabs.is_empty() { 0 } else { self.tabs.len() - 1 };
        }
    }

    pub fn switch_tab(&mut self, id: usize) {
        if id < self.tabs.len() {
            self.current_tab = id;
        }
    }

    pub fn get_current_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.current_tab)
    }

    pub fn render(&self) {
        if let Some(tab) = self.get_current_tab() {
            // Render the current tab's content
            println!("Rendering tab with URL: {}", tab.url);
        } else {
            println!("No tabs open.");
        }
    }
}