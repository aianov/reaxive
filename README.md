# ReaXive

## A reactive state management library for Dioxus inspired by MobX.

## Features

- Observable values that automatically trigger UI updates
- Store pattern for global state management
- Simple API similar to MobX
- Seamless integration with Dioxus
- Multi-field stores with `reaxive_store!` macro
- Reactive components with `reactive!` macro

## Installation

Add ReaXive to your Cargo.toml:

```toml
[dependencies]
reaxive = "1.0.0"
dioxus = "0.6"
```

## Usage

### Basic Store with Multiple Fields

```rust
use reaxive::prelude::*;
use dioxus::prelude::*;

// Define a custom type for your store
#[derive(Clone, Debug)]
pub struct User {
    pub name: String,
    pub last_name: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "John".to_string(),
            last_name: "Doe".to_string(),
        }
    }
}

// Create a store with multiple observable fields using the reaxive_store! macro
reaxive_store!(CounterStore {
    count: i32 = 0,
    user: User = User::default()
});

// Implement methods for your store
impl CounterStore {
    pub fn increment(&self) {
        self.count.update(|count| *count += 1);
    }

    pub fn decrement(&self) {
        self.count.update(|count| *count -= 1);
    }

    pub fn get_count(&self) -> i32 {
        self.count.get()
    }

    pub fn set_name(&self, name: String) {
        self.user.update(|user| user.name = name);
    }

    pub fn set_last_name(&self, last_name: String) {
        self.user.update(|user| user.last_name = last_name);
    }

    pub fn get_user(&self) -> User {
        self.user.get()
    }

    pub fn get_full_name(&self) -> String {
        let user = self.user.get();
        format!("{} {}", user.name, user.last_name)
    }

    pub fn reset_user(&self) {
        self.user.set(User::default());
    }
}

// Create a hook to use the store
pub fn use_counter() -> CounterStore {
    use_store::<CounterStore>()
}
```

### Reactive Components

```rust
use reaxive::prelude::*;
use dioxus::prelude::*;

// Use the reactive! macro to make components automatically update when store values change
reactive! {
    #[component]
    pub fn CounterPage() -> Element {
        let store = use_counter();

        rsx! {
            div { class: "counter-page",
                h1 { "Counter & User Demo" }

                // User section
                div { class: "user-section",
                    h2 { "👤 User Profile" }
                    p { "Full Name: {store.get_full_name()}" }

                    input {
                        placeholder: "First Name",
                        value: "{store.get_user().name}",
                        oninput: {
                            let store = store.clone();
                            move |evt: Event<FormData>| store.set_name(evt.value())
                        }
                    }
                    input {
                        placeholder: "Last Name",
                        value: "{store.get_user().last_name}",
                        oninput: {
                            let store = store.clone();
                            move |evt: Event<FormData>| store.set_last_name(evt.value())
                        }
                    }
                    button {
                        onclick: {
                            let store = store.clone();
                            move |_| store.reset_user()
                        },
                        "Reset User"
                    }
                }

                // Counter section
                div { class: "counter-section",
                    h2 { "🔢 Counter" }
                    p { "Current value: {store.get_count()}" }

                    button {
                        onclick: {
                            let store = store.clone();
                            move |_| store.increment()
                        },
                        "➕ Increment"
                    }
                    button {
                        onclick: {
                            let store = store.clone();
                            move |_| store.decrement()
                        },
                        "➖ Decrement"
                    }
                }
            }
        }
    }
}
```

### Simple Single-Value Store

For simpler use cases, you can use the `simple_store!` macro:

```rust
use reaxive::prelude::*;

// Create a simple store with a single observable value
simple_store!(CountStore, i32, 0);

// Use it in your app
pub fn use_count() -> CountStore {
    use_store::<CountStore>()
}

reactive! {
    #[component]
    fn SimpleCounter() -> Element {
        let count_store = use_count();

        rsx! {
            div {
                p { "Count: {count_store.get()}" }
                button {
                    onclick: move |_| count_store.update(|c| *c += 1),
                    "Increment"
                }
            }
        }
    }
}
```

## Key Features

- **Zero Boilerplate**: Use `reaxive_store!` and `reactive!` macros for minimal setup
- **Type Safety**: Full Rust type safety with automatic inference
- **Global State**: Stores are automatically managed globally with `use_store`
- **Reactive Updates**: Components automatically re-render when observable values change
- **Thread Safe**: All stores are thread-safe by default
- **Multiple Data Types**: Store multiple different types in a single store
- **Flexible API**: Direct field access with `.get()`, `.set()`, and `.update()` methods

## License

This project is licensed under the MIT License - see the LICENSE file for details.
