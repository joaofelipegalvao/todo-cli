Gerenciador de tarefas em linha de comando - Projeto de estudo em Rust 🦀

## 📝 Todo CLI

Este projeto foi desenvolvido em 4 etapas, marcada por tags:

| Versão | Descrição | Conceitos |
|--------|-----------|-----------|
| [v1] | CLI básica com add/list | `OpenOptions`, `writeln!`, `enumerate`, `match`, `?` operator |
| [v2] | Comando done para marcar conclusão | `parse()`, `.map().collect()`, `.replace()`, `Vec<String>`, `.join()`, `fs::write()` |
| [v3] | Comando remove para deletar tarefas | `Vec::remove()`, validação de índices, tratamento de erros |
| [v4] | Comando undone para desmarcar conclusão | manipulação inversa de estados, lógica booleana |

[v1]: https://github.com/joaofelipegalvao/todo-cli/releases/tag/v0.1.0
[v2]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.1.0...v0.2.0
[v3]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.2.0...v0.3.0
[v4]: https://github.com/joaofelipegalvao/todo-cli/compare/v0.3.0...v0.4.0

## 🚀 Como usar

```bash
# Adicionar tarefa
cargo run -- add "Estudar Rust"

# Listar tarefas
cargo run -- list

# Marcar tarefa como concluída
cargo run -- done 1

# Desmarcar tarefa como concluída
cargo run -- undone 1

# Remover tarefa
cargo run -- remove 1

# Ver versão específica
git checkout v0.1.0  # ou qualquer tag
```

## 📋 Comandos disponíveis

| Comando | Descrição | Exemplo |
|---------|-----------|---------|
| add | Adiciona nova tarefa | todo add "Estudar Rust" |
| list | Lista todas as tarefas | todo list |
| done | Marca tarefa como concluída | todo done 1 |
| undone | Desmarca tarefa como concluída | todo undone 1 |
| remove | Remove tarefa | todo remove 1 |

## 💡 O que aprendi

- ✅ Manipulação de arquivos com OpenOptions
- ✅ Escrita com writeln! macro
- ✅ Enumeração com enumerate()
- ✅ Pattern matching com match
- ✅ Tratamento de erros com ? operator
- ✅ CLI argument parsing
- ✅ Conversão de tipos com parse()
- ✅ Transformação de coleções com .map().collect()
- ✅ Substituição de strings com .replace()
- ✅ Sobrescrita de arquivos com fs::write()
- ✅ Remoção de elementos com Vec::remove()

## 🎯 Próximos passos

- [x] Comando done para marcar como concluída
- [x] Comando undone para desmarcar conclusão
- [x] Comando remove para deletar tarefas
- [ ] Persistência de estado (concluídas/pendentes)
- [ ] Testes unitários

---

Nota: Este é um projeto de aprendizado. Cada tag representa um passo evolutivo.
