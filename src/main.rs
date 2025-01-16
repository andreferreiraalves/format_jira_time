use core::panic;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() <= 0 {
        panic!("nenhum argumento informado");
    }

    let mut param1 = args[1].clone();
    param1 = param1.to_uppercase();
    let splited = param1.split(" ");

    let mut valor_total: i32 = 0;

    for split_value in splited {
        valor_total += get_valor("W", split_value);
        valor_total += get_valor("D", split_value);
        valor_total += get_valor("H", split_value);
    }

    println!("Total de horas: {}h", valor_total);
}

fn get_valor(tipo: &str, split_value: &str) -> i32 {
    if split_value.contains(tipo) {
        let valor_limpo = split_value.replace(tipo, "");

        return match tipo {
            "W" => valor_limpo.parse::<i32>().unwrap() * 40,
            "D" => valor_limpo.parse::<i32>().unwrap() * 8,
            "H" => valor_limpo.parse::<i32>().unwrap(),
            _ => 0,
        };
    }

    0
}
