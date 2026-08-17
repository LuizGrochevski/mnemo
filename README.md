# mnemo

Ferramenta de flashcards estilo Anki, feita para rodar direto no terminal. Sem interface gráfica, sem sincronização na nuvem, sem distração: só você, seus decks e o algoritmo de repetição espaçada decidindo quando é hora de revisar cada card de novo.

Escrita em Rust, com banco local em SQLite. Nasceu como um projeto pra variar do universo de segurança/rede (Sentinel-RS, netwatch, TrapRS) e praticar Rust em outro domínio.

## Por que existe

Repetição espaçada é a técnica mais eficiente pra fixar conhecimento a longo prazo — a ideia é revisar cada informação exatamente no momento em que você está prestes a esquecê-la, nem antes (perda de tempo) nem depois (já esqueceu). O `mnemo` implementa isso com o algoritmo **SM-2**, o mesmo usado nas primeiras versões do Anki e do SuperMemo.

## Como funciona hoje (MVP)

- Criação de decks (`mnemo deck <nome>`)
- Adição de cards com frente/verso (`mnemo add <deck> <frente> <verso>`)
- Sessão de revisão diária (`mnemo review <deck>`) que:
  - Mostra apenas os cards que estão "devidos" (due) na data atual
  - Pede uma nota de 0 a 5 pra cada card, indicando o quão bem você lembrou
  - Recalcula o próximo intervalo de revisão via SM-2 e atualiza o banco
- Persistência local em SQLite (`mnemo.db`), sem dependência de rede

## Stack

- **Rust** (edition 2021)
- `rusqlite` (SQLite embutido, feature `bundled` — sem precisar instalar libsqlite3 separado)
- `clap` (parsing de CLI via derive macros)
- `chrono` (datas de vencimento dos cards)
- `anyhow` (tratamento de erros simplificado)

Ambiente de desenvolvimento: Termux/Android, sem Docker.

## Uso

```bash
cargo build --release

./target/release/mnemo deck "rust"
./target/release/mnemo add "rust" "O que é ownership?" "Sistema de gerenciamento de memória sem garbage collector"
./target/release/mnemo review "rust"
```

## Roadmap

### Curto prazo
- [ ] Comando `mnemo stats` — quantidade de cards por deck, quantos due hoje, streak de dias revisados
- [ ] Comando `mnemo list <deck>` — listar todos os cards de um deck (sem entrar em modo revisão)
- [ ] Comando `mnemo delete` — remover card ou deck inteiro
- [ ] Validação de decks duplicados e mensagens de erro mais amigáveis

### Médio prazo
- [ ] Importação de cards via CSV (`mnemo import <deck> <arquivo.csv>`)
- [ ] Exportação de deck pra CSV/Markdown (backup e portabilidade)
- [ ] Editar card existente (`mnemo edit`)
- [ ] Suporte a múltiplos "baralhos" com tags/categorias
- [ ] Modo TUI com `ratatui` pra sessão de revisão (reaproveitando aprendizado do syswatch-tui)

### Longo prazo
- [ ] Estatísticas visuais (gráfico ASCII de retenção ao longo do tempo)
- [ ] Suporte a cards com múltiplas linhas / markdown simples no front-back
- [ ] Sincronização opcional entre dispositivos (arquivo único `mnemo.db`, sync manual via git/rsync)
- [ ] Suporte a imagens/áudio nos cards (cloze deletion básico)
- [ ] Testes automatizados (unit tests pro SM-2 e integração pro fluxo de review)

## Licença

Projeto pessoal de estudo, sem licença formal definida ainda.

