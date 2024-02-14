use jsonwebtoken::{encode, Algorithm, EncodingKey, Header, decode, DecodingKey, errors::ErrorKind};
use serde::{Serialize, Deserialize};
use std::time::SystemTime;

pub struct JwtSecretKey {
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub sub: String, 
    pub method: String,
    pub password: String,
    pub exp: usize,
    pub success: bool,
    pub error: String,
}

pub struct JwtReturn {
    pub success: bool,
    pub error: String,
    pub token: String,
}

pub fn generate_token(id: &str, method: &str, password: &str, secret_key: &str) -> JwtReturn {
    let header = Header::new(Algorithm::HS256);
    let claims = UserClaims {
        sub: id.to_string(),
        method: method.to_string(),
        password: password.to_string(),
        exp: 43200,
        success: true,
        error: "".to_string(),
    };
    let token = encode(&header, &claims, &EncodingKey::from_secret(secret_key.as_bytes())).unwrap();
    if token.is_empty() {
        return JwtReturn {
            success: false,
            error: "Token generation failed".to_string(),
            token: "".to_string(),
        };
    }
    JwtReturn {
        success: true,
        error: "".to_string(),
        token,
    }
}

pub fn verify_token(token: &str, secret_key: &str) -> Result<UserClaims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(secret_key.as_ref());
    let decoded = decode::<UserClaims>(token, &decoding_key, &Default::default());

    match decoded {
        Ok(mut token_data) => {
            let current_time = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize;

            if current_time <= token_data.claims.exp {
                Ok(token_data.claims)
            } else {
                token_data.claims.success = false;
                token_data.claims.error = "Token expired".to_string();
                Err(jsonwebtoken::errors::Error::from(ErrorKind::ExpiredSignature))
            }
        }
        Err(err) => Err(err),
    }
}