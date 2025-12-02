pub struct HelloService;

impl HelloService {
    pub fn say_hello(&self) -> String {
        "Hello from Rust-Autumn".to_string()
    }
}
