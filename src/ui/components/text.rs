use dioxus::prelude::*;

#[derive(Props)]
pub struct TextProps<'a> {
    value: String,
    #[props(default = "")]
    class: &'a str,
    #[props(default = "text-white")]
    color_class: &'a str,
}

pub fn Text<'a>(cx: Scope<'a, TextProps<'a>>) -> Element<'a> {
    render! {
        p {
            class: format_args!("{} {}", cx.props.color_class, cx.props.class),
            "{cx.props.value}",
        }
    }
}
