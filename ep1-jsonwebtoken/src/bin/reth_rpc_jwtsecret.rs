use jsonwebtoken::{encode, get_current_timestamp, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iat: u64,
}

fn main() {
    // run reth with jwtsecret: reth node --rpc.jwtsecret="0000000000000000000000000000000000000000000000000000000000000abc"
    let jwtsecret = "0000000000000000000000000000000000000000000000000000000000000abc";
    let key = EncodingKey::from_secret(&hex::decode(jwtsecret).unwrap());
    let claims = Claims {
        iat: get_current_timestamp(),
    };
    let header = Header::new(Algorithm::HS256);
    println!("jwtsecret: {jwtsecret}\nclaims: {claims:?}\nheader: {header:?}\n");

    // encode jwt
    let jwt = encode(&header, &claims, &key).unwrap();
    println!("{jwt}");
}
