pub mod context;
pub mod macros;
pub mod observable;
pub mod store;

pub use context::*;
pub use observable::*;
pub use store::*;

pub mod prelude {
    pub use crate::{
        clear_all_stores, create_store, get_context_store, get_store, has_store, observable,
        observable_bool, observable_map, observable_number, observable_option, observable_string,
        observable_vec, provide_store, register_store, remove_store, store_action,
        store_action_mut, store_count, use_context_store, use_reactive, use_store, GlobalStore,
        Observable, ObservableBool, ObservableF64, ObservableI32, ObservableMap, ObservableOption,
        ObservableString, ObservableU32, ObservableValue, ObservableVec, ObserverContext, Store,
        StoreRegistry,
    };

    pub use crate::{
        action, create_global_store, mobx_store, multi_store, reactive, simple_store,
        store_with_actions,
    };
}

pub struct Reaxion;

impl Reaxion {
    pub fn version() -> &'static str {
        "1.0.0"
    }

    pub fn init() {
        store::init_global_stores();
    }
}
