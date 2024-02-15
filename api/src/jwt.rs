use jsonwebtoken::{encode, Algorithm, EncodingKey, Header, decode, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use rocket::State;

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
    let expiration_time = SystemTime::now()
        .checked_add(Duration::from_secs(43200))  
        .expect("Failed to calculate expiration time");
    let claims = UserClaims {
        sub: id.to_string(),
        method: method.to_string(),
        password: password.to_string(),
        exp: expiration_time.duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to calculate duration since epoch")
        .as_secs() as usize,
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

pub fn verify_token(token: &str, secret_key: &str, jwt: &State<JwtSecretKey>) -> UserClaims {
    if secret_key != jwt.secret {
        return UserClaims {
            sub: "".to_string(),
            method: "".to_string(),
            password: "".to_string(),
            exp: 0,
            success: false,
            error: "Invalid secret key".to_string(),
        };
    }

    match decode::<UserClaims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token_data) => {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize;

            if current_time <= token_data.claims.exp {
                UserClaims {
                    sub: token_data.claims.sub,
                    method: token_data.claims.method,
                    password: token_data.claims.password,
                    exp: token_data.claims.exp,
                    success: true,
                    error: "".to_string(),
                }
            } else {
                UserClaims {
                    sub: "".to_string(),
                    method: "".to_string(),
                    password: "".to_string(),
                    exp: 0,
                    success: false,
                    error: "Token has expired".to_string(),
                }
            }
        }
        Err(err) => UserClaims {
            sub: "".to_string(),
            method: "".to_string(),
            password: "".to_string(),
            exp: 0,
            success: false,
            error: format!("JWT Verification Error: {:?}", err),
        },
    }
}