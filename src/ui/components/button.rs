use dioxus::prelude::*;

#[derive(Props)]
pub struct ButtonProps<'a> {
    text: &'a str,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps>) -> Element<'a> {
    render! {
        button {
            class: "btn btn-primary",
            onclick: move |e| cx.props.on_click.call(e),
            "{cx.props.text}"
        }
    }
}
