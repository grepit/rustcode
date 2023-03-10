//find type of object
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn main() {
 let a: String = "hello rust".into();
 //let a: &str = "hello rust";

   // let location:&str = "US";
   println!("-->: {} ",a);
  

 let mut s = String::from("hello");
 let r3 = &mut s; // no problem
  chapter4_2(r3);
   let mut sb = String::from("hello world");
   sb.clear();
  first_word(&sb);



    let sd = String::from("hello world");

    let hello = &sd[0..5];
    let world = &sd[6..11];
    println!("a {}",hello);
    println!("b {}",world);

}

fn chapter4_1() {
    let s = String::from("hello");

    takes_ownership4_1(&s);             
                              

     println!("{}",s) ; 

    let x = 5;                     
    makes_copy4_1(x);                 
} 

 fn takes_ownership4_1(some_string: &str){
 
//fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy4_1(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn chapter4_2(some_string:  & mut str){
    println!("see {}",some_string);
    let mut abc = some_string.to_string();
    abc.push_str("hhh");
 println!("hi {}",abc)   
    
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("item {}",i);
        if item == b' ' {
          println!("found space {}",i);
            return i;
        }
    }

    s.len()
}
