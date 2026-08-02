# Zombie Dice (Rust)

Implementação em terminal do jogo de dados **Zombie Dice**, feita em Rust.

Esse projeto é um refazimento de uma versão que eu tinha feito antes em C++. A ideia principal foi usar o Zombie Dice como desculpa pra aprender Rust na prática — especialmente ownership/borrowing, enums com dados associados, e organização de código em módulos.

> **Estado do projeto:** funcional, mas cru. A lógica do jogo (regras do Zombie Dice) está correta até onde testei, mas boa parte da interface é só `println!`/`stdin` hardcoded no terminal, sem tratamento robusto de erros de input. É um projeto de aprendizado, não um produto polido.

## Como rodar

```bash
cargo run
```

O jogo pede o nome dos jogadores separados por vírgula (entre 2 e 6 jogadores), embaralha a ordem de turno, e a partir daí segue o fluxo padrão do Zombie Dice: cada jogador rola os dados, decide se continua arriscando ou para pra guardar os cérebros, até alguém alcançar 13 cérebros.

## Regras do jogo (resumo)

Zombie Dice é jogado com 13 dados de 3 cores, cada uma com uma distribuição diferente de faces:

| Cor       | Cérebro | Pegada (tiro perdido) | Tiro |
|-----------|:-------:|:----------------------:|:----:|
| Verde     | 3       | 2                       | 1    |
| Amarelo   | 2       | 2                       | 2    |
| Vermelho  | 1       | 2                       | 3    |

Em cada rodada, o jogador ativo:
1. Rola 3 dados do saco.
2. Cada face `Cérebro` conta ponto acumulado na rodada; cada face `Tiro` conta como "abatido"; a face `Pegada` significa que aquele dado específico será rolado de novo (retorna pro saco na próxima puxada).
3. Se acumular **3 tiros** na mesma rodada, o jogador perde tudo que acumulou naquela rodada e o turno passa.
4. O jogador pode escolher parar (**hold**) a qualquer momento antes disso, guardando os cérebros acumulados na rodada para o total definitivo.
5. O primeiro jogador a atingir **13 cérebros** acumulados no total aciona a rodada final — todos os outros jogadores jogam mais uma vez para tentar superá-lo. Quem tiver mais cérebros no fim vence; em caso de empate, os empatados jogam rodadas extras até haver um vencedor único.

## Arquitetura

O projeto é organizado como uma **máquina de estados** simples, orquestrada em `main.rs` por um loop que chama `Update` a cada iteração:

```
main.rs
├── game_controler.rs   -> Estado (enum), InfoTemp (struct), Update() e todas as funções de cada estado
├── dice_bag.rs          -> DiceBag: gerencia o "saco" e a "mesa" de dados
├── zdice.rs              -> Zdice: dado individual (cor, faces, face que caiu) + Face (enum)
└── player.rs             -> Player: nome, cérebros comidos, turnos jogados
```

### Máquina de estados

```
Welcome -> PreparingMatch -> Playing <-> Rolling
                                |           |
                                v           v
                             Holding      Lost
                                |           |
                                v           v
                            TurnResult <----+
                             /    |    \
                            v     v     v
                          Draw  Playing  Win
```

- **Welcome**: lê o nome dos jogadores via stdin.
- **PreparingMatch**: embaralha e exibe a ordem de turno.
- **Playing**: exibe as opções do turno (rolar, encerrar turno, sair).
- **Rolling**: puxa 3 dados da mesa e processa os resultados (cérebro/tiro).
- **Holding**: consolida os cérebros acumulados na rodada ao total do jogador.
- **Lost**: trata o caso de 3 tiros na mesma rodada.
- **TurnResult**: decide se o turno está completo, se houve empate ou vitória.
- **Draw**: filtra os jogadores empatados para uma rodada de desempate.
- **Win**: anuncia o vencedor e encerra a partida.
- **Quitting**: encerra o loop principal.

### `DiceBag`

Controla dois vetores: `dados_saco` (dados disponíveis pra puxar) e `dados_mesa` (dados atualmente rolados na mesa). A cada `puxar()`:
1. Devolve os dados de `Pegada` da mesa pro saco.
2. Se necessário, devolve os dados de `Cérebro` da mesa pro saco (pra manter dados suficientes disponíveis).
3. Embaralha o saco e move 3 dados pra mesa, rolando cada um (sorteando uma nova face).

### `Zdice`

Representa um dado individual: guarda a cor (que define a distribuição de faces) e a face que caiu na última rolagem.

### `Player`

Guarda nome, total de cérebros comidos (consolidados) e quantidade de turnos jogados — usado para saber quando um turno "completo" (todos os jogadores jogaram a mesma quantidade de vezes) aconteceu.

## Limitações conhecidas

- Sem validação robusta de input (`read_line` sem tratamento de erro em vários pontos).
- Mensagens e fluxo de UI estão hardcoded no terminal, sem separação entre lógica de jogo e apresentação.
- Não há testes automatizados.
- O código tem comentários em português misturados com anotações de "TODO" pessoais deixadas durante o desenvolvimento.

## Motivação

Esse projeto nasceu de eu já ter implementado o mesmo jogo em C++ antes e querer comparar a experiência de escrever a mesma lógica em Rust — principalmente pra sentir na prática as diferenças de ownership, pattern matching com enums, e como o compilador força a lidar com casos que em C++ eu talvez deixasse passar batido.
