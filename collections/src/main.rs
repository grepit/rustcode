
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    //the below line will fail because of borrowing concep
    //let first = &v[0];
    //this will work though
    let first = v[0];

    v.push(6);

    println!("The first element is: {first}");
}

