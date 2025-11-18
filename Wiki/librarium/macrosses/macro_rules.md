[← back to library](../../librarium.md)

# Декларативные макросы = macro_rules!

Данный тип макросов является улучшенным вариантом классических C++ макросов (`#define MACRO`), предназначенных для прямой замены макро-кода на компилируемый вариант.

```rs
// compiles OK
macro_rules! foo {
    ($($i:ident = $l:tt),+) => {
        $(
            print!("[{}] ", stringify!($i));
            bar!($l);
        )+
    };
}

macro_rules! bar {
    ($l:tt) => {
        println!("tt = {}", $l)
    };
}

pub fn add(left: u64, right: u64) -> u64 {
    foo!(digit = 1, number = 2, tree = 3, value = 4);
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```
