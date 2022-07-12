use crate::customer;
use crate::progress_timer;

pub struct ServingCustomer {
    pub customer: customer::Customer,
    pub progress: progress_timer::ProgressTimer
}

pub struct Clerk {
    pub name: String,
    pub serving: Option<ServingCustomer>
}

impl Clerk {
    pub fn new(name: String) -> Clerk {
        Clerk { name: name, serving: None }
    }

    pub fn serve(&mut self, customer: customer::Customer) {
        self.serving = Some(ServingCustomer {
            customer,
            progress: progress_timer::ProgressTimer::start(4)
        });
    }

    pub fn tick(&mut self) {
        if let Some(s) = &self.serving {
            if s.progress.done() {
                self.serving = None
            }
        }
    }
}
