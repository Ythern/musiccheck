use std::fs;
use reqwest;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let directory = "C:/Users/Ythern/Documents/Musique";
    let blacklist = vec![
        String::from("Various Artists"),
        String::from("Music Bee")

    ];
    let key = "pySEUWvBbSVmDRIsxvno";
    let secret = "KQlCvdHDnEMYpDJtZMqHRRktSMZqxMQP";
    let len = directory.len() + 1;
    let paths = fs::read_dir(directory).unwrap();
    let client = reqwest::Client::builder()
        .user_agent("ythern")
        .build()
        .unwrap();

    for path in paths {
        let path_string = path.unwrap().path().display().to_string().split_off(len);
        let path_str = &path_string;
        if !blacklist.contains(path_str) {
            let url = &("https://api.discogs.com/database/search?q=".to_owned() +
                &str::replace(path_str, " ", "%20")
                + "&type=artist&key=" + key
                + "&secret=" + secret);
            let body = client.get(url)
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await;
            println!("body = {:#?}", body);
            println!("{}",url)
        }
    }
}




