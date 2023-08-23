use std::fmt::Display;

use crate::customer::Customer;

#[derive(Debug, Clone)]
pub struct FoodQueue {
    id: usize,
    queue: Vec<Customer>,
    capacity: usize,
}

#[derive(Debug)]
pub enum FoodQueueError {
    Full,
    Empty,
    CustomerNotFound,
}

impl Display for FoodQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}\n{}\n{}\n",
            self.id(),
            self.capacity(),
            self.len()
        ))?;

        for customer in self.queue.as_slice() {
            f.write_fmt(format_args!("{}\n", customer))?;
        }

        Ok(())
    }
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

    pub fn add_customer(&mut self, customer: Customer) -> Result<&Customer, FoodQueueError> {
        if self.is_full() {
            return Err(FoodQueueError::Full);
        }

        self.queue.push(customer);
        Ok(self.queue.last().unwrap())
    }

    pub fn remove_customer(&mut self, customer_pos: usize) -> Result<Customer, FoodQueueError> {
        if self.is_empty() {
            return Err(FoodQueueError::Empty);
        }

        Ok(self.queue.remove(customer_pos))
    }

    pub fn get_customer(&self, customer_pos: usize) -> Result<&Customer, FoodQueueError> {
        Ok(self
            .queue
            .get(customer_pos)
            .ok_or_else(|| FoodQueueError::CustomerNotFound))?
    }
}
