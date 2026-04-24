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
- **Observabilidade:** [Tracing](https://tracing.rs) (Instrumentação) e [OpenTelemetry](https://opentelemetry.io) (Exportação de Telemetria)

## 📋 Status dos Endpoints (API)

| Método | Endpoint                                | Descrição                 | Status       |
| :----- | :-------------------------------------- | :------------------------ | :----------- |
| `GET`  | `/health`                               | Check de saúde do sistema | ✅ Concluído |
| `GET`  | `/auth/confirm-email/{user_id}/{token}` | Registrar um novo usuário | ✅ Concluído |
| `POST` | `/auth/register`                        | Registrar um novo usuário | ✅ Concluído |

## 🏁 Configuração Local

1. **Clonar o repositório:**
   ```bash
   git clone https://github.com/galetedanilo/iam-service.git
   cd iam-service
   cargo run
   ```
