use std::cmp::Ordering;

use crate::{
    customer::Customer,
    food_queue::{FoodQueue, FoodQueueError},
};

pub const STOCK_LOW_THRESHOLD: usize = 10;
pub const STOCK_MAX_THRESHOLD: usize = 50;

#[derive(Debug, Clone)]
pub struct Shop {
    queues: Vec<FoodQueue>,
    stock: usize,
}

#[derive(Debug)]
pub enum ShopError {
    Full,
    QueueError(FoodQueueError),
}

impl Shop {
    pub fn new(queue_layout: &[usize]) -> Self {
        let mut queues = Vec::with_capacity(queue_layout.len());

        for i in 0..queue_layout.len() {
            queues.push(FoodQueue::new(i, queue_layout[i]));
        }

        Self { queues, stock: 0 }
    }

    pub fn stock(&self) -> usize {
        self.stock
    }

    pub fn view_data(&self) -> &[FoodQueue] {
        self.queues.as_slice()
    }

    pub fn add_customer(&mut self, customer: Customer) -> Result<&Customer, ShopError> {
        match self
            .queues
            .iter_mut()
            .filter(|queue| !queue.is_full())
            .min_by(|queue1, queue2| {
                if queue1.len() > queue2.len() {
                    Ordering::Greater
                } else if queue1.len() < queue2.len() {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }) {
            Some(queue) => Ok(queue.add_customer(customer).unwrap()),
            None => Err(ShopError::Full),
        }
    }
}
