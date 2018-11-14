pub trait Summarizible {
    fn summary(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizible for NewsArticle {
    fn summary(&self) -> String {
        format!("{} {}", self.author, self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizible for Tweet {
    fn summary(&self) -> String {
        format!("{} {}", self.username, self.content)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("headline"),
        author: String::from("author"),
        content: String::from("content"),
    };

    println!("{}", article.summary());

    let tweet = Tweet {
        username: String::from("username"),
        content: String::from("content"),
        reply: true,
        retweet: false,
    };

    println!("{}", tweet.summary());
}
