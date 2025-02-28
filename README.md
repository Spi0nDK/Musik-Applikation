# Musik-Applikation
Projektet er en musik applikation i Rust, som bruges til at afspille sange.

Funktionerne: song_list(), play_song() og simulate_internet()

Funktionalitet af funktionerne:
* song_list() finder alle sangene som er inde i src/music og har filtypen .mp3m hvis ikke der er fundet en sang inde i mappen sender den en error besked tilbage med "Ingen sange fundet!"
* play_song() afspiller en af de valgte sange ved brug af rodio. Når man har startet en sang kan man pause, unpause, stoppe eller toggle internet (pause sangen.)
* simulate_internet() bruges til at simulere et internet afbrud, lige nu kan det kun blive forced af brugeren.

Vi har valgt at dele tingene lidt op, så det kun er main som bliver kørt inde i main.rs, imens af metoderne bliver kørt inde i lib.rs og alle sangene er gemt inde i src/music
