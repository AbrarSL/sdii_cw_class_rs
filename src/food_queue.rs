use crate::customer::Customer;

#[derive(Debug, Clone)]
pub struct FoodQueue {
    id: usize,
    queue: Vec<Customer>,
    capacity: usize,
}

impl FoodQueue {
    pub fn new(id: usize, capacity: usize) -> Self {
        Self {
            id,
            queue: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn view_data(&self) -> &[Customer] {
        self.queue.as_slice()
    }

    pub fn is_full(&self) -> bool {
        self.queue.len() >= self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }
}
