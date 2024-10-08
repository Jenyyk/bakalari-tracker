use dotenv::dotenv;
use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
    fs::{File, OpenOptions},
    io::Write
};
use daemonize::Daemonize;

fn main() {
    // loads .env variables
    dotenv().ok();
    let bakalari_url = std::env::var("BAKALARI_URL").expect("Chybí url v .env souboru");
    let refresh_interval = Duration::from_millis(std::env::var("MS_SLEEP_BETWEEN_CHECKS")
        .expect("Chybí čas spaní v .env souboru")
        .parse::<u64>()
        .unwrap_or(600000));

    // creating a daemon to run in the background
    let stdout = File::create("tmp/daemon.out").unwrap();
    let stderr = File::create("tmp/daemon.err").unwrap();

    println!("Starting daemon and logging...");
    let daemonize = Daemonize::new()
        .working_directory("./")
        .pid_file("tmp/bakalari_tracker.pid")
        .stdout(stdout)
        .stderr(stderr)
        .umask(0o027);
    daemonize.start().expect("Failed to start daemon");

    // main loop
    loop {
        // spawns thread to contact bakaláři server
        let url_clone = bakalari_url.clone();
        thread::spawn(move || {
            // logs time of request
            let request_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            // sends a get request to the API
            let response = reqwest::blocking::get(url_clone);
            let response_status = response.unwrap().status();
            // checks if response is good


            // file logging part
            // opens log, creates if doesnt exist
            let log_file = OpenOptions::new()
                .read(true)
                .append(true)
                .create(true)
                .open("log.csv");

            // combines all outputs into string
            let mut result = String::new();
            result.push_str(&request_time.to_string());
            result.push_str(",");
            if response_status == 200 { result.push_str("True"); }
            else { result.push_str("False"); }
            result.push_str(",");
            result.push_str(&response_status.to_string());
            result.push('\n');

            // writes into file
            log_file.expect("failed opening").write(result.as_bytes()).expect("failed writing");
        });

        // waits desired ammount of time before contacting again
        thread::sleep(refresh_interval);
    }
}
