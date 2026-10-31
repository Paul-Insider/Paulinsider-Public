use paulinsider_web_foundations::{language::initial_language, media_range::parse_single_range};

fn main() {
    let language = initial_language(Some("CA"), "fr-CA,fr;q=0.9,en;q=0.8");
    println!("Initial home: {}", language.home_path());

    let range = parse_single_range("bytes=0-31", 4096).expect("valid demonstration range");
    println!("Content-Range: {}", range.content_range());
    println!("Selected bytes: {}", range.byte_len());
}
