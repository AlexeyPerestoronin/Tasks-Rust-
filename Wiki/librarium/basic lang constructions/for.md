[← back to library](../../librarium.md)

Цикл `for` в Rust используется для итерации по коллекциям, таким как массивы, векторы, диапазоны и другие итераторы. 

Вот основные способы его использования:  
1. **Итерация по диапазону**:  
    Цикл for может использоваться для итерации по диапазону чисел.
    ```rust
    for number in 1..5 {
        println!("Number: {}", number);
    }
    ```
    Здесь 1..5 означает диапазон от 1 до 4 (5 не включается).
0. **Итерация по массиву или вектору**:  
    Вы можете использовать for для перебора элементов массива или вектора.
    ```rust
    let array = [10, 20, 30, 40, 50];
    for element in array.iter() {
        println!("Element: {}", element);
    }
    ```
0. **Итерация с индексами с помощью `enumerate`**:  
    Используйте enumerate, чтобы получить индекс и значение элемента.
    ```rust
    let array = [10, 20, 30, 40, 50];
    for (index, value) in array.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }
    ```
0. **Итерация по строкам**:  
    Цикл for может использоваться для итерации по символам строки.
    ```rust
    let text = "hello";
    for character in text.chars() {
        println!("Character: {}", character);
    }
    ```
0. **Итерация по коллекциям с изменением**:  
    Используйте iter_mut, чтобы изменять элементы коллекции во время итерации.
    ```rust
    let mut numbers = vec![1, 2, 3, 4, 5];
    for number in numbers.iter_mut() {
        *number *= 2;
    }
    println!("Doubled numbers: {:?}", numbers);
    ```
0. **Итерация по ключам и значениям в хэш-таблице**:  
    Вы можете использовать for для перебора пар ключ-значение в HashMap.
    ```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert("Alice", 10);
    scores.insert("Bob", 20);

    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
    ```

Цикл `for` в Rust предоставляет удобный и безопасный способ итерации по различным коллекциям и диапазонам, обеспечивая при этом возможность работы с индексами и изменением элементов.