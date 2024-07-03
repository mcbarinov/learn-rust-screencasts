use jsonwebtoken::{encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: u64,
    role: String,
    exp: u64,
}

fn main() {
    let secret = "my-secret";

    // process jwt with ok exp
    println!("exp: ok");
    let claims_ok = Claims {
        user_id: 123,
        role: "admin".to_string(),
        exp: get_current_timestamp() + 100, // 100 seconds after the current time
    };
    process_jwt_with_exp(secret, claims_ok);
    println!("--------------------------------------------------------------------------------\n");

    // process jwt with bad exp
    println!("exp: bad");
    let claims_bad = Claims {
        user_id: 123,
        role: "admin".to_string(),
        exp: get_current_timestamp() - 100, // 100 seconds before the current time
    };
    process_jwt_with_exp(secret, claims_bad);
}

fn process_jwt_with_exp(secret: &str, claims: Claims) {
    let header = Header::new(Algorithm::HS256);
    println!("secret: {secret}\nclaims: {claims:?}\nheader: {header:?}\n");

    // encode jwt
    let jwt = encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap();
    println!("encoded token: {jwt}\n");

    let validation = Validation::new(header.alg);

    // decode jwt and validate signature
    let decoded_token: TokenData<Claims> =
        jsonwebtoken::decode(&jwt, &DecodingKey::from_secret(secret.as_ref()), &validation).unwrap();
    println!("decoded token.header: {:?}\n", decoded_token.header);
    println!("decoded token.claims: {:?}", decoded_token.claims);
}
