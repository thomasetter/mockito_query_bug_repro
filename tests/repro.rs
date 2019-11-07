use mockito::Matcher;

#[test]
fn query_in_exact_test() {
    env_logger::init();
    let mocker = mockito::mock(
        "GET",
        Matcher::AnyOf(vec![
            Matcher::Exact("/bar?stuff".to_string()),
        ]),
    )
    .expect(1)
    .create();
    let moved_mocker = mocker;
    let client = reqwest::Client::new();
    println!(
        "{:?}",
        client.get(&format!("{}/bar?stuff", mockito::server_url())).send()
    );
    moved_mocker.assert();
}
