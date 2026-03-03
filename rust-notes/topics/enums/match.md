# Match

Match - механизм управление потоком. Я уже использовал его в практической работе `bank_account`, так что уже немного с ним знаком. Пример:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Каждая ветка `match` шаблон и некоторый код. `match` ищет первое совпадение и выполняет определённый для этого варианта код. Код связанный с каждым ответвлением является выражением, а результат выполнения этого выражения - это значение которое возвращается для всего `match`.

Если необходимо выполнить несколько строк кода для ответвления этот код помещается в фигурные скобки, после них запятая не нужна.

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Привязывание значений

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```

`match` способен не только проверять вариант, он также может распаковывать данные внутри него. То есть `state` не существует заранее, он создастся только при совпадении шаблона.

### Option + match
В этом абзаце книги нам просто показывают, как удобно обрабатывать `Option` при помощи `match`. И ещё раз показывают, что `match` одновременно проверяет варианты и распаковывает данные из него. `i` не существует пока не будет совпадения с веткой `Some`.

```rust
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

### Универсальные шаблоны и заполнитель `_`.
`match` даёт возможность обработать сразу несколько шаблонов одним кодом. Например вариантов 9. Для двух вариантов есть специфичный код, а для всех остальных мы хотим, чтобы исполнялся один и тот же код. Сначала описываем варианты с индивидуальным кодом так как `match` проверяет ветки одну за другой. Последней веткой в шаблоне указываем имя переменной, которую потом используем в коде.

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}
```

Если в коде переменная не используется, нужно использовать специальный заполнитель `_`.
```rust
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
```

Если необходимо, чтобы при совпадении с каким либо шаблоном, не происходило ничего используем `()` в блоке кода
```rust
match dice_roll {
  3 => add_fancy_hat(),
  7 => remove_fancy_hat(),
  _ => (),
}
```
