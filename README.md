# Deva Shared

Código partilhado entre os repositórios Deva CLI, MCP e Cloud.

## Crates

- `core` - Configuração, logging, tipos partilhados
- `github` - Cliente API do GitHub
- `azure_devops` - Cliente API do Azure DevOps
- `telegram` - Bot do Telegram
- `generator` - Motor de scaffolding

## Uso

Este repositório é usado como submodule nos outros repos Deva.

## Desenvolvimento

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace
```
