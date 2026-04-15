# 👤 IAM Service

Um microsserviço de alta performance desenvolvido em **Rust**, focado no gerenciamento de identidade e acesso de usuários (IAM). O projeto utiliza os princípios de **Domain-Driven Design (DDD)** para garantir um código escalável, testável e de fácil manutenção.

## 🏗️ Arquitetura (DDD)

O projeto está sendo estruturado seguindo as camadas do DDD:

- **Domain:** Entidades, Objetos de Valor e Regras de Negócio (Lógica pura).
- **Application:** Casos de uso (Use Cases) que orquestram a lógica da aplicação.
- **Infrastructure:** Implementações técnicas (ScyllaDB, adaptadores de rede).
- **Presentation/API:** Camada de entrada (Axum Handlers). (Em desenvolvimento)

## 📂 Estrutura de Pastas

```text
src/
├── 📁 domain/         # Regras de negócio e lógica pura
├── 📁 application/    # Orquestração e Casos de Uso
├── 📦 infrastructure/ # Persistência (ScyllaDB) e Clientes Externos
├── 🚀 presentation/   # Camada de API (Axum) e Handlers
└── 🦀 main.rs         # Ponto de entrada do microserviço
```

## 🚀 Tecnologias Principais

- **Linguagem:** [Rust](https://www.rust-lang.org)
- **Framework Web:** [Axum](https://github.com/tokio-rs/axum)
- **Runtime:** [Tokio](https://tokio.rs)
- **Banco de Dados:** [ScyllaDB](https://www.scylladb.com)
- **Serialização:** [Serde](https://serde.rs)
- **Validação:** [Validator](https://github.com/Keats/validator)
- **Segurança:** [Jsonwebtoken](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/)

## 🛠 Fitness Functions

Este projeto utiliza Fitness Functions automatizadas via GitHub Actions:

- Mínimo de 80% de cobertura de testes.
- Complexidade controlada via Clippy.

Se o build falhar, verifique se você adicionou testes para as novas funcionalidades ou se a lógica da sua função pode ser simplificada (refatorada).

## 📋 Status dos Endpoints (API)

| Método | Endpoint         | Descrição                 | Status                               |
| :----- | :--------------- | :------------------------ | :----------------------------------- |
| `GET`  | `/health`        | Check de saúde do sistema | 🚧 Em progresso (Application/Domain) |
| `POST` | `/auth/register` | Registrar um novo usuário | ✅ Concluído                         |

## 🏁 Configuração Local

1. **Clonar o repositório:**
   ```bash
   git clone https://github.com/galetedanilo/iam-service.git
   cd iam-service
   cargo run
   ```
