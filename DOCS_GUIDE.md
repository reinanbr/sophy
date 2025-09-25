# 📚 Guia: Como Publicar sua Biblioteca Rust no docs.rs

Este guia mostra como criar uma documentação excelente e publică-la no docs.rs para sua biblioteca Rust.

## 🎯 O que é o docs.rs?

O [docs.rs](https://docs.rs) é o serviço oficial de documentação do Rust que automaticamente gera e hospeda documentação para todas as crates publicadas no [crates.io](https://crates.io).

## 📋 Pré-requisitos

1. **Conta no crates.io**: Você precisa ter uma conta no crates.io
2. **Biblioteca Rust**: Um projeto Rust configurado como biblioteca (`[lib]` no Cargo.toml)
3. **Documentação**: Código bem documentado com comentários `///` e `//!`

## 🚀 Passo a Passo

### 1. Configure o Cargo.toml Adequadamente

Adicione metadados completos ao seu `Cargo.toml`:

```toml
[package]
name = "sua-biblioteca"
version = "0.1.0"
edition = "2021"
authors = ["Seu Nome <seu.email@example.com>"]
description = "Uma descrição clara e concisa da sua biblioteca"
license = "MIT OR Apache-2.0"
homepage = "https://github.com/usuario/repositorio"
repository = "https://github.com/usuario/repositorio"
documentation = "https://docs.rs/sua-biblioteca"
readme = "README.md"
keywords = ["math", "numerical", "algoritmos"] # Máximo 5 keywords
categories = ["mathematics", "science", "algorithms"] # Use categorias válidas
exclude = [
    "/.github/*",
    "/examples/*", 
    "/target/*",
    "*.png"
]

[lib]
name = "sua_biblioteca"
path = "src/lib.rs"

# Configuração específica para docs.rs
[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

### 2. Escreva Documentação Excelente

#### 2.1 Documentação do Módulo Principal (`lib.rs`)

```rust
//! # Nome da Sua Biblioteca 🎯
//!
//! [![Crates.io](https://img.shields.io/crates/v/sua-biblioteca.svg)](https://crates.io/crates/sua-biblioteca)
//! [![Documentation](https://docs.rs/sua-biblioteca/badge.svg)](https://docs.rs/sua-biblioteca)
//! [![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/usuario/repo)
//!
//! Uma descrição detalhada da sua biblioteca, explicando:
//! - O que ela faz
//! - Para que serve
//! - Seus principais benefícios
//!
//! ## ✨ Features
//!
//! - 🎯 **Feature 1**: Descrição da feature
//! - 🚀 **Feature 2**: Descrição da feature
//! - 🛡️ **Feature 3**: Descrição da feature
//!
//! ## 🚀 Quick Start
//!
//! Adicione ao seu `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! sua-biblioteca = "0.1.0"
//! ```
//!
//! ## 📖 Examples
//!
//! ### Exemplo Básico
//!
//! ```rust
//! use sua_biblioteca::funcao_exemplo;
//!
//! let resultado = funcao_exemplo(42);
//! println!("Resultado: {}", resultado);
//! ```
//!
//! ## 🏗️ Architecture
//!
//! A biblioteca está organizada nos seguintes módulos:
//! - [`modulo1`]: Descrição do módulo 1
//! - [`modulo2`]: Descrição do módulo 2

pub mod modulo1;
pub mod modulo2;
```

#### 2.2 Documentação de Funções

```rust
/// Breve descrição da função (uma linha).
///
/// Descrição mais detalhada da função, explicando:
/// - O que ela faz
/// - Quando usar
/// - Como funciona (se necessário)
///
/// ## Arguments
///
/// * `param1` - Descrição do primeiro parâmetro
/// * `param2` - Descrição do segundo parâmetro
///
/// ## Returns
///
/// Descrição do que a função retorna.
///
/// ## Errors
///
/// Quando a função pode falhar (se aplicável).
///
/// ## Examples
///
/// ### Exemplo Básico
/// ```rust
/// use sua_biblioteca::sua_funcao;
///
/// let resultado = sua_funcao(10, 20);
/// assert_eq!(resultado, 30);
/// ```
///
/// ### Exemplo Avançado
/// ```rust
/// use sua_biblioteca::sua_funcao;
///
/// // Exemplo mais complexo
/// let dados = vec![1, 2, 3, 4, 5];
/// let processados = dados.iter()
///     .map(|&x| sua_funcao(x, 10))
///     .collect::<Vec<_>>();
/// ```
pub fn sua_funcao(param1: i32, param2: i32) -> i32 {
    param1 + param2
}
```

#### 2.3 Documentação de Módulos

```rust
//! # Módulo Exemplo
//!
//! Este módulo fornece funcionalidades para [descrição].
//!
//! ## Overview
//!
//! Explicação geral do módulo e suas responsabilidades.
//!
//! ## Usage
//!
//! ```rust
//! use sua_biblioteca::modulo_exemplo;
//!
//! // Exemplo de uso
//! ```
//!
//! ## Available Functions
//!
//! - [`funcao1`]: Descrição breve
//! - [`funcao2`]: Descrição breve
```

### 3. Crie Arquivos de Licença

Crie os arquivos `LICENSE-MIT` e `LICENSE-APACHE` na raiz do projeto:

```bash
# No terminal
curl -sSf https://raw.githubusercontent.com/github/choosealicense.com/gh-pages/_licenses/mit.txt > LICENSE-MIT
curl -sSf https://raw.githubusercontent.com/github/choosealicense.com/gh-pages/_licenses/apache-2.0.txt > LICENSE-APACHE
```

### 4. Teste a Documentação Localmente

```bash
# Gere e abra a documentação
cargo doc --open

# Teste os exemplos da documentação
cargo test --doc

# Verifique warnings
cargo doc 2>&1 | grep warning
```

### 5. Publique no crates.io

```bash
# 1. Faça login no crates.io (apenas uma vez)
cargo login SEU_TOKEN_AQUI

# 2. Verifique se tudo está ok
cargo check
cargo test
cargo doc

# 3. Publique (dry-run primeiro)
cargo publish --dry-run

# 4. Publique para valer
cargo publish
```

## 🎨 Dicas para Documentação Excelente

### ✅ Faça

- **Use emojis** com moderação para tornar a documentação mais atrativa
- **Inclua exemplos práticos** para cada função pública
- **Explique o contexto** - quando e por que usar
- **Use seções organizadas** (`## Arguments`, `## Returns`, etc.)
- **Teste todos os exemplos** com `cargo test --doc`
- **Inclua badges** no topo da documentação
- **Mantenha consistência** no estilo de documentação

### ❌ Evite

- **Documentação vaga** - seja específico sobre o que faz
- **Exemplos que não compilam** - sempre teste
- **Links quebrados** - verifique todos os links
- **Documentação desatualizada** - mantenha sincronizada com o código
- **Excesso de texto** - seja conciso mas completo

## 🔧 Configurações Avançadas

### Features Condicionais

```toml
# Cargo.toml
[features]
default = ["std"]
std = []
serde-support = ["serde"]

[dependencies]
serde = { version = "1.0", optional = true }

# No código
/// Esta função está disponível apenas com a feature `serde-support`
#[cfg(feature = "serde-support")]
pub fn funcao_serde() { }
```

### Documentação Específica para docs.rs

```rust
// Use cfg(docsrs) para funcionalidades específicas do docs.rs
#[cfg_attr(docsrs, doc = "Funcionalidade extra visível apenas no docs.rs")]
pub struct MinhaStruct;
```

## 🚀 Checklist Final

Antes de publicar, verifique:

- [ ] Metadados completos no `Cargo.toml`
- [ ] Documentação em todos os itens públicos
- [ ] Exemplos funcionando (`cargo test --doc`)
- [ ] Sem warnings na documentação (`cargo doc`)
- [ ] README.md atualizado
- [ ] Arquivos de licença presentes
- [ ] Versão bumped apropriadamente
- [ ] Testes passando (`cargo test`)
- [ ] Código formatado (`cargo fmt`)
- [ ] Lints resolvidos (`cargo clippy`)

## 📊 Monitoramento

Após publicar:

1. **Verifique no docs.rs**: `https://docs.rs/sua-biblioteca`
2. **Monitore builds**: Verifique se a documentação compilou corretamente
3. **Atualize badges**: URLs dos badges devem apontar para sua crate
4. **Compartilhe**: Divulgue sua biblioteca na comunidade Rust

## 🎯 Exemplo Real: Sophy

Este guia foi baseado na biblioteca Sophy. Veja o código fonte completo para referência:
- **Repositório**: [github.com/reinanbr/sophy](https://github.com/reinanbr/sophy)
- **Documentação**: [docs.rs/sophy](https://docs.rs/sophy) (após publicação)

## 📚 Recursos Adicionais

- [The Rust Book - Documentation](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html)
- [Rustdoc Guide](https://doc.rust-lang.org/rustdoc/index.html)
- [docs.rs FAQ](https://docs.rs/about)
- [Cargo.toml Reference](https://doc.rust-lang.org/cargo/reference/manifest.html)

---

**💡 Dica Final**: Documentação excelente é um investimento. Ela não apenas ajuda outros desenvolvedores a usar sua biblioteca, mas também demonstra profissionalismo e cuidado com o projeto!