use std::fs::File;
use std::io::{Write};
use std::path::Path;
use rand::Rng;

pub fn write_file(){
    let mut file = File::create(Path::new("./data1.csv")).expect("create failed");;
    let mut index = 0;
    let mut rng = rand::thread_rng();
    let mut total = 0;
    while index < 10 *1024 {
        let nameSuffix = rng.next_u32();
        let age: u32 = rng.gen();
        let locationSuffix: u32 = rng.gen();
        let name = format!("name{},{},location{}\n", nameSuffix, age, locationSuffix);

        file.write_all(name.as_bytes()).expect("write failed");;
        index = index + 1;
    }

    file.write_all("\nutf8".as_bytes()).expect("write failed");
    println!("the total is {}", total);
}