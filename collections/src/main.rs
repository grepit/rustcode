
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    //the below line will fail because of borrowing concep
    //let first = &v[0];
    //this will work though
    let first = v[0];

    v.push(6);

    println!("The first element is: {first}");


//imutable print
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

//mutable print 
   let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
println!("{:?}", v);

   let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");
}

