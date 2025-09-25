# 🚀 Comandos Rápidos - Git Workflow Sophy

## ⚡ Workflow Diário

### Iniciar Nova Feature
```bash
git checkout -b feature/nome-da-feature
# desenvolver...
git add -A
git commit  # usa template automático
git push -u origin feature/nome-da-feature
```

### Commit Ideal
```bash
# Verificar o que mudou
git status
git diff

# Adicionar mudanças
git add arquivo1.rs arquivo2.rs  # específicos
# OU
git add -A  # todos os arquivos

# Commit com template (abre editor)
git commit

# Commit direto (para mudanças simples)
git commit -m "feat(methods): add new integration method"
```

### Finalizar Feature
```bash
# Voltar para main e atualizar
git checkout main
git pull origin main

# Merge da feature
git merge --no-ff feature/nome-da-feature
git push origin main

# Limpeza
git branch -d feature/nome-da-feature
git push origin --delete feature/nome-da-feature
```

## 🏷️ Release Process

### Preparar Release
```bash
# 1. Atualizar versão no Cargo.toml
# sophy-lib/Cargo.toml: version = "0.2.0"

# 2. Commit da versão
git add sophy-lib/Cargo.toml
git commit -m "chore: bump version to 0.2.0"

# 3. Criar tag anotada
git tag -a v0.2.0 -m "feat: add integration methods

New Features:
- Simpson's rule integration
- Trapezoidal integration
- Adaptive quadrature

Improvements:
- Better error handling
- Enhanced documentation"

# 4. Push tudo
git push origin main
git push origin v0.2.0
```

### Tipos de Release
```bash
# Patch (0.1.23 → 0.1.24) - bugs, docs
git tag -a v0.1.24 -m "fix: critical bug in newton-raphson"

# Minor (0.1.24 → 0.2.0) - nova feature
git tag -a v0.2.0 -m "feat: add integration methods"

# Major (0.2.0 → 1.0.0) - breaking changes
git tag -a v1.0.0 -m "feat!: redesign API

BREAKING CHANGE: raphson now returns Result<f64, Error>"
```

## 🔧 Comandos Úteis

### Informações
```bash
git status                    # Status atual
git log --oneline -10        # Últimos 10 commits
git tag -l                   # Listar tags
git remote -v                # Ver repositórios remotos
git branch -a                # Listar todas as branches
```

### Correções
```bash
git commit --amend           # Corrigir último commit
git reset --soft HEAD~1      # Desfazer último commit (manter mudanças)
git reset --hard HEAD~1      # Desfazer último commit (perder mudanças)
git revert HEAD              # Reverter último commit (seguro)
```

### Sincronização
```bash
git fetch origin             # Buscar mudanças remotas
git pull --rebase origin main # Pull com rebase (histórico linear)
git push --force-with-lease origin feature-branch # Push forçado seguro
```

### Tags
```bash
git tag -d v1.0.0                    # Deletar tag local
git push origin --delete v1.0.0     # Deletar tag remota
git show v1.0.0                     # Ver detalhes da tag
git push --tags                     # Push de todas as tags
```

## 📋 Checklist Pré-Commit

- [ ] `cargo fmt` - Código formatado
- [ ] `cargo clippy` - Sem warnings
- [ ] `cargo test` - Testes passando
- [ ] `cargo test --doc` - Doc tests OK
- [ ] Mensagem de commit clara e descritiva
- [ ] Mudanças relacionadas agrupadas

## 📋 Checklist Pré-Release

- [ ] Versão atualizada no `Cargo.toml`
- [ ] `CHANGELOG.md` atualizado (se existir)
- [ ] Todos os testes passando
- [ ] Documentação atualizada
- [ ] Exemplos funcionando
- [ ] Tag criada com descrição detalhada
- [ ] CI/CD passando no GitHub

## 🎯 Exemplos de Commits

### ✅ Bons Commits
```bash
feat(methods): add bisection root finding method
fix(raphson): handle edge case when derivative is zero  
docs: update README with integration examples
test(methods): add comprehensive integration tests
chore: update dependencies to latest versions
refactor(base): simplify constant definitions
```

### ❌ Commits Ruins
```bash
update stuff
fix bug
changes
WIP
asdf
updated files
```

## 🔄 Git Flow Resumido

```
main (estável)
├── develop (desenvolvimento)
├── feature/nova-feature (temporária)
├── fix/bug-critico (temporária)  
└── release/v1.0.0 (temporária)
```

### Fluxo:
1. `feature/*` → `develop` (via PR)
2. `develop` → `release/v*` (preparar release)
3. `release/v*` → `main` (finalizar release)
4. Tag criada em `main`

## 🚨 Comandos de Emergência

### Desfazer Push (CUIDADO!)
```bash
git reset --hard HEAD~1
git push --force-with-lease origin main
```

### Recuperar Commit Perdido
```bash
git reflog                # Ver histórico completo
git cherry-pick abc123    # Recuperar commit específico
```

### Resolver Conflitos
```bash
git status                # Ver arquivos em conflito
# editar arquivos manualmente
git add arquivo-resolvido.rs
git commit                # Finalizar merge
```

---

## 💡 Dicas Finais

1. **Commits pequenos e frequentes** são melhores que grandes
2. **Mensagens claras** ajudam toda a equipe
3. **Teste sempre** antes de fazer commit
4. **Use branches** para features e experimentos
5. **Tags anotadas** são melhores que tags simples
6. **Pre-commit hooks** previnem problemas
7. **Consistência** é mais importante que perfeição

**🎯 Lembre-se**: Um bom workflow Git economiza tempo e evita problemas!