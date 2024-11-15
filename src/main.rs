use dotenv::dotenv;
use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
    fs::OpenOptions,
    io::Write,
    sync::{Arc, Mutex}
};
use http::StatusCode;

mod webhook;

fn main() {
    // loads .env variables
    dotenv().ok();
    // checks and loads the url, if exists, screams at user if not
    let bakalari_url = match std::env::var("BAKALARI_URL") {
        Ok(url) => url,
        Err(e) => {
            println!("Čtení url selhalo, je buď neplatné, nebo chybí");
            println!("{}",e);
            thread::sleep(Duration::from_secs(10));
            "".to_string()
        },
    };

    // checks and loads the refresh interval, if exists
    let refresh_interval = Duration::from_millis(std::env::var("MS_SLEEP_BETWEEN_CHECKS")
        .expect("Chybí čas spaní v .env souboru")
        .parse::<u64>()
        .unwrap_or(600000));

    // loads optional notification channel url
    let mut discord_url: String = "".to_string();
    match std::env::var("DISCORD_NOTIF_URL") {
        Ok(url) => discord_url = url,
        Err(_) => {},
    };
    let mut discord_msg: String = "change".to_string();
    match std::env::var("DISCORD_NOTIF_MSG") {
        Ok(msg) => discord_msg = msg,
        Err(_) => {},
    };


    // creating a daemon to run in the background
    // !ONLY FOR UNIX SYSTEMS (fuck windows (written from windows))
    #[cfg(unix)]
    {
        use daemonize::Daemonize;
        use std::fs::{File, create_dir_all};

        let _ = create_dir_all("./tmp");
        let stdout = File::create("tmp/daemon.out").unwrap();
        let stderr = File::create("tmp/daemon.err").unwrap();

        println!("Spouštím Daemon a logguju...");
        let daemonize = Daemonize::new()
        .working_directory("./")
        .pid_file("tmp/bakalari_tracker.pid")
        .stdout(stdout)
        .stderr(stderr)
        .umask(0o027);
        daemonize.start().expect("Spouštění Daemonu selhalo");
    }
    #[cfg(windows)]
    println!("Daemon není k dispozici na windows, womp womp");

    // creating a shared state to check for changes
    let last_response = Arc::new(Mutex::new(None));

    // main loop
    loop {
        // clones all variables for threads
        let url_clone = bakalari_url.clone();
        let discord_url_clone = discord_url.clone();
        let discord_msg_clone = discord_msg.clone();
        let last_response_clone = Arc::clone(&last_response);

        // spawns thread to contact bakaláři server
        thread::spawn(move || {
            // logs time of request
            let request_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            // sends a get request to the API
            let response = reqwest::blocking::get(url_clone);
            // checks if response is good
            let response_status = match response {
                Ok(res) => res.status(),
                // error is usually only caused by timeout, so return timeout error code
                Err(_e) => StatusCode::from_u16(408).unwrap(),
            };


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

            // lock the shared state of last check
            let mut last = last_response_clone.lock().unwrap();

            // sending a crash notification
            if let Some(last_response) = &*last {
                if *last_response != response_status && response_status != 200 && discord_url_clone != "" {
                    let _ = webhook::send_webhook(&discord_url_clone, &discord_msg_clone);
                }
            }

            // update shared state
            *last = Some(response_status);
        });

        // waits desired ammount of time before contacting again
        thread::sleep(refresh_interval);
    }
}
