mod option_op;

pub mod test_trait;
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
