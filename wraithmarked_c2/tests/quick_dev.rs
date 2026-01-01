#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8080")?;

    // hc.do_get("/hello?name=Jen").await?.print().await?;
    //
    hc.do_get("/hello2/mike").await?.print().await?;
    Ok(())
}
