use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to connect adress ");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("failed to connect server ");
    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
#[tokio::test]
async fn health_check() {
    let adress = spawn_app();
    let req = reqwest::Client::new();

    let response = req
        .get(&format!("{}/health_check", &adress))
        .send()
        .await
        .expect("Failed to connect server ");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subcribe_return_200_for_valid_data() {
    let address = spawn_app();
    let request = reqwest::Client::new();

    let body = "name=le%20gand&email=Uish_sja%40gmail.com";

    let response = request
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed to execute request ");

    assert_eq!(200, response.status().as_u16());
}
#[tokio::test]
async fn subcribe_return_400_for_unvalid_data() {
    let app_address = spawn_app();
    let req = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "email is missing  "),
        ("email=gundealer%40gmail.com", "name is missing "),
        ("", "both is missing"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = req
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute Request");
        assert_eq!(
            400,
            response.status().as_u16(),
            "failed to exexuted {}",error_message
        );
    }
}
