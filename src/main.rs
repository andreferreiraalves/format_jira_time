use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() <= 0 {
        println!("nenhum argumento informado");
        process::exit(1);
    }

    let mut param1 = args[1].clone();
    param1 = param1.to_uppercase();
    let splited = param1.split(" ");

    let mut valor_total: f64 = 0.0;

    for split_value in splited {
        valor_total += get_valor("W", split_value);
        valor_total += get_valor("D", split_value);
        valor_total += get_valor("H", split_value);
        valor_total += get_valor("M", split_value);
    }

    println!("Total de horas: {}h", valor_total);
}

fn get_valor(tipo: &str, split_value: &str) -> f64 {
    if split_value.contains(tipo) {
        let valor_limpo = split_value.replace(tipo, "");

        return match tipo {
            "W" => valor_limpo.parse::<f64>().unwrap() * 40.0,
            "D" => valor_limpo.parse::<f64>().unwrap() * 8.0,
            "M" => valor_limpo.parse::<f64>().unwrap() / 60.0,
            "H" => valor_limpo.parse::<f64>().unwrap(),
            _ => 0.0,
        };
    }

    0.0
}
