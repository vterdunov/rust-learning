extern crate hello_world;

use hello_world::aggregator::Summarizible;
use hello_world::aggregator::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("username"),
        content: String::from("content"),
        reply: true,
        retweet: false,
    };

    println!("{}", tweet.summary());
}
