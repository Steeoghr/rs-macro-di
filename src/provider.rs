use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use once_cell::sync::Lazy;

pub struct ServiceProvider {
    singletons: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
    scoped: HashMap<TypeId, Box<dyn Fn() -> Arc<dyn Any + Send + Sync> + Send + Sync>>,
    transients: HashMap<TypeId, Box<dyn Fn() -> Arc<dyn Any + Send + Sync> + Send + Sync>>,
    scoped_instances: RefCell<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl ServiceProvider {
    pub fn new() -> Self {
        ServiceProvider {
            singletons: HashMap::new(),
            scoped: HashMap::new(),
            transients: HashMap::new(),
            scoped_instances: RefCell::new(HashMap::new()),
        }
    }

    pub fn add_singleton<T: Any + Send + Sync>(&mut self, instance: Arc<T>) {
        self.singletons.insert(TypeId::of::<T>(), instance);
    }

    pub fn add_scoped<T: Any + Default + Send + Sync + 'static>(&mut self) {
        self.scoped.insert(
            TypeId::of::<T>(),
            Box::new(|| Arc::new(T::default())),
        );
    }

    pub fn add_transient<T: Any + Default + Send + Sync + 'static>(&mut self) {
        self.transients.insert(
            TypeId::of::<T>(),
            Box::new(|| Arc::new(T::default())),
        );
    }

    pub fn provide<T: Any + Send + Sync>(&self) -> Arc<T> {
        // Try to get the singleton instance first
        if let Some(service) = self.singletons.get(&TypeId::of::<T>()) {
            return service.clone().downcast::<T>().expect("Failed to downcast singleton service");
        }

        // If not found, try to get a scoped instance
        if let Some(instance) = self.scoped_instances.borrow().get(&TypeId::of::<T>()) {
            return instance.clone().downcast::<T>().expect("Failed to downcast scoped service");
        }

        if let Some(factory) = self.scoped.get(&TypeId::of::<T>()) {
            let instance = factory().downcast::<T>().expect("Failed to downcast scoped service");
            self.scoped_instances.borrow_mut().insert(TypeId::of::<T>(), instance.clone());
            return instance;
        }

        // If not found, try to get a transient instance
        if let Some(factory) = self.transients.get(&TypeId::of::<T>()) {
            return factory().downcast::<T>().expect("Failed to downcast transient service");
        }

        // If not found in either, panic
        panic!("Service of type {:?} not found", std::any::type_name::<T>());
    
    }

    pub fn clear_singletons(&mut self) {
        self.singletons.clear();
    }

    pub fn clear_scoped(&mut self) {
        self.scoped.clear();
    }

    pub fn clear_scoped_instances(&mut self) {
        self.scoped_instances.borrow_mut().clear();
    }

    pub fn clear_scope(&mut self) {
        self.clear_singletons();
        self.clear_scoped();
    }
}

static INSTANCE: Lazy<Mutex<ServiceProvider>> = Lazy::new(|| Mutex::new(ServiceProvider::new()));

pub fn get_service_provider() -> &'static Mutex<ServiceProvider> {
    &INSTANCE
}

pub fn get_mut_service_provider() -> MutexGuard<'static, ServiceProvider> {
    return INSTANCE.lock().unwrap();
}

pub fn clear_provider_scope() {
    let mut provider = get_mut_service_provider();
    provider.clear_scope();
}

pub fn clear_scoped_instances() {
    let mut provider = get_mut_service_provider();
    provider.clear_scoped_instances();
}

pub fn with_scope<F>(scoped_function: F)
where
    F: FnOnce() + Send + 'static,
{
    scoped_function();
    clear_scoped_instances();
}