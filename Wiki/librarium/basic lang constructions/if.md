[← back to library](../../librarium.md)

В языке программирования Rust конструкция `if` используется для выполнения условных операций.  

Вот основные способы использования if в Rust:  
1. **Простое условие**:  
    Используйте if для выполнения блока кода, если условие истинно.
    ```rust
    let number = 5;
    if number > 0 {
        println!("The number is positive");
    }
    ```
0. **Условие с else**:  
    Добавьте блок else для выполнения кода, если условие ложно.
    ```rust
    let number = -3;
    if number > 0 {
        println!("The number is positive");
    } else {
        println!("The number is not positive");
    }
    ```
0. **Условие с else if**:  
    Используйте else if для проверки нескольких условий последовательно.
    ```rust
    let number = 0;
    if number > 0 {
        println!("The number is positive");
    } else if number < 0 {
        println!("The number is negative");
    } else {
        println!("The number is zero");
    }
    ```
0. **Условное присваивание**:  
    Используйте if в выражениях для присваивания значений переменным.
    ```rust
    let condition = true;
    let number = if condition { 5 } else { 10 };
    println!("The number is: {}", number);
    ```
0. **Вложенные условия**:  
    Вы можете вкладывать конструкции if друг в друга для более сложной логики.
    ```rust
    let x = 5;
    let y = 10;
    if x > 0 {
        if y > 0 {
            println!("Both x and y are positive");
        }
    }
    ```
0. **Использование с логическими операторами**:  
    Вы можете комбинировать условия с помощью логических операторов && (и) и || (или).
    ```rust
    let x = 5;
    let y = 10;
    if x > 0 && y > 0 {
        println!("Both x and y are positive");
    }
    ```
0. **Использование в циклах**:  
    if может использоваться внутри циклов для выполнения условных операций на каждой итерации.
    ```rust
    for i in 1..=5 {
        if i % 2 == 0 {
            println!("{} is even", i);
        } else {
            println!("{} is odd", i);
        }
    }
    ```

Эти примеры демонстрируют, как конструкция `if` может быть использована для управления потоком выполнения программы в зависимости от условий.

