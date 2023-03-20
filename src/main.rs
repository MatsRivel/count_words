use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let file_names = vec!["../File1.txt", "../File2.txt", "../File3.txt"];
    let data = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    for file_name in file_names {
        let data_clone = Arc::clone(&data);
        
        let handle = thread::spawn(move || {
            let content_string = match fs::read_to_string(file_name){
                Ok(val) => val.replace("\n", "").replace("\r", ""),     // If somethng was read, add it.
                Err(_) => "".to_string(),   // If nothing was read, add nothing.
            };
            let mut data_out = data_clone.lock().unwrap();
            data_out.push(content_string);
        });
        
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let stored_data = data.lock().unwrap();
    for element in stored_data.iter(){
        print!("{} | {}\n",element, element.len());

    }
}