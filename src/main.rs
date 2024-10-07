use dotenv::dotenv;
use std::{
    thread,
    time::Duration
};

fn main() {
    // loads .env variables
    dotenv().ok();
    let bakalari_url = std::env::var("BAKALARI_URL").expect("Chybí url v .env souboru");
    let refresh_interval = Duration::from_millis(std::env::var("MS_SLEEP_BETWEEN_CHECKS")
        .expect("Chybí čas spaní v .env souboru")
        .parse::<u64>()
        .unwrap_or(600000)
    );
    // main loop
    loop {
        // spawns thread to contact bakaláři server
        let url_clone = bakalari_url.clone();
        thread::spawn(move || {
            // sends a get request to the API
            let response = reqwest::blocking::get(url_clone);
            let response_status = response.unwrap().status();
            if response_status == 200 { println!("good") }
        });

        // waits desired ammount of time before contacting again
        thread::sleep(refresh_interval);
    }
}
