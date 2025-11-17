[← back to library](../../librarium.md)

В Rust перечисления (`enum`) позволяют определять типы, которые могут принимать несколько различных значений, называемых вариантами. Это мощный инструмент для моделирования данных, которые могут иметь несколько форм.

Вот основные способы объявления enum:  
1. **Объявление простого enum**:  
    Перечисление может содержать несколько вариантов без дополнительных данных.
    ```rust
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    ```
0. **enum с данными**:  
    Варианты перечисления могут содержать дополнительные данные.
    ```rust
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    ```

Основные способы использования enum:  
1. **Использование enum**:  
    Вы можете создавать переменные, используя варианты перечисления.
    ```rust
    let direction = Direction::Up;
    ```
0. **Сопоставление с match**:  
    match используется для обработки различных вариантов enum.
    ```rust
    match direction {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!"),
    }
    ```
0. **Создание и использование enum с данными**:  
    Вы можете создавать экземпляры вариантов с данными и извлекать их с помощью match.
    ```rust
    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
    ```
0. **Методы для enum**:  
    Вы можете определять методы для enum с помощью impl.
    ```rust
    impl Direction {
        fn opposite(&self) -> Direction {
            match self {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            }
        }
    }

    let opposite_direction = direction.opposite();
    ```
0. **Использование enum в структурах**:  
    enum может быть частью структуры.
    ```rust
    struct Player {
        name: String,
        direction: Direction,
    }

    let player = Player {
        name: String::from("Alice"),
        direction: Direction::Left,
    };
    ```

Перечисления в Rust обеспечивают гибкость и безопасность при работе с данными, которые могут принимать несколько различных форм, и позволяют выразительно моделировать сложные структуры данных.