
use serde::{Serialize, Deserialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
pub enum LogType {
    Info,
    Debug,
    Error,
    Warning,
    Event,
}

#[derive(Serialize, Deserialize)]
pub struct Log {
    pub user_id: Option<String>,
    pub system: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub message: Option<String>,
    pub source: Option<Source>,
    pub log_level: Option<LogType>,
    pub timestamp: SystemTime,
}

#[derive(Serialize, Deserialize)]
pub struct Source {
    pub name: Option<String>,
    pub version: Option<String>,
    pub environment: Option<String>,
    pub service: Option<String>,
    pub ip: Option<String>,
}

pub trait LogExt {
    fn new() -> Self;
    fn user_id(self, user_id: String) -> Self;
    fn system(self, system: String) -> Self;
    fn file(self, file: String) -> Self;
    fn line(self, line: u32) -> Self;
    fn message(self, message: String) -> Self;
    fn source(self, source: Source) -> Self;
    fn log_level(self, log_level: LogType) -> Self;
    fn timestamp(self, timestamp: SystemTime) -> Self;
}

impl LogExt for Log {
    fn new() -> Self {
        Self {
            user_id: None,
            system: None,
            file: None,
            line: None,
            message: None,
            source: None,
            log_level: None,
            timestamp: SystemTime::now(),
        }
    }
    fn user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }
    fn system(mut self, system: String) -> Self {
        self.system = Some(system);
        self
    }
    fn file(mut self, file: String) -> Self {
        self.file = Some(file);
        self
    }
    fn line(mut self, line: u32) -> Self {
        self.line = Some(line);
        self
    }
    fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }
    fn source(mut self, source: Source) -> Self {
        self.source = Some(source);
        self
    }
    fn log_level(mut self, log_level: LogType) -> Self {
        self.log_level = Some(log_level);
        self
    }
    fn timestamp(mut self, timestamp: SystemTime) -> Self {
        self.timestamp = timestamp;
        self
    }
}

pub trait SourceExt {
    fn new() -> Self;
    fn name(self, name: String) -> Self;
    fn version(self, version: String) -> Self;
    fn environment(self, environment: String) -> Self;
    fn service(self, service: String) -> Self;
    fn ip(self, ip: String) -> Self;
}

impl SourceExt for Source {
    fn new() -> Self {
        Self {
            name: None,
            version: None,
            environment: None,
            service: None,
            ip: None,
        }
    }
    fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    fn version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }
    fn environment(mut self, environment: String) -> Self {
        self.environment = Some(environment);
        self
    }
    fn service(mut self, service: String) -> Self {
        self.service = Some(service);
        self
    }
    fn ip(mut self, ip: String) -> Self {
        self.ip = Some(ip);
        self
    }
}