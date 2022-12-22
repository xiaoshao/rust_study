struct  Extra {
    content: String,
    length: i32
}

pub fn option_op(){

    let opt = Some(Extra {
        content: String::from("hello world"),
        length: 100
    });


    let option = opt.as_ref().and_then(|ex| match ex  {
        Extra{
            content: content1,
            length: length1
        } if content1.len() > 10 && length1.eq(&100) => {
            Some(content1)
        }
        _ => None
    });

    println!("the content is {}", option.unwrap());

}