use std::process::Command;

fn main() {
    let url = std::env::args().nth(1).expect("No URL given");

    println!("YouRSS - Downloader");
    println!("Trying to download audio from following URL:");
    println!("{}", url);

    let output = Command::new("youtube-dl")
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg("-o")
        .arg("./static/mp3/%(title)s.$(ext)s")
        .arg(url)
        .output()
        .expect("Failed to download file.");

    println!("{:?}", output.stdout.as_slice());
}
