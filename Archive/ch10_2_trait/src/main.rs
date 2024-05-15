use std::fmt::format;

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read More........")
    }
}
pub struct NewArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize_author(&self) -> String {
        // format!("{}, by {}", self.headline, self.author)
        format!("@{}", self.author)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}, by {}", self.content, self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewArticle {
        author: String::from("horse_ebooks"),
        headline: String::from("of course, as you probably already know, people"),
        content: String::from("of course, as you probably already know, people"),
    };

    println!("1 new article: {}", article.summarize());
}
