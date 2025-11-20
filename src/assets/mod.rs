use dioxus::prelude::*;

/// Assets in Dioxus Web Applications
///
/// There are two main ways to include files in your Dioxus app:
///
/// 1. include_str!()  - Embed file content directly in binary (larger file size)
/// 2. asset!()        - Reference file path with cache busting (recommended)
///
// APPROACH 1: include_str!() - File Content Embedded in Binary
//
/// Embed a CSS file directly into the binary
/// Use this for small files only (increases binary size)
pub const INLINE_CSS: &str = r#"
    .container {
        max-width: 1200px;
        margin: 0 auto;
    }

    .btn {
        padding: 10px 20px;
        cursor: pointer;
    }
"#;

#[component]
pub fn WithInlineCss() -> Element {
    rsx! {
        style { "{INLINE_CSS}" }
        div {
            class: "container",
            h1 { "Inline CSS Example" }
            button { class: "btn", "Click me" }
        }
    }
}
///
// APPROACH 2: asset!() - Reference External Files (RECOMMENDED)
///
/// Use asset!() to reference images and files
/// Dioxus automatically:
/// - Generates unique filenames with cache hashes
/// - Handles file serving
/// - Updates browser cache when files change
///
/// Example usage:
/// ```rust
/// #[component]
/// fn ImageExample() -> Element {
///     let logo_path = asset!("assets/logo.png");
///     let style_path = asset!("assets/style.css");
///
///     rsx! {
///         document::Stylesheet { href: style_path }
///         img { src: logo_path }
///     }
/// }
/// ```

/// Why asset!() is Better:
///
/// 1. Cache Busting
///    - Old: image.png (stays in browser cache even after update)
///    - New: image-abc123hash.png (browser knows it's different)
///
/// 2. Prevents Name Collisions
///    - Multiple assets/logo.png files won't conflict
///    - Each gets unique hashed name
///
/// 3. Smaller Binary
///    - Files aren't embedded in executable
///    - Loaded at runtime via HTTP
///
/// 4. Better Performance
///    - Assets can be served from CDN
///    - Lazy loaded when needed

#[component]
pub fn AssetExample() -> Element {
    // Note: asset!() works at compile time
    // Actual file paths would be like:
    // let logo = asset!("assets/logo.png");
    // let favicon = asset!("assets/favicon.ico");

    rsx! {
        div {
            h1 { "Asset Examples" }

            // Example 1: Image
            div {
                h2 { "Images" }
                // img { src: asset!("assets/logo.png") }
                p { "Images use cache-busted filenames for updates" }
            }

            // Example 2: Stylesheet
            div {
                h2 { "Stylesheets" }
                // document::Stylesheet { href: asset!("assets/style.css") }
                p { "CSS files are automatically cache-busted" }
            }

            // Example 3: Other Files
            div {
                h2 { "Other Assets" }
                p { "SVGs, fonts, and other files work the same way" }
            }
        }
    }
}

// ============================================================================
// COMPARISON TABLE
// ============================================================================

/// include_str!() vs asset!()
///
/// | Feature | include_str!() | asset!() |
/// |---------|---|---|
/// | Binary Size | Larger (files embedded) | Smaller (files external) |
/// | Cache Busting | No | Yes ✓ |
/// | Update Speed | Slow (rebuild binary) | Fast (update file) |
/// | Best For | Small config files | Images, CSS, fonts |
/// | Lazy Loading | No | Yes ✓ |
/// | CDN Support | No | Yes ✓ |

// ============================================================================
// PRACTICAL EXAMPLE: Complete Component with Assets
// ============================================================================

#[component]
pub fn CompleteExample() -> Element {
    rsx! {
        // Load external stylesheet (cache busted)
        document::Stylesheet {
            href: "/styles/main.css" // In real app: asset!("assets/main.css")
        }

        div {
            class: "app",

            // Header with logo
            header {
                // img {
                //     src: asset!("assets/logo.png"),
                //     alt: "Logo"
                // }
                h1 { "My App" }
            }

            // Main content
            main {
                p { "Images and styles are auto cache-busted!" }
                button { "Learn More" }
            }

            // Footer with icons
            footer {
                p { "© 2024 My Company" }
                // img {
                //     src: asset!("assets/favicon.ico"),
                //     alt: "Favicon"
                // }
            }
        }
    }
}
