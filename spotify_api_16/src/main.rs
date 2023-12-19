use futures::TryFutureExt;
use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_usls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}

fn print_track(tracks: Vec<&Track>) {
    for track in tracks {
        println!("{}", track.name);
        println!("{}", track.album.name);
        println!(
            "{}",
            track
                .album
                .artists
                .iter()
                .map(|artist| artist.name.to_string())
                .collect::<String>()
        );
        println!("{}", track.popularity);
        println!("{}", track.external_urls.spotify);
        println!("===============================================================");
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let search_query = &args[1];
    let auth_token = &args[2];

    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = search_query
    );

    // Need Client to make a request
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {}", auth_token))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    
    let json: APIResponse = serde_json::from_str(&res.text().await.unwrap()).unwrap();
    match res.status() {
        reqwest::StatusCode::OK => {
            Ok(()) => print_track(parsed.tracks.items.iter().collect()),
            Err(()) => println!("Response didn't match the Struct"),
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Neet to Grab new Token")
        }

        _ => {
            println!("Something unexpected happened")
        }
    };
}
