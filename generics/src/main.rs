use generics::largest_thing::*;
use generics::news::{NewsArticle, Summary, Tweet};

fn news_stuff() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as your probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    println!("1 new tweet: {}", tweet.summarize());
}

fn main() {
    news_stuff();

    let number_list = vec![23, 45, 11, 55, 10000000, 2323];
    println!("Result: {}", largest_no_clone_no_copy(&number_list));

    let char_list = vec!['a', 'd', 't', 'h'];
    println!("Result: {}", largest_no_clone_no_copy(&char_list));

    let str_list = vec![
        "hello".to_string(),
        "world".to_string(),
        "abcde".to_string(),
    ];
    println!("Result: {}", largest_no_clone_no_copy(&str_list));

    let s1 = String::from("hello");
    let result;
    {
        let s2 = String::from("world");
        {
            result = largest_lifetimes(s1.as_str(), s2.as_str());
        }

        println!("The result is {}", result);
    }

    let line = String::from("Call me Ishmael...");
    let first_sentence; //= line.split(' ').next().expect("There was no space...");
    {
        first_sentence = line.split(' ').next().expect("There was no space...");
    }
    let ie = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{:?}", ie);
}
