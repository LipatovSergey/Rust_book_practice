pub fn run_examples() {
    example_1()
}

// Этот пример показывает как пришлось бы реализовать функцию которая возвращает первое слово
// которое она находит в переданной ей строке, если в строке нет пробелов, то возвращает всю
// строку.
fn example_1() {
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes(); // преобразуем в массив байтов

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }
}
