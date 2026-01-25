use std::fs;
use std::io::Write;
use std::{env, error::Error, fs::OpenOptions, process};

use colored::Colorize;

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
            "Uso: todo <comando> [argumentos]\nComandos: add, list, done, undone, remove, clear"
                .into(),
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

            println!("{}", "✓ Tarefa adicionada".green());
        }
        "list" => match fs::read_to_string("todos.txt") {
            Ok(conteudo) => {
                let linhas_validas: Vec<&str> =
                    conteudo.lines().filter(|l| !l.trim().is_empty()).collect();

                if linhas_validas.is_empty() {
                    println!("Nenhuma tarefa");
                } else {
                    println!("\n Tarefas:\n");

                    let mut concluidas = 0;
                    let total = linhas_validas.len();

                    for (i, linha) in linhas_validas.iter().enumerate() {
                        let numero = format!("{}.", i + 1);

                        if linha.contains("[x]") {
                            let texto = linha.replace("[x]", "").trim().to_string();

                            println!(
                                "{} {} {}",
                                numero.dimmed(),
                                "".green(),
                                texto.green().strikethrough()
                            );
                            concluidas += 1;
                        } else {
                            let texto = linha.replace("[ ]", "").trim().to_string();
                            println!(
                                "{} {} {}",
                                numero.dimmed(),
                                "󰔟".yellow(),
                                texto.bright_white()
                            );
                        }
                    }

                    println!("\n{}", "─".repeat(30).dimmed());

                    let percentual = (concluidas as f32 / total as f32 * 100.0) as u32;
                    let stats = format!("{} de {} concluídas ({}%)", concluidas, total, percentual);

                    if percentual == 100 {
                        println!("{}", stats.green().bold());
                    } else if percentual >= 50 {
                        println!("{}", stats.yellow());
                    } else {
                        println!("{}", stats.red());
                    }

                    println!();
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

            let mut linhas: Vec<String> = conteudo
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| l.to_string())
                .collect();

            if numero == 0 || numero > linhas.len() {
                return Err("Número de tarefa inválido".into());
            }

            let indice = numero - 1;

            if linhas[indice].contains("[x]") {
                return Err("Tarefa já está marcada como concluída".into());
            }

            linhas[indice] = linhas[indice].replace("[ ]", "[x]");

            fs::write("todos.txt", linhas.join("\n") + "\n")?;

            println!("{}", "✓ Tarefa marcada como concluída".green());
        }

        "undone" => {
            if args.len() < 3 {
                return Err("Uso: todo undone <número>".into());
            }

            let numero: usize = args[2].parse()?;

            let conteudo = fs::read_to_string("todos.txt")?;

            let mut linhas: Vec<String> = conteudo
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| l.to_string())
                .collect();

            if numero == 0 || numero > linhas.len() {
                return Err("Número de tarefa inválido".into());
            }

            let indice = numero - 1;

            if linhas[indice].contains("[ ]") {
                return Err("Tarefa já está desmarcada".into());
            }

            linhas[indice] = linhas[indice].replace("[x]", "[ ]");

            fs::write("todos.txt", linhas.join("\n") + "\n")?;

            println!("{}", "✓ Tarefa desmarcada".yellow());
        }

        "remove" => {
            if args.len() < 3 {
                return Err("Uso: todo remove <número>".into());
            }

            let numero: usize = args[2].parse()?;

            let conteudo = fs::read_to_string("todos.txt")?;

            let mut linhas: Vec<String> = conteudo
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| l.to_string())
                .collect();

            if numero == 0 || numero > linhas.len() {
                return Err("Está tarefa não existe ou já foi removida".into());
            }

            let indice = numero - 1;
            linhas.remove(indice);

            fs::write("todos.txt", linhas.join("\n") + "\n")?;

            println!("{}", "✓ Tarefa removida".red());
        }

        "clear" => {
            if fs::metadata("todos.txt").is_ok() {
                fs::remove_file("todos.txt")?;
                println!("{}", "✓ Todas as tarefas foram removidas".red().bold());
            } else {
                println!("Nenhuma tarefa para remover");
            }
        }

        _ => {
            return Err(format!("Comando desconhecido: {}", comando).into());
        }
    }

    Ok(())
}
