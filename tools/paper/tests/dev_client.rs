use anyhow::Result;

#[tokio::test]
async fn dev_client() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:9000")?;

    hc.do_post("/admin/v1/accounts", "").await?.print().await?;

    hc.do_get("/trader/v1/accounts/accountNumbers")
        .await?
        .print()
        .await?;

    // hc.do_get("/trader/v1/accounts").await?.print().await?;

    // hc.do_get(
    //     "/trader/v1/accounts/3A4C2AAD83EC088E054616845C9E48A80FEFEE050A547530198126A67DA9E47B",
    // )
    // .await?
    // .print()
    // .await?;

    // hc.do_get("/trader/v1/accounts/{accountNumber}/orders")
    //     .await?
    //     .print()
    //     .await?;

    // hc.do_post("/trader/v1/accounts/{accountNumber}/orders", "")
    //     .await?
    //     .print()
    //     .await?;

    // hc.do_get("/trader/v1/accounts/{accountNumber}/orders/{orderId}")
    //     .await?
    //     .print()
    //     .await?;

    // hc.do_delete("/trader/v1/accounts/{accountNumber}/orders/{orderId}")
    //     .await?
    //     .print()
    //     .await?;

    // hc.do_put("/trader/v1/accounts/{accountNumber}/orders/{orderId}", "")
    //     .await?
    //     .print()
    //     .await?;

    // hc.do_get("/trader/v1/orders").await?.print().await?;

    // hc.do_post("/trader/v1/accounts/{accountNumber}/previewOrder", "")
    //     .await?
    //     .print()
    //     .await?;

    // hc.do_get("/trader/v1/accounts/{accountNumber}/transactions")
    //     .await?
    //     .print()
    //     .await?;

    // hc.do_get("/trader/v1/accounts/{accountNumber}/transactions/{transactionId}")
    //     .await?
    //     .print()
    //     .await?;

    // hc.do_get("/trader/v1/userPreference").await?.print().await?;

    Ok(())
}
