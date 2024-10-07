use dotenv::dotenv;
use std::{
    thread,
    time::Duration
};

fn main() {
    dotenv().ok();
    let bakalari_url = std::env::var("BAKALARI_URL").expect("Chybí url v .env souboru");
    let refresh_interval = Duration::from_millis(std::env::var("MS_SLEEP_BETWEEN_CHECKS")
        .expect("Chybí čas spaní v .env souboru")
        .parse::<u64>()
        .unwrap_or(600000)
    );
    println!("{bakalari_url}");
    loop {
        thread::spawn(|| {
            println!("printing from thread") // debug
        });
        thread::sleep(refresh_interval);
    }
}
