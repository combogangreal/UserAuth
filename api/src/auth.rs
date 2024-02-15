use sqlite;
use crate::{utils, jwt, SECRET_KEY};
use rocket::State;

// Represents a return type for the auth functions
#[allow(dead_code)]
pub struct ReturnType {
    pub success: bool,
    pub jwt: String,
    pub error: String
}


// Represents all the sign in methods for a user
#[allow(dead_code)]
#[derive(Debug)]
pub struct SignInMethod {
    pub email: bool,
    pub username: bool,
    pub phone_number: bool,
    pub value: String
}
impl SignInMethod {
    pub fn from_email(email: String) -> SignInMethod {
        SignInMethod {
            email: true,
            username: false,
            phone_number: false,
            value: email
        }
    }
    pub fn from_username(username: String) -> SignInMethod {
        SignInMethod {
            email: false,
            username: true,
            phone_number: false,
            value: username
        }
    }
    pub fn from_phone(phone: String) -> SignInMethod {
        SignInMethod {
            email: false,
            username: false,
            phone_number: true,
            value: phone
        }
    }
}

// Sign up a user 
pub fn sign_up(email: String, username: String, password: String, phone_number: String) -> ReturnType {
    let _ = utils::setup_db();
    let conn = sqlite::open("database.db").unwrap();
    let id = utils::generate_id();
    let n_jwt = jwt::generate_token(&id, &email, &password, SECRET_KEY).token;
    if utils::username_exists(username.clone()) {
        return ReturnType {
            success: false,
            jwt: "".to_string(),
            error: "Username already exists".to_string()
        }
    }
    if utils::email_exists(email.clone()) {
        return ReturnType {
            success: false,
            jwt: "".to_string(),
            error: "Email already exists".to_string()
        }
    }
    if utils::phone_exists(phone_number.clone()) {
        return ReturnType {
            success: false,
            jwt: "".to_string(),
            error: "Phone number already exists".to_string()
        }
    }
    let query = format!("
    INSERT INTO users (id, email, username, password, phone, created_at, updated_at, current_jwt, jwt_verified)
    VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')
    ", id, email, username, password, phone_number, utils::get_timestamp(), utils::get_timestamp(), n_jwt, true);
    let sql = conn.execute(query);
    if sql.is_err() {
        return ReturnType {
            success: false,
            jwt: "".to_string(),
            error: sql.unwrap_err().to_string()
        }
    }
    ReturnType {
        success: true,
        jwt: n_jwt,
        error: "".to_string()
    }
}

// Sign in a user
pub fn sign_in(method: SignInMethod, password: String, secret: &State<jwt::JwtSecretKey>) -> ReturnType {
    let _ = utils::setup_db();
    let conn = sqlite::open("database.db").unwrap();
    let mut query = "";
    let mut index = 1;
    if method.email {
        query = "SELECT * FROM Users WHERE email = ?";
    } else if method.username {
        query = "SELECT * FROM Users WHERE email = ?";
        index = 2;
    } else if method.phone_number {
        query = "SELECT * FROM Users WHERE email = ?";
        index = 3;
    }
    for row in conn
    .prepare(query)
    .unwrap()
    .into_iter()
    .bind((index, method.value.as_str()))
    .unwrap()
    .map(|row| row.unwrap())
    {
        let pass = row.read::<&str, _>("password");
        let email = row.read::<&str, _>("email");
        let username = row.read::<&str, _>("username");
        let phone = row.read::<&str, _>("phone");
        if method.email {
            if email != method.value {
                return ReturnType {
                    success: false,
                    jwt: "".to_string(),
                    error: "Incorrect email".to_string()
                }
            }
        } else if method.username {
            if username != method.value {
                return ReturnType {
                    success: false,
                    jwt: "".to_string(),
                    error: "Incorrect username".to_string()
                }
            }
        } else if method.phone_number {
            if phone != method.value {
                return ReturnType {
                    success: false,
                    jwt: "".to_string(),
                    error: "Incorrect phone number".to_string()
                }
            }
        }
        if pass != password {
            return ReturnType {
                success: false,
                jwt: "".to_string(),
                error: "Incorrect password".to_string()
            }
        }
        let current_jwt = row.read::<&str, _>("current_jwt");
        let jwt_verified = row.read::<&str, _>("jwt_verified");
        if jwt_verified == "true" {
            if jwt::verify_token(current_jwt, SECRET_KEY, secret).success {
                return ReturnType {
                    success: true,
                    jwt: current_jwt.to_string(),
                    error: "".to_string(),
                }
            }
        } else {
            let id = row.read::<&str, _>("id");
            let token = jwt::generate_token(id, email, &password, SECRET_KEY);
            let query = format!("UPDATE users SET current_jwt = '{}', jwt_verified = 'true' WHERE id = '{}'", token.token, id);
            let sql = conn.execute(query);
            if !sql.is_err() {
                return ReturnType {
                    success: true,
                    jwt: token.token,
                    error: "".to_string(),
                }
            } else {
                return ReturnType {
                    success: false,
                    jwt: "".to_string(),
                    error: sql.unwrap_err().to_string(),
                }
            }
        }
    }
    ReturnType {
        success: true,
        jwt: jwt::generate_token("", &method.value, &password, SECRET_KEY).token,
        error: "".to_string()
    }
}

// Sign out a user
pub fn sign_out(method: String) -> ReturnType {
    let conn = sqlite::open("database.db").unwrap();   
    let query = format!("UPDATE users SET current_jwt = '', jwt_verified = 'false' WHERE email = '{}' OR username = '{}' OR phone = '{}'", method, method, method);
    let sql = conn.execute(query);
    if !sql.is_err() {
        return ReturnType {
            success: true,
            jwt: "".to_string(),
            error: "".to_string(),
        }
    } else {
        return ReturnType {
            success: false,
            jwt: "".to_string(),
            error: sql.unwrap_err().to_string(),
        }
    }
}