[← back to library](../../librarium.md)

# Процедурные макросы

Тип макросов, основанный TokenStream-парсинге и вызывающийся подобно декларативным макросам через `!`.  

Крейт определения макроса:
```rs
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(item: TokenStream) -> TokenStream {
    println!("make_answer's item is: {item}");
    "fn answer1() -> u32 { 41 }".parse().unwrap()
}
```

Крейт использования макроса:
```rs
make_answer!(some Rust code, or text or anything else);
```


