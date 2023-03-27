use dioxus::prelude::*;

#[derive(Props)]
pub struct LinkProps<'a> {
    value: &'a str,
    #[props(default = "")]
    class: &'a str,
    on_click: EventHandler<'a, Event<MouseData>>,
}

pub fn Link<'a>(cx: Scope<'a, LinkProps<'a>>) -> Element<'a> {
    render! {
        p {
            class: format_args!("link link-primary {}", cx.props.class),
            onclick: |e| cx.props.on_click.call(e),
            "{cx.props.value}",
        }
    }
}
