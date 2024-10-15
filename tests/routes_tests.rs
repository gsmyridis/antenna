use tokio;
use reqwest::Client;

mod app;
use app::spawn_app;


#[tokio::test]
async fn health_check_success() {
    let app = spawn_app().await;
    let client = Client::new();
    let endpoint = format!("{}/health_check", app.address);
    dbg!(&endpoint);
    let response = client.get(endpoint)
        .send()
        .await
        .expect("Failed to execute GET request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}


#[tokio::test]
async fn subscribe_valid() {
    // Organize
    let app = spawn_app().await;
    let client = Client::new();
    let endpoint = format!("{}/subscription", app.address);
    let body = "name=giorgos%20smyridis&email=gsmyridis%40github.com";
    let response = client
        .post(endpoint)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status().as_u16(), 200);

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch save subscription");

    assert_eq!(saved.email, "gsmyridis@github.com");
    assert_eq!(saved.name, "giorgos smyridis");

}


#[tokio::test]
async fn subscribe_invalid_data() {
    let app = spawn_app().await;
    let client = Client::new();
    let endpoint = format!("{}/subscription", app.address);
    let test_cases = vec![
        ("name=giorgos%20smyridis", "missing email"),
        ("email=georgesmyr%40icloud.com", "missing name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&endpoint)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
    
        assert_eq!(
            response.status().as_u16(), 
            400,
            "The API did not fail with 400 Bad Request when the payload was{error_message}"
        );
    }
}
