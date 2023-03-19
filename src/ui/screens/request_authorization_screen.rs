use std::sync::mpsc::SyncSender;

use anyhow::Result;
use dioxus::prelude::*;
use dioxus_desktop::use_window;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::spotify::client_config::ClientConfig;

#[derive(Clone)]
pub struct RequestAuthorizationScreenProps {
    pub url: String,
    pub sender: SyncSender<String>,
}

pub fn RequestAuthorizationScreen(cx: Scope<RequestAuthorizationScreenProps>) -> Element {
    let window = use_window(cx);
    let sender = &cx.props.sender;

    window.webview.load_url(cx.props.url.as_str());

    cx.spawn({
        to_owned![window, sender];
        async move {
            let res = await_action().await;
            dbg!(&res);
            match res {
                Ok(redirect_url) => {
                    let _ = sender.send(redirect_url);
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
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8888").await?;
    let (mut stream, _) = listener.accept().await?;

    let mut buf = [0; 4096];
    let _ = stream.read(&mut buf).await?;
    let data = String::from_utf8(buf.to_vec())?;
    let data: Vec<&str> = data.split_ascii_whitespace().take(2).collect();
    let data = *data.last().unwrap_or(&"");

    let mut redirect_url = "".to_string();
    if data.contains("/callback?code") {
        stream.shutdown().await?;
        let domain = ClientConfig::get_local_server_addr(8888);
        redirect_url = format!("{}{}", &domain, data);
    }
    Ok(redirect_url)
}
