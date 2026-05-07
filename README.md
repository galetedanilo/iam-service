# 👤 IAM Service

Um microserviço de alta performance focado em gerenciamento de identidade e acesso (IAM), desenvolvido em **Rust** utilizando a **Edition 2024**. O projeto segue os princípios de **Domain-Driven Design (DDD)** e arquitetura baseada em eventos para garantir escalabilidade, segurança e alta disponibilidade.

## 🏗️ Arquitetura e Padrões

- **DDD (Domain-Driven Design):** Divisão clara entre Domínio, Aplicação, Infraestrutura e Apresentação.
- **Event-Driven:** Comunicação assíncrona robusta utilizando **RabbitMQ**.
- **Outbox Pattern:** Implementado para garantir que eventos de domínio sejam persistidos e entregues de forma confiável (_At-least-once delivery_).
- **Resiliência:** Estratégias de retentativas com **Exponential Backoff** e **Jitter** para operações de infraestrutura.

## 📂 Estrutura do Projeto

```text
src/
├── 📁 domain/         # Entidades, Objetos de Valor e Regras de Negócio
├── 📁 application/    # Casos de Uso e orquestração da lógica
├── 📦 infrastructure/ # Adaptadores: ScyllaDB, RabbitMQ (Lapin), Outbox
├── 🚀 presentation/   # Camada API (Axum) e Handlers de entrada
└── 🦀 main.rs         # Inicialização, DI e configuração do runtime
```

## 🚀 Stack Tecnológica

### Core & Web

- **Linguagem:** [Rust (Edition 2024)](https://rust-lang.org)
- **Runtime:** [Tokio](https://tokio.rs) (Full features)
- **Web Framework:** [Axum v0.8](https://github.com)
- **Mensageria:** [Lapin](https://github.com) (AMQP/RabbitMQ)

### Persistência e Segurança

- **Banco de Dados:** [ScyllaDB](https://scylladb.com) (Drivers `scylla` e `scylla-migrate`)
- **Criptografia:** [Argon2](https://wikipedia.org) para hashing seguro de senhas.
- **Tokens:** [JWT](https://jsonwebtoken.io) utilizando backend `aws_lc_rs` para alta performance.

### Observabilidade e Resiliência

- **Tracing:** `tracing` com exportação via `opentelemetry` (OTLP/gRPC).
- **Retentativas:** `tokio-retry` para tolerância a falhas em sistemas externos.
- **Serialização:** `serde` e `serde_json`.

## 📊 Endpoints da API

| Método | Endpoint                    | Descrição                           | Status       |
| :----- | :-------------------------- | :---------------------------------- | :----------- |
| `GET`  | `/health`                   | Health check do sistema             | ✅ Concluído |
| `POST` | `/auth/register`            | Registro de usuário + Evento Outbox | ✅ Concluído |
| `POST` | `/auth/authentication`      | Login e geração de JWT              | ✅ Concluído |
| `GET`  | `/auth/confirm-email/{jwt}` | Validação de cadastro               | ✅ Concluído |
| `POST` | `/auth/forgot-password`     | Recuperação de conta                | ✅ Concluído |
| `POST` | `/auth/reset-password`      | Redefinição de senha                | ✅ Concluído |

## 🛠️ Configuração Local

1.  **Clone o repositório:**

    ```bash
    git clone https://github.com
    cd iam-service
    ```

2.  **Variáveis de Ambiente:**
    Crie um arquivo `.env` seguindo o exemplo abaixo:

    ```env
    DATABASE_URL=127.0.0.1:9042
    RABBITMQ_URL=amqp://guest:guest@localhost:5672/dev
    JWT_SECRET=seu_secret_aqui
    ```

3.  **Execução:**
    ```bash
    cargo run
    ```
