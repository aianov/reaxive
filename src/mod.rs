pub mod context;
pub mod macros;
pub mod observable;
pub mod store;

pub use context::*;
pub use observable::*;
pub use store::*;

pub mod prelude {
    pub use crate::lib::{
        clear_all_stores, create_named_context, create_store, current_context, get_context_store,
        get_store, has_store, observable, observable_bool, observable_map, observable_number,
        observable_option, observable_string, observable_vec, provide_store, register_store,
        remove_store, store_action, store_action_mut, store_count, switch_to_context,
        use_context_store, use_reactive, use_store, GlobalStore, Observable, ObservableBool,
        ObservableF64, ObservableI32, ObservableMap, ObservableOption, ObservableString,
        ObservableU32, ObservableValue, ObservableVec, ObserverContext, Store, StoreRegistry,
    };

    pub use crate::{
        action, create_global_store, mobx_store, multi_store, reactive, simple_store,
        store_with_actions, use_store as use_global_store,
    };

    #[cfg(feature = "dioxus")]
    pub use crate::lib::{
        get_store_from_context, use_provide_store, use_store_from_context, NamedStoreProvider,
        StoreProvider,
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
