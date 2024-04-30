/// Defines the `Logger` trait.
pub trait Logger {
    /// Logs a message at the `info` level.
    ///
    /// # Arguments
    ///
    /// * `msg`: A string instance that represents the message to be logged.
    fn info(&self, msg: &str);

    /// Logs a message at the `info` level.
    ///
    /// # Arguments
    ///
    /// * `msg`: A string instance that represents the message to be logged.
    fn warn(&self, msg: &str);

    /// Logs a message at the `info` level.
    ///
    /// # Arguments
    ///
    /// * `msg`: A string instance that represents the message to be logged.
    fn error(&self, msg: &str);

    /// Logs a message at the `info` level.
    ///
    /// # Arguments
    ///
    /// * `msg`: A string instance that represents the message to be logged.
    fn debug(&self, msg: &str);

    /// Logs a message at the `info` level.
    ///
    /// # Arguments
    ///
    /// * `msg`: A string instance that represents the message to be logged.
    fn trace(&self, msg: &str);

    /// Logs a message at the `info` level.
    ///
    /// # Arguments
    ///
    /// * `msg`: A string instance that represents the message to be logged.
    fn critical(&self, msg: &str);
}
