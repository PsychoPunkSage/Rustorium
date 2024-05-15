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

//// TRAITS as Parameters
// fn notify<T: Summary>(item: &T) {
fn notify(item: &impl Summary) {
    // Reference to something that implements Summary.
    println!("Breaking News!! read more from {}", item.summarize_author());
}

/*
Examples::>

pub fn notify<T: Summary + Display>(item1: &T, item2: &T) {...}
pub fn notify(item1: &(impl Summary + Display), item2:  &impl Summary) {...}
pub fn notify<T, U>(item1: &T, item2: &U) -> f64
where T: Display + Clone,
U: Clone + Debug
{

}
*/

//// TRAITS as Return Value
fn returns_summarizable() -> impl Summary {
    /*
    We can only return one `type`. Like if we are returning a `Tweet` we can't return `Tweet` or vice versa.
     */
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
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
    notify(&article);
    notify(&tweet);

    //// TRAITS as Return Value
    println!(
        "{} by {}",
        returns_summarizable().summarize(),
        returns_summarizable().summarize_author()
    );
}
