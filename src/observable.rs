use dioxus::prelude::{Readable, Writable};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};

thread_local! {
    static CURRENT_OBSERVER: RefCell<Option<Rc<RefCell<dyn FnMut()>>>> = RefCell::new(None);
}

#[derive(Clone)]
pub struct ObserverContext {
    observer: Rc<RefCell<dyn FnMut()>>,
}

impl ObserverContext {
    pub fn new<F: FnMut() + 'static>(update_fn: F) -> Self {
        let observer = Rc::new(RefCell::new(update_fn));

        CURRENT_OBSERVER.with(|current| {
            *current.borrow_mut() = Some(observer.clone());
        });

        Self { observer }
    }
}

impl Drop for ObserverContext {
    fn drop(&mut self) {
        CURRENT_OBSERVER.with(|current| {
            *current.borrow_mut() = None;
        });
    }
}

pub trait Observable<T: Clone + 'static> {
    fn get(&self) -> T;
    fn set(&self, value: T);
    fn update<F>(&self, updater: F)
    where
        F: FnOnce(&mut T);
    fn subscribe<F: Fn(&T) + Send + Sync + 'static>(&self, callback: F) -> usize;
    fn unsubscribe(&self, id: usize);
}

#[derive(Clone)]
pub struct ObservableValue<T: Clone + 'static> {
    value: Arc<Mutex<T>>,
    subscribers: Arc<Mutex<HashMap<usize, Box<dyn Fn(&T) + Send + Sync>>>>,
    next_id: Arc<Mutex<usize>>,
    local_subscribers: Rc<RefCell<Vec<Weak<RefCell<dyn FnMut()>>>>>,
}

impl<T: Clone + 'static> ObservableValue<T> {
    pub fn new(initial: T) -> Self {
        Self {
            value: Arc::new(Mutex::new(initial)),
            subscribers: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(0)),
            local_subscribers: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn value(&self) -> T {
        self.get()
    }

    pub fn set_value(&self, value: T) {
        self.set(value);
    }

    pub fn update_value<F>(&self, updater: F)
    where
        F: FnOnce(&mut T),
    {
        self.update(updater);
    }

    pub fn on_change<F: Fn(&T) + Send + Sync + 'static>(&self, callback: F) -> usize {
        self.subscribe(callback)
    }

    pub fn off_change(&self, id: usize) {
        self.unsubscribe(id);
    }

    pub fn map<U, F>(&self, mapper: F) -> U
    where
        F: FnOnce(&T) -> U,
    {
        self.track_access();
        let value = self.value.lock().unwrap();
        mapper(&*value)
    }

    pub fn when<F>(&self, predicate: F) -> bool
    where
        F: FnOnce(&T) -> bool,
    {
        self.track_access();
        let value = self.value.lock().unwrap();
        predicate(&*value)
    }

    fn notify_subscribers(&self) {
        let value = self.value.lock().unwrap().clone();

        let subscribers = self.subscribers.lock().unwrap();
        for callback in subscribers.values() {
            callback(&value);
        }

        let mut local_subs = self.local_subscribers.borrow_mut();
        local_subs.retain(|weak| {
            if let Some(strong) = weak.upgrade() {
                if let Ok(mut cb) = strong.try_borrow_mut() {
                    cb();
                }
                true
            } else {
                false
            }
        });
    }

    fn track_access(&self) {
        CURRENT_OBSERVER.with(|observer| {
            if let Some(ref update_fn) = *observer.borrow() {
                let mut local_subs = self.local_subscribers.borrow_mut();
                let weak_ref = Rc::downgrade(update_fn);

                let update_ptr = weak_ref.as_ptr();
                if !local_subs
                    .iter()
                    .any(|sub| std::ptr::addr_eq(sub.as_ptr(), update_ptr))
                {
                    local_subs.push(weak_ref);
                }
            }
        });
    }
}

impl<T: Clone + 'static> Observable<T> for ObservableValue<T> {
    fn get(&self) -> T {
        self.track_access();
        self.value.lock().unwrap().clone()
    }

    fn set(&self, value: T) {
        *self.value.lock().unwrap() = value;
        self.notify_subscribers();
    }

    fn update<F>(&self, updater: F)
    where
        F: FnOnce(&mut T),
    {
        {
            let mut value = self.value.lock().unwrap();
            updater(&mut *value);
        }
        self.notify_subscribers();
    }

    fn subscribe<F: Fn(&T) + Send + Sync + 'static>(&self, callback: F) -> usize {
        let id = {
            let mut next_id = self.next_id.lock().unwrap();
            let id = *next_id;
            *next_id += 1;
            id
        };

        self.subscribers
            .lock()
            .unwrap()
            .insert(id, Box::new(callback));
        id
    }

    fn unsubscribe(&self, id: usize) {
        self.subscribers.lock().unwrap().remove(&id);
    }
}

pub fn observable<T: Clone + 'static>(initial: T) -> ObservableValue<T> {
    ObservableValue::new(initial)
}

pub fn observable_option<T: Clone + 'static>(initial: Option<T>) -> ObservableValue<Option<T>> {
    ObservableValue::new(initial)
}

pub fn observable_vec<T: Clone + 'static>(initial: Vec<T>) -> ObservableValue<Vec<T>> {
    ObservableValue::new(initial)
}

pub fn observable_map<K: Clone + 'static, V: Clone + 'static>(
    initial: HashMap<K, V>,
) -> ObservableValue<HashMap<K, V>> {
    ObservableValue::new(initial)
}

pub fn observable_bool(initial: bool) -> ObservableValue<bool> {
    ObservableValue::new(initial)
}

pub fn observable_string(initial: impl Into<String>) -> ObservableValue<String> {
    ObservableValue::new(initial.into())
}

pub fn observable_number<T: Clone + 'static>(initial: T) -> ObservableValue<T> {
    ObservableValue::new(initial)
}

pub type ObservableBool = ObservableValue<bool>;
pub type ObservableString = ObservableValue<String>;
pub type ObservableI32 = ObservableValue<i32>;
pub type ObservableU32 = ObservableValue<u32>;
pub type ObservableF64 = ObservableValue<f64>;
pub type ObservableVec<T> = ObservableValue<Vec<T>>;
pub type ObservableOption<T> = ObservableValue<Option<T>>;
pub type ObservableMap<K, V> = ObservableValue<HashMap<K, V>>;

pub fn use_reactive() -> impl Fn() {
    let mut reactive_update = dioxus::prelude::use_signal(|| 0u32);

    dioxus::prelude::use_hook(|| {
        let update_ui = {
            let mut reactive_update = reactive_update.clone();
            move || {
                reactive_update.set(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos() as u32,
                );
            }
        };

        ObserverContext::new(update_ui)
    });

    let _trigger = reactive_update.read();

    || {}
}
