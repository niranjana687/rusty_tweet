#[allow(unused_imports)]
use egg_mode::auth;
use egg_mode::user;
use futures::{StreamExt, TryStreamExt};
use serde::Serialize;

#[tokio::main]
async fn main() -> std::io::Result<()> {
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

    if auth::verify_tokens(&token).await.is_ok() {
        println!("Success");
    }

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
