use reqwest::StatusCode;

#[tokio::test]
async fn health_check_works() {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3000/healthz")
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn signup_and_login_flow() {
    let client = reqwest::Client::new();

    // 1. Register new user
    let signup_res = client.post("http://localhost:3000/signup")
        .json(&serde_json::json!({
            "email": "test@example.com",
            "password": "password123"
        }))
        .send()
        .await
        .expect("Signup request failed");

    assert_eq!(signup_res.status(), reqwest::StatusCode::CREATED);

    // 2. Login with same credentials
    let login_res = client.post("http://localhost:3000/login")
        .json(&serde_json::json!({
            "email": "test@example.com",
            "password": "password123"
        }))
        .send()
        .await
        .expect("Login request failed");

    assert_eq!(login_res.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn project_access_requires_auth() {
    let client = reqwest::Client::new();

    let res = client.get("http://localhost:3000/projects")
        .send()
        .await
        .expect("Request failed");

    assert_eq!(res.status(), reqwest::StatusCode::UNAUTHORIZED);
}
