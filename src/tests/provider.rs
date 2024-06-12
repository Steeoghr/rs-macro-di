use crate::provider::with_scope;
use crate::{add_scoped, add_singleton, add_transient, clear_provider_scope, provide};
use super::*;
use std::sync::{Arc, RwLock};
use test_utility::{TestScopedService, TestSingletonService, TestTransientService};
use std::thread;
use std::time::Duration;

#[test]
fn test_provider_scoped_service() {
    clear_provider_scope!();
    // Aggiungi un servizio singleton usando la macro
    add_scoped!(TestScopedService);

    // Simula una pausa per assicurarsi che il tempo sia passato
    thread::sleep(Duration::from_secs(1));

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

    // Simula una pausa per assicurarsi che il tempo sia passato
    thread::sleep(Duration::from_secs(1));

    let singleton_service: Arc<TestSingletonService> = provide!(TestSingletonService);

    // let result = add(2, 2);
    assert_eq!(singleton_service.test, "singleton");
}

#[test]
fn test_provider_transient_service() {
    clear_provider_scope!();

    // Aggiungi un servizio singleton usando la macro
    add_transient!(TestTransientService);

    // Simula una pausa per assicurarsi che il tempo sia passato
    thread::sleep(Duration::from_secs(1));

    let transient_service: Arc<TestTransientService> = provide!(TestTransientService);

    let find_result = transient_service.find();
    // let result = add(2, 2);
    assert_eq!(transient_service.test, "transient");
    assert_eq!(find_result, "find");
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

#[test]
fn test_scoped_service_time_consistency() {
    clear_provider_scope!();

    add_scoped!(TestScopedService);

    // Simula una pausa per assicurarsi che il tempo sia passato
    thread::sleep(Duration::from_secs(1));

    let service1: Arc<TestScopedService> = provide!(TestScopedService);
    let initial_time = service1.created_at;
    let service2: Arc<TestScopedService> = provide!(TestScopedService);
    let subsequent_time = service2.created_at;

    assert_eq!(service2.test, "scoped");
    assert_eq!(initial_time, subsequent_time, "The created_at time should remain consistent for scoped services.");
}

#[test]
fn test_transient_service_time_inconsistency() {
    clear_provider_scope!();

    add_transient!(TestTransientService);

    // Simula una pausa per assicurarsi che il tempo sia passato
    thread::sleep(Duration::from_secs(1));

    let service1: Arc<TestTransientService> = provide!(TestTransientService);
    let initial_time = service1.created_at;
    let service2: Arc<TestTransientService> = provide!(TestTransientService);
    let subsequent_time = service2.created_at;

    assert_eq!(service2.test, "transient");
    assert_ne!(initial_time, subsequent_time, "The created_at time shouldn't remain consistent for transient services.");
}

#[test]
fn test_clear_scoped() {
    clear_provider_scope!();
    let initial_time = Arc::new(RwLock::new(None));
    let subsequent_time = Arc::new(RwLock::new(None));

    add_scoped!(TestScopedService);

    {
        let initial_time_ref = Arc::clone(&initial_time);
        with_scope(move || {
            let service1: Arc<TestScopedService> = provide!(TestScopedService);
            let mut initial_time_lock = initial_time_ref.write().unwrap();
            *initial_time_lock = Some(service1.created_at);
        });
    }

    {

        
        let subsequent_time_ref = Arc::clone(&subsequent_time);
        with_scope(move || {
            let service2: Arc<TestScopedService> = provide!(TestScopedService);
            let mut subsequent_time_lock = subsequent_time_ref.write().unwrap();
            *subsequent_time_lock = Some(service2.created_at);
        });
    }

    let initial_time_value = initial_time.read().unwrap().expect("initial_time should be set");
    let subsequent_time_value = subsequent_time.read().unwrap().expect("subsequent_time should be set");

    assert_ne!(
        initial_time_value,
        subsequent_time_value,
        "The created_at time shouldn't remain consistent for scoped services worked in different scoped block.."
    );
}

// #[test]
// fn test_provider_failure_get_service() {
//     clear_provider_scope!();

//     let result = std::panic::catch_unwind(|| {
//         // Prova a recuperare un servizio non registrato
//         let _service: Arc<TestTransientService> = provide!(TestTransientService);
//     });

//     assert!(result.is_err(), "Expected to panic when trying to get an unregistered service");
// }