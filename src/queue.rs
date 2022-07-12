use crate::customer;
use std::collections::{VecDeque};

pub struct Queue {
    pub customers: VecDeque<customer::Customer>
}

impl Queue {
    pub fn new() -> Queue {
        Queue { customers: VecDeque::new() }
    }

    pub fn len(& self) -> usize {
        self.customers.len()
    }

    pub fn enqueue(&mut self, customer: customer::Customer) {
        self.customers.push_back(customer);
    }

    pub fn dequeue(&mut self) -> Option<customer::Customer> {
        self.customers.pop_front()
    }
}
