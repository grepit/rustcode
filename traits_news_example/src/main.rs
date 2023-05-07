
use traits_news_example::{NewsArticle, Summary, Tweet};

// fn main() {
//     let news_article = NewsArticle {
//         headline: String::from("Breaking News"),
//         location: String::from("New York"),
//         author: String::from("John Doe"),
//         content: String::from("Lorem ipsum dolor sit amet."),
//     };

//     println!("Headline: {}", news_article.headline);
//     println!("Location: {}", news_article.location);
//     println!("Author: {}", news_article.author);
//     println!("Content: {}", news_article.content);
//     println!("Summary: {}", news_article.summarize());
// }

fn main() {
    let tweet = Tweet {
        username: String::from("John Doe"),
        content: String::from("example tweet content"),
        reply: false,
        retweet: false,
    };
    let summary = tweet.summarize();
    println!("--- here we overwrote the default");
    println!("{}", summary); // prints "example_user: example tweet content"
    

    let news_article = NewsArticle {
        headline: String::from("example headline"),
        location: String::from("example location"),
        author: String::from("example author"),
        content: String::from("example article content"),
    };
    let summary = news_article.summarize();
   
    println!("--- here we used the default");
    println!("{}", summary); // prints "(Read always...)"
}