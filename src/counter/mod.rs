use dioxus::prelude::*;

/// Counter Component Example
///
/// Demonstrates:
/// - use_signal() for state management
/// - Event handlers with move closures
/// - Reactive UI updates
///
/// In React:
/// ```javascript
/// const [count, setCount] = useState(0);
/// ```
///
/// In Dioxus:
/// ```rust
/// let mut count = use_signal(|| 0);
/// ```
///
/// Key Differences:
/// - Dioxus signals are simpler and more efficient
/// - No separate setter function needed
/// - Automatic re-rendering on value changes
#[component]
pub fn Count() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            class: "counter-container",
            h1 { "Count: {count}" }

            div {
                class: "button-group",
                button {
                    onclick: move |_| count += 1,
                    class: "btn btn-increment",
                    "Increment"
                },
                button {
                    onclick: move |_| count -= 1,
                    class: "btn btn-decrement",
                    "Decrement"
                },
                button {
                    onclick: move |_| count.set(0),
                    class: "btn btn-reset",
                    "Reset"
                }
            }

            p { "Current count: {count}" }
        }
    }
}

/// Advanced Counter with Callbacks
///
/// Shows how to use derived signals and computed values
#[component]
pub fn CounterWithStats() -> Element {
    let mut count = use_signal(|| 0);

    // Derived signal - computed value based on count
    let is_even = count() % 2 == 0;
    let doubled = count() * 2;

    rsx! {
        div {
            h2 { "Advanced Counter" }
            p { "Value: {count}" }
            p { "Even: {is_even}" }
            p { "Doubled: {doubled}" }

            button { onclick: move |_| count += 1, "+" }
            button { onclick: move |_| count -= 1, "-" }
        }
    }
}
