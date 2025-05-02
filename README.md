# Migração WEbdEX para Solana

## Descrição do Projeto

Este projeto é a implementação da migração do sistema de negociação automatizada **WEbdEX** da blockchain Polygon para a blockchain Solana. Utilizando a linguagem Rust e o framework Anchor, reimplementamos a lógica dos contratos inteligentes originais (escritos em Solidity) para aproveitar a velocidade e os baixos custos de transação da Solana.

O objetivo principal foi adaptar os módulos centrais do WEbdEX, incluindo gerenciamento de bots, usuários, subcontas, pagamentos e execução de estratégias, para o ambiente de execução da Solana (SPL - Solana Program Library).

Este repositório contém o código-fonte do programa Solana (`smart contract`) e os scripts necessários para sua construção, implantação e teste na Devnet da Solana.

Agora, vamos deixar claro qual foi o objetivo dessa migração e quais tecnologias foram usadas.

## Objetivo da Migração

Migrar a funcionalidade existente do WEbdEX da EVM (Polygon) para o ambiente nativo da Solana, garantindo a paridade funcional onde aplicável e explorando otimizações possíveis na nova arquitetura.

## Tecnologias Utilizadas

* **Linguagem de Programação:** Rust
* **Framework de Desenvolvimento:** Anchor Framework
* **Blockchain:** Solana
* **Bibliotecas Solana:** Solana Program Library (SPL)
* **Ferramentas:** Solana CLI, Anchor CLI, Node.js, Yarn, Mocha (para testes)

## Configuração do Ambiente

Para configurar o ambiente de desenvolvimento e interagir com o projeto, siga os passos abaixo:

### Pré-requisitos

Certifique-se de ter as seguintes ferramentas instaladas:

1.  **Rust:** Instale o `rustup` seguindo as instruções em [rustup.rs](https://rustup.rs/).
2.  **Solana CLI:** Instale a ferramenta de linha de comando da Solana. Siga as instruções em [docs.solana.com/cli/install](https://docs.solana.com/cli/install).
3.  **Anchor CLI:** Instale o framework Anchor. Siga as instruções em [docs.anchor.projectserum.com/getting-started/installation](https://docs.anchor.projectserum.com/getting-started/installation).
4.  **Node.js e Yarn:** Instale Node.js (versão LTS recomendada) e o Yarn globalmente:
    ```bash
    npm install -g yarn
    ```

### Configuração da Carteira e Rede

Você precisará de uma carteira Solana e fundos (SOL) na Devnet para implantar e testar o programa.

1.  Crie uma nova carteira (se você já tiver uma, pode pular este passo e usar a sua):
    ```bash
    solana-keygen new
    ```
2.  Configure a CLI para usar a Devnet como cluster padrão:
    ```bash
    solana config set --url [https://api.devnet.solana.com](https://api.devnet.solana.com)
    ```
3.  Configure a CLI para usar o arquivo de chave da sua carteira (substitua o caminho se sua chave estiver em outro local):
    ```bash
    solana config set --keypair ~/.config/solana/id.json
    ```
4.  Peça um airdrop de SOL na Devnet para sua carteira (solicite uma quantidade suficiente, como 5 SOL):
    ```bash
    solana airdrop 5
    ```
    Você pode verificar seu saldo com `solana balance`.

### Clonando o Repositório e Instalando Dependências

Clone o código-fonte do GitHub e instale as dependências do projeto (principalmente para os testes de integração):

```bash
git clone (https://github.com/luzalex23/hackatom_webdex.git)
cd hackatom_webdex #
yarn install

Verifique o arquivo Anchor.toml na raiz do projeto para confirmar que as configurações de cluster e carteira estão corretas.


Agora, vou explicar como construir e implantar o programa na Devnet.

```markdown
## Construindo o Programa

Para compilar o código Rust do programa Solana, execute:

```bash
anchor build

Este comando irá gerar o binário do programa (.so) e o arquivo IDL (.json) na pasta target.

Implantação na Devnet
Para implantar o programa compilado na rede de testes Devnet da Solana, execute:

anchor deploy

O Anchor fará o upload do programa para a blockchain e exibirá o Program ID final. Este ID é o endereço do seu programa na Devnet. Anote-o.

Nota: A implantação de um programa na Solana custa SOL. Certifique-se de que a carteira configurada (~/.config/solana/id.json) tem saldo suficiente para cobrir este custo.

Verificação da Implantação:

Você pode verificar a implantação acessando um explorador Solana (como Solana Explorer ou Solscan). Selecione a rede Devnet (geralmente no canto superior direito). Cole o Program ID que você obteve na barra de busca e pressione Enter. A página deverá exibir "Executable: Yes", confirmando que o programa está ativo na blockchain.


```markdown
## Testando a Interação

```bash
anchor test --skip-deploy

Observando a Saída dos Testes:

Uma execução bem-sucedida dos testes no terminal mostrará:

Mensagens de log do script de teste.

As assinaturas das transações para cada instrução chamada. Estas são as provas de que as transações foram processadas pela Devnet.

Resultados de asserções e/ou dados de contas buscados e impressos pelo script, validando a lógica e o armazenamento de dados do programa.

Esta saída é a demonstração funcional da migração.

```markdown
## Estrutura do Projeto

O código-fonte do programa está organizado da seguinte forma:

* `programs/hackatom_webdex/src/lib.rs`: Ponto de entrada do programa, define instruções e contextos.
* `programs/hackatom_webdex/src/state.rs`: Definição das estruturas de dados (`#[account]`) que representam as contas onde o estado do programa é armazenado (como `Bot`, `User`, `SubAccount`).
* `programs/hackatom_webdex/src/modules/`: Esta pasta contém os diferentes módulos (como `factory.rs`, `manager.rs`, `sub_accounts.rs`, etc.) que encapsulam a lógica de negócio, reimplementando a funcionalidade dos contratos Solidity originais.
* `tests/hackatom_webdex.ts`: O script de teste de integração em TypeScript.
* `Anchor.toml`: Configuração do projeto Anchor.
* `Cargo.toml`: Configuração do pacote Rust e dependências.


## Contato

Para dúvidas ou mais informações sobre este projeto, entre em contato com:

* [Alexsandro Luz/Piratas do Silício - (https://www.linkedin.com/in/alexsandro-luz/)
