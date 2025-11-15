use anyhow::Result;

#[tokio::test]
async fn dev_client() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:9000/trader/v1")?;

    hc.do_get("/accounts/accountNumbers").await?.print().await?;

    hc.do_get("/accounts").await?.print().await?;

    hc.do_get("/accounts/{accountNumber}")
        .await?
        .print()
        .await?;

    hc.do_get("/accounts/{accountNumber}/orders")
        .await?
        .print()
        .await?;

    hc.do_post("/accounts/{accountNumber}/orders", "")
        .await?
        .print()
        .await?;

    hc.do_get("/accounts/{accountNumber}/orders/{orderId}")
        .await?
        .print()
        .await?;

    hc.do_delete("/accounts/{accountNumber}/orders/{orderId}")
        .await?
        .print()
        .await?;

    hc.do_put("/accounts/{accountNumber}/orders/{orderId}", "")
        .await?
        .print()
        .await?;

    hc.do_get("/orders").await?.print().await?;

    hc.do_post("/accounts/{accountNumber}/previewOrder", "")
        .await?
        .print()
        .await?;

    hc.do_get("/accounts/{accountNumber}/transactions")
        .await?
        .print()
        .await?;

    hc.do_get("/accounts/{accountNumber}/transactions/{transactionId}")
        .await?
        .print()
        .await?;

    hc.do_get("/userPreference").await?.print().await?;

    Ok(())
}
