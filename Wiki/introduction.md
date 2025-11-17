# Оглавление
* Тезаурус
* Процедура установки Rust
    * Настройка IDE VSCode для работы с Rust
    * Настройка и сборка проекта Rust
    * Без использования Cargo
    * С использованием Cargo
* Тестирование кода на Rust
    * cargo команды
* Идеи проектов для реализации на Rust
    * HashSum
    * SQL-BD
    * Web-сайт для мастерской


<h1 style="text-align: center; color: #ff9933; font-size: 56px;">Rust</h1>

# Тезаурус
* **Крейт** — это наименьший объем кода, который компилятор Rust рассматривает за раз.  
Крейт может быть одним из двух видов: бинарный крейт или библиотечный крейт.
* **Пакет** — это набор из одного или нескольких крейтов, предоставляющий набор функциональности.  
Пакет содержит файл `Cargo.toml`, в котором описывается, как собирать эти крейты.
* **Мономорфизация** — это процесс превращения обобщённого кода в конкретный код путём подстановки конкретных типов, использующихся при компиляции
* **Типаж (Trait)** - это конструкция, которая сообщает компилятору Rust о функциональности, которой обладает определённый тип и которой он может поделиться с другими типами.

# Процедура установки Rust
1. Скачать последний дистрибутив в [официального сайта](https://www.rust-lang.org/tools/install) и запустить установочник.
2. Добавить в PATH путь к Rust: C:\Users\<UserName>\.cargo\bin
3. Проверить доступность rustc:
```bash
where rustc
```
```bash
rustc.exe --version
```
4. Обновиться до последней стабильной версии:
```bash
rustup update
```

## Настройка IDE VSCode для работы с Rust
* для подчёркивания синтаксиса:
```
Name: Rust Syntax
Id: dustypomerleau.rust-syntax
Description: Improved Rust syntax highlighting
Version: 0.6.1
Publisher: Dusty Pomerleau
VS Marketplace Link: https://marketplace.visualstudio.com/items?itemName=dustypomerleau.rust-syntax
```
* для анализа кода:
```
Name: rust-analyzer
Id: rust-lang.rust-analyzer
Description: Rust language support for Visual Studio Code
Version: 0.3.2062
Publisher: The Rust Programming Language 
VS Marketplace Link: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
```

* для управления пакетами:
```
Name: Dependi
Id: fill-labs.dependi
Description: Empowers developers to efficiently manage dependencies and address vulnerabilities in Rust, Go, JavaScript, Typescript, PHP and Python projects.
Version: 0.7.7
Publisher: Fill Labs
VS Marketplace Link: https://marketplace.visualstudio.com/items?itemName=fill-labs.dependi
```

## Настройка и сборка проекта Rust
## Без использования Cargo
1. `rustc.exe main.rs` → компиляция проекта

## С использованием Cargo
1. `cargo new project_name` → создание проекта (создаётся новая директория и все необходимые конфигурационные файлы)
2. `cargo build` → компиляция проекта
3. `cargo run` → компиляция и запуск проекта
4. `cargo check` → быстрая проверка на возможность компиляции (без компиляции)
5. `cargo build --release` → сборка проекта в release-конфигурации
6. `cargo run --release`→ сборка и запуск проекта в release-конфигурации

# Тестирование кода на Rust
Тестами являются функции, которые помечены атрибутом `#[test]`  
Существует 3 типа тестов:  
1. модульные тесты - для тестирования отдельно взятого функционала конкретного модуля.
2. интеграционные тесты - для тестирования взаимодействия между модулями.
3. тесты документации - ???

## cargo команды
1. `cargo test` - запускает все тесты.
2. `cargo test scenario_1` - запускает все тесты в имени которых есть `scenario_1`.
3. `cargo test --help` - выводит список доступных действий с тестами:  
Вот некоторые из самых полезных:  
*  `cargo test -- --test-threads=1` - запуск тестов однопоточном режиме.
* `cargo test -- --show-output` - не игнорировать вывод успешных тестов.
