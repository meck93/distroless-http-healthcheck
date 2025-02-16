use std::{env, process::ExitCode};

const DEFAULT_TIMEOUT: u64 = 5;
const HELP_MESSAGE: &str = "\
USAGE:
    healthcheck <URL>

ARGS:
    <URL>    The endpoint URL to check (e.g., http://localhost:8080/health)

OPTIONS:
    help    Show this help message

ENVIRONMENT:
    HEALTHCHECK_HTTP_REQUEST_TIMEOUT    Set request timeout in seconds (default: 5)

EXAMPLE:
    healthcheck http://localhost:8080/health";

#[inline]
fn http_get(endpoint: &str, timeout: u64) -> Result<minreq::Response, minreq::Error> {
    minreq::get(endpoint).with_timeout(timeout).send()
}

#[inline]
fn is_valid_http_url(endpoint: &str) -> Result<(), String> {
    let result = url::Url::parse(endpoint);
    if result.is_err() {
        return Err(format!("Invalid URL: {}", endpoint));
    }
    let url = result.unwrap();
    if url.scheme() != "http" {
        return Err(format!(
            "Invalid URL scheme: '{}'. Only HTTP is supported.",
            url.scheme()
        ));
    }
    Ok(())
}

#[inline]
fn get_timeout() -> u64 {
    env::var("HEALTHCHECK_HTTP_REQUEST_TIMEOUT")
        .unwrap_or(DEFAULT_TIMEOUT.to_string())
        .parse::<u64>()
        .unwrap_or(DEFAULT_TIMEOUT)
}

#[inline]
fn healthcheck(args: Vec<String>, timeout: u64) -> ExitCode {
    if args.len() != 2 || args.get(1).is_some_and(|arg| arg == "help") {
        println!("{}", HELP_MESSAGE);
        return if args.get(1).is_some_and(|arg| arg == "help") {
            ExitCode::from(0)
        } else {
            ExitCode::from(1)
        };
    }

    let endpoint = args.get(1).unwrap();

    let url_validation_result = is_valid_http_url(endpoint);
    if url_validation_result.is_err() {
        println!("{}", url_validation_result.unwrap_err());
        println!("\n{}", HELP_MESSAGE);
        return ExitCode::from(1);
    }

    let result = http_get(endpoint, timeout);
    if result.is_err() {
        println!("{}", result.unwrap_err());
        return ExitCode::from(1);
    }

    let http_status_code = result.unwrap().status_code;
    if http_status_code >= 300 {
        println!("Received status code {}", http_status_code);
        return ExitCode::from(1);
    }

    ExitCode::from(0)
}

fn main() -> ExitCode {
    healthcheck(env::args().collect(), get_timeout())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_valid_url_is_reachable() {
        let result = http_get("http://google.com", DEFAULT_TIMEOUT);

        assert!(result.is_ok())
    }

    #[test]
    fn verify_invalid_url_is_not_reachable() {
        const FAST_TIMEOUT: u64 = 1; // 1 second -> to ensure we don't wait for the default timeout
        let result = http_get("http://test.local/path", FAST_TIMEOUT);

        assert!(result.is_err())
    }

    #[test]
    fn verify_url_validation() {
        assert!(is_valid_http_url("http://example.com").is_ok());

        assert!(is_valid_http_url("https://example.com").is_err());
        assert!(is_valid_http_url("example.com").is_err());
        assert!(is_valid_http_url("ftp://example.com").is_err());
    }

    #[test]
    fn test_run_with_help_argument() {
        let args = vec!["program".to_string(), "help".to_string()];
        assert_eq!(healthcheck(args, DEFAULT_TIMEOUT), ExitCode::from(0));
    }

    #[test]
    fn test_run_with_valid_url() {
        let args = vec!["program".to_string(), "http://google.com".to_string()];
        assert_eq!(healthcheck(args, DEFAULT_TIMEOUT), ExitCode::from(0));
    }

    #[test]
    fn test_run_with_invalid_url() {
        let args = vec!["program".to_string(), "not_a_url".to_string()];
        assert_eq!(healthcheck(args, DEFAULT_TIMEOUT), ExitCode::from(1));
    }

    #[test]
    fn test_run_with_https_url() {
        let args = vec!["program".to_string(), "https://google.com".to_string()];
        assert_eq!(healthcheck(args, DEFAULT_TIMEOUT), ExitCode::from(1));
    }
}
