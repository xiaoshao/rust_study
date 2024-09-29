use crossbeam::scope;

#[derive(Debug)]
struct User {
    name: String
}
pub fn thread_scope() {

    let user = User{
        name: "zwshao".to_string()
    };

    scope(|s| {
        s.spawn(|_| {
            println!("the name is {}", &user.name);
        });

        s.spawn(|_| {
            println!("the name is {}", &user.name);
        });
    }).unwrap();
}