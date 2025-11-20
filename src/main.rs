use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogCards {}
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
            button { onclick: skip, "skip" }
            button { onclick: save, "save"}
        }
      }
    }
}

// include_str!() vs asset!()
// let content = include_str!("file.txt");
// // "file.txt" content is BAKED INTO the executable
// // Size grows = larger binary
//
// asset!() (Dioxus):
// let path = asset!("image.png");
// // Returns: "/path/to/image-abc123.png"
// // Does NOT include in executable
// // Loaded at runtime via HTTP request
//
// ---
// What "Unique Name" Means
//
// Instead of:
// image.png  ← Same name every time
//
// The macro generates:
// image-abc123.png  ← Unique hash added to filename
// image-xyz789.png  ← Different user's browser gets different name
//
// Why This Matters
//
// 1. Prevents Name Collisions:
// - If you have 2 images both named logo.png in different folders, they won't conflict
//
// 2. Improves Caching:
// - When you update image.png, the new version gets a different hash name
// - Browsers automatically use the new version instead of cached old version
// - Old version is never downloaded again
//
// Example:
// First release:  image-v1-abc.png
// Second release: image-v2-xyz.png  ← Browser knows it's different, downloads new one
//
// using of struct and all..
// first we create instance and then pass it in the component

// #[derive(Props, PartialEq, Clone)]
// struct Cat {
//     name: String,
// }

// #[component]
// fn App() -> Element {
//     let cat = Cat {
//         name: "chippu".to_string(),
//     };
//
//     rsx! {
//
//         div {
//             h1 { CatCard { cat } }
//         }
//         div {
//             button {
//                 onclick: move |_| info!("Clicked!"),
//                 "Click Me"
//             }
//         }
//     }
// }
//
// #[component]
// fn CatCard(cat: Cat) -> Element {
//     rsx! {
//         "hello {cat.name}"
//     }
// }
//
//
//
//
// // hooks
//  Different from React:
// - Dioxus is simpler, less boilerplate
// - No need for useState separately
//
// ---
// Rules for Hooks (Important!)
//
// 1. Only call hooks inside components - Not in regular functions
// 2. Call in same order every time - Don't use hooks in if statements
// 3. Top of component - Call hooks before rendering
//
// Bad (breaking rules):
// if user_clicked {
//     let state = use_hook(|| "value");  // Inside if statement!
// }
//
// Good:
// let state = use_hook(|| "value");  // Top level
// if user_clicked {
//     // Use state here, but don't call hook here
// }
//
//
// signal (in readt usestate)
// //The Basic Idea
// In React, we do:
// javascriptconst [count, setCount] = useState(0);
// In Dioxus with signals:
// rustlet mut count = use_signal(|| 0);
// Example
//#[component]
// fn Count() -> Element {
//     let mut count = use_signal(|| 0);
//     rsx! {
//         document::Stylesheet{href: CSS}
//         div {
//             h1 {  "Count: {count}" }
//             div {
//                 button {
//                     onclick: move |_| count += 1,
//                     style: "cursor: pointer; margin-right: 5px",
//                     "increment"
//                 },
//                 button {
//                     onclick: move |_| count -= 1,
//                     style: "cursor: pointer",
//                     "decrement"
//                 }
//             }
//         }
//     }
// }
