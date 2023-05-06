pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {

        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        //Notice the print function will never work here
        //The reason you can't print to the console inside an implementation of a trait like in your example code is because the summarize method is expected to return a String, not to print anything to the console.
       //println!("This is a message printed to the console.");
        format!("{}: {}", self.username, self.content)
        
    }
}