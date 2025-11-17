[← back to library](../../librarium.md)

Цикл `loop` в Rust является бесконечным циклом, который продолжает выполняться до тех пор, пока не будет явно прерван. Он может быть полезен в различных сценариях.

Вот основные способы его использования:  
1. **Бесконечный цикл с условием выхода**:  
    Используйте loop для создания цикла, который будет выполняться до тех пор, пока не будет выполнено определенное условие. Выход из цикла осуществляется с помощью break.
    ```rust
    let mut count = 0;
    loop {
        count += 1;
        if count == 10 {
            break;
        }
    }
    println!("Exited the loop after {} iterations", count);
    ```
0. **Обработка ввода пользователя**:  
    loop может быть полезен для обработки ввода пользователя, когда вы не знаете заранее, сколько раз пользователь захочет ввести данные.
    ```rust
    use std::io;

    loop {
        println!("Enter a number (or 'exit' to quit):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "exit" {
            break;
        }

        match input.trim().parse::<i32>() {
            Ok(num) => println!("You entered: {}", num),
            Err(_) => println!("Please enter a valid number"),
        }
    }
    ```
0. **Повторение до успешного выполнения**:  
    Используйте loop, чтобы повторять операцию до тех пор, пока она не будет выполнена успешно.
    ```rust
    let mut attempts = 0;
    loop {
        attempts += 1;
        if perform_operation() {
            println!("Operation succeeded after {} attempts", attempts);
            break;
        }
    }

    fn perform_operation() -> bool {
        // Симулируем операцию, которая может завершиться неудачей
        // Например, возвращаем true с вероятностью 1/3
        rand::random::<u8>() % 3 == 0
    }
    ```
0. **Использование с метками для управления вложенными циклами**:  
    loop может быть использован с метками для управления сложными вложенными циклами.
    ```rust
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // Выход из внутреннего цикла
            break 'inner;
        }

        // Выход из внешнего цикла
        break 'outer;
    }
    println!("Exited both loops");
    ```

Цикл `loop` предоставляет гибкость для создания различных сценариев повторения, особенно когда количество итераций заранее неизвестно или зависит от внешних условий.