use std::{io};
use std::sync::{Arc, Mutex};
use musicapplication::{song_list, play_song};

fn main() {
    let internet_status = Arc::new(Mutex::new(true));

    match song_list() {
        Ok(songs) => {
            let mut n = 1;
            println!("Vælg din sang ved at indtaste tallet foran sangen");
            for song in &songs {
                if let Ok(rel_path) = song.strip_prefix("src/music") {
                    println!("{}. {:?}", n, rel_path.display());
                }
                n +=1;
            }
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();

            match choice.trim().parse::<usize>() {
                Ok(index) if index > 0 && index <= songs.len() => {
                    let s = &songs[index - 1];
                    println!("Du afspiller nu sangen: {:?}", &s);
                    play_song(s.clone(), Arc::clone(&internet_status));
                }
                _ => println!("Ugyldigt valg!"),
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}