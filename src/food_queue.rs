use crate::customer::Customer;

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
}
