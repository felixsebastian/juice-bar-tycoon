

mod store;
mod blender;
mod clerk;
mod queue;
mod customer;
mod renderer;
mod progress_timer;

const BLEND_SPEED: f32 = 10.;
const SERVING_TIME: u128 = 10000;

struct MenuItem {
    name: String,
    ingredients: [Ingredient]
}

struct Ingredient {
    effort: f32
}

fn main() {    
    let mut boost_juice = store::Store::new("Boost Juice".to_string());

    boost_juice.blenders.push(blender::Blender::new(3000.));
    boost_juice.blenders.push(blender::Blender::new(3000.));
    boost_juice.blenders.push(blender::Blender::new(3000.));
    boost_juice.blenders.push(blender::Blender::new(3000.));
    boost_juice.blenders.push(blender::Blender::new(2000.));
    boost_juice.blenders.push(blender::Blender::new(3000.));
    boost_juice.blenders.push(blender::Blender::new(3000.));
    boost_juice.blenders.push(blender::Blender::new(1000.));
    boost_juice.blenders.push(blender::Blender::new(3000.));
    boost_juice.blenders.push(blender::Blender::new(2000.));

    boost_juice.clerks.push(clerk::Clerk::new("Jack".to_string()));
    boost_juice.clerks.push(clerk::Clerk::new("Jojo".to_string()));

    boost_juice.queue.enqueue(customer::Customer::new("Angela".to_string()));
    boost_juice.queue.enqueue(customer::Customer::new("Tim".to_string()));
    boost_juice.queue.enqueue(customer::Customer::new("Casey".to_string()));
    boost_juice.queue.enqueue(customer::Customer::new("Greg".to_string()));
    boost_juice.queue.enqueue(customer::Customer::new("Anna".to_string()));
    boost_juice.queue.enqueue(customer::Customer::new("George".to_string()));
    boost_juice.queue.enqueue(customer::Customer::new("Ben".to_string()));

    let mut fps = fps_clock::FpsClock::new(30);

    loop {
        clearscreen::clear().unwrap();
        renderer::render_text(&boost_juice);
        boost_juice.tick();
        fps.tick();
    }
}
