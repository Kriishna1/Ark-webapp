use ark_rest::Client;

#[tokio::test]
#[ignore]
async fn foo() {
    let client = Client::new("http://localhost:7070".to_string());
    let res = client.get_info().await;

    assert!(res.is_ok())
}
