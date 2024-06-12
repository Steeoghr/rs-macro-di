# Rust Macro Dependency Injection

## Import lib

```toml
rs-macro-di = "0.2.0"
```

## Add dependency

```rust
// Add singleton service
add_singleton!(Arc::new(SingletonService::new()));

// Add scoped service
add_scoped!(ScopedService);

// Add transient service
add_transient!(ScopedService);
```

## Use a configured service

```rust
let singleton_service: Arc<ServiceClass> = provide!(ServiceClass);
```

## Or without macro
```rust
// Get provider
let provider = provider!();

// Add singleton service
provider.add_singleton(Arc::new(SingletonService::new()));

// Add scoped service
provider.add_scoped<ScopedService>();

// Add transient service
provider.add_transient<ScopedService>();
```

```rust
// Get provider
let provider = provider!();

let singleton_service: Arc<ServiceClass> = provider.provide(ServiceClass);
```