### Explanation

- **Imports**: We import necessary modules from `actix_web`, `serde`, and the standard library.
- **Data Structures**:

  - `InputData`: Used to deserialize incoming JSON data for counter increment.
  - `OutputData`: Used to serialize response data into JSON.
- **Shared State**:

  - `AppState`: Holds a `Mutex`-protected counter that can be shared across handlers.
- **Handlers**:

  - `index`: Responds to GET requests at the root path with a welcome message.
  - `get_counter`: Returns the current value of the counter in JSON format.
  - `increment_counter`: Increments the counter by the value provided in the POST request's JSON body.
- **Main Function**:

  - Sets up the server to listen on `127.0.0.1:8080`.
  - Initializes the shared application state.
  - Configures middleware and routes.
  - Starts the HTTP server.

### Dependencies

Make sure your `Cargo.toml` includes the necessary dependencies:

```toml
[dependencies]
actix-web = "4.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Middleware and Logging

- **Logger Middleware**: Enables logging of incoming requests and responses. Useful for debugging and monitoring.

```rust
.wrap(middleware::Logger::default())
```

To see logs in the console, set the `RUST_LOG` environment variable:

```bash
export RUST_LOG=info
```

### Enabling CORS (Optional)

If your frontend is hosted on a different origin, you may need to enable Cross-Origin Resource Sharing (CORS):

```rust
use actix_cors::Cors;

.wrap(
    Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600),
)
```

Add `actix-cors` to your `Cargo.toml`:

```toml
actix-cors = "0.6"
```

### Interacting with the Substrate Node

To interact with your Substrate node from the backend, you can use the `substrate-api-client` crate or make HTTP requests to your node's RPC endpoints.

#### Example Using `substrate-api-client`

```rust
// Add to Cargo.toml
substrate-api-client = "0.11.0"
```

```rust
use substrate_api_client::{Api, RpcClient};

async fn get_chain_name() -> Result<String, Box<dyn std::error::Error>> {
    let client = RpcClient::new("ws://127.0.0.1:9944");
    let api = Api::new(client).await?;
    let chain_name = api.get_system_chain().await?;
    Ok(chain_name)
}
```

#### Handler Incorporating Substrate Interaction

```rust
async fn chain_info() -> impl Responder {
    match get_chain_name().await {
        Ok(name) => HttpResponse::Ok().body(format!("Connected to chain: {}", name)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
```

Add the route in your `App` configuration:

```rust
.route("/chain-info", web::get().to(chain_info))
```

### Error Handling

Implement proper error handling for production applications. Use `Result` types in your handlers and map errors to appropriate HTTP responses.

```rust
async fn safe_handler() -> Result<HttpResponse, actix_web::Error> {
    // Your logic here
    Ok(HttpResponse::Ok().body("Success"))
}
```

### Environment Variables

For configuration management, consider using environment variables and crates like `dotenv`:

```toml
dotenv = "0.15.0"
```

Load environment variables at the start of your `main` function:

```rust
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // Rest of your code
}
```

Access variables:

```rust
let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
```

### Full `Cargo.toml` Example

```toml
[package]
name = "backend"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.0"
actix-cors = "0.6"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
substrate-api-client = "0.11.0"
dotenv = "0.15.0"
```

### Running the Backend Server

1. **Set Environment Variables** (optional):

   Create a `.env` file in your project root:

   ```
   SERVER_ADDRESS=127.0.0.1:8080
   ```
2. **Run the Server**:

   ```bash
   cargo run
   ```

   Or with logging enabled:

   ```bash
   RUST_LOG=info cargo run
   ```

### Testing the Endpoints

- **Get Counter Value**:

  ```bash
  curl http://127.0.0.1:8080/counter
  ```
- **Increment Counter**:

  ```bash
  curl -X POST -H "Content-Type: application/json" -d '{"value": 5}' http://127.0.0.1:8080/counter/increment
  ```
- **Get Chain Info**:

  ```bash
  curl http://127.0.0.1:8080/chain-info
  ```

### Next Steps

- **Secure the API**: Implement authentication and authorization mechanisms as needed.
- **Database Integration**: Connect to a database (e.g., PostgreSQL, Redis) using crates like `diesel` or `sqlx`.
- **Advanced Error Handling**: Use custom error types and integrate with Actix's error handling.
- **Logging and Monitoring**: Enhance logging and consider integrating with monitoring tools.

---

**Note**: Make sure to adjust the code to fit the specific requirements of your application, especially when interacting with your Substrate node and handling real data.

---

Feel free to ask if you need further assistance or additional features in your backend implementation!
