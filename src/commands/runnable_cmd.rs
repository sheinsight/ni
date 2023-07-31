pub trait RunnableCmd {
  fn run_with(&self, package_manager: &String);
}