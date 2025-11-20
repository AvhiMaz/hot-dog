#![allow(dead_code)]
#![allow(non_snake_case)]

use dioxus::prelude::*;
fn main() {
    dioxus::launch(App);
}
#[derive(Clone, Copy)]
struct MusicPlayer {
    song: Signal<String>, // Signal inside the context!
}
fn App() -> Element {
    // Provide a signal-based context
    let song = use_signal(|| "Drift Away".to_string());
    use_context_provider(|| MusicPlayer { song });

    rsx! {
        Player {}
        NowPlaying {}
    }
}
fn Player() -> Element {
    rsx! {
        button {
            onclick: move |_| {
                // Any component can modify it
                consume_context::<MusicPlayer>().song.set("Vienna".to_string())
            },
            "Shuffle"
        }
    }
}
fn NowPlaying() -> Element {
    let player = use_context::<MusicPlayer>();
    rsx! {
        // This will auto-update when song changes!
        h3 { "Now playing: {player.song}" }
    }
}
