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
| [v0.8.0] | Prioridades + Filtros de prioridade | `Option<T>`, tuplas, pattern matching com tuplas, pipeline de filtros, validação de flags, títulos dinâmicos, psicologia de cores, defaults inteligentes, fail fast |
| [v0.9.0] | Ordenação por prioridade | `.sort_by()`, `Ordering` enum, funções de mapeamento, `u8`, pipeline otimizado (filtrar → ordenar), flags booleanas |
| [v1.0.0] | Busca + Refatoração de exibição | comando `search`, separação parsing/exibição, funções atômicas (`exibir_tarefa`), funções orquestradas (`exibir_tarefas`), reuso sem duplicação |

[v0.1.0]: https://github.com/joaofelipegalvao/todo-cli/releases/tag/v0.1.0
[v0.2.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.1.0...v0.2.0
[v0.3.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.2.0...v0.3.0
[v0.4.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.3.0...v0.4.0
[v0.4.1]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.4.0...v0.4.1
[v0.4.2]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.4.1...v0.4.2
[v0.5.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.4.2...v0.5.0
[v0.6.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.5.0...v0.6.0
[v0.7.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.6.0...v0.7.0
[v0.8.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.7.0...v0.8.0
[v0.9.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.8.0...v0.9.0
[v1.0.0]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.9.0...v1.0.0

## Comandos disponíveis

| Comando | Descrição | Exemplo |
|---------|-----------|---------|
| `add <tarefa>` | Adiciona nova tarefa (prioridade medium) | `todo add "Estudar Rust"` |
| `add <tarefa> --high` | Adiciona tarefa de alta prioridade | `todo add "Reunião urgente" --high` |
| `add <tarefa> --low` | Adiciona tarefa de baixa prioridade | `todo add "Organizar mesa" --low` |
| `list` | Lista todas as tarefas | `todo list` |
| `list --pending` | Lista apenas tarefas pendentes | `todo list --pending` |
| `list --done` | Lista apenas tarefas concluídas | `todo list --done` |
| `list --high` | Lista apenas tarefas de alta prioridade | `todo list --high` |
| `list --low` | Lista apenas tarefas de baixa prioridade | `todo list --low` |
| `list --sort` | Lista tarefas ordenadas por prioridade | `todo list --sort` |
| `list --pending --high` | Combina filtros de status e prioridade | `todo list --pending --high` |
| `list --pending --sort` | Pendentes ordenadas por prioridade | `todo list --pending --sort` |
| `search <termo>` | Busca tarefas que contêm o termo | `todo search "rust"` |
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
- Tuplas `(T, U)` para retornar múltiplos valores de funções
- `.sort_by()` para ordenação customizada com comparador
- `Ordering` enum para comparações type-safe (Less, Equal, Greater)
- `.cmp()` para comparar valores ordenáveis

### Controle de fluxo e erros

- Pattern matching com `match` para subcomandos
- Match aninhado para decisões em múltiplos níveis
- Pattern matching com tuplas `(a, b)` para combinar condições
- Tratamento de erros com `?` operator (propagação automática)
- `Result<T, E>` para funções que podem falhar
- `Box<dyn Error>` para erros genéricos
- `if let` para pattern matching simplificado
- Validação de entrada e pré-condições
- Validação de flags conflitantes (fail fast)
- Mensagens de erro específicas e educativas
- `Option<T>` para valores opcionais (evita "magic values")

### CLI e UX

- `env::args()` para capturar argumentos
- Subcomandos com pattern matching
- Flags opcionais (`--pending`, `--done`, `--high`, `--low`)
- Parsing de argumentos com múltiplas flags
- Combinação de filtros (status + prioridade)
- Validação de argumentos (quantidade, tipo, estado)
- `println!` vs `eprintln!` (stdout vs stderr)
- `process::exit()` para códigos de saída
- Hierarquia visual com cores e formatação
- Feedback imediato com cores semânticas
- Respiração visual (espaços em branco importam)
- Defaults inteligentes (medium como padrão)
- Títulos dinâmicos baseados em contexto
- Prevenção de erros do usuário

### Design e cores

- `colored` crate para cores cross-platform
- `.dimmed()`, `.bold()`, `.strikethrough()` para formatação
- Cores semânticas (verde = sucesso, vermelho = atenção)
- Psicologia de cores (🔴 vermelho = urgente, 🟡 amarelo = normal, 🟢 verde = baixa)
- Sistema de prioridades visuais com emojis
- Hierarquia visual (números dimmed, conteúdo destacado)
- Múltiplos sinais (cor + ícone + riscado) para acessibilidade
- Conversão `as f32` para cálculos de percentual
- `as u32` para truncar decimais
- Reforço positivo (sempre verde para concluídas)
- Redução de poluição visual (prioridade só em pendentes)

### Funções e organização

- Funções auxiliares para evitar duplicação de código (DRY)
- Parâmetros com slices (`&[&str]`) para eficiência
- Reuso de código com funções especializadas
- Separação de responsabilidades (parsing vs exibição)
- Funções de parsing (`extrair_prioridade`)
- Retorno de tuplas para múltiplos valores
- Pipeline de transformações (filtros em sequência)
- Modularização de lógica visual (`emoji_prioridade`)
- Funções de mapeamento (`prioridade_ordem`) para conversão conceito → número
- Escolha de tipos apropriados (`u8` vs `i32`) baseada em semântica
- Funções atômicas (`exibir_tarefa`) para renderização unitária
- Funções orquestradas (`exibir_tarefas`) para exibição completa com estatísticas
- Separação clara: parsing de dados vs renderização visual
- Design de CLI maduro sem duplicação de código

### Debug e qualidade

- Encontrar bugs através de testes manuais
- Uso de `eprintln!` para debug prints
- Investigação de arquivos com `cat` e `od`
- Validação de pré-condições (evitar estados inválidos)
- Pensamento em edge cases (arquivo vazio, índices inválidos)
- Refatoração iterativa sem quebrar funcionalidade
- Consistência entre comandos (filtrar em todos)
- Otimização de pipeline (filtrar antes de ordenar)
- Análise de complexidade (Big-O) para decisões de performance
- Princípio YAGNI (You Aren't Gonna Need It) - não adicionar complexidade desnecessária
- Opt-in complexity - features complexas são opcionais

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
# Adicionar tarefas com diferentes prioridades
todo add "Estudar Rust"                    # prioridade medium (padrão)
todo add "Reunião urgente" --high          # alta prioridade
todo add "Organizar mesa" --low            # baixa prioridade

# Listar todas (ordem de criação)
todo list

# Listar ordenadas por prioridade
todo list --sort

# Buscar tarefas
todo search "rust"                         # encontra todas com "rust" no texto
todo search "urgente"                      # mantém numeração original

# Filtrar por status
todo list --pending
todo list --done

# Filtrar por prioridade
todo list --high
todo list --low

# Combinar filtros
todo list --pending --high                 # pendentes de alta prioridade
todo list --done --low                     # concluídas de baixa prioridade

# Combinar filtros + ordenação
todo list --pending --sort                 # pendentes ordenadas
todo list --high --sort                    # altas ordenadas (já são do mesmo nível)

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
cargo run -- add "Reunião urgente" --high
cargo run -- list
cargo run -- list --sort
cargo run -- search "rust"
cargo run -- list --pending --sort
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
- [x] Filtros de status (--pending, --done)
- [x] Sistema de prioridades (alta/média/baixa)
- [x] Filtros de prioridade (--high, --low)
- [x] Combinação de filtros (status + prioridade)
- [x] Validação de flags conflitantes
- [x] Títulos dinâmicos contextuais
- [x] Ordenação por prioridade (--sort)
- [x] Pipeline otimizado (filtrar → ordenar)
- [x] Comando search para buscar tarefas
- [x] Refatoração: separação parsing/exibição
- [x] Funções atômicas e orquestradas
- [x] Reuso de código sem duplicação

### Próximos passos

- [ ] Categorias/tags (`#trabalho`, `#casa`)
- [ ] Editar tarefa (`edit 1 "novo texto"`)
- [ ] Data de criação/vencimento
- [ ] Ordenação por data (`--sort date`)
- [ ] Formato JSON para dados estruturados
- [ ] Testes unitários
- [ ] Refatoração com structs

---

**Projeto desenvolvido como parte do aprendizado de Rust** 🦀  
*Cada commit representa um passo no processo de aprendizado*
