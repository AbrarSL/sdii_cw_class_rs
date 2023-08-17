use crate::food_queue::FoodQueue;

#[derive(Debug, Clone)]
pub struct Shop {
    queues: Vec<FoodQueue>,
    stock: usize,
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
}
