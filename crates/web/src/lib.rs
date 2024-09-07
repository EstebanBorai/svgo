use leptos::{component, view, IntoView};
use leptos_meta::{provide_meta_context, Title};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="SVGO | SVG Optimizer for Rust & WebAssembly (WASM)"/>
    }
}
