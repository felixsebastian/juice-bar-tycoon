use crate::store;

pub fn render_text(store: &store::Store) {
    store.welcome();
    print!("\n");

    println!("There are {} customers in the queue:", store.queue.len());
    print!("\n");

    let mut i = 1;

    for c in &store.queue.customers {
        println!("{}. {}", i, c.name);
        i += 1;
    }

    print!("\n");

    println!("There are {} clerks at the counter:", store.clerks.len());

    let mut j = 1;

    for c in &store.clerks {
        print!("{}. {}", j, c.name);
        
        if let Some(serving) = &c.serving {
            print!(", now serving: {} ({})", serving.customer.name, serving.progress)
        }

        print!("\n");
        j += 1;
    }

    print!("\n");

    for (k, b) in store.blenders.iter().enumerate() {
        println!(
            "Blender {:0>3} | power: {}w | active: {}",
            k,
            b.power,
            if b.active { "YES" } else { "NO" }
        );
    }
}
