// pub trait Summary {
//     fn summarize(&self) -> String;
// }


pub trait Summary {
    fn summarize(&self) -> String {
        //String::from("(Read more...)")
        println!("just the default");
        "Read more... returned".to_string()
    }
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



pub struct NewsArticleB {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticleB {}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

pub struct WorldNews {
    pub headline: String,

}

impl Summary for WorldNews {
    // you see here desptite having no implementation still worldnews gets printed ! magic right..
    //you want this feature to be able have some default functionalites for some methods and then for the rest you can overwite it

}

