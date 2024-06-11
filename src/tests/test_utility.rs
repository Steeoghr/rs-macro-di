use std::time::SystemTime;
use chrono::{DateTime, Utc};


pub struct TestSingletonService {
    pub test: String,
    pub created_at: DateTime<Utc>,
}

impl TestSingletonService {
    pub fn new() -> Self {
        Self {
            test: "singleton".to_string(),
            created_at: SystemTime::now().into(),
        }
    }
}

impl Default for TestSingletonService {
    fn default() -> Self {
        Self::new()
    }
}


pub struct TestScopedService {
    pub test: String,
}

impl TestScopedService {
    pub fn find(&self) -> String {
        "find".to_string()
    }
}

impl Default for TestScopedService {
    fn default() -> Self {
        Self { test: "scoped".to_string() }
    }
}

