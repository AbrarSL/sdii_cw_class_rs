#[derive(Debug, Clone)]
pub struct Customer {
    first_name: String,
    last_name: String,
    no_items: usize,
}

impl Customer {
    pub fn new(first_name: String, last_name: String, no_items: usize) -> Self {
        Self {
            first_name,
            last_name,
            no_items,
        }
    }

    pub fn first_name(&self) -> &str {
        self.first_name.as_str()
    }

    pub fn last_name(&self) -> &str {
        self.last_name.as_str()
    }

    pub fn no_items(&self) -> usize {
        self.no_items
    }
}
