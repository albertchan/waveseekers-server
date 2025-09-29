mod common;

#[tokio::test]
async fn heartbeat_returns_successfully() {
    // Arrange
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/api/v1/health", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
}
