extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::fs;
use std::env;


#[derive(Serialize, Deserialize)]
struct TweetData {
    id_str: String,
    retweeted: bool,
    source: String,
    full_text: String,
    created_at: String,
    retweet_count: String,
    favorite_count: String,
}

#[derive(Serialize, Deserialize)]
struct Tweet {
    tweet: TweetData
}

fn main() {
    println!("Deleting tuits!");

    let archive_filepath = env::var("TUISTER_ARCHIVE_FILEPATH").expect("Invalid archive path.");
    let archive_content = fs::read_to_string(archive_filepath)
        .expect("Unable to open archive file");

    let parsed_archive = serde_json::from_str(&archive_content);

    if !parsed_archive.is_ok() {
        println!("Couldn't parse file.")
    }

    let tweets: Vec<Tweet> = parsed_archive.unwrap();


    let mut to_keep = 0;
    for t in tweets.iter() {
        let favorite_count: i32 = t.tweet.favorite_count.parse().unwrap();
        let retweet_count: i32 = t.tweet.retweet_count.parse().unwrap();
        if favorite_count != 0 || retweet_count != 0  {
            to_keep += 1;
            println!("Content <{}>: {} - {} :: {}|{}", t.tweet.full_text, t.tweet.id_str, t.tweet.created_at, favorite_count, retweet_count)
        }
    }

    println!("To keep: {}", to_keep)
}