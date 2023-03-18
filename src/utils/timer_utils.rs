use std::{time::Duration, future::Future};

use tokio::time::error::Elapsed;

pub async fn delay_ms(millis: u64) {
    tokio::time::sleep(Duration::from_millis(millis)).await
}

pub async fn timeout_ms<F>(millis: u64, future: F) -> Result<<F as Future>::Output, Elapsed>
where
    F : Future
{
    tokio::time::timeout(Duration::from_millis(millis), future).await
}
