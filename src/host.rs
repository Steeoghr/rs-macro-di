use std::collections::HashMap;
use std::sync::Mutex;
use inventory::iter;

use crate::provider::{get_service_provider, ServiceProvider};

#[derive(Clone)]
pub struct Route {
    pub method: String,
    pub path: String,
    pub handler: fn(String) -> String,
    pub has_body: bool,
}

inventory::collect!(Route);

pub struct WebHost {
    provider: &'static Mutex<ServiceProvider>,
    routes: HashMap<String, Route>,
}

impl WebHost {
    pub fn new<T: IStartup + 'static>() -> Self {
        let provider = get_service_provider();

        {
            let mut sp = provider.lock().unwrap();
            T::configure_services(&mut sp);
        }

        let mut web_host = WebHost {
            provider,
            routes: HashMap::new(),
        };

        web_host.add_controllers();
        web_host
    }

    pub fn add_controllers(&mut self) {
        for route in iter::<Route> {
            let key = format!("{}:{}", route.method, route.path);
            self.routes.insert(key, route.clone());
        }
    }

    pub fn handle_request(&self, method: &str, path: &str, body: Option<String>) -> Option<String> {
        let key = format!("{}:{}", method, path);
        if let Some(route) = self.routes.get(&key) {
            return Some((route.handler)(path.to_string()));
        }
        None
    }

    pub fn start(&self) {
        let routes = self.routes.clone(); // Clone the routes for the new thread
        std::thread::spawn(move || {
            // Simulate a request handling loop
            for (key, route) in &routes {
                let response = (route.handler)("".to_string());
                println!("Handled {}: {}", key, response);
            }
        }).join().unwrap();
    }
}

pub trait IStartup {
    fn configure_services(provider: &mut ServiceProvider);
}
