mod move_fn_test;
mod datafusion_example;

#[test]
fn test(){
    let mut st = String::from("hello world");
    let s2 = " cindy";

    st.push_str(s2);

    let s3 = 'c';
    st.push(s3);

    println!("s2 is {s2}");
    println!("st is  {st}");
    println!("s3 is  {s3}");
}