use std::error::Error;

pub trait CommandHandler {
    fn get_runnable_cmd(&self, package_manager: &str) -> Result<String, Box<dyn Error>>;
}
