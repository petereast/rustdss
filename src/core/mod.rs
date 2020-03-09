// Exists for the lifetime of the application.
pub struct Core {}

impl Core {
    pub fn start() -> Self {
        // Could do something interesting using a threadpool - key-hash sharding for example
        println!("[core] starting core");
        Self {}
    }
}
