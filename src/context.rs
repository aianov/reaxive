use crate::Store;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct StoreContext {
    stores: Arc<Mutex<HashMap<TypeId, Arc<dyn std::any::Any + Send + Sync>>>>,
    name: String,
}

impl StoreContext {
    pub fn new() -> Self {
        Self {
            stores: Arc::new(Mutex::new(HashMap::new())),
            name: "default".to_string(),
        }
    }

    pub fn with_name(name: impl Into<String>) -> Self {
        Self {
            stores: Arc::new(Mutex::new(HashMap::new())),
            name: name.into(),
        }
    }

    pub fn register<S: Store + Send + Sync>(&self, store: S) {
        let type_id = store.id();
        self.stores.lock().unwrap().insert(type_id, Arc::new(store));
    }

    pub fn get<S: Store>(&self) -> Option<S> {
        self.stores
            .lock()
            .unwrap()
            .get(&TypeId::of::<S>())
            .and_then(|store| store.downcast_ref::<S>())
            .cloned()
    }

    pub fn get_or_create<S: Store + Default + Send + Sync>(&self) -> S {
        if let Some(store) = self.get::<S>() {
            store
        } else {
            let store = S::default();
            self.register(store.clone());
            store
        }
    }

    pub fn has<S: Store>(&self) -> bool {
        self.stores.lock().unwrap().contains_key(&TypeId::of::<S>())
    }

    pub fn remove<S: Store>(&self) {
        self.stores.lock().unwrap().remove(&TypeId::of::<S>());
    }

    pub fn clear(&self) {
        self.stores.lock().unwrap().clear();
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn count(&self) -> usize {
        self.stores.lock().unwrap().len()
    }

    pub fn clone_to(&self, name: impl Into<String>) -> Self {
        Self {
            stores: self.stores.clone(),
            name: name.into(),
        }
    }
}

impl Default for StoreContext {
    fn default() -> Self {
        Self::new()
    }
}

static mut DEFAULT_CONTEXT: Option<StoreContext> = None;

pub fn get_default_context() -> &'static StoreContext {
    unsafe {
        if DEFAULT_CONTEXT.is_none() {
            DEFAULT_CONTEXT = Some(StoreContext::new());
        }
        DEFAULT_CONTEXT.as_ref().unwrap()
    }
}

pub fn set_default_context(context: StoreContext) {
    unsafe {
        DEFAULT_CONTEXT = Some(context);
    }
}

pub fn provide_store<S: Store + Send + Sync>(store: S) {
    let context = get_default_context();
    context.register(store);
}

pub fn use_context_store<S: Store + Default + Send + Sync>() -> S {
    let context = get_default_context();
    context.get_or_create::<S>()
}

pub fn get_context_store<S: Store>() -> Option<S> {
    let context = get_default_context();
    context.get::<S>()
}

#[cfg(feature = "dioxus")]
mod dioxus_support {
    use super::*;
    use dioxus::prelude::*;

    #[component]
    pub fn StoreProvider(children: Element) -> Element {
        let context = StoreContext::new();
        use_context_provider(|| context);

        rsx! {
            {children}
        }
    }

    #[component]
    pub fn NamedStoreProvider(name: String, children: Element) -> Element {
        let context = StoreContext::with_name(name);
        use_context_provider(|| context);

        rsx! {
            {children}
        }
    }

    pub fn use_provide_store<S: Store + Send + Sync>(store: S) {
        let context = use_context::<StoreContext>();
        context.register(store);
    }

    pub fn use_store_from_context<S: Store + Default + Send + Sync>() -> S {
        let context = use_context::<StoreContext>();
        context.get_or_create::<S>()
    }

    pub fn get_store_from_context<S: Store>() -> Option<S> {
        let context = use_context::<StoreContext>();
        context.get::<S>()
    }
}

#[cfg(feature = "dioxus")]
pub use dioxus_support::*;
pub struct ContextManager {
    contexts: HashMap<String, StoreContext>,
    current: String,
}

impl ContextManager {
    pub fn new() -> Self {
        let mut manager = Self {
            contexts: HashMap::new(),
            current: "default".to_string(),
        };
        manager
            .contexts
            .insert("default".to_string(), StoreContext::new());
        manager
    }

    pub fn create_context(&mut self, name: impl Into<String>) -> &StoreContext {
        let name = name.into();
        self.contexts
            .insert(name.clone(), StoreContext::with_name(name.clone()));
        self.contexts.get(&name).unwrap()
    }

    pub fn get_context(&self, name: &str) -> Option<&StoreContext> {
        self.contexts.get(name)
    }

    pub fn set_current(&mut self, name: impl Into<String>) {
        let name = name.into();
        if self.contexts.contains_key(&name) {
            self.current = name;
        }
    }

    pub fn current(&self) -> &StoreContext {
        self.contexts.get(&self.current).unwrap()
    }

    pub fn remove_context(&mut self, name: &str) {
        if name != "default" {
            self.contexts.remove(name);
        }
    }

    pub fn clear(&mut self) {
        self.contexts.retain(|k, _| k == "default");
        self.current = "default".to_string();
    }
}

impl Default for ContextManager {
    fn default() -> Self {
        Self::new()
    }
}

static mut GLOBAL_CONTEXT_MANAGER: Option<ContextManager> = None;

pub fn get_context_manager() -> &'static mut ContextManager {
    unsafe {
        if GLOBAL_CONTEXT_MANAGER.is_none() {
            GLOBAL_CONTEXT_MANAGER = Some(ContextManager::new());
        }
        GLOBAL_CONTEXT_MANAGER.as_mut().unwrap()
    }
}

pub fn create_named_context(name: impl Into<String>) -> &'static StoreContext {
    let manager = get_context_manager();
    manager.create_context(name)
}

pub fn switch_to_context(name: impl Into<String>) {
    let manager = get_context_manager();
    manager.set_current(name);
}

pub fn current_context() -> &'static StoreContext {
    let manager = get_context_manager();
    manager.current()
}
