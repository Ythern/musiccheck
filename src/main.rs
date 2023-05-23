use std::fs;
use reqwest;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut i = 0;
    let one_sec = Duration::new(1,0);
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
        let artist_directory = directory.to_owned() + "/" + &path_str;
        let artist_len = artist_directory.len() + 1;
        let paths_artist = fs::read_dir(artist_directory).unwrap();
        let mut owned_album = Vec::new();
        for path_artist in paths_artist {
            let path_artist_string = path_artist.unwrap().path().display().to_string().split_off(artist_len);
            owned_album.push(remove_whitespace(&path_artist_string));
        }
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
            thread::sleep(one_sec);
            let id = &body.unwrap()["results"][0]["id"];
            let url_artist = &("https://api.discogs.com/artists/".to_owned() + &id.to_string()
                + "/releases");
            let release_list = client.get(url_artist)
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await;
            thread::sleep(one_sec);
            for releases in release_list.unwrap()["releases"].as_array(){
                let len = releases.len();
                while i < len {
                    let album = releases[i]["title"].as_str().unwrap().to_string();
                    if !owned_album.contains(&remove_whitespace(&album)) {
                        println!("{} - {}", path_str, album)
                    }
                    i += 1;
                }
            }
        }
    }
}

fn remove_whitespace(s: &str) -> String {
    s.replace(&['\u{200b}'][..], "").chars().filter(|c| !c.is_whitespace()).collect()
}




