# Todo CLI 🦀

> Gerenciador de tarefas em linha de comando - Projeto de estudo em Rust

Um gerenciador de tarefas simples, colorido e funcional desenvolvido para aprender Rust na prática, com foco em CLI, manipulação de arquivos, tratamento de erros e UX visual.

## Evolução do projeto

Este projeto foi desenvolvido de forma incremental. Cada versão adiciona uma feature e conceitos novos:

| Versão | Descrição | Conceitos |
|--------|-----------|-----------|
| [v0.1.0] | CLI básica com add/list | `OpenOptions`, `writeln!`, `enumerate`, `match`, `?` operator |
| [v0.2.0] | Comando done para marcar conclusão | `parse()`, `.map().collect()`, `.replace()`, `Vec<String>`, `.join()`, `fs::write()` |
| [v0.3.0] | Comando remove para deletar tarefas | `Vec::remove()`, validação de índices, tratamento de erros |
| [v0.4.0] | Comando undone para desmarcar | manipulação inversa de estados, lógica booleana |
| [v0.4.1] | Correção: bug no comando list | `trim()`, filtro de linhas vazias, tratamento robusto |
| [v0.4.2] | Correção: validações de estado | validação de duplicação, mensagens específicas, pré-condições |
| [v0.5.0] | Comando clear para limpar tudo | `fs::remove_file()`, `fs::metadata()`, tratamento completo |
| [v0.6.0] | Interface visual com cores | `colored`, hierarquia visual, design UX, formatação dinâmica |
| [v0.7.0] | Filtros avançados (--pending, --done) | flags opcionais, `.filter()`, `.copied()`, funções auxiliares, slices |

[v0.1.0]: https://github.com/joaofelipegalvao/todo-cli/releases/tag/v0.1.0
[v0.2.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.1.0...v0.2.0
[v0.3.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.2.0...v0.3.0
[v0.4.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.3.0...v0.4.0
[v0.4.1]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.4.0...v0.4.1
[v0.4.2]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.4.1...v0.4.2
[v0.5.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.4.2...v0.5.0
[v0.6.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.5.0...v0.6.0
[v0.7.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.6.0...v0.7.0

## Comandos disponíveis

| Comando | Descrição | Exemplo |
|---------|-----------|---------|
| `add <tarefa>` | Adiciona nova tarefa | `todo add "Estudar Rust"` |
| `list` | Lista todas as tarefas | `todo list` |
| `list --pending` | Lista apenas tarefas pendentes | `todo list --pending` |
| `list --done` | Lista apenas tarefas concluídas | `todo list --done` |
| `done <número>` | Marca tarefa como concluída | `todo done 1` |
| `undone <número>` | Desmarca tarefa | `todo undone 1` |
| `remove <número>` | Remove tarefa específica | `todo remove 1` |
| `clear` | Remove todas as tarefas | `todo clear` |

```bash
# Ver código de uma versão específica
git checkout v0.1.0  # ou qualquer tag acima
```

## O que aprendi

### Manipulação de arquivos

- `OpenOptions` com `.create()` e `.append()` para adicionar sem sobrescrever
- `writeln!` macro para escrita formatada
- `fs::read_to_string()` para leitura completa
- `fs::write()` para sobrescrever arquivo inteiro
- `fs::remove_file()` para deletar arquivos
- `fs::metadata()` para verificar existência sem abrir

### Strings e coleções

- `enumerate()` para obter índices + valores em loops
- `parse()` para conversão string → número com validação
- `.map().collect()` para transformar iteradores
- `.replace()` para substituição de texto
- `.contains()` para busca em strings
- `.trim()` para remover espaços em branco
- `.to_string()` para resolver lifetimes (`&str` → `String`)
- `.join()` para concatenar com separador
- `.filter()` para selecionar elementos
- `.copied()` para converter `&&str` → `&str` em iteradores
- `Vec::remove()` para deletar por índice
- `.repeat()` para strings repetidas
- Slices `&[&str]` para passar fatias de dados sem copiar

### Controle de fluxo e erros

- Pattern matching com `match` para subcomandos
- Match aninhado para decisões em múltiplos níveis
- Tratamento de erros com `?` operator (propagação automática)
- `Result<T, E>` para funções que podem falhar
- `Box<dyn Error>` para erros genéricos
- `if let` para pattern matching simplificado
- Validação de entrada e pré-condições
- Mensagens de erro específicas (não genéricas)

### CLI e UX

- `env::args()` para capturar argumentos
- Subcomandos com pattern matching
- Flags opcionais (`--pending`, `--done`)
- Parsing de argumentos com flags
- Validação de argumentos (quantidade, tipo, estado)
- `println!` vs `eprintln!` (stdout vs stderr)
- `process::exit()` para códigos de saída
- Hierarquia visual com cores e formatação
- Feedback imediato com cores semânticas
- Respiração visual (espaços em branco importam)

### Design e cores

- `colored` crate para cores cross-platform
- `.dimmed()`, `.bold()`, `.strikethrough()` para formatação
- Cores semânticas (verde = sucesso, vermelho = atenção)
- Hierarquia visual (números dimmed, conteúdo destacado)
- Múltiplos sinais (cor + ícone + riscado) para acessibilidade
- Conversão `as f32` para cálculos de percentual
- `as u32` para truncar decimais

### Funções e organização

- Funções auxiliares para evitar duplicação de código (DRY)
- Parâmetros com slices (`&[&str]`) para eficiência
- Reuso de código com funções especializadas
- Separação de responsabilidades (parsing vs exibição)

### Debug e qualidade

- Encontrar bugs através de testes manuais
- Uso de `eprintln!` para debug prints
- Investigação de arquivos com `cat` e `od`
- Validação de pré-condições (evitar estados inválidos)
- Pensamento em edge cases (arquivo vazio, índices inválidos)
- Refatoração iterativa sem quebrar funcionalidade
- Consistência entre comandos (filtrar em todos)

### Lifetimes e ownership

- Problema de lifetime com `.trim()` retornando `&str`
- Solução com `.to_string()` para criar `String` owned
- Diferença entre referência temporária e valor próprio
- Compilador detectando uso de referências inválidas
- `.copied()` para trabalhar com referências duplas (`&&str`)

## Instalação

```bash
# Clonar repositório
git clone https://github.com/joaofelipegalvao/todo-cli
cd todo-cli

# Compilar
cargo build --release

# Instalar globalmente (opcional)
sudo cp target/release/todo-cli /usr/local/bin/todo
```

## Como usar

### Após instalar globalmente

```bash
# Adicionar tarefas
todo add "Estudar Rust"
todo add "Fazer exercícios"

# Listar todas
todo list

# Listar apenas pendentes
todo list --pending

# Listar apenas concluídas
todo list --done

# Marcar como concluída
todo done 1

# Desmarcar
todo undone 1

# Remover tarefa
todo remove 1

# Limpar todas
todo clear
```

### Com Cargo (desenvolvimento)

```bash
cargo run -- add "Estudar Rust"
cargo run -- list
cargo run -- list --pending
cargo run -- list --done
cargo run -- done 1
cargo run -- undone 1
cargo run -- remove 1
cargo run -- clear
```

## Roadmap

### Implementado

- [x] Comando add para adicionar tarefas
- [x] Comando list para listar todas
- [x] Comando done para marcar como concluída
- [x] Comando undone para desmarcar
- [x] Comando remove para deletar específica
- [x] Comando clear para limpar todas
- [x] Validação completa de erros
- [x] Tratamento robusto de arquivo
- [x] Interface visual com cores
- [x] Contador de progresso com percentual
- [x] Hierarquia visual e formatação
- [x] Filtros avançados (--pending, --done)

### Próximos passos

- [ ] Prioridades (alta/média/baixa)
- [ ] Categorias/tags (`#trabalho`, `#casa`)
- [ ] Busca (`search "rust"`)
- [ ] Editar tarefa (`edit 1 "novo texto"`)
- [ ] Data de criação/vencimento
- [ ] Formato JSON para dados estruturados
- [ ] Testes unitários
- [ ] Refatoração com structs

---

**Projeto desenvolvido como parte do aprendizado de Rust** 🦀  
*Cada commit representa um passo no processo de aprendizado*
