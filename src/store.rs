use crate::clerk;
use crate::blender;
use crate::queue;

pub struct Store {
    pub name: String,
    pub clerks: Vec<clerk::Clerk>,
    pub blenders: Vec<blender::Blender>,
    pub queue: queue::Queue
}

impl Store {
    pub fn new(name: String) -> Store {
        Store {
            name: name,
            clerks: vec![],
            blenders: vec![],
            queue: queue::Queue::new()
        }
    }

    pub fn welcome(& self) {
        println!("Welcome to {}", self.name)
    }

    pub fn tick(&mut self) {
        for c in &mut self.clerks {
            c.tick();

            if let None = c.serving {
                if self.queue.len() > 0 {
                    if let Some(customer) = self.queue.dequeue() {
                        c.serve(customer)
                    }
                }
            }
            // if let Some(time) = c.time {
            //     if time.try_elapsed().expect("Getting process time failed").as_millis() > SERVING_TIME {
            //         c.active = false;
            //         c.time = None;
            //     }
            // }
        }
    }
}
