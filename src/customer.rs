use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
pub struct Customer {
    first_name: String,
    last_name: String,
    no_items: usize,
}

impl PartialEq for Customer {
    fn eq(&self, other: &Self) -> bool {
        self.full_name() == other.full_name()
    }

    fn ne(&self, other: &Self) -> bool {
        self.full_name() != other.full_name()
    }
}

impl Display for Customer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}\n{}\n{}",
            self.first_name(),
            self.last_name(),
            self.no_items()
        ))
    }
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

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name(), self.last_name())
    }

    pub fn no_items(&self) -> usize {
        self.no_items
    }
}
