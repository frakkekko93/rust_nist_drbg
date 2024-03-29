use std::io::prelude::*;
use std::fs::*;

/*  Function used to format a single log message. */
pub fn format_message(failed: bool, alg_name: String, test_name: String, message: String) -> String{
    let mut res = String::new();

    if failed {
        res.push_str("TEST FAILED (");
        res.push_str(&alg_name);
        res.push_str(") - ");
        res.push_str(&test_name);
        res.push_str(": ");
        res.push_str(&message);
        res.push_str("\n");
    }
    else {
        res.push_str("TEST PASSED (");
        res.push_str(&alg_name);
        res.push_str(") - ");
        res.push_str(&test_name);
        res.push_str(": ");
        res.push_str(&message);
        res.push_str("\n");
    }

    res
}

/*  Checks if the result passed is equal to the expected value and shows the appropriate desired message. */
pub fn check_res<T: std::cmp::PartialEq>(result: T, expected: T, test_name: String, module_name: String, fail_msg: String, succ_msg: String) -> usize {
    let mut file;
    match OpenOptions::new().append(true).create(true).open("src/self_tests/logs/test_log.log") {
        Err(err) => {
            panic!("Couldn't open {module_name} test log! (err: {})", err);
        }
        Ok(handle) => {
            file = handle;
        }
    }

    let mut test_id = "".to_string();
    test_id.push_str(test_name.as_str());
    if result != expected {
        match file.write_all(format_message(true, module_name.clone(), test_id, fail_msg).as_bytes()) {
            Err(err) => {
                panic!("Couldn't write to {} log file! (err: {})", module_name, err);
            }
            Ok(_) => {}
        }
        return 1;
    }
    else {
        match file.write_all(format_message(false, module_name.clone(), test_id, succ_msg).as_bytes()) {
            Err(err) => {
                panic!("Couldn't write to {} log file! (err: {})", module_name, err);
            }
            Ok(_) => {}
        }
        return 0;
    }
}

// Writes a into the desired log.
pub fn write_to_log(message: String) {
    let mut file;
    match OpenOptions::new().append(true).create(true).open("src/self_tests/logs/test_log.log") {
        Err(err) => {
            panic!("Couldn't open log! (err: {})", err);
        }
        Ok(handle) => {
            file = handle;
        }
    }

    // Adding timestamp to first logging entry
    let mut actual_mes = message.clone();
    let time = chrono::offset::Local::now().to_string();
    let mut day_hour = time[..19].to_string();
    day_hour.insert_str(day_hour.len(), ": ");
    
    if message.contains("*** STARTING") {
        actual_mes.insert_str(5, &day_hour);
    }

    match file.write_all(actual_mes.as_bytes()) {
        Err(err) => {
            panic!("Couldn't write to log file! (err: {})", err);
        }
        Ok(_) => {}
    };
}