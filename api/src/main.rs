#[warn(unused_assignments)]
#[macro_use] extern crate rocket;
use user_auth_temp::{auth, utils};
use rocket::form::Form;

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



#[launch]
fn rocket() -> _ {
    let sql = utils::setup_db();
    if sql.is_err() {
        panic!("Failed to setup database: {:?}", sql.unwrap_err());
    }
    utils::setup_test_user();
    rocket::build().mount("/", routes![index, register, login])
}