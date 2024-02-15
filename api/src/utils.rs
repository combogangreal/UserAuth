use sqlite;
use std::time::SystemTime;
use rand::{distributions::Alphanumeric, Rng};

// Return data structure for utility methods
pub struct AuthReturn {
    pub token: String,
    pub success: bool,
    pub error: String,
}

// User data structure
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

// Setup database
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
            current_jwt TEXT NOT NULL,
            jwt_verified TEXT NOT NUll,
            PRIMARY KEY (id, username, email, phone)
        );
    ";
    conn.execute(query)
}

// Gets the current timestamp
pub fn get_timestamp() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    since_the_epoch.as_secs().to_string()
}

// Generates a random id
pub fn generate_id() -> String{
    let s: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(7)
    .map(char::from)
    .collect();
    s
}

// Checks if user's username exists
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

// Checks if user's email exists
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

// Checks if user's phone exists
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

// Checks if a string is a email
pub fn is_email(email: String) -> bool {
    let re = regex::Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    re.is_match(email.as_str())
}

// Checks if a string is a phone
pub fn is_phone(phone: String) -> bool {
    let re = regex::Regex::new(r"^[0-9]{10}$").unwrap();
    re.is_match(phone.as_str())
}