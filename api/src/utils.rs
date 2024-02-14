use sqlite;
use crate::auth;
use std::time::SystemTime;
use rand::{distributions::Alphanumeric, Rng};

pub struct AuthReturn {
    pub token: String,
    pub success: bool,
    pub error: String,
}

#[allow(dead_code)]
pub struct User {
    id: String,
    username: String,
    password: String,
    email: String,
    phone: String,
    created_at: String,
    updated_at: String,
}

pub fn setup_db() -> Result<(), sqlite::Error>{
    let conn = sqlite::open("database.db").unwrap();
    let query = "
        CREATE TABLE IF NOT EXISTS Users (
            id TEXT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            email TEXT NOT NULL,
            phone TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            PRIMARY KEY (id, username, email, phone)
        );
    ";
    conn.execute(query)
}

pub fn get_timestamp() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    since_the_epoch.as_secs().to_string()
}

pub fn generate_id() -> String{
    let s: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(7)
    .map(char::from)
    .collect();
    s
}

pub fn setup_test_user() -> AuthReturn {
    let su = auth::sign_up("test@example.com".to_string(), "combogangreal".to_string(), "testpass123".to_string(), "1234567890".to_string());
    AuthReturn {
        token: "testtoken".to_string(),
        success: su.success,
        error: su.error,
    }
}

pub fn sign_in(method: auth::SignInMethod, password: String) -> AuthReturn {
    let si = auth::sign_in(method, password);
    AuthReturn {
        token: "testtoken".to_string(),
        success: si.success,
        error: si.error,
    }
}

pub fn username_exists(username: String) -> bool {
    let conn = sqlite::open("database.db").unwrap();
    let query = "SELECT * FROM users WHERE username = ?";
    for row in conn
        .prepare(query)
        .unwrap()
        .into_iter()
        .bind((1, username.as_str()))
        .unwrap()
        .map(|row| row.unwrap())
    {
        let name =  row.read::<&str, _>("username");
        if name == username {
            return true;
        }   
    }
    return false;
}

pub fn email_exists(email: String) -> bool {
    let conn = sqlite::open("database.db").unwrap();
    let query = "SELECT * FROM users WHERE email = ?";
    for row in conn
        .prepare(query)
        .unwrap()
        .into_iter()
        .bind((1, email.as_str()))
        .unwrap()
        .map(|row| row.unwrap())
    {
        let mail =  row.read::<&str, _>("email");
        if mail == email {
            return true;
        }   
    }
    return false;
}

pub fn phone_exists(phone: String) -> bool {
    let conn = sqlite::open("database.db").unwrap();
    let query = "SELECT * FROM users WHERE phone = ?";
    for row in conn
        .prepare(query)
        .unwrap()
        .into_iter()
        .bind((1, phone.as_str()))
        .unwrap()
        .map(|row| row.unwrap())
    {
        let phonen =  row.read::<&str, _>("phone");
        if phonen == phone {
            return true;
        }   
    }
    return false;
}

pub fn is_email(email: String) -> bool {
    let re = regex::Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    re.is_match(email.as_str())
}

pub fn is_phone(phone: String) -> bool {
    let re = regex::Regex::new(r"^[0-9]{10}$").unwrap();
    re.is_match(phone.as_str())
}