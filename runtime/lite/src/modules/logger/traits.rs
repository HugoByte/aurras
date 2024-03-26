pub trait Logger {
    fn info(&self, msg: &str);
    fn warn(&self, msg: &str);
    fn error(&self, msg: &str);
    fn debug(&self, msg: &str);
    fn trace(&self, msg: &str);
    fn critical(&self, msg: &str);
}
