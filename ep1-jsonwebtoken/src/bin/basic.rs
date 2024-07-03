use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: u64,
    role: String,
}

fn main() {
    let secret = "my-secret";
    let claims = Claims {
        user_id: 123,
        role: "admin".to_string(),
    };
    let header = Header::new(Algorithm::HS256);
    println!("secret: {secret}\nclaims: {claims:?}\nheader: {header:?}\n");

    // encode jwt
    let jwt = encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap();
    println!("encoded token: {jwt}\n");

    let mut validation = Validation::new(header.alg);
    validation.set_required_spec_claims::<&str>(&[]);

    // decode jwt and validate signature
    let decoded_token: TokenData<Claims> =
        jsonwebtoken::decode(&jwt, &DecodingKey::from_secret(secret.as_ref()), &validation).unwrap();
    println!("decoded token.header: {:?}\n", decoded_token.header);
    println!("decoded token.claims: {:?}", decoded_token.claims);
}
