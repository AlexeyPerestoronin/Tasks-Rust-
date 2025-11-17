[← back to library](../../librarium.md)

Ключевое слово `move` перед замыканием позволяет замыканию захватывать переменные по значению, что фактически означает получение замыканием собственной копии переменной!

Для демонстрации сказанного, сравните вывод:
1. **Случай 1: захват локальной переменной по значению**:  
    ```rust
    fn main() -> Result<(), Box<dyn Error>> {
        let mut check_closure = {
            let mut counter = 0;
            move || {
                if counter < 5 {
                    counter += 1;
                    Some(counter)
                } else {
                    None
                }
            }
        };

        while let Some(v) = check_closure() {
            println!("v = {v}");
            thread::sleep(Duration::from_secs(1));
        }

        Ok(())
    }
    ```
    Вывод:
    ```
    v = 1
    v = 2
    v = 3
    v = 4
    v = 5
    ```

0. **Случай 2: захват глобальной переменной по значению**:  
    ```rust
    fn main() -> Result<(), Box<dyn Error>> {
        let mut counter = 0;
        let mut check_closure = {
            move || {
                if counter < 5 {
                    counter += 1;
                    Some(counter)
                } else {
                    None
                }
            }
        };

        while let Some(v) = check_closure() {
            println!("v = {v} (counter = {counter})");
            thread::sleep(Duration::from_secs(1));
        }

        Ok(())
    }
    ```
    Вывод:
    ```
    v = 1 (counter = 0)
    v = 2 (counter = 0)
    v = 3 (counter = 0)
    v = 4 (counter = 0)
    v = 5 (counter = 0)
    ```