use dioxus::prelude::*;

pub enum LinkTextType {
    Text(String),
    Link(String, Box<dyn Fn()>),
}

#[derive(Props)]
pub struct LinkTextProps<'a> {
    #[props(default = "")]
    class: &'a str,
    types: Vec<LinkTextType>,
}

pub fn LinkText<'a>(cx: Scope<'a, LinkTextProps<'a>>) -> Element<'a> {
    render! {
        p {
            class: format_args!("text-white {}", cx.props.class),
            cx.props.types.iter().map(|item| match item {
                LinkTextType::Text(value) => rsx! {
                    span { "{value}" }
                },
                LinkTextType::Link(value, on_click) => rsx! {
                    span {
                        class: "link link-primary",
                        onclick: |_| on_click(),
                        "{value}",
                    }
                }
            })
        }
    }
}
