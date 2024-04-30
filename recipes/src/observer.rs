trait Callback {
    fn call(&self, arg: i32);
}
struct Publisher {
    callbacks: Vec<Box<dyn Callback>>
}

impl Publisher {
    fn new() -> Self {
        Publisher {callbacks: vec![]}
    }

    fn subscribe(&mut self, item: Box<dyn Callback>) {
        self.callbacks.push(item)
    }

    fn publish(&mut self, arg: i32) {
        for c in &self.callbacks {
            c.call(arg);
        }
    }
}

struct ListenerType1;

impl Callback for ListenerType1 {
    fn call(&self, arg: i32) {
        println!("ListenerType1 called with argument: {}", arg);
    }
}

struct ListenerType2;

impl Callback for ListenerType2 {
    fn call(&self, arg: i32) {
        println!("ListenerType2 called with argument: {}", arg);
    }
}


fn main() {

    let mut publisher = Publisher::new();
    let l1 = ListenerType1;
    let l2 = ListenerType1;
    let l3 = ListenerType2;
    publisher.subscribe(Box::new(l1));
    publisher.subscribe(Box::new(l2));
    publisher.subscribe(Box::new(l3));
    publisher.publish(32);

}