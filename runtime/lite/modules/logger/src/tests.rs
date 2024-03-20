use super::*;
use std::{fs, thread::sleep, time::Duration};

#[test]
fn test_creating_log_file() {
    let _ = CoreLogger::new(Some("test1.log"));
    assert!(fs::metadata("test1.log").is_ok());
    fs::remove_file("test1.log").unwrap();
}

#[test]
fn test_writing_to_log_file() {
    let logger = CoreLogger::new(Some("test2.log"));

    logger.info("test info");
    logger.warn("test warn");
    logger.error("test error");
    logger.debug("test debug");

    // wait for the log to be written
    sleep(Duration::from_millis(1));

    let logs = fs::read_to_string("test2.log").unwrap();
    assert!(!logs.trim().is_empty());

    fs::remove_file("test2.log").unwrap();
}

#[test]
fn test_logger_in_multi_threads(){
    let logger = CoreLogger::new(Some("test3.log"));
    let mut handles = vec![];

    for _ in 0..5 {
        let logger_cpy = logger.clone();

        handles.push(std::thread::spawn(move || {
            logger_cpy.info("test log msg");
        }));
    }

    for handle in handles {
        handle.join().expect("logger thread failed");
    }

    fs::remove_file("test3.log").unwrap();
}