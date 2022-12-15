use futures::StreamExt;
use leptos::*;
use serde::{Deserialize, Serialize};
use tauri_sys::{event, tauri};

#[derive(Serialize, Deserialize)]
struct GreetCmdArgs {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GreetPayload {
    message: String,
}

async fn greet(name: String) -> String {
    tauri::invoke("greet", &GreetCmdArgs { name })
        .await
        .unwrap()
}

async fn listen_on_event() -> String {
    let mut events = event::listen::<GreetPayload>("custom-event").await.unwrap();
    let event = events.next().await.unwrap();
    log::debug!("Received event {:#?}", event);
    event.payload.message
}

#[component]
pub fn SimpleCounter(cx: Scope, name: String) -> Element {
    let (value, set_value) = create_signal(cx, 0);
    let (event_msg, set_event_msg) = create_signal(cx, "No `event` from Tauri.".to_string());

    let event_res = create_local_resource(cx, move || (), |_| listen_on_event());

    let event_msg_memo = create_memo(cx, move |_| {
        set_event_msg(event_res.read().unwrap_or_default());
    });

    create_effect(cx, move |_| event_msg_memo);

    let msg = create_local_resource(cx, move || name.to_owned(), |name| greet(name));

    view! { cx,
        <div>
            <button on:click=move |_| set_value(0)>"Clear"</button>
            <button on:click=move |_| set_value.update(|value| *value -= 1)>"-1"</button>
            <span>"Value: " {move || value().to_string()} "!"</span>
            <button on:click=move |_| set_value.update(|value| *value += 1)>"+1"</button>
            <p>{msg}</p>
            <p>{event_msg}</p>
        </div>
    }
}
