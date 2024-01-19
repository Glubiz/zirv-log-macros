use dotenvy::dotenv;
use serde::Serialize;
use std::time::SystemTime;

#[allow(dead_code)]
#[derive(Serialize)]
enum LogType {
    Info,
    Debug,
    Error,
    Warning,
    Event,
}

#[allow(dead_code)]
#[derive(Serialize)]
struct Payload {
    file: Option<String>,
    line: Option<u32>,
    message: Option<String>,
    source: Option<Source>,
    log_level: Option<LogType>,
    timestamp: SystemTime,
}

#[allow(dead_code)]
#[derive(Serialize)]
struct Source {
    name: Option<String>,
    version: Option<String>,
    environment: Option<String>,
    service: Option<String>,
    ip: Option<String>,
}

impl Payload {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            file: None,
            line: None,
            message: None,
            source: None,
            log_level: None,
            timestamp: SystemTime::now(),
        }
    }
    #[allow(dead_code)]
    fn file(mut self, file: String) -> Self {
        self.file = Some(file);
        self
    }
    #[allow(dead_code)]
    fn line(mut self, line: u32) -> Self {
        self.line = Some(line);
        self
    }
    #[allow(dead_code)]
    fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }
    #[allow(dead_code)]
    fn source(mut self, source: Source) -> Self {
        self.source = Some(source);
        self
    }
    #[allow(dead_code)]
    fn log_level(mut self, log_level: LogType) -> Self {
        self.log_level = Some(log_level);
        self
    }
}

impl Source {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            name: None,
            version: None,
            environment: None,
            service: None,
            ip: None,
        }
    }
    #[allow(dead_code)]
    fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    #[allow(dead_code)]
    fn version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }
    #[allow(dead_code)]
    fn environment(mut self, environment: String) -> Self {
        self.environment = Some(environment);
        self
    }
    #[allow(dead_code)]
    fn service(mut self, service: String) -> Self {
        self.service = Some(service);
        self
    }
    #[allow(dead_code)]
    fn ip(mut self, ip: String) -> Self {
        self.ip = Some(ip);
        self
    }
}

#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        {
            let payload = $crate::Payload::new()
                            .file(file!().to_string())
                            .line(line!())
                            .message($msg.to_string())
                            .log_level($crate::LogType::Info)
                            .source($crate::Source::new());

            tokio::spawn(async move {
                if let Err(e) = $crate::save(payload).await {
                    eprintln!("Failed to send log info: {}", e);
                }
            });
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        {
            let payload = $crate::Payload::new()
                            .file(file!().to_string())
                            .line(line!())
                            .message($msg.to_string())
                            .log_level($crate::LogType::Debug)
                            .source($crate::Source::new());

            tokio::spawn(async move {
                if let Err(e) = $crate::save(payload).await {
                    eprintln!("Failed to send log debug: {}", e);
                }
            });
        }
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        {
            let payload = $crate::Payload::new()
                            .file(file!().to_string())
                            .line(line!())
                            .message($msg.to_string())
                            .log_level($crate::LogType::Error)
                            .source($crate::Source::new());

            tokio::spawn(async move {
                if let Err(e) = $crate::save(payload).await {
                    eprintln!("Failed to send log error: {}", e);
                }
            });
        }
    };
}

#[macro_export]
macro_rules! warning {
    ($msg:expr) => {
        {
            let payload = $crate::Payload::new()
                            .file(file!().to_string())
                            .line(line!())
                            .message($msg.to_string())
                            .log_level($crate::LogType::Warning)
                            .source($crate::Source::new());

            tokio::spawn(async move {
                if let Err(e) = $crate::save(payload).await {
                    eprintln!("Failed to send log warning: {}", e);
                }
            });
        }
    };
}

#[macro_export]
macro_rules! event {
    ($msg:expr) => {
        {
            let payload = $crate::Payload::new()
                            .file(file!().to_string())
                            .line(line!())
                            .message($msg.to_string())
                            .log_level($crate::LogType::Event)
                            .source($crate::Source::new());

            tokio::spawn(async move {
                if let Err(e) = $crate::save(payload).await {
                    eprintln!("Failed to send log event: {}", e);
                }
            });
        }
    };
}

#[allow(dead_code)]
async fn save(payload: Payload) -> Result<(), reqwest::Error> {
    dotenv().ok();
    let base_url = std::env::var("LOGGING_URL").unwrap_or("https://localhost:6000".to_string());
    let path = match payload.log_level {
        Some(LogType::Info) => "/log/info",
        Some(LogType::Debug) => "/log/debug",
        Some(LogType::Error) => "/log/error",
        Some(LogType::Warning) => "/log/warning",
        Some(LogType::Event) => "/log/event",
        None => "/log/info",
    };

    reqwest::Client::new()
        .post(format!("{}{}", base_url, path))
        .json(&payload)
        .send()
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};

    #[tokio::test]
    async fn test_info_macro() {
        // Set up a mock server
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/log/info"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        // Use the macro
        info!("Test message");

        // You can add more assertions here to verify the request to the mock server
        // Note: You might need to add a short delay before the assertion
        // to ensure the async task spawned by the macro has time to execute.
    }

    #[tokio::test]
    async fn test_debug_macro() {
        // Set up a mock server
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/log/debug"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        // Use the macro
        debug!("Test message");

        // You can add more assertions here to verify the request to the mock server
        // Note: You might need to add a short delay before the assertion
        // to ensure the async task spawned by the macro has time to execute.
    }

    #[tokio::test]
    async fn test_error_macro() {
        // Set up a mock server
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/log/error"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        // Use the macro
        error!("Test message");

        // You can add more assertions here to verify the request to the mock server
        // Note: You might need to add a short delay before the assertion
        // to ensure the async task spawned by the macro has time to execute.
    }

    #[tokio::test]
    async fn test_event_macro() {
        // Set up a mock server
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/log/event"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        // Use the macro
        event!("Test message");

        // You can add more assertions here to verify the request to the mock server
        // Note: You might need to add a short delay before the assertion
        // to ensure the async task spawned by the macro has time to execute.
    }

    #[tokio::test]
    async fn test_warning_macro() {
        // Set up a mock server
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/log/warning"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        // Use the macro
        warning!("Test message");

        // You can add more assertions here to verify the request to the mock server
        // Note: You might need to add a short delay before the assertion
        // to ensure the async task spawned by the macro has time to execute.
    }
}