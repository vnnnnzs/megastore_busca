# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## 📋 Descrição
Projeto desenvolvido para o desafio MegaStore, focado em resolver problemas de performance em catálogos de produtos utilizando a linguagem **Rust**.

## 🛠️ Tecnologias e Algoritmos
* **Tabelas Hash (HashMap):** Utilizadas para garantir buscas por nome com complexidade **O(1)**.
* **Grafos (Lista de Adjacência):** Implementados para gerar recomendações inteligentes de produtos relacionados.
* **Testes Unitários:** O sistema conta com testes integrados para validar a lógica de busca e as conexões do grafo.

## 🚀 Como testar
No terminal, utilize os comandos:
* `cargo run`: Para executar o sistema de busca.
* `cargo test`: Para rodar os testes automatizados e validar o sistema.

## 📄 Documentação Técnica
O projeto foca em escalabilidade, permitindo que a busca permaneça instantânea independentemente do tamanho do catálogo.
