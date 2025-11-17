[← back to library](../../librarium.md)

В языке программирования Rust конструкция `match` используется для сопоставления шаблонов и является мощным инструментом для управления потоком выполнения программы.  

Вот основные способы использования match в Rust:  
1. **Сопоставление с литералами**:  
    Вы можете сопоставлять значения с конкретными литералами.
    ```rust
    let number = 3;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }
    ```
0. **Сопоставление с диапазонами**:  
    Можно использовать диапазоны для сопоставления.
    ```rust
    let number = 5;
    match number {
        1..=5 => println!("Between 1 and 5"),
        _ => println!("Greater than 5"),
    }
    ```
0. **Сопоставление с перечислениями (enums)**:  
    match часто используется с перечислениями для обработки различных вариантов.
    ```rust
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    let direction = Direction::Up;
    match direction {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!"),
    }
    ```
0. **Сопоставление с кортежами**:  
    Можно сопоставлять кортежи, чтобы извлекать значения.  
    ```rust
    let pair = (2, 3);
    match pair {
        (0, y) => println!("First is zero, second is {}", y),
        (x, 0) => println!("First is {}, second is zero", x),
        _ => println!("Neither are zero"),
    }
    ```
0. **Сопоставление с массивами и срезами**:  
    match может использоваться для работы с массивами и срезами.
    ```rust
    let array = [1, 2, 3];
    match array {
        [1, _, _] => println!("Starts with 1"),
        [_, 2, _] => println!("Middle is 2"),
        _ => println!("No match"),
    }
    ```
0. **Сопоставление с помощью if (guard clauses)**:  
    Можно использовать дополнительные условия для более точного сопоставления.  
    ```rust
    let number = Some(4);
    match number {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("Greater than or equal to five: {}", x),
        None => println!("No value"),
    }
    ```
0. **Игнорирование значений**:  
    Используйте _ для игнорирования значений, которые вам не нужны.  
    ```rust
    let some_value = Some(10);
    match some_value {
        Some(_) => println!("Got a value"),
        None => println!("No value"),
    }
    ```
0. **Деструктуризация структур**:  
    match может использоваться для деструктуризации структур.
    ```rust
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 0, y: 7 };
    match point {
        Point { x: 0, y } => println!("On the y-axis at {}", y),
        Point { x, y: 0 } => println!("On the x-axis at {}", x),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    ```

Эти примеры показывают, насколько гибким и мощным может быть `match` в Rust для управления потоком выполнения программы и обработки различных типов данных.