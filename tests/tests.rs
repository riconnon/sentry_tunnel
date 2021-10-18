#[cfg(test)]
mod tests {
    use gotham::test::TestServer;
    use gotham::hyper::http::{header, HeaderValue, StatusCode};

    use mime::Mime;
    use sentry_tunnel::config::Config;
    use sentry_tunnel::server::router;
    use sentry_tunnel::tunnel::BodyError;

    #[test]
    fn test_correct_behaviour() {
        let test_config = Config {
            remote_host: "https://sentry.example.com/".to_string(),
            project_ids: vec!["5".to_string()],
            port: 7878,
            tunnel_path: "/tunnel".to_string(),
            ip: "0.0.0.0".to_string(),
        };
        let test_server = TestServer::new(router(
            &test_config.tunnel_path.clone(),
            test_config.clone(),
        ))
        .unwrap();
        let json = r#"{"sent_at":"2021-10-14T17:10:40.136Z","sdk":{"name":"sentry.javascript.browser","version":"6.13.3"},"dsn":"https://public@sentry.example.com/5"}
        {"type":"session"}
        {"sid":"751d80dc94e34cd282a2cf1fe698a8d2","init":true,"started":"2021-10-14T17:10:40.135Z","timestamp":"2021-10-14T17:10:40.135Z","status":"ok","errors":0,"attrs":{"release":"test_project@1.0"}"#;
        let mime = "application/json".parse::<Mime>().unwrap();
        let response = test_server
            .client()
            .post(
                "http://localhost".to_owned() + &test_config.tunnel_path,
                json,
                mime,
            )
            .with_header(
                header::CONTENT_LENGTH,
                HeaderValue::from_str(&format!("{}", json.as_bytes().len())).unwrap(),
            )
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn test_invalid_project_id() {
        let test_config = Config {
            remote_host: "https://sentry.example.com/".to_string(),
            project_ids: vec!["5".to_string()],
            port: 7878,
            tunnel_path: "/tunnel".to_string(),
            ip: "0.0.0.0".to_string(),
        };
        let test_server = TestServer::new(router(
            &test_config.tunnel_path.clone(),
            test_config.clone(),
        ))
        .unwrap();
        let json = r#"{"sent_at":"2021-10-14T17:10:40.136Z","sdk":{"name":"sentry.javascript.browser","version":"6.13.3"},"dsn":"https://public@sentry.example.com/4"}
        {"type":"session"}
        {"sid":"751d80dc94e34cd282a2cf1fe698a8d2","init":true,"started":"2021-10-14T17:10:40.135Z","timestamp":"2021-10-14T17:10:40.135Z","status":"ok","errors":0,"attrs":{"release":"test_project@1.0"}"#;
        let mime = "application/json".parse::<Mime>().unwrap();
        let response = test_server
            .client()
            .post(
                "http://localhost".to_owned() + &test_config.tunnel_path,
                json,
                mime,
            )
            .with_header(
                header::CONTENT_LENGTH,
                HeaderValue::from_str(&format!("{}", json.as_bytes().len())).unwrap(),
            )
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = response.read_body().unwrap();
        let expc = format!("{}", BodyError::InvalidProjectId);

        assert_eq!(&body[..], expc.as_bytes());
    }

        #[test]
    fn test_missing_dsn() {
        let test_config = Config {
            remote_host: "https://sentry.example.com/".to_string(),
            project_ids: vec!["5".to_string()],
            port: 7878,
            tunnel_path: "/tunnel".to_string(),
            ip: "0.0.0.0".to_string(),
        };
        let test_server = TestServer::new(router(
            &test_config.tunnel_path.clone(),
            test_config.clone(),
        ))
        .unwrap();
        let json = r#"{"sent_at":"2021-10-14T17:10:40.136Z","sdk":{"name":"sentry.javascript.browser","version":"6.13.3"}}
        {"type":"session"}
        {"sid":"751d80dc94e34cd282a2cf1fe698a8d2","init":true,"started":"2021-10-14T17:10:40.135Z","timestamp":"2021-10-14T17:10:40.135Z","status":"ok","errors":0,"attrs":{"release":"test_project@1.0"}"#;
        let mime = "application/json".parse::<Mime>().unwrap();
        let response = test_server
            .client()
            .post(
                "http://localhost".to_owned() + &test_config.tunnel_path,
                json,
                mime,
            )
            .with_header(
                header::CONTENT_LENGTH,
                HeaderValue::from_str(&format!("{}", json.as_bytes().len())).unwrap(),
            )
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = response.read_body().unwrap();
        let expc = format!("{}", BodyError::MissingDsnKeyInHeader);

        assert_eq!(&body[..], expc.as_bytes());
    }
}
