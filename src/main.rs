#[allow(unused_variables)]
#[allow(unused_imports)]
use egg_mode::{auth, user, tweet::DraftTweet, tweet::{retweets_of_me}};
use egg_mode::entities;
use futures::{StreamExt, TryStreamExt};
use serde::Serialize;
use egg_mode::search::{self, ResultType};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let con_token = egg_mode::KeyPair::new(
        "••••••",
        "••••••",
    );
    let access_token = egg_mode::KeyPair::new(
        "••••••",
        "•••••",
    );

    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };

    if auth::verify_tokens(&token).await.is_ok() {
        println!("Success");
    }
    
    let con_token = egg_mode::KeyPair::new(
        "4E9IhvYn8IVpAOQOopx9YUj8B",
        "VUdZ9EU2K75RU9rwXdPzsGBez1cq7dY3oIlCIfXOBs5MQaGx81",
    );
    let access_token = egg_mode::KeyPair::new(
        "1274672431286775808-axPGIZ8FnMROdhr0H5sRl1TGwelO72",
        "OSvvc6MdHu34JjOtxSjEVUGJh2NhVdEhpgqoKZqvpFFbn",
    );
    let token = egg_mode::Token::Access {
    consumer: con_token,
    access: access_token,
    };
    //verifying authentication
    if auth::verify_tokens(&token).await.is_ok() {
        println!("Verfication: Success!");
    }

    //To load the profile information of a single user
    let me = user::show("niran_jana", &token).await.unwrap();
    println!("{} (@{})", me.name, me.screen_name);

    //post a new tweet 
     let post = DraftTweet::new("iiii");
     let tweet = post.send(&token).await.unwrap();

    //thread
    let draft = DraftTweet::new("ammmm");
    let tweet = draft.send(&token).await.unwrap();

    let draft = DraftTweet::new("in love withhh")
                            .in_reply_to(tweet.id);
    let tweet = draft.send(&token).await.unwrap();

    let draft = DraftTweet::new("neeeeee")
                            .in_reply_to(tweet.id);
    let tweet = draft.send(&token).await.unwrap();

    //search

    let search = search::search("niran_jana")
                    .result_type(ResultType::Recent)
                    .call(&token)
                    .await
                    .unwrap();
    for tweet in &search.statuses {
        println!("(@{}), {}", tweet.user.as_ref().unwrap().screen_name, tweet.text);
    }

    // //followers of 
    // let follwers = user::followers_of("niran_jana", &token).await.unwrap();

    // for f in &follwers {
    //     println!("{}", follwers.name);
    // }


    //collection of tweets
    let collection = retweets_of_me(&token);


    //slice 
    let slice = &text[entities.range.0..entities.range.1];

    let user = user::UserID::ScreenName("niran_jana".into());

    let mut writer = csv::Writer::from_path("map.csv").unwrap();

    let f = egg_mode::tweet::user_timeline::<user::UserID>(user, false, true, &token);

    let (f, feed) = f.start().await.unwrap();

    for tweet in &*feed {
        writer
            .serialize(Tweet {
                userid: tweet.user.as_ref().unwrap().screen_name.to_string(),
                tweet: tweet.text.to_string(),
            })
            .unwrap();
    }

    println!("\nOlder Tweets\n");

    let (f, feed) = f.older(None).await.unwrap();

    for tweet in &*feed {
        writer
            .serialize(Tweet {
                userid: tweet.user.as_ref().unwrap().screen_name.to_string(),
                tweet: tweet.text.to_string(),
            })
            .unwrap();
    }

    println!("\nMore older Tweets \n");

    let (f, feed) = f.older(None).await.unwrap();

    for tweet in &*feed {
        writer
            .serialize(Tweet {
                userid: tweet.user.as_ref().unwrap().screen_name.to_string(),
                tweet: tweet.text.to_string(),
            })
            .unwrap();
    }

    // let file = std::fs::File::create("map.csv").unwrap();
    // let mut writer = csv::Writer::from_path("map.csv").unwrap();

    //writer.serialize(map).unwrap();
    //writer.flush();

    // f.take(20)
    //     .try_for_each(|res| {
    //         println!("{}", res.screen_name);
    //         futures::future::ok(())
    //     })
    //     .await
    //     .unwrap();

    Ok(())
}

#[derive(Serialize)]
struct Tweet {
    userid: String,
    tweet: String,
}
