[← back to library](../../librarium.md)

Цикл `while` в Rust используется для выполнения блока кода до тех пор, пока условие истинно.

Вот основные способы его использования:  
1. **Простой цикл while**:  
    Используйте while для выполнения кода, пока условие истинно.
    ```rust
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Liftoff!");
    ```
0. **Цикл с пользовательским вводом**:  
    while может быть полезен для обработки ввода пользователя, когда вы хотите продолжать запрашивать данные, пока не будет введено определенное значение.
    ```rust
    use std::io;

    let mut input = String::new();
    while input.trim() != "exit" {
        println!("Enter a command (type 'exit' to quit):");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
    }
    ```
0. **Цикл с условием выхода**:  
    Используйте while для выполнения операций до тех пор, пока не будет достигнуто определенное условие.
    ```rust
    let mut sum = 0;
    let mut number = 1;
    while sum < 100 {
        sum += number;
        number += 1;
    }
    println!("Sum reached: {}", sum);
    ```
0. **Цикл с изменением состояния**:  
    while может использоваться для изменения состояния программы до достижения определенного результата.
    ```rust
    let mut balance = 1000.0;
    let interest_rate = 0.05;
    let target_balance = 2000.0;
    let mut years = 0;

    while balance < target_balance {
        balance += balance * interest_rate;
        years += 1;
    }
    println!("It will take {} years to reach the target balance.", years);
    ```
0. **Цикл с вложенными условиями**:  
    Вы можете использовать while с вложенными условиями для более сложной логики.
    ```rust
    let mut x = 0;
    while x < 10 {
        if x % 2 == 0 {
            println!("{} is even", x);
        } else {
            println!("{} is odd", x);
        }
        x += 1;
    }
    ```

Цикл `while` предоставляет гибкость для выполнения повторяющихся операций до тех пор, пока выполняется определенное условие, что делает его полезным для сценариев, где количество итераций заранее неизвестно.