use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

struct MutexUser{
    name:String
}

pub fn mutex_d(){
    let mutex_user = Arc::new(Mutex::new(MutexUser{name: "zw hello".to_string()}));

    let user_c = mutex_user.clone();

    let t1 = thread::spawn(move || {
        let mut user_c_lock = user_c.lock().unwrap();

        user_c_lock.name = "sprawn changed".to_string();

        drop(user_c_lock);
    });

    let user_b = mutex_user.clone();

    let t2 = thread::spawn(move ||{
        sleep(Duration::from_secs(2));

        println!("the final name is {} ", user_b.lock().unwrap().name);
    });

    t2.join().unwrap();

    t1.join().unwrap();

}