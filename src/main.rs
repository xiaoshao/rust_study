use test_option_op::option_op;

pub mod test_trait;
pub mod test_iterator;

pub mod test_macro;
pub mod test_arc_demo;
pub mod test_file_op;
pub mod test_mutex_demo;
pub mod test_option_op;

pub mod test_parquet_demo;
pub mod test_thread_scope;

fn main() {
    let hello = String::from("hello world");


    option_op::option_op();
    let new_life = Life {
        name: &hello
    };


    println!("{:?}", new_life);
}

fn move_life(_ : String) {

}
#[derive(Debug, Clone)]
struct Life<'a> {
    name: &'a String,
}
