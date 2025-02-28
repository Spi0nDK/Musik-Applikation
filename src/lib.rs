use std::{fs, io};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use rodio::{Decoder, OutputStream, Sink};

pub fn song_list() -> Result<Vec<PathBuf>, String> {
    let music_dir = Path::new("src/music");
    let mut songs = Vec::new();

    if let Ok(entries) = fs::read_dir(music_dir) {
        for entry in entries.flatten() {
            if let Some(extension) = entry.path().extension() {
                if extension == "mp3" {
                    songs.push(entry.path());
                }
            }
        }
    }
    if songs.is_empty() {
        Err(String::from("Ingen sange fundet!"))
    } else {
        Ok(songs)
    }
}

pub fn play_song(song_path: PathBuf, internet_status: Arc<Mutex<bool>>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = fs::File::open(&song_path).unwrap();
    let source = Decoder::new(io::BufReader::new(file)).unwrap();
    sink.append(source);
    sink.play();

    loop {
        println!("(P)ause, (R)esume, (Q)uit, (T)oggle internet: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim().to_uppercase();

        match command.as_ref() {
            "P" => sink.pause(),
            "R" => {
                let internet = internet_status.lock().unwrap();
                if *internet {
                    sink.play();
                } else {
                    println!("Ingen internetforbindelse! Vent på genoprettelse");
                }
            },
            "Q" => break,
            "T" => {
                simulate_internet(Arc::clone(&internet_status), &sink)
            }
            _ => println!("Ugyldigt input!"),
        }
    }
}

pub fn simulate_internet(internet_status: Arc<Mutex<bool>>, sink: &Sink) {
    let mut status = internet_status.lock().unwrap();
    *status = !*status;
    if *status {
        println!("Internetforbindelsen er nu genoprettet.");
        sink.play();
    } else {
        println!("Internetforbindelsen er nu afbrudt.");
        sink.pause();
    }
}