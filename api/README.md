# UserAuth Api
This is the local api part, this holds the user creation, and authentication.

## Note
For JWT Generation, we use a secret key for validation, so you will have to change the 
```rust
pub const SECRET_KEY: &str = "your_secret_key";
```
line in the src/lib.rs (specifically on line 5) where 'your_secret_key' will be the secret key.

## Running
You can either build, or just run

### Run
```bash
cargo run 
```

### Build
```bash
cargo build --release
```

## Usage
This is used in the frontend, but you can also directly use it by a curl or postman request.

### Register
```bash
curl -X POST -d "username=hello&email=test@example.com&phone=9492583966&password=testpass123" http://localhost:8000/register
```

### Login
```bash
curl -X POST -d "method=test@example.com&password=testpass123" http://localhost:8000/login
```

### Generate JWT
Replace 'your_secret_key' with your actual secret key.
```bash
curl -X POST -d "id=qj6ukFv&method=test2@example.com&password=testpass1234&secret_key=your_secret_key" http://127.0.0.1:8000/generate_jwt
```

### Validate JWT
Replace 'your_jwt_token' with your actual jwt token, and 'your_secret_key' with your actual secret key.
```bash
curl -X POST -d "token=your_jwt_token&access_token=your_secret_key" http://127.0.0.1:8000/verifyjwt
```

## License
This project is licensed under the [MIT License](https://opensource.org/licenses/MIT)