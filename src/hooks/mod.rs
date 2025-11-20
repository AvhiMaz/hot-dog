// hooks
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
