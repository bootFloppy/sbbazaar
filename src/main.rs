use std::{thread, time, fs};

use std::time::SystemTime;
use chrono::offset::Utc; use chrono::DateTime;use chrono::SecondsFormat;


fn main() {
    let mut count = 0u32;

    let set_time = time::Duration::from_secs(3600);
    
    

    // Infinite loop
    loop {
        //count += 1;
        
        //println!("{}", count);

        let response = reqwest::blocking::get("https://api.hypixel.net/v2/skyblock/bazaar").unwrap();
        //println!("{}", response.text().unwrap());

        let now = Utc::now();
        let str_timestamp =  format!("{}", now.timestamp());

        let _ = fs::write("$HOME/Bazaar/".to_owned() + &str_timestamp + ".json", response.text().unwrap());

        thread::sleep(set_time);
    }
}
