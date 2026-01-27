use std::fs;
use std::io::Write;
use std::{env, error::Error, fs::OpenOptions, process};

use colored::{ColoredString, Colorize};

fn main() {
    if let Err(e) = run() {
        eprintln!("Erro: {}", e);
        process::exit(1);
    }
}

fn prioridade_ordem(pri: &str) -> u8 {
    match pri {
        "high" => 0,
        "medium" => 1,
        "low" => 2,
        _ => 3,
    }
}

fn extrair_prioridade(linha: &str) -> (&str, String) {
    let sem_checkbox = linha
        .replace("[ ]", "")
        .replace("[x]", "")
        .trim()
        .to_string();

    if sem_checkbox.contains("(high)") {
        let texto = sem_checkbox.replace("(high)", "").trim().to_string();
        ("high", texto)
    } else if sem_checkbox.contains("(low)") {
        let texto = sem_checkbox.replace("(low)", "").trim().to_string();
        ("low", texto)
    } else {
        ("medium", sem_checkbox)
    }
}

fn emoji_prioridade(prioridade: &str) -> ColoredString {
    match prioridade {
        "high" => "".red(),
        "low" => "".green(),
        _ => "".yellow(),
    }
}

fn exibir_tarefa(numero: usize, linha: &str) {
    let numero_fmt = format!("{}.", numero);
    let concluida = linha.contains("[x]");

    let (prioridade, texto) = extrair_prioridade(linha);
    let emoji = emoji_prioridade(prioridade);

    if concluida {
        println!(
            "{} {} {} {}",
            numero_fmt.dimmed(),
            emoji,
            "󰄵".green(),
            texto.green().strikethrough()
        );
    } else {
        println!(
            "{} {} {} {}",
            numero_fmt.dimmed(),
            emoji,
            "".yellow(),
            texto.bright_white()
        );
    }
}

fn exibir_listas(tarefas: &[&str], titulo: &str) {
    println!("\n {}:\n", titulo);

    let mut concluidas = 0;
    let total = tarefas.len();

    for (i, linha) in tarefas.iter().enumerate() {
        exibir_tarefa(i + 1, linha);

        if linha.contains("[x]") {
            concluidas += 1;
        }
    }

    println!("\n{}", "─".repeat(30).dimmed());

    let percentual = if total > 0 {
        (concluidas as f32 / total as f32 * 100.0) as u32
    } else {
        0
    };

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

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(
            "Uso: todo <comando> [argumentos]\nComandos: add, list [--pending| --done], done, undone, remove, clear"
                .into(),
        );
    }

    let comando = &args[1];

    match comando.as_str() {
        "add" => {
            if args.len() < 3 {
                return Err("Uso: todo add <tarefa> [--high | --low]".into());
            }

            let tarefa = &args[2];

            let linha = match args.len() {
                3 => format!("[ ] {}", tarefa),

                4 => {
                    let flag = args[3].as_str();
                    match flag {
                        "--high" => format!("[ ] (high) {}", tarefa),
                        "--low" => format!("[ ] (low) {}", tarefa),

                        _ => {
                            return Err(
                                format!("Flag inválida '{}'. Use --high ou --low", flag).into()
                            );
                        }
                    }
                }

                _ => {
                    return Err(
                        "Uso: todo add <tarefa> [--high | --low]. Apenas uma flag é permitida"
                            .into(),
                    );
                }
            };

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("todos.txt")?;

            writeln!(file, "{}", linha)?;

            println!("{}", "✓ Tarefa adicionada".green());
        }

        "list" => {
            let mut filtro_status = "all";
            let mut filtro_prioridade: Option<&str> = None;
            let mut ordenar = false;

            for arg in &args[2..] {
                match arg.as_str() {
                    "--pending" => {
                        if filtro_status != "all" {
                            return Err(
                                "Use apenas um filtro de status (--pending ou --done).\nExemplo válido: todo list --pending --high".into()
                            );
                        }
                        filtro_status = "pending";
                    }
                    "--done" => {
                        if filtro_status != "all" {
                            return Err(
                                "Use apenas um filtro de status (--done ou --pending).".into()
                            );
                        }
                        filtro_status = "done";
                    }
                    "--high" => {
                        if filtro_prioridade.is_some() {
                            return Err(
                                "Use apenas um filtro de prioridade (--high ou --low)".into()
                            );
                        }
                        filtro_prioridade = Some("high");
                    }
                    "--low" => {
                        if filtro_prioridade.is_some() {
                            return Err(
                                "Use apenas um filtro de prioridade (--low ou --high)".into()
                            );
                        }

                        filtro_prioridade = Some("low");
                    }
                    "--sort" => {
                        if ordenar {
                            return Err("Use --sort apenas uma vez.".into());
                        }
                        ordenar = true;
                    }
                    _ => return Err(format!("Filtro inválido: {}", arg).into()),
                }
            }

            match fs::read_to_string("todos.txt") {
                Ok(conteudo) => {
                    let mut linhas_validas: Vec<&str> =
                        conteudo.lines().filter(|l| !l.trim().is_empty()).collect();

                    if linhas_validas.is_empty() {
                        println!("Nenhuma tarefa");
                        return Ok(());
                    }

                    linhas_validas = match filtro_status {
                        "pending" => linhas_validas
                            .iter()
                            .filter(|l| l.contains("[ ]"))
                            .copied()
                            .collect(),
                        "done" => linhas_validas
                            .iter()
                            .filter(|l| l.contains("[x]"))
                            .copied()
                            .collect(),
                        _ => linhas_validas,
                    };

                    if let Some(pri) = filtro_prioridade {
                        linhas_validas = linhas_validas
                            .iter()
                            .filter(|linha| {
                                let (prioridade, _) = extrair_prioridade(linha);
                                prioridade == pri
                            })
                            .copied()
                            .collect();
                    }

                    if linhas_validas.is_empty() {
                        println!("Nenhuma tarefa encontrada com esses filtros");
                        return Ok(());
                    }

                    if ordenar {
                        linhas_validas.sort_by(|a, b| {
                            let (pri_a, _) = extrair_prioridade(a);
                            let (pri_b, _) = extrair_prioridade(b);
                            prioridade_ordem(pri_a).cmp(&prioridade_ordem(pri_b))
                        });
                    }

                    let titulo = match (filtro_status, filtro_prioridade) {
                        ("pending", Some("high")) => "Tarefas pendentes de alta prioridade",
                        ("pending", Some("low")) => "Tarefas pendentes de baixa prioridade",
                        ("pending", None) => "Tarefas pendentes",
                        ("done", Some("high")) => "Tarefas concluídas de alta prioridade",
                        ("done", Some("low")) => "Tarefas concluídas de baixa prioridade",
                        ("done", None) => "Tarefas concluídas",
                        (_, Some("high")) => "Alta prioridade",
                        (_, Some("low")) => "Baixa prioridade",
                        _ => "Tarefas",
                    };

                    exibir_listas(&linhas_validas, titulo);
                }

                Err(_) => {
                    println!("Nenhuma tarefa");
                }
            }
        }

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

        "search" => {
            if args.len() < 3 {
                return Err("Uso: todo search <termo>".into());
            }

            let termo = &args[2];

            match fs::read_to_string("todos.txt") {
                Ok(conteudo) => {
                    let linhas_validas: Vec<&str> =
                        conteudo.lines().filter(|l| !l.trim().is_empty()).collect();

                    if linhas_validas.is_empty() {
                        println!("Nenhuma tarefa");
                        return Ok(());
                    }

                    let mut resultados: Vec<(usize, &str)> = Vec::new();

                    for (i, linha) in linhas_validas.iter().enumerate() {
                        if linha.to_lowercase().contains(&termo.to_lowercase()) {
                            resultados.push((i + 1, linha));
                        }
                    }

                    if resultados.is_empty() {
                        println!("Nenhum resultado para '{}'", termo);
                    } else {
                        println!("\n Resultados para \"{}\":\n", termo);

                        for (numero, linha) in &resultados {
                            exibir_tarefa(*numero, linha);
                        }

                        println!("\n{} Resultados(s) encontrado(s)\n", resultados.len());
                    }
                }

                Err(_) => {
                    println!("Nenhuma tarefa");
                }
            }
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
