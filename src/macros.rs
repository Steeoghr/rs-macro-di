#[macro_export]
macro_rules! provide {
    ($service_type:ty) => {
        {
            let service_provider = $crate::provider::get_service_provider();
            let sp = service_provider.lock().unwrap();
            sp.provide::<$service_type>()
        }
    };
}

#[macro_export]
macro_rules! add_singleton {
    ($service:expr) => {
        {
            let service_provider = $crate::provider::get_service_provider();
            let mut sp = service_provider.lock().unwrap();
            sp.add_singleton($service);
        }
    };
}

#[macro_export]
macro_rules! add_scoped {
    ($service_type:ty) => {
        {
            let service_provider = $crate::provider::get_service_provider();
            let mut sp = service_provider.lock().unwrap();
            sp.add_scoped::<$service_type>();
        }
    };
}

#[macro_export]
macro_rules! add_transient {
    ($service_type:ty) => {
        {
            let service_provider = $crate::provider::get_service_provider();
            let mut sp = service_provider.lock().unwrap();
            sp.add_transient::<$service_type>();
        }
    };
}

#[macro_export]
macro_rules! provider {
    () => {
        {
            let service_provider = $crate::provider::get_service_provider();
            let sp = service_provider.lock().unwrap();
            sp
        }
    };
}