# ReaXive

## A reactive state management library for Dioxus inspired by MobX.

## Features

- Observable values that automatically trigger UI updates
- Store pattern for global state management
- Simple API similar to MobX
- Seamless integration with Dioxus
- Multi-field stores with `reaxive_store!` macro
- Reactive components with `reaxive!` macro

## Installation

Add ReaXive to your Cargo.toml:

```toml
[dependencies]
reaxive = "1.0.3"
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
        self.count.set(|count| *count += 1); // or .inc()
    }

    pub fn decrement(&self) {
        self.count.set(|count| *count -= 1); // or .dec()
    }

    pub fn get_count(&self) -> i32 {
        self.count.get()
    }

    pub fn set_name(&self, name: String) {
        self.user.set(|user| user.name = name);
    }

    pub fn set_last_name(&self, last_name: String) {
        self.user.set(|user| user.last_name = last_name);
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
```

### Reactive Components

```rust
use reaxive::prelude::*;
use dioxus::prelude::*;

// Use the reaxive! macro to make components automatically update when store values change
reaxive! {
    #[component]
    pub fn CounterPage() -> Element {
        let store = CounterStore::new();

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

## Key Features

- **Zero Boilerplate**: Use `reaxive_store!` and `reaxive!` macros for minimal setup
- **Type Safety**: Full Rust type safety with automatic inference
- **Global State**: Stores are automatically managed globally with macro `reaxive!`
- **Reactive Updates**: Components automatically re-render when observable values change
- **Thread Safe**: All stores are thread-safe by default
- **Multiple Data Types**: Store multiple different types in a single store
- **Flexible API**: Direct field access with `.get()`, `.set()`, and other methods

## License

This project is licensed under the MIT License - see the LICENSE file for details.
