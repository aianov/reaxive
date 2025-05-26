use crate::lib::ObservableValue;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};

pub trait Store: Clone + 'static {
    fn id(&self) -> TypeId;
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

#[derive(Clone)]
pub struct StoreRegistry {
    stores: Arc<Mutex<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>>,
}

impl StoreRegistry {
    pub fn new() -> Self {
        Self {
            stores: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register<S: Store + Send + Sync + 'static>(&self, store: S) {
        let type_id = store.id();
        self.stores.lock().unwrap().insert(type_id, Arc::new(store));
    }

    pub fn get<S: Store + 'static>(&self) -> Option<S> {
        self.stores
            .lock()
            .unwrap()
            .get(&TypeId::of::<S>())
            .and_then(|store| store.downcast_ref::<S>())
            .cloned()
    }

    pub fn get_or_create<S: Store + Default + Send + Sync + 'static>(&self) -> S {
        if let Some(store) = self.get::<S>() {
            store
        } else {
            let store = S::default();
            self.register(store.clone());
            store
        }
    }

    pub fn has<S: Store + 'static>(&self) -> bool {
        self.stores.lock().unwrap().contains_key(&TypeId::of::<S>())
    }

    pub fn remove<S: Store + 'static>(&self) {
        self.stores.lock().unwrap().remove(&TypeId::of::<S>());
    }

    pub fn clear(&self) {
        self.stores.lock().unwrap().clear();
    }

    pub fn count(&self) -> usize {
        self.stores.lock().unwrap().len()
    }
}

static GLOBAL_STORE_REGISTRY: LazyLock<Mutex<StoreRegistry>> =
    LazyLock::new(|| Mutex::new(StoreRegistry::new()));

pub fn init_global_stores() {}

pub fn get_global_registry() -> std::sync::MutexGuard<'static, StoreRegistry> {
    GLOBAL_STORE_REGISTRY.lock().unwrap()
}

pub fn create_store<S: Store + Send + Sync + 'static>(store: S) -> S {
    let registry = get_global_registry();
    registry.register(store.clone());
    store
}

pub fn use_store<S: Store + Default + Send + Sync + 'static>() -> S {
    let registry = get_global_registry();
    registry.get_or_create::<S>()
}

pub fn get_store<S: Store + 'static>() -> Option<S> {
    let registry = get_global_registry();
    registry.get::<S>()
}

pub fn register_store<S: Store + Send + Sync + 'static>(store: S) {
    let registry = get_global_registry();
    registry.register(store);
}

pub fn remove_store<S: Store + 'static>() {
    let registry = get_global_registry();
    registry.remove::<S>();
}

pub fn has_store<S: Store + 'static>() -> bool {
    let registry = get_global_registry();
    registry.has::<S>()
}

pub fn clear_all_stores() {
    let registry = get_global_registry();
    registry.clear();
}

pub fn store_count() -> usize {
    let registry = get_global_registry();
    registry.count()
}

pub fn store_action<S: Store + 'static, F, R>(action: F) -> Option<R>
where
    F: FnOnce(&S) -> R,
{
    let registry = get_global_registry();
    registry.get::<S>().map(|store| action(&store))
}

pub fn store_action_mut<S: Store + 'static, F, R>(action: F) -> Option<R>
where
    F: FnOnce(&mut S) -> R,
{
    let registry = get_global_registry();
    if let Some(mut store) = registry.get::<S>() {
        Some(action(&mut store))
    } else {
        None
    }
}

pub trait GlobalStore: Store + Default + Send + Sync {
    fn global() -> Self {
        use_store::<Self>()
    }

    fn reset() {
        let new_store = Self::default();
        register_store(new_store);
    }
}

impl<T: Store + Default + Send + Sync> GlobalStore for T {}

#[macro_export]
macro_rules! simple_store {
    ($name:ident, $type:ty, $default:expr) => {
        #[derive(Clone)]
        pub struct $name {
            pub value: $crate::lib::ObservableValue<$type>,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    value: $crate::lib::observable($default),
                }
            }

            pub fn get(&self) -> $type {
                self.value.get()
            }

            pub fn set(&self, value: $type) {
                self.value.set(value);
            }

            pub fn update<F>(&self, updater: F)
            where
                F: FnOnce(&mut $type),
            {
                self.value.update(updater);
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl $crate::lib::Store for $name {
            fn id(&self) -> std::any::TypeId {
                std::any::TypeId::of::<$name>()
            }
        }
    };
}

#[macro_export]
macro_rules! reaxion_store {
    (
        $name:ident {
            $(
                $field:ident: $type:ty = $default:expr
            ),* $(,)?
        }
    ) => {
        #[derive(Clone)]
        pub struct $name {
            $(
                pub $field: $crate::lib::ObservableValue<$type>,
            )*
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    $(
                        $field: $crate::lib::observable($default),
                    )*
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl $crate::lib::Store for $name {
            fn id(&self) -> std::any::TypeId {
                std::any::TypeId::of::<$name>()
            }
        }
    };
}
