use dotenvy::dotenv;
use  crate::models::{LogType, Log};

mod models;


#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        use  $crate::models::{LogType, Log, Source, LogExt, SourceExt};

        let payload = Log::new()
                        .file(file!().to_string())
                        .line(line!())
                        .message($msg.to_string())
                        .log_level(LogType::Info)
                        .source(Source::new());

        tokio::spawn(async move {
            if let Err(e) = $crate::save(payload).await {
                eprintln!("Failed to send log info: {}", e);
            }
        });
    };
}

#[macro_export]
macro_rules! debug {
    ($msg:expr) => {
        use  $crate::models::{LogType, Log, Source, LogExt, SourceExt};
        let payload = Log::new()
                        .file(file!().to_string())
                        .line(line!())
                        .message($msg.to_string())
                        .log_level(LogType::Debug)
                        .source(Source::new());

        tokio::spawn(async move {
            if let Err(e) = $crate::save(payload).await {
                eprintln!("Failed to send log debug: {}", e);
            }
        });
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        use  $crate::models::{LogType, Log, Source, LogExt, SourceExt};
        
        let payload = Log::new()
                        .file(file!().to_string())
                        .line(line!())
                        .message($msg.to_string())
                        .log_level(LogType::Error)
                        .source(Source::new());

        tokio::spawn(async move {
            if let Err(e) = $crate::save(payload).await {
                eprintln!("Failed to send log error: {}", e);
            }
        });
    };
}

#[macro_export]
macro_rules! warning {
    ($msg:expr) => {
        use  $crate::models::{LogType, Log, Source, LogExt, SourceExt};

        let payload = Log::new()
                        .file(file!().to_string())
                        .line(line!())
                        .message($msg.to_string())
                        .log_level(LogType::Warning)
                        .source(Source::new());

        tokio::spawn(async move {
            if let Err(e) = $crate::save(payload).await {
                eprintln!("Failed to send log warning: {}", e);
            }
        });
    };
}

#[macro_export]
macro_rules! event {
    ($msg:expr) => {
        use  $crate::models::{LogType, Log, Source, LogExt, SourceExt};

        let payload = Log::new()
                        .file(file!().to_string())
                        .line(line!())
                        .message($msg.to_string())
                        .log_level(LogType::Event)
                        .source(Source::new());

        tokio::spawn(async move {
            if let Err(e) = $crate::save(payload).await {
                eprintln!("Failed to send log event: {}", e);
            }
        });
    };
}

#[allow(dead_code)]
async fn save(mut payload: Log) -> Result<(), reqwest::Error> {
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

    payload.system = std::env::var("SYSTEM").ok();

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
    }
}