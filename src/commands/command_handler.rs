pub trait CommandHandler {
    fn get_runnable_cmd(&self, package_manager: &String) -> String;
}
