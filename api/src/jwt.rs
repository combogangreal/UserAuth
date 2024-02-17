use jsonwebtoken::{encode, Algorithm, EncodingKey, Header, decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use rocket::State;

// Common JWT Key
pub struct JwtSecretKey {
    pub secret: String,
}

// Claims for JWT Token
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserClaims {
    pub sub: String, 
    pub iat: usize,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub method: String,
    pub password: String,
    pub exp: usize,
    pub success: bool,
    pub error: String,
}

// Reperesents the return of the JWT generation
pub struct JwtReturn {
    pub success: bool,
    pub error: String,
    pub token: String,
}

// Generate JWT Token
pub fn generate_token(id: &str, email: &str, username: &str, phone: &str, method: &str, password: &str, secret_key: &str) -> JwtReturn {
    let header = Header::new(Algorithm::HS256);
    let expiration_time = SystemTime::now()
        .checked_add(Duration::from_secs(43200))  
        .expect("Failed to calculate expiration time");
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let claims = UserClaims {
        sub: id.to_string(),
        iat: iat,
        username: username.to_string(),
        email: email.to_string(),
        phone: phone.to_string(),
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

// Verify JWT Token
pub fn verify_token(token: &str, secret_key: &str, jwt: &State<JwtSecretKey>) -> UserClaims {
    if secret_key != jwt.secret {
        return UserClaims {
            sub: "".to_string(),
            iat: 0,
            username: "".to_string(),
            email: "".to_string(),
            phone: "".to_string(),
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
                    iat: token_data.claims.iat,
                    username: token_data.claims.username,
                    email: token_data.claims.email,
                    phone: token_data.claims.phone,
                    method: token_data.claims.method,
                    password: token_data.claims.password,
                    exp: token_data.claims.exp,
                    success: true,
                    error: "".to_string(),
                }
            } else {
                UserClaims {
                    sub: "".to_string(),
                    iat: 0,
                    username: "".to_string(),
                    email: "".to_string(),
                    phone: "".to_string(),
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
            iat: 0,
            username: "".to_string(),
            email: "".to_string(),
            phone: "".to_string(),
            method: "".to_string(),
            password: "".to_string(),
            exp: 0,
            success: false,
            error: format!("JWT Verification Error: {:?}", err),
        },
    }
}

pub fn decode_token(token: &str, secret_key: &str, jwt: &State<JwtSecretKey>) -> UserClaims {
    if secret_key != jwt.secret {
        return UserClaims {
            sub: "".to_string(),
            iat: 0,
            username: "".to_string(),
            email: "".to_string(),
            phone: "".to_string(),
            method: "".to_string(),
            password: "".to_string(),
            exp: 0,
            success: false,
            error: "Invalid secret key".to_string(),
        };
    }

    let decoded_token = decode::<UserClaims>(token, &DecodingKey::from_secret(jwt.secret.as_bytes()), &Validation::new(Algorithm::HS256));

    if decoded_token.is_err() {
        return UserClaims {
            sub: "".to_string(),
            iat: 0,
            username: "".to_string(),
            email: "".to_string(),
            phone: "".to_string(),
            method: "".to_string(),
            password: "".to_string(),
            exp: 0,
            success: false,
            error: decoded_token.err().unwrap().to_string(),
        };
    }

    UserClaims {
        sub: decoded_token.clone().unwrap().claims.sub,
        iat: decoded_token.clone().unwrap().claims.iat,
        username: decoded_token.clone().unwrap().claims.username,
        email: decoded_token.clone().unwrap().claims.email,
        phone: decoded_token.clone().unwrap().claims.phone,
        method: decoded_token.clone().unwrap().claims.method,
        password: decoded_token.clone().unwrap().claims.password,
        exp: decoded_token.clone().unwrap().claims.exp,
        success: true,
        error: "".to_string(),
    }
}