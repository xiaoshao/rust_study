use std::sync::Arc;
use std::thread;

struct ArcUser {
    name: String,
}

pub fn arc_demo() {
    let arc_v = Arc::new(ArcUser {
        name: "hello world".to_string(),
    });

    let user_c = arc_v.clone();

    let t1 = thread::spawn(move || {
        println!("hello user {}", user_c.name);
    });

    let user_b = arc_v.clone();

    let t2 = thread::spawn(move || {
        println!("hello 2 user {}", user_b.name);
    });

    t1.join().unwrap();
    t2.join().unwrap();
}