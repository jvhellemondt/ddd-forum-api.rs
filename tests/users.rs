#[cfg(test)]
mod tests {
    use reqwest::Client;
    use serde_json::json;
    use fake::faker::internet::raw::*;
    use fake::faker::name::raw::*;
    use fake::locales::EN;
    use fake::Fake;
    use tokio;

    const BASE_URL: &str = "http://localhost:8080";

    fn generate_fake_user() -> (String, String, String, String) {
        let username: String = Username(EN).fake();
        let email: String = SafeEmail(EN).fake();
        let first_name: String = FirstName(EN).fake();
        let last_name: String = LastName(EN).fake();
        (username, email, first_name, last_name)
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let client = Client::new();
        let (_username, email, _first_name, _last_name) = generate_fake_user();
        let url = format!("{}/users?email={}", BASE_URL, email);

        let response = client.get(&url).send().await.expect("Failed to send request");

        assert_eq!(response.status(), 404, "Expected 404 Not Found");
    }

    #[tokio::test]
    async fn test_put_user_not_found() {
        let client = Client::new();
        let data = json!({ "last_name": "Snow" });
        let url = format!("{}/users/update/101", BASE_URL);

        let response = client.put(&url).json(&data).send().await.expect("Failed to send request");

        assert_eq!(response.status(), 404, "Expected 404 Not Found");
    }

    #[tokio::test]
    async fn test_post_validation_error() {
        let client = Client::new();
        let (username, email, first_name, last_name) = generate_fake_user();
        let data = json!({
            "username": username,
            "email": email,
            "first_name": first_name,
            "last_name": last_name,
            "notValidField": "error"
        });
        let url = format!("{}/users/create", BASE_URL);

        let response = client.post(&url).json(&data).send().await.expect("Failed to send request");

        assert_eq!(response.status(), 400, "Expected 400 Validation Error");
    }

    #[tokio::test]
    async fn test_post_create_user() {
        let client = Client::new();
        let (username, email, first_name, last_name) = generate_fake_user();
        let data = json!({
            "username": username,
            "email": email,
            "first_name": first_name,
            "last_name": last_name
        });
        let url = format!("{}/users/create", BASE_URL);

        let response = client.post(&url).json(&data).send().await.expect("Failed to send request");

        assert_eq!(response.status(), 201, "Expected 201 Created");
    }

    #[tokio::test]
    async fn test_post_username_already_in_use() {
        let client = Client::new();
        let (_username, email, first_name, last_name) = generate_fake_user();
        let data = json!({
            "username": "john",
            "email": email,
            "first_name": first_name,
            "last_name": last_name
        });
        let url = format!("{}/users/create", BASE_URL);

        let response = client.post(&url).json(&data).send().await.expect("Failed to send request");

        assert_eq!(response.status(), 409, "Expected 409 Username Already in Use");
    }

    #[tokio::test]
    async fn test_post_email_already_in_use() {
        let client = Client::new();
        let (username, _email, first_name, last_name) = generate_fake_user();
        let data = json!({
            "username": username,
            "email": "john@example.com",
            "first_name": first_name,
            "last_name": last_name
        });
        let url = format!("{}/users/create", BASE_URL);

        let response = client.post(&url).json(&data).send().await.expect("Failed to send request");

        assert_eq!(response.status(), 409, "Expected 409 Email Already in Use");
    }
}
