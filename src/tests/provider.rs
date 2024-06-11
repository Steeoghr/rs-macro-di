



use crate::{add_scoped, add_singleton, provide, clear_provider_scope};
use super::*;
use std::sync::Arc;
use test_utility::{TestScopedService, TestSingletonService};
use std::thread;
use std::time::Duration;

#[test]
fn test_provider_scoped_service() {
    clear_provider_scope!();
    // Aggiungi un servizio singleton usando la macro
    add_scoped!(TestScopedService);
    let scoped_service: Arc<TestScopedService> = provide!(TestScopedService);

    // let result = add(2, 2);
    assert_eq!(scoped_service.test, "scoped");
    assert_eq!(scoped_service.find(), "find");
}

#[test]
fn test_provider_singleton_service() {
    clear_provider_scope!();

    // Aggiungi un servizio singleton usando la macro
    add_singleton!(Arc::new(TestSingletonService::new()));
    let singleton_service: Arc<TestSingletonService> = provide!(TestSingletonService);

    // let result = add(2, 2);
    assert_eq!(singleton_service.test, "singleton");
}

#[test]
fn test_singleton_service_time_consistency() {
    clear_provider_scope!();

    let service1 = TestSingletonService::new();
    let initial_time = service1.created_at;
    add_singleton!(Arc::new(service1));

    // Simula una pausa per assicurarsi che il tempo sia passato
    thread::sleep(Duration::from_secs(1));

    let service2: Arc<TestSingletonService> = provide!(TestSingletonService);
    let subsequent_time = service2.created_at;

    assert_eq!(service2.test, "singleton");
    assert_eq!(initial_time, subsequent_time, "The created_at time should remain consistent for singleton services.");
}