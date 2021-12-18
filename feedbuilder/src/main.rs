

fn main() {
    let host_url = "http://192.168.178.33:8765".to_string();

    let feedpath = format!("{}/rss/feed.xml", host_url);
    let logopath = format!("{}/images/logo.png", host_url);

    println!("Path of feed.xml: {}", feedpath);
}
