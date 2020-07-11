extern crate egg_mode;
use egg_mode::user::show;


async fn user_function() {
    let token = Token(1274672431286775808-axPGIZ8FnMROdhr0H5sRl1TGwelO72);
    let rustlang = show("rustlang", &token).await.unwrap();
    println!("{} (@{})", rustlang.name, rustlang.screen_name);


}

fn main() {

}
