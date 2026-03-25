pub fn build_response(message: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        message.len(),
        message
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_response_contains_message() {
        let msg = "Hello";
        let response = build_response(msg);

        assert!(response.contains(msg));
    }

    #[test]
    fn test_build_response_has_status() {
        let response = build_response("Test");

        assert!(response.starts_with("HTTP/1.1 200 OK"));
    }

    #[test]
    fn test_content_length_correct() {
        let msg = "Hello";
        let response = build_response(msg);

        assert!(response.contains("Content-Length: 5"));
    }
}