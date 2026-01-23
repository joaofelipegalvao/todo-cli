use std::fs;
use std::io::Write;
use std::{env, error::Error, fs::OpenOptions, process};

fn main() {
    if let Err(e) = run() {
        eprintln!("Erro: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(
            "Uso: todo <comando> [argumentos]\nComandos: add, list, done, undone, remove".into(),
        );
    }

    let comando = &args[1];

    match comando.as_str() {
        "add" => {
            if args.len() < 3 {
                return Err("Uso: todo add <tarefa>".into());
            }

            let tarefa = &args[2];

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("todos.txt")?;

            writeln!(file, "[ ] {}", tarefa)?;

            println!("✓ Tarefa adicionada");
        }
        "list" => match fs::read_to_string("todos.txt") {
            Ok(conteudo) => {
                for (i, linha) in conteudo.lines().enumerate() {
                    println!("{}. {}", i + 1, linha);
                }
            }
            Err(_) => {
                println!("Nenhuma tarefa");
            }
        },

        "done" => {
            if args.len() < 3 {
                return Err("Uso: todo done <número>".into());
            }

            let numero: usize = args[2].parse()?;

            let conteudo = fs::read_to_string("todos.txt")?;

            let mut linhas: Vec<String> = conteudo.lines().map(|l| l.to_string()).collect();

            if numero == 0 || numero > linhas.len() {
                return Err("Número de tarefa inválido".into());
            }

            let indice = numero - 1;
            linhas[indice] = linhas[indice].replace("[ ]", "[x]");

            fs::write("todos.txt", linhas.join("\n") + "\n")?;

            println!("✓ Tarefa marcada como concluída");
        }

        "undone" => {
            if args.len() < 3 {
                return Err("Uso: todo undone <número>".into());
            }

            let numero: usize = args[2].parse()?;

            let conteudo = fs::read_to_string("todos.txt")?;

            let mut linhas: Vec<String> = conteudo.lines().map(|l| l.to_string()).collect();

            if numero == 0 || numero > linhas.len() {
                return Err("Número de tarefa inválido".into());
            }

            let indice = numero - 1;
            linhas[indice] = linhas[indice].replace("[x]", "[ ]");

            fs::write("todos.txt", linhas.join("\n") + "\n")?;

            println!("✓ Tarefa desmarcada");
        }

        "remove" => {
            if args.len() < 3 {
                return Err("Uso: todo remove <número>".into());
            }

            let numero: usize = args[2].parse()?;

            let conteudo = fs::read_to_string("todos.txt")?;

            let mut linhas: Vec<String> = conteudo.lines().map(|l| l.to_string()).collect();

            if numero == 0 || numero > linhas.len() {
                return Err("Número de tarefa inválido".into());
            }

            let indice = numero - 1;
            linhas.remove(indice);

            fs::write("todos.txt", linhas.join("\n") + "\n")?;

            println!("✓ Tarefa removida");
        }
        _ => {
            return Err(format!("Comando desconhecido: {}", comando).into());
        }
    }

    Ok(())
}
