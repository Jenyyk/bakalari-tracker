use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let bakalari_url = std::env::var("BAKALARI_URL").expect("Chybí url v .env souboru");
    println!("{bakalari_url}");
}
