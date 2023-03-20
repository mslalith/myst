use std::{sync::RwLock, rc::Rc, future::Future};

use anyhow::Result;
use dioxus::prelude::*;
use dioxus_desktop::use_window;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::spotify::client_config::ClientConfig;

pub struct RequestAuthorizationScreenProps<F, O>
where
    F : FnMut(String) -> O + 'static,
    O : Future<Output = ()>
{
    pub url: String,
    pub on_complete: Rc<RwLock<F>>
}

pub fn RequestAuthorizationScreen<F, O>(cx: Scope<RequestAuthorizationScreenProps<F, O>>) -> Element
where
    F : FnMut(String) -> O + 'static,
    O : Future<Output = ()>
{
    let window = use_window(cx);
    let on_complete = &cx.props.on_complete;

    window.webview.load_url(cx.props.url.as_str());

    cx.spawn({
        to_owned![window, on_complete];

        async move {
            let res = await_action().await;
            match res {
                Ok(redirect_url) => {
                    {
                        let mut callback = on_complete.write().unwrap();
                        let future = (callback)(redirect_url);
                        drop(callback);
                        future
                    }.await;
                    window.close();
                },
                Err(_) => todo!(),
            }
        }
    });

    cx.render(rsx! {
        div {}
    })
}

async fn await_action() -> Result<String> {
    let port = ClientConfig::load_config()?.port;
    let addr = format!("127.0.0.1:{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    let (mut stream, _) = listener.accept().await?;

    let mut buf = [0; 4096];
    let _ = stream.read(&mut buf).await?;
    let data = String::from_utf8(buf.to_vec())?;
    let data: Vec<&str> = data.split_ascii_whitespace().take(2).collect();
    let data = *data.last().unwrap_or(&"");

    let mut redirect_url = "".to_string();
    if data.contains("/callback?code") {
        stream.shutdown().await?;
        let domain = ClientConfig::get_local_server_addr(port);
        redirect_url = format!("{}{}", &domain, data);
    }
    Ok(redirect_url)
}
