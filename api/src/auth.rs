use sqlite;
use crate::utils;

#[allow(dead_code)]
pub struct ReturnType {
    pub success: bool,
    pub error: String
}

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

impl SignInMethod {
}

#[allow(dead_code)]

pub fn sign_up(email: String, username: String, password: String, phone_number: String) -> ReturnType {
    let _ = utils::setup_db();
    let conn = sqlite::open("database.db").unwrap();
    if utils::username_exists(username.clone()) {
        return ReturnType {
            success: false,
            error: "Username already exists".to_string()
        }
    }
    if utils::email_exists(email.clone()) {
        return ReturnType {
            success: false,
            error: "Email already exists".to_string()
        }
    }
    if utils::phone_exists(phone_number.clone()) {
        return ReturnType {
            success: false,
            error: "Phone number already exists".to_string()
        }
    }
    let query = format!("
    INSERT INTO users (id, email, username, password, phone, created_at, updated_at)
    VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}')
    ", utils::generate_id(), email, username, password, phone_number, utils::get_timestamp(), utils::get_timestamp());
    let sql = conn.execute(query);
    if sql.is_err() {
        return ReturnType {
            success: false,
            error: sql.unwrap_err().to_string()
        }
    }
    ReturnType {
        success: true,
        error: "".to_string()
    }
}

pub fn sign_in(method: SignInMethod, password: String) -> ReturnType {
    println!("{:?}", method);
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
                    error: "Incorrect email".to_string()
                }
            }
        } else if method.username {
            if username != method.value {
                return ReturnType {
                    success: false,
                    error: "Incorrect username".to_string()
                }
            }
        } else if method.phone_number {
            if phone != method.value {
                return ReturnType {
                    success: false,
                    error: "Incorrect phone number".to_string()
                }
            }
        }
        if pass != password {
            return ReturnType {
                success: false,
                error: "Incorrect password".to_string()
            }
        }
    }
    ReturnType {
        success: true,
        error: "".to_string()
    }
}