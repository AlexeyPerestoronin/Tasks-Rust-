[← back to library](../../../librarium.md)

Шаблоны позволяют компилятору выбирать ветви кода исходя из данных, через их сопоставление с шаблоном.  
Важно, чтобы совокупность возможных ветвей кода <span style="text-decoration: underline;">была исчерпывающей</span> для обрабатываемых данных.  

Далее приведены известные случаи использования шаблонов сопоставления данных от самых простых к самым сложным:
1. **Шаблон сопоставления в инструкции `let`**:  
    Здесь `x` это шаблон, которому соответствует значение 5:  
    ```rust
    let x = 5;
    ```

0. **Деструктурирование кортежа через шаблон**:  
    ```rust
    let (v1, v2, v3) = (1, 2, 3);
    ```

0. **Простое выражение `match`**:  
    ```rust
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
    ```

0. **Множественное ветвление через `if`**:  
    ```rust
    fn main() {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {color}, as the background");
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }
    ```

0. **Сопоставление с кортежем в цикле `for`**:  
    ```rust
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
    ```

0. **Условные циклы через `while let`**:  
    ```rust
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{top}");
        }
    ```

0. **Сопоставление с шаблоном в аргументах функции**:  
    ```rust
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({x}, {y})");
    }

    fn main() {
        let point = (3, 5);
        print_coordinates(&point);
    }
    ```

0. **Сопоставление с литералами**:  
    ```rust
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    ```

0. **Комбинированное сопоставление в `match` (через `|`)**:  
    ```rust
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    ```

0. **Условное сопоставление в `match` (через `if`)**:  
    ```rust
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
    ```
    ```rust
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
    ```

0. **Сопоставление диапазона (через `..=`)**:  
    ```rust
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    ```

0. **Деструктуризация структур**:  
    ```rust
    struct Point {
        x: i32,
        y: i32,
    }
    ```

    1. Неопровержимая деструктуризация:  
        ```rust
        fn printPoint(Point { x, y }: Point) {
            println!("Point: x = {x}, y = {y}")
        }

        fn main() {
            let p = Point { x: 0, y: 7 };

            let Point { x: a, y: b } = p;
            assert_eq!(0, a);
            assert_eq!(7, b);

            printPoint(p);

            // объединённая дестру
            let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
        }
        ```

    0. Опровержимая деструктуризация:
        ```rust
        fn main() {
            let p = Point { x: 0, y: 7 };

            match p {
                Point { x, y: 0 } => println!("On the x axis at {x}"),
                Point { x: 0, y } => println!("On the y axis at {y}"),
                Point { x, y } => {
                    println!("On neither axis: ({x}, {y})");
                }
            }
        }
        ```

0. **Деструктуризация кортежей со структурами**:  
    ```rust
    struct Point {
        x: i32,
        y: i32,
    }

    fn main() {
        let p = Point { x: 0, y: 7 };

        // объединённая дестру
        let ((feet, inches), Point { x, y }) = ((3, 10), p);
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    }
    ```

0. **Деструктуризация перечислений**:  
    ```rust
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    fn main() {
        let msg = Message::ChangeColor(255, 255, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(255, 255, 255) => {
                println!("Change to black color!")
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}")
            }
        }
    }
    ```

0. **Деструктуризация встроенных перечислений**:  
    ```rust
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    fn main() {
        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}")
            }
            _ => (),
        }
    }
    ```

0. **Шаблоны игнорирующие значения**:  
    1. Игнорирование аргумента функции:  
        ```rust
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {y}");
        }

        fn main() {
            foo(3, 4);
        }
        ```

    0. Игнорирование отдельных частей шаблона:  
        ```rust
        fn main() {
            let mut setting_value = Some(5);
            let new_setting_value = Some(10);

            match (&mut setting_value, &new_setting_value) {
                (_, None) => {
                    println!("Can't overwrite with non-existent!");
                },
                (None, Some(new_value)) => {
                    setting_value.replace(*new_value);
                }
                (Some(old_value), Some(new_value)) => {
                    *old_value = *new_value;
                },
                _ => {
                    panic!("unreachable code!");
                }
            }

            println!("setting is {setting_value:?}");
        }
        ```
        ```rust
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {first}, {third}, {fifth}")
            }
        }
        ```

    0. Игнорирование оставшейся части (через `..`):  
        ```rust
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };

        match origin {
            Point { x, .. } => println!("x is {x}"),
        }
        ```
        ```rust
        fn main() {
            let numbers = (2, 4, 8, 16, 32);

            match numbers {
                (first, .., last) => {
                    println!("Some numbers: {first}, {last}");
                }
            }
        }
        ```

0. **Связывание переменной сопоставления (через `@`)**:  
    ```rust
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
    ```
