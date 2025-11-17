[← back to library](../../../librarium.md)

Ассоциированные типы - это альтернатива Rust использования обобщённых типов.  

Классический пример ассоциированного типа, это трейт `Iterator`:  
```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```
Здесь `Item`, это ассоциированный тип.  

В отличии от обобщённых типов, <span style="text-decoration: underline;">реализация типажа с ассоциированным типом происходит единожды для реализующего типа</span> (т.к. возможно только одно определение `impl Iterator for Enumerator`):  
```rust
struct Enumerator {
    index: i32,
    stop: i32,
}

impl Iterator for Enumerator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_item : Option<Self::Item> = None;
        if self.index < self.stop {
            next_item = Some(self.index);
            self.index += 1;
        }

        return next_item;
    }
}
```