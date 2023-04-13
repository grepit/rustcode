
use std::collections::HashMap;

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


        let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}",s3);
    println!("{}",s2);


    let s5 = String::from("tic");
    let s6 = String::from("tac");
    let s7 = String::from("toe");

    let s = s5 + "-" + &s6 + "-" + &s7;
    println!("{}",s);


let hello = "Здравствуйте";
//notice the below is bytes so you have to incroment by 2 
let answer = &hello[0..2];
println!("{}",answer);


 

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}",scores);

    //here how you get a single value by key
    let my_key = String::from("Blue");

    let score = scores.get(&my_key).copied().unwrap_or(0);
    println!("{}",score);

    //this how you can overwrite/update existing key
    scores.insert(String::from("Blue"), 25);

    //or simply check if its there update otherwise insert
    scores.entry(String::from("Blue")).or_insert(50);
    
    println!("{:?}", scores);

    //this is how to get all the values
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

}

