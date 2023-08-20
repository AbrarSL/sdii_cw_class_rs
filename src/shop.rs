use std::cmp::Ordering;

use crate::{
    customer::Customer,
    food_queue::{FoodQueue, FoodQueueError},
};

pub const STOCK_LOW_THRESHOLD: usize = 10;
pub const STOCK_MAX_THRESHOLD: usize = 50;
pub const ITEM_PRICE: usize = 500;

#[derive(Debug, Clone)]
pub struct Shop {
    queues: Vec<FoodQueue>,
    stock: usize,
}

#[derive(Debug)]
pub enum ShopError {
    Full,
    QueueNotFound,
    StockInsufficient,
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

    fn set_stock(&mut self, stock: usize) {
        self.stock = stock;
    }

    pub fn len(&self) -> usize {
        self.queues.len()
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

    pub fn remove_customer(
        &mut self,
        queue_no: usize,
        customer_pos: usize,
    ) -> Result<Customer, ShopError> {
        self.queues
            .get_mut(queue_no)
            .ok_or_else(|| ShopError::QueueNotFound)?
            .remove_customer(customer_pos)
            .or_else(|error| Err(ShopError::QueueError(error)))
    }

    pub fn serve_customer(&mut self, queue_no: usize) -> Result<Customer, ShopError> {
        let (new_stock, overflow) = self.stock().overflowing_sub(
            self.queues
                .get(queue_no)
                .ok_or_else(|| ShopError::QueueNotFound)?
                .get_customer(0)
                .or_else(|error| Err(ShopError::QueueError(error)))?
                .no_items(),
        );

        if overflow {
            return Err(ShopError::StockInsufficient);
        }
        self.set_stock(new_stock);

        Ok(self.remove_customer(queue_no, 0).unwrap())
    }

    pub fn get_sorted_customers(&self) -> Vec<&Customer> {
        let mut sorted_list = self
            .queues
            .iter()
            .map(|queue| queue.view_data().iter())
            .flatten()
            .collect::<Vec<&Customer>>();
        sorted_list.sort();

        sorted_list
    }
}
