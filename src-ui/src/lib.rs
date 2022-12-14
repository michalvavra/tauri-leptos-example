use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "shell"])]
    pub async fn open(path: &str, open_with: Option<&str>);

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"], catch)]
    pub fn listen(event_name: &str, cb: &Closure<dyn FnMut(JsValue)>) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
struct GreetCmdArgs {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct GreetPayload {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct TauriEvent {
    event: String,
    payload: GreetPayload,
}

fn tauri_listen(event_name: &str, cb: impl FnMut(JsValue) + 'static) {
    let cb = Closure::wrap(Box::new(cb) as Box<dyn FnMut(JsValue)>);
    let _ = listen(event_name, &cb);
    cb.forget();
}

async fn greet(name: String) -> String {
    let res = invoke("greet", to_value(&GreetCmdArgs { name }).unwrap()).await;
    let msg = res.as_string().unwrap();

    msg
}

#[component]
pub fn SimpleCounter(cx: Scope, name: String) -> Element {
    let (value, set_value) = create_signal(cx, 0);
    let (event_msg, set_event_msg) =
        create_signal(cx, "No `event` message from Tauri.".to_string());

    let message = create_local_resource(cx, move || name.to_owned(), |name| greet(name));
    let tauri_event_listener = create_memo(cx, move |_| {
        tauri_listen("custom-event", move |e| {
            log::debug!("Received event {:#?}", e);

            let tauri_event: TauriEvent = from_value(e).unwrap();
            set_event_msg(tauri_event.payload.message);
        });
    });

    create_effect(cx, move |_| tauri_event_listener);

    view! { cx,
        <div>
            <button on:click=move |_| set_value(0)>"Clear"</button>
            <button on:click=move |_| set_value.update(|value| *value -= 1)>"-1"</button>
            <span>"Value: " {move || value().to_string()} "!"</span>
            <button on:click=move |_| set_value.update(|value| *value += 1)>"+1"</button>
            <p>{message}</p>
            <p>{event_msg}</p>
        </div>
    }
}
