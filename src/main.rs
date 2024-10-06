use chrono::{Local,Utc};

fn main() {
    //get current date and time in utc
    let now = Utc::now();
    println!("Current date and time in UTC: {}",now);

    //format date and time
    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted date and time: {}",formatted);

    //get local time
    let local = Local::now();
    println!("Current date and time in local: {}",local);
}



