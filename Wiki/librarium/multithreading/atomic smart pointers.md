[← back to library](../../librarium.md)

## `Arc<T>`
"Atomic Reference Counting" - атомарный счетчик ссылок.  
Подобен `Rc<T>`, но безопасен для использования в многопоточных средах.
```rust
use std::sync::Arc;
let a = Arc::new(5);
let b = Arc::clone(&a);
```

## `Mutex<T>`
Предоставляет механизм блокировки для управления доступом к данным в многопоточной среде.
```rust
use std::sync::Mutex;
let m = Mutex::new(5);
let mut num = m.lock().unwrap();
*num += 1;
```