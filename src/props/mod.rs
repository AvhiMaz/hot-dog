use dioxus::prelude::*;

/// Props - Passing data to components
///
/// Create a struct with data, derive Props, Clone, PartialEq
/// Then pass it to your component

#[derive(Props, Clone, PartialEq)]
pub struct CatProps {
    pub name: String,
}

#[component]
pub fn CatCard(props: CatProps) -> Element {
    rsx! {
        div {
            h3 { "Hello {props.name}" }
            button {
                onclick: move |_| println!("Clicked {}", props.name),
                "Click Me"
            }
        }
    }
}

#[component]
pub fn App() -> Element {
    let cat = CatProps {
        name: "chippu".to_string(),
    };

    rsx! {
        div {
            h1 { "My Cat App" }
            CatCard { name: cat.name }
        }
    }
}
