#[warn(unused_assignments)]
#[macro_use] extern crate rocket;
use user_auth_temp::jwt::JwtSecretKey;
use user_auth_temp::{auth, utils, jwt, SECRET_KEY};
use rocket::form::Form;
use rocket::State;
use rocket::http::Status;

// Index route, doesnt do anything
#[get("/")]
fn index() -> String {
    return "Hello, please use a proper api link".to_string();
}


// Data forms for the routes
#[derive(FromForm)]
struct RegistrationForm {
    username: String,
    email: String,
    phone: String,
    password: String,
}

#[derive(FromForm)]
struct LoginData {
    method: String,
    password: String,
}

#[derive(FromForm)]
struct VerifyJwtForm {
    token: String,
    access_token: String,
}

#[derive(FromForm)]
struct GenerateJwtForm {
    id: String, 
    method: String, 
    password: String, 
    secret_key: String
}

#[derive(FromForm)]
struct LogoutForm {
    method: String,
}

// Registers a user
#[post("/register", data = "<form>")]
fn register(form: Form<RegistrationForm>) -> String {
    let user = auth::sign_up(
        form.username.clone(),
        form.email.clone(),
        form.phone.clone(),
        form.password.clone()
    );

    if user.success {
        format!("User created, JWT: {}", user.jwt)
    } else {
        format!("User creation failed: {}", user.error)
    }
}

// Logs in to the website
#[post("/login", data = "<data>")]
fn login(data: Form<LoginData>, secret: &State<JwtSecretKey>) -> String {
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
        format!("User logged in, JWT: {}" , user.jwt)
    } else {
        format!("User login failed: {}", user.error)
    }
}

// Verifies a jwt for a user
#[post("/verifyjwt", data = "<data>")]
fn verify_jwt(data: Form<VerifyJwtForm>, secret: &State<JwtSecretKey>) -> Result<String, Status> {
    let token = &data.token;
    let access_token = &data.access_token;

    if access_token != &secret.secret {
        return Err(Status::Unauthorized);
    } else {
        let verified = jwt::verify_token(token, access_token, secret);

        if verified.success {
            Ok(format!("User jwt verified: {}", token))
        } else {
            println!("User jwt verification failed: {}", verified.error);
            Err(Status::Unauthorized)
        }
    }
}

// Generates a jwt for a user
#[post("/generate_jwt", data = "<data>")]
fn generate_jwt(data: Form<GenerateJwtForm>) -> String {
    let id = data.id.clone();
    let method = data.method.clone();
    let password = data.password.clone();
    let secret_key = data.secret_key.clone();

    let user = jwt::generate_token(id.as_str(), method.as_str(), password.as_str(), secret_key.as_str());

    if user.success {
        format!("User jwt generated: {}", user.token)
    } else {
        format!("User jwt generation failed: {}", user.error)
    }
}

// Logs out of the website
#[post("/logout", data = "<data>")]
fn logout(data: Form<LogoutForm>) -> String {
    let method = &data.method;
    let user = auth::sign_out(method.to_string());
    if user.success {
        format!("User logged out: {}", method)
    } else {
        format!("User logout failed: {}", user.error)
    }
}



#[launch]
fn rocket() -> _ {
    let sql = utils::setup_db();
    if sql.is_err() {
        panic!("Failed to setup database: {:?}", sql.unwrap_err());
    }
    let jwt_secret = jwt::JwtSecretKey { secret: SECRET_KEY.to_string() };
    rocket::build().manage(jwt_secret).mount("/", routes![index, register, login, verify_jwt, generate_jwt, logout])
}