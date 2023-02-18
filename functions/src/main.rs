//find type of object
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn main() {
 let a: String = "hello rust".into();
 //let a: &str = "hello rust";

   // let location:&str = "US";
   println!("-->: {} ",a);
  
  chapter4_1()

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
