/*extern crate egg_mode;
extern crate tokio;
use egg_mode::user::show;



fn main() {
    let con_token = egg_mode::KeyPair::new("consumer key", "consumer secret");
    let request_token = egg_mode::request_token(&con_token, "oob").unwrap();
    let auth_url = egg_mode::authorize_url(&request_token);

}*/

use egg_mode::auth;
use egg_mode::user;
use tokio::main;

#[main]

let con_token = egg_mode::KeyPair::new("4E9IhvYn8IVpAOQOopx9YUj8B", "VUdZ9EU2K75RU9rwXdPzsGBez1cq7dY3oIlCIfXOBs5MQaGx81");
let access_token = egg_mode::KeyPair::new("1274672431286775808-axPGIZ8FnMROdhr0H5sRl1TGwelO72", "OSvvc6MdHu34JjOtxSjEVUGJh2NhVdEhpgqoKZqvpFFbn");
let token = egg_mode::Token::Access{
    consumer: con_token,
    access: access_token,
};

