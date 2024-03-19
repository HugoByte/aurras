#[cfg(test)]
mod tests {
    use crate::logger::CoreLogger;
    use crate::Logger;
    use std::fs;

    #[test]
    fn test_log_file_creation() {
        let log_file_path = "./log.log";
        assert!(
            !file_exists(log_file_path),
            "Log file should not exist before creating CoreLogger"
        );

        let _core_logger = CoreLogger::new();

        assert!(
            file_exists(log_file_path),
            "Log file should be created by CoreLogger"
        );
    }

    fn file_exists(file_path: &str) -> bool {
        fs::metadata(file_path).is_ok()
    }

    #[test]
    fn test_logger_functions() {
        let logger = CoreLogger::new();

        logger.info("This is an info message");
        logger.warn("This is a warning message");
        logger.error("This is an error message");
        logger.debug("This is a debug message");
    }
}
