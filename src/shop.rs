use std::{
    cmp::Ordering,
    fmt::Display,
    fs::File,
    io::{self, Read, Write},
};

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

impl Display for Shop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}\n{}\n", self.stock(), self.len()))?;

        for queue in self.view_data() {
            f.write_fmt(format_args!("{}", queue))?;
        }

        Ok(())
    }
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

    pub fn save_to_file(&self, file: &mut File) -> io::Result<usize> {
        file.write(self.to_string().as_bytes())
    }

    pub fn load_from_file(&mut self, file: &mut File) {
        // TODO remove all unwrap calls
        let mut file_data = String::new();
        file.read_to_string(&mut file_data).unwrap();

        let mut lines = file_data.lines();

        let new_stock: usize = lines.next().unwrap().parse().unwrap();
        let no_queues: usize = lines.next().unwrap().parse().unwrap();

        let mut new_queues: Vec<FoodQueue> = Vec::with_capacity(no_queues);

        for _ in 0..no_queues {
            let queue_id = lines.next().unwrap().parse().unwrap();
            let queue_capacity = lines.next().unwrap().parse().unwrap();
            let queue_length = lines.next().unwrap().parse().unwrap();

            let mut new_queue = FoodQueue::new(queue_id, queue_capacity);

            for _ in 0..queue_length {
                let first_name = lines.next().unwrap();
                let last_name = lines.next().unwrap();
                let no_items = lines.next().unwrap().parse().unwrap();

                let customer =
                    Customer::new(first_name.to_string(), last_name.to_string(), no_items);

                new_queue.add_customer(customer).unwrap();
            }

            new_queues.push(new_queue);
        }

        self.queues = new_queues;
        self.stock = new_stock;
    }
}
