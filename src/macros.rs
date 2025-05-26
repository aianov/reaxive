#[macro_export]
macro_rules! multi_store {
    (
        $store_name:ident {
            $(
                $field_name:ident: $field_type:ty = $initial_value:expr
            ),* $(,)?
        }

        actions {
            $(
                fn $method_name:ident(&self $(, $param_name:ident: $param_type:ty)*) {
                    $($body:tt)*
                }
            )*
        }
    ) => {
        #[derive(Clone)]
        pub struct $store_name {
            $(
                pub $field_name: $crate::ObservableValue<$field_type>,
            )*
        }

        impl $store_name {
            pub fn new() -> Self {
                Self {
                    $(
                        $field_name: $crate::observable($initial_value),
                    )*
                }
            }

            $(
                pub fn $method_name(&self $(, $param_name: $param_type)*) {
                    $($body)*
                }
            )*
        }

        impl $crate::Store for $store_name {
            fn id(&self) -> std::any::TypeId {
                std::any::TypeId::of::<$store_name>()
            }
        }
    };
}

#[macro_export]
macro_rules! create_global_store {
    ($store_type:ty) => {{
        use std::sync::LazyLock;
        static STORE: LazyLock<$store_type> = LazyLock::new(|| <$store_type>::new());
        STORE.clone()
    }};
}

#[macro_export]
macro_rules! use_store {
    ($store_type:ty) => {
        $crate::create_global_store!($store_type)
    };
}

#[macro_export]
macro_rules! action {
    ($store:expr, $action:expr) => {
        $action
    };
}

#[macro_export]
macro_rules! reaxive {
    (
        #[component]
        $(#[$attr:meta])*
        $vis:vis fn $name:ident($($param:ident: $param_type:ty),* $(,)?) -> Element {
            $($body:tt)*
        }
    ) => {
        #[component]
        $(#[$attr])*
        $vis fn $name($($param: $param_type),*) -> Element {
            let reaxive_update = dioxus::prelude::use_signal(|| 0u32);

            let _observer_context = dioxus::prelude::use_hook(|| {
                let update_ui = {
                    let mut reaxive_update = reaxive_update.clone();
                    move || {
                        reaxive_update.set(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u32);
                    }
                };

                $crate::ObserverContext::new(update_ui)
            });

            let _ = reaxive_update.read();

            $($body)*
        }
    };
}

#[macro_export]
macro_rules! store {
    (
        $store_name:ident {
            $(
                $field_name:ident: $field_type:ty = $initial_value:expr
            ),* $(,)?
        }
    ) => {
        $crate::multi_store! {
            $store_name {
                $(
                    $field_name: $field_type = $initial_value
                ),*
            }

            actions {}
        }
    };
}

#[macro_export]
macro_rules! store_with_actions {
    (
        $store_name:ident {
            $(
                $field_name:ident: $field_type:ty = $initial_value:expr
            ),* $(,)?
        }

        impl {
            $(
                fn $method_name:ident(&self $(, $param_name:ident: $param_type:ty)*) {
                    $($body:tt)*
                }
            )*
        }
    ) => {
        $crate::multi_store! {
            $store_name {
                $(
                    $field_name: $field_type = $initial_value
                ),*
            }

            actions {
                $(
                    fn $method_name(&self $(, $param_name: $param_type)*) {
                        $($body)*
                    }
                )*
            }
        }
    };
}

#[macro_export]
macro_rules! reaxive_store {
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
                pub $field: $crate::ObservableValue<$type>,
            )*
        }

        impl $name {
            /// Creates a new instance that automatically connects to the global store
            /// Works like ModX - just call new() and get the global state!
            pub fn new() -> Self {
                $crate::use_store::<Self>()
            }

            /// Internal method for creating actual instances (used by the store system)
            fn create_instance() -> Self {
                Self {
                    $(
                        $field: $crate::observable($default),
                    )*
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::create_instance()
            }
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl $crate::Store for $name {
            fn id(&self) -> std::any::TypeId {
                std::any::TypeId::of::<$name>()
            }
        }
    };
}
