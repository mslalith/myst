use dioxus::prelude::*;

#[derive(Props)]
pub struct InputProps<'a> {
    value: &'a str,
    placeholder: &'a str,
    on_change: EventHandler<'a, Event<FormData>>,
}

pub fn Input<'a>(cx: Scope<'a, InputProps>) -> Element<'a> {
    render! {
        input {
            class: "input input-bordered w-full",
            oninput: move |e| cx.props.on_change.call(e),
            placeholder: "{cx.props.placeholder}",
            value: "{cx.props.value}",
        }
    }
}
