#[warn(unused_assignments)]
#[macro_use] extern crate rocket;
use user_auth_temp::jwt::JwtSecretKey;
use user_auth_temp::{auth, utils, jwt, SECRET_KEY};
use rocket::form::Form;
use rocket::State;
use rocket::http::{Status, Method};
use rocket_cors::{Cors, CorsOptions};
use rocket::serde::json::{Value, json};

// Index route, doesnt do anything
#[get("/")]
fn index() -> String {
    return "Hello, please use a proper api link".to_string();
}

fn cors() -> Cors {
    let cors_options = CorsOptions {
        allowed_origins: CorsOptions::default().allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: CorsOptions::default().allowed_headers,
        allow_credentials: CorsOptions::default().allow_credentials,
        ..Default::default()
    }
    .to_cors()
    .expect("Cors configuration is invalid");

    Cors::from(cors_options)
}


// Data forms for the routes
#[derive(FromForm)]
struct RegistrationForm {
    username: String,
    email: String,
    phone: String,
    password: String,
    secret_key: String,
}

#[derive(FromForm)]
struct LoginData {
    method: String,
    password: String,
    secret_key: String,
}

#[derive(FromForm)]
struct VerifyJwtForm {
    token: String,
    secret_key: String,
}

#[derive(FromForm)]
struct GenerateJwtForm {
    id: String, 
    username: String,
    email: String,
    phone: String,
    method: String, 
    password: String, 
    secret_key: String
}

#[derive(FromForm)]
struct LogoutForm {
    method: String,
    secret_key: String,

}

#[derive(FromForm)]
struct DecodeJwtForm {
    token: String,
    secret_key: String,
}

// Registers a user
#[post("/register", data = "<form>")]
fn register(form: Form<RegistrationForm>, secret: &State<JwtSecretKey>) -> String {
    let secret_key = &form.secret_key;

    if secret_key != &secret.secret {
        return format!("Secret key is invalid");
    }
    let user = auth::sign_up(
        form.email.clone(),
        form.username.clone(),
        form.password.clone(),
        form.phone.clone(),
    );

    if user.success {
        println!("User registered: {}", user.jwt);
        format!("{}", user.jwt)
    } else {
        format!("{}", user.error)
    }
}

// Logs in to the website
#[post("/login", data = "<data>")]
fn login(data: Form<LoginData>, secret: &State<JwtSecretKey>) -> String {
    let secret_key = &data.secret_key;

    if secret_key != &secret.secret {
        return format!("Secret key is invalid");
    }
    let method = &data.method;
    let password = data.password.clone();

    let user = if utils::is_email(method.to_string()) {
        auth::sign_in(auth::SignInMethod::from_email(method.to_string()), password, secret)
    } else if utils::is_phone(method.to_string()) {
        auth::sign_in(auth::SignInMethod::from_phone(method.to_string()), password, secret)
    } else {
        auth::sign_in(auth::SignInMethod::from_username(method.to_string()), password, secret)
    };

    if user.success {
        format!("{}" , user.jwt)
    } else {
        format!("{}", user.error)
    }
}

// Verifies a jwt for a user
#[post("/verifyjwt", data = "<data>")]
fn verify_jwt(data: Form<VerifyJwtForm>, secret: &State<JwtSecretKey>) -> Result<String, Status> {
    let token = &data.token;
    let access_token = &data.secret_key;

    if access_token != &secret.secret {
        return Err(Status::Unauthorized);
    } else {
        let verified = jwt::verify_token(token, access_token, secret);

        if verified.success {
            Ok(format!("{}", token))
        } else {
            println!("{}", verified.error);
            Err(Status::Unauthorized)
        }
    }
}

// Generates a jwt for a user
#[post("/generate_jwt", data = "<data>")]
fn generate_jwt(data: Form<GenerateJwtForm>, secret: &State<JwtSecretKey>) -> String {
    let secret_key = &data.secret_key;

    if secret_key != &secret.secret {
        return format!("Secret key is invalid");
    }
    let id = data.id.clone();
    let username = data.username.clone();
    let email = data.email.clone();
    let phone = data.phone.clone();
    let method = data.method.clone();
    let password = data.password.clone();
    let secret_key = data.secret_key.clone();

    let user = jwt::generate_token(id.as_str(), &username.as_str(), &email.as_str(), &phone.as_str(), method.as_str(), password.as_str(), secret_key.as_str());

    if user.success {
        format!("{}", user.token)
    } else {
        format!("{}", user.error)
    }
}

// Logs out of the website
#[post("/logout", data = "<data>")]
fn logout(data: Form<LogoutForm>, secret: &State<JwtSecretKey>) -> String {
    let secret_key = &data.secret_key;

    if secret_key != &secret.secret {
        return format!("Secret key is invalid");
    }
    let method = &data.method;
    let user = auth::sign_out(method.to_string());
    if user.success {
        format!("User logged out: {}", method)
    } else {
        format!("User logout failed: {}", user.error)
    }
}

#[post("/decodejwt", data = "<data>")]
fn decode_jwt(data: Form<DecodeJwtForm>, secret: &State<JwtSecretKey>) -> Value {
    let secret_key = &data.secret_key;

    if secret_key != &secret.secret {
        return json!({"sub": "", "iat": 0, "username": "", "email": "", "phone": "", "method": "", "password": "", "exp": 0, "success": false, "error": "Invalid secret key."})
    }
    let token = &data.token;
    let decoded = jwt::decode_token(token, secret_key, secret);
    json!({
        "sub": decoded.sub, 
        "iat": decoded.iat,
        "username": decoded.username, 
        "email": decoded.email, 
        "phone": decoded.phone, 
        "method": decoded.method, 
        "password": decoded.password, 
        "exp": decoded.exp, 
        "success": decoded.success, 
        "error": decoded.error
    })
}

#[launch]
fn rocket() -> _ {
    let sql = utils::setup_db();
    if sql.is_err() {
        panic!("Failed to setup database: {:?}", sql.unwrap_err());
    }
    let jwt_secret = jwt::JwtSecretKey { secret: SECRET_KEY.to_string() };
    rocket::build().manage(jwt_secret).mount("/", routes![index, register, login, verify_jwt, generate_jwt, logout, decode_jwt]).attach(cors())
}