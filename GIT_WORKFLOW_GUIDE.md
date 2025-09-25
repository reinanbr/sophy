# 🚀 Guia Completo: Git Workflow Ideal para Projetos Rust

Este guia apresenta as melhores práticas para commits, pushes e tags em projetos Rust, baseado no projeto Sophy e padrões da comunidade.

## 📋 Índice

1. [Estrutura de Commits Convencionais](#commits-convencionais)
2. [Workflow de Desenvolvimento](#workflow-desenvolvimento)  
3. [Versionamento Semântico](#versionamento-semântico)
4. [Tags e Releases](#tags-releases)
5. [Automação com GitHub Actions](#github-actions)
6. [Checklist Pré-Release](#checklist-pre-release)
7. [Comandos Práticos](#comandos-praticos)

## 🎯 Commits Convencionais

### Formato Padrão
```
<tipo>[escopo opcional]: <descrição>

[corpo opcional]

[rodapé(s) opcional(is)]
```

### Tipos de Commit
- **feat**: Nova funcionalidade
- **fix**: Correção de bug
- **docs**: Mudanças na documentação
- **style**: Formatação, espaços em branco, etc.
- **refactor**: Refatoração de código
- **test**: Adicionar ou corrigir testes
- **chore**: Tarefas de manutenção (deps, configs)
- **perf**: Melhoria de performance
- **ci**: Mudanças em CI/CD
- **build**: Mudanças no sistema de build

### Exemplos de Commits Ideais

```bash
# Nova funcionalidade
git commit -m "feat(methods): add bisection root finding method

- Implement bisection algorithm for root finding
- Add comprehensive tests and documentation
- Include usage examples in docstring

Closes #15"

# Correção de bug
git commit -m "fix(raphson): handle edge case when derivative is zero

- Add check for zero derivative before iteration
- Return appropriate error instead of panicking
- Update tests to cover edge cases

Fixes #23"

# Documentação
git commit -m "docs: improve API documentation with examples

- Add comprehensive examples for all public functions
- Include mathematical background in module docs
- Fix broken links and typos

Co-authored-by: Contributor <email@example.com>"

# Chore (manutenção)
git commit -m "chore: update dependencies to latest versions

- Update serde to 1.0.195
- Update tokio to 1.35.0
- Remove unused dev dependencies"
```

## 🔄 Workflow de Desenvolvimento

### 1. Estrutura de Branches

```
main (produção)
├── develop (desenvolvimento)
├── feature/nova-funcionalidade
├── fix/correcao-bug
└── release/v0.2.0
```

### 2. Fluxo Ideal

```bash
# 1. Criar branch para nova feature
git checkout -b feature/add-integration-methods

# 2. Desenvolver com commits pequenos e frequentes
git add -A
git commit -m "feat(methods): add simpson integration method skeleton"

git add src/methods/integration.rs
git commit -m "feat(methods): implement simpson's rule algorithm"

git add tests/integration_tests.rs
git commit -m "test(methods): add comprehensive tests for simpson integration"

git add src/methods/mod.rs src/lib.rs
git commit -m "docs(methods): add integration module documentation"

# 3. Push da branch
git push -u origin feature/add-integration-methods

# 4. Criar Pull Request no GitHub

# 5. Após aprovação, merge para main
git checkout main
git pull origin main
git merge --no-ff feature/add-integration-methods
git push origin main

# 6. Limpar branch local
git branch -d feature/add-integration-methods
```

## 📦 Versionamento Semântico (SemVer)

### Formato: `MAJOR.MINOR.PATCH`

- **MAJOR** (1.0.0): Breaking changes (incompatível)
- **MINOR** (0.1.0): Nova funcionalidade (compatível)
- **PATCH** (0.0.1): Correção de bugs (compatível)

### Exemplos para Sophy

```bash
# Versão atual: 0.1.23

# Adicionar nova função (compatível)
# 0.1.23 → 0.2.0
git tag -a v0.2.0 -m "feat: add integration methods module"

# Corrigir bug
# 0.1.23 → 0.1.24
git tag -a v0.1.24 -m "fix: handle edge cases in newton-raphson"

# Breaking change (mudar assinatura de função)
# 0.1.23 → 1.0.0
git tag -a v1.0.0 -m "feat!: redesign API for better ergonomics

BREAKING CHANGE: raphson function now returns Result<f64, Error>"
```

## 🏷️ Tags e Releases

### Criando Tags Anotadas

```bash
# Tag simples para patch
git tag -a v0.1.24 -m "fix: critical bug in newton-raphson convergence"

# Tag detalhada para release maior
git tag -a v0.2.0 -m "feat: add integration methods

New Features:
- Simpson's rule integration
- Trapezoidal rule integration
- Adaptive quadrature methods

Improvements:
- Better error handling in raphson method
- Performance optimizations
- Enhanced documentation

Breaking Changes: None"

# Push da tag
git push origin v0.2.0

# Push de todas as tags
git push --tags
```

### Criando Release no GitHub

```bash
# Após criar a tag, criar release no GitHub
gh release create v0.2.0 \\
  --title "Sophy v0.2.0 - Integration Methods" \\
  --notes-from-tag \\
  --generate-notes
```

## ⚙️ Automação com GitHub Actions

### Criar `.github/workflows/release.yml`

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    
    - name: Run tests
      run: cargo test --all-features
    
    - name: Check formatting
      run: cargo fmt -- --check
    
    - name: Run clippy
      run: cargo clippy -- -D warnings
    
    - name: Build documentation  
      run: cargo doc --no-deps
    
    - name: Publish to crates.io
      if: startsWith(github.ref, 'refs/tags/v')
      run: cargo publish
      env:
        CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
    
    - name: Create GitHub Release
      uses: softprops/action-gh-release@v1
      if: startsWith(github.ref, 'refs/tags/v')
      with:
        generate_release_notes: true
```

### Criar `.github/workflows/ci.yml`

```yaml
name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        rust: [stable, beta, nightly]
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Setup Rust ${{ matrix.rust }}
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        override: true
        components: rustfmt, clippy
    
    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Check formatting
      run: cargo fmt -- --check
    
    - name: Run clippy
      run: cargo clippy -- -D warnings
    
    - name: Run tests
      run: cargo test --all-features --verbose
    
    - name: Run doc tests
      run: cargo test --doc
    
    - name: Build documentation
      run: cargo doc --no-deps --all-features
```

## ✅ Checklist Pré-Release

### Antes de Cada Commit

- [ ] Código formatado: `cargo fmt`
- [ ] Sem warnings: `cargo clippy`
- [ ] Testes passando: `cargo test`
- [ ] Documentação atualizada
- [ ] Commit segue convenção

### Antes de Cada Release

```bash
# 1. Verificar tudo está funcionando
cargo check --all-features
cargo test --all-features
cargo test --doc
cargo clippy -- -D warnings
cargo fmt -- --check

# 2. Atualizar versão no Cargo.toml
# Editar sophy-lib/Cargo.toml: version = "0.2.0"

# 3. Atualizar CHANGELOG.md (se existir)

# 4. Commit das mudanças de versão
git add -A
git commit -m "chore: bump version to 0.2.0"

# 5. Criar e push da tag
git tag -a v0.2.0 -m "feat: add integration methods

New Features:
- Simpson's rule integration
- Trapezoidal rule integration
- Comprehensive test suite

Improvements:
- Better error handling
- Enhanced documentation"

git push origin main
git push origin v0.2.0

# 6. Verificar se CI passou
# 7. Verificar se docs.rs compilou
# 8. Criar release no GitHub (manual ou automático)
```

## 🛠️ Comandos Práticos

### Gerenciamento de Branches

```bash
# Listar branches
git branch -a

# Criar e trocar para nova branch
git checkout -b feature/nova-funcionalidade

# Trocar para branch existente
git checkout main

# Deletar branch local
git branch -d feature/funcionalidade-completa

# Deletar branch remota
git push origin --delete feature/funcionalidade-completa
```

### Comandos de Commit

```bash
# Commit interativo (escolher arquivos)
git add -p

# Commit com template de mensagem
git commit -t .gitmessage

# Corrigir último commit
git commit --amend

# Commit vazio (para trigger CI)
git commit --allow-empty -m "ci: trigger workflow"
```

### Gestão de Tags

```bash
# Listar tags
git tag -l

# Criar tag anotada
git tag -a v1.0.0 -m "Release v1.0.0"

# Deletar tag local
git tag -d v1.0.0

# Deletar tag remota
git push origin --delete v1.0.0

# Push específico de tag
git push origin v1.0.0

# Push de todas as tags
git push --tags
```

### Sincronização com Remote

```bash
# Atualizar refs remotas
git fetch origin

# Pull com rebase (histórico linear)
git pull --rebase origin main

# Push forçado (cuidado!)
git push --force-with-lease origin feature-branch
```

## 🎯 Template de Mensagem de Commit

Criar `.gitmessage` na raiz do projeto:

```
# <tipo>[escopo]: <descrição curta>
#
# <corpo detalhado>
#
# <rodapé>
#
# Tipos:
# feat     : Nova funcionalidade
# fix      : Correção de bug  
# docs     : Documentação
# style    : Formatação
# refactor : Refatoração
# test     : Testes
# chore    : Manutenção
# perf     : Performance
# ci       : CI/CD
# build    : Build
#
# Exemplo:
# feat(methods): add bisection method for root finding
#
# - Implement bisection algorithm
# - Add comprehensive tests
# - Include usage examples
#
# Closes #15
```

Configurar Git para usar o template:

```bash
git config commit.template .gitmessage
```

## 📊 Exemplo Prático: Release v0.2.0

### Workflow Completo

```bash
# 1. Criar branch de desenvolvimento
git checkout -b develop

# 2. Desenvolver features
git checkout -b feature/integration-methods
# ... desenvolvimento ...
git commit -m "feat(methods): add simpson integration method"
git commit -m "test(methods): add integration tests"
git commit -m "docs(methods): document integration methods"
git push origin feature/integration-methods

# 3. Merge para develop via PR
git checkout develop
git merge --no-ff feature/integration-methods

# 4. Criar branch de release
git checkout -b release/v0.2.0

# 5. Finalizar release
# Atualizar version no Cargo.toml
# Atualizar documentação
git commit -m "chore: prepare release v0.2.0"

# 6. Merge para main
git checkout main
git merge --no-ff release/v0.2.0

# 7. Criar tag e push
git tag -a v0.2.0 -m "feat: add integration methods

New Features:
- Simpson's rule numerical integration
- Trapezoidal rule integration  
- Adaptive quadrature methods

Improvements:
- Enhanced error handling in newton-raphson
- Better performance for large iterations
- Comprehensive documentation with examples

Tests:
- Added 50+ integration test cases
- Improved test coverage to 95%

Documentation:
- Complete API documentation
- Mathematical background explanations
- Usage examples for all public functions"

git push origin main
git push origin v0.2.0

# 8. Limpeza
git branch -d feature/integration-methods
git branch -d release/v0.2.0
```

## 🎉 Resultado Final

Seguindo esse workflow, você terá:

- ✅ **Histórico limpo e semântico**
- ✅ **Releases automáticas e confiáveis**  
- ✅ **Documentação sempre atualizada**
- ✅ **CI/CD robusto**
- ✅ **Versionamento consistente**
- ✅ **Colaboração facilitada**

## 📚 Recursos Adicionais

- [Conventional Commits](https://conventionalcommits.org/)
- [Semantic Versioning](https://semver.org/)
- [Git Flow](https://nvie.com/posts/a-successful-git-branching-model/)
- [GitHub Flow](https://guides.github.com/introduction/flow/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

---

**💡 Dica Final**: A consistência é mais importante que a perfeição. Escolha um padrão e mantenha-o em todo o projeto!