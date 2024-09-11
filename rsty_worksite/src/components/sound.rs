use leptos::*;

#[component]
pub fn SoundPage() -> impl IntoView {
    wasm_bindgen_futures::spawn_local(async {
        let sound = web_sys::HtmlAudioElement::new_with_src(
            "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3",
        )
        .unwrap();
        let _ = sound.play().unwrap();
    });

    view! {
        <div>
        <h1>"Sound Page"</h1>
        <p>"You should hear a sound when this page is loaded or refreshed."</p>
        <audio controls autoplay>
            <source src="https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3" type="audio/mpeg" />
            "Your browser does not support the audio element."
        </audio>
    </div>
    }
}
