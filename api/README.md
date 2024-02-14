# UserAuth Api
This is the local api part, this holds the user creation, and authentication.

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

## License
This project is licensed under the [MIT License](https://opensource.org/licenses/MIT)