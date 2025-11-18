use anyhow::Result;

#[tokio::test]
async fn dev_client() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:9000")?;

    // Create a new account
    hc.do_post("/admin/v1/accounts", "").await?.print().await?;

    // Get account numbers to find the one we just created
    let account_numbers_response = hc.do_get("/trader/v1/accounts/accountNumbers").await?;
    account_numbers_response.print().await?;

    // Parse the response to get the hash value
    let body = account_numbers_response.json_body()?;
    let accounts = body.as_array().expect("Should be an array");
    let hash_value = accounts[0]["hashValue"]
        .as_str()
        .expect("hashValue should be a string");

    println!(
        "\n>>> Testing account reset for hash_value: {}\n",
        hash_value
    );

    // Reset the account
    hc.do_post(&format!("/admin/v1/accounts/{}/reset", hash_value), "")
        .await?
        .print()
        .await?;

    // Verify account still exists with $200k balance
    let account_response = hc
        .do_get(&format!("/trader/v1/accounts/{}", hash_value))
        .await?;
    account_response.print().await?;

    // Verify balance is $200,000
    let account_body = account_response.json_body()?;
    let cash_available = account_body["currentBalances"]["cashAvailableForTrading"]
        .as_f64()
        .expect("cashAvailableForTrading should be a number");
    assert_eq!(
        cash_available, 200_000.0,
        "Balance should be reset to $200,000"
    );

    println!(
        "\n>>> Testing account deletion for hash_value: {}\n",
        hash_value
    );

    // Delete the account using hash value
    hc.do_delete(&format!("/admin/v1/accounts/{}", hash_value))
        .await?
        .print()
        .await?;

    // Verify account is gone - should return empty array
    hc.do_get("/trader/v1/accounts/accountNumbers")
        .await?
        .print()
        .await?;

    hc.do_get("/trader/v1/accounts").await?.print().await?;

    Ok(())
}
