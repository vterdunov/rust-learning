pub mod aggregator {

  pub trait Summarizible {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
      format!("Read more from {}...", self.author_summary())
    }

    fn notify<T: Summarizible>(item: T) {
      println!("Breaking news! {}", item.summary())
    }
  }

  pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
  }

  impl Summarizible for Tweet {
    fn author_summary(&self) -> String {
      format!("@{}", self.username)
    }
  }

}
