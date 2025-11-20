use dioxus::prelude::*;

mod assets;
mod contextapi;
mod counter;
mod hooks;
mod props;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Title {}
        DogCards{}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div {
            h1 { "Hot Dog" }
        }
    }
}

#[component]
fn DogCards() -> Element {
    let skip = move |_| {};
    let save = move |_| {};
    let img_src = use_hook(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");

    rsx! {
      div {
        img { src: "{img_src}" }
        div {
            button { onclick: skip, "skip"}
            button { onclick: save, "save"}
        }
      }
    }
}
