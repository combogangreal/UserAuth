#[warn(unused_assignments)]
#[macro_use] extern crate rocket;
use user_auth_temp::jwt::JwtSecretKey;
use user_auth_temp::{auth, utils, jwt};
use rocket::form::Form;
use rocket::State;
use rocket::http::Status;

#[get("/")]
fn index() -> String {
    return "Hello, please use a proper api link".to_string();
}

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

#[post("/register", data = "<form>")]
fn register(form: Form<RegistrationForm>) -> String {
    let user = auth::sign_up(
        form.username.clone(),
        form.email.clone(),
        form.phone.clone(),
        form.password.clone(),
    );

    if user.success {
        "User created".to_string()
    } else {
        format!("User creation failed: {}", user.error)
    }
}

#[post("/login", data = "<data>")]
fn login(data: Form<LoginData>) -> String {
    let method = &data.method;
    let password = data.password.clone();

    let user = if utils::is_email(method.to_string()) {
        auth::sign_in(auth::SignInMethod::from_email(method.to_string()), password)
    } else if utils::is_phone(method.to_string()) {
        auth::sign_in(auth::SignInMethod::from_phone(method.to_string()), password)
    } else {
        auth::sign_in(auth::SignInMethod::from_username(method.to_string()), password)
    };

    if user.success {
        "User logged in".to_string()
    } else {
        format!("User login failed: {}", user.error)
    }
}

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


#[launch]
fn rocket() -> _ {
    let sql = utils::setup_db();
    if sql.is_err() {
        panic!("Failed to setup database: {:?}", sql.unwrap_err());
    }
    let secret_key = "your_secret_key".to_string();
    let jwt_secret = jwt::JwtSecretKey { secret: secret_key };
    rocket::build().manage(jwt_secret).mount("/", routes![index, register, login, verify_jwt, generate_jwt])
}