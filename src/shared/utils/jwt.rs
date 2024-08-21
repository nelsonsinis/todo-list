use std::env;

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid;

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: Option<usize>,
    sub: uuid::Uuid,
}

pub struct JWT {
    claims: Claims,
}

impl JWT {
    pub fn new(id: uuid::Uuid) -> Self {
        Self {
            claims: Claims { sub: id, exp: None },
        }
    }

    pub fn sign(&mut self, exp: Option<usize>) -> String {
        self.claims.exp = Some(self.calculate_expiration(exp.unwrap_or(100)));

        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref());

        encode(&header, &self.claims, &encoding_key).unwrap()
    }

    pub fn validate(&self, token: String) -> Option<Claims> {
        let decoding_key = DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref());
        let mut validation = Validation::new(Algorithm::HS256);

        validation.validate_exp = true;

        match decode::<Claims>(token.as_str(), &decoding_key, &validation) {
            Ok(data) => Some(data.claims),
            Err(_) => None,
        }
    }

    fn calculate_expiration(&self, exp: usize) -> usize {
        exp * 60 * 60 * 24 * 365
    }
}
