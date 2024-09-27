// Задача
// Реализовать утилиту аналог консольной команды cut (man cut). Утилита должна принимать строки через STDIN, разбивать по разделителю (TAB) на колонки и выводить запрошенные.

// Реализовать поддержку утилитой следующих ключей:

// -f — "fields" - выбрать поля (колонки)

// -d — "delimiter" - использовать другой разделитель

// -s — "separated" - только строки с разделителем


use std::env;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (fields, delimiter, only_separated) = parse_args(&args);

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => continue,
        };

        // Проверяем наличие разделителя, если задан флаг -s
        if only_separated && !line.contains(delimiter) {
            continue;
        }

        // Разбиваем строку по разделителю
        let columns: Vec<&str> = line.split(delimiter).collect();

        // Отбираем запрашиваемые поля
        let output: Vec<String> = fields.iter()
            .filter_map(|&index| {
                if index > 0 && index <= columns.len() {
                    Some(columns[index - 1].to_string())
                } else {
                    None
                }
            })
            .collect();

        // Выводим результат, если есть что выводить
        if !output.is_empty() {
            println!("{}", output.join(&delimiter.to_string()));
        }
    }
}

// Функция для разбора аргументов
fn parse_args(args: &[String]) -> (Vec<usize>, char, bool) {
    let mut fields = Vec::new();
    let mut delimiter = '\t'; // Разделитель по умолчанию
    let mut only_separated = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-f" => {
                if let Some(field_str) = args.get(i + 1) {
                    fields = field_str.split(',')
                        .filter_map(|f| f.parse::<usize>().ok())
                        .collect();
                }
                i += 1; // Пропустить следующий аргумент
            }
            "-d" => {
                if let Some(delim_str) = args.get(i + 1) {
                    delimiter = delim_str.chars().next().unwrap_or('\t');
                }
                i += 1; // Пропустить следующий аргумент
            }
            "-s" => only_separated = true,
            _ => {}
        }
        i += 1;
    }

    (fields, delimiter, only_separated)
}
