# Rust Macro Dependency Injection

## Import lib

```toml
rs-macro-di = "0.2.0"
```

or add with cargo:

```
cargo add rs-macro-di
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
let service: Arc<ServiceClass> = provide!(ServiceClass);
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

let service: Arc<ServiceClass> = provider.provide(ServiceClass);
```

## Use a scoped block
```rust
// Add scoped service
add_scoped!(ScopedService);

// scoped service instances are cleared at the end of with_scope
with_scope(|| {
    let service: Arc<ServiceClass> = provide!(ServiceClass);
})
```