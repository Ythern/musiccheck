use std::fs;
use std::fs::File;
use std::io::Write;
use reqwest;
use std::thread;
use std::time::Duration;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Answer {
    artists: Vec<Artist>,
}

#[derive(Deserialize)]
struct Artist {
    id: String,
}

#[tokio::main]
async fn main() {
    let mut data_file = File::create("download_list.txt").expect("creation failed");
    let mut i = 0;
    let one_sec = Duration::new(2,0);
    let directory = "D:/Musique";
    let blacklist = vec![
        String::from("Various Artists"),
        String::from("MusicBee"),
        String::from("AJR"),
        String::from("Apashe"),
        String::from("Delta Heavy"),
        String::from("PSY"),
        String::from("Eurythmics"),
        String::from("Ken Ashcorp"),
        String::from("Smash Mouth"),
        String::from("Unlike Pluto"),
        String::from("Imagine Dragons"),
        String::from("Lemaitre")
    ];
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
            owned_album.push(simplify(&(&path_artist_string)));
        }
        if !blacklist.contains(path_str) {
            let url = &("https://musicbrainz.org/ws/2/artist/?query=".to_owned() +
                &str::replace(path_str, " ", "%20")
                + "&fmt=json");
            println!("{}", url);
            let body = client.get(url)
                .send()
                .await
                .unwrap();
            thread::sleep(one_sec);
            let answer: Answer = body.json().await.unwrap();
            if !answer.artists.is_empty() {
                let id = &answer.artists.get(0).unwrap().id;
                let url_artist = &("https://musicbrainz.org/ws/2/release-group?query=%22%22%20AND%20arid:".to_owned() +
                    &id
                    + "%20NOT%20primarytype:single%20NOT%20secondarytype:*&fmt=json");
                println!("{}", url_artist);
                let release_list = client.get(url_artist)
                    .send()
                    .await
                    .unwrap()
                    .json::<serde_json::Value>()
                    .await;
                thread::sleep(one_sec);
                for releases in release_list.unwrap()["release-groups"].as_array() {
                    let len = releases.len();
                    while i < len {
                        let album = &releases[i]["title"].as_str().unwrap().to_string();
                        if !owned_album.contains(&simplify(&album)) {
                            data_file.write((path_str.to_owned() + " - " + album + "\n").as_bytes()).expect("write failed");
                        }
                        i += 1;
                    }
                    i = 0;
                }
            }
        }
    }
}

fn simplify(s: &str) -> String {
    s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect()
}





