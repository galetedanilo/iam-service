use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use uuid::Uuid;

// 1. Definimos o Extractor que conterá o Correlation ID
#[derive(Debug, Clone)]
pub struct CorrelationId(pub Uuid);

// 2. Implementamos FromRequestParts para o Axum conseguir ler os Headers automaticamente
impl<S> FromRequestParts<S> for CorrelationId
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Busca pelo header padrão. Se não achar, busca com letras minúsculas
        let header_value = parts
            .headers
            .get("x-correlation-id")
            .or_else(|| parts.headers.get("X-Correlation-ID"));

        match header_value {
            Some(value) => {
                let value_str = value
                    .to_str()
                    .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid header".to_string()))?;

                let uuid = Uuid::parse_str(value_str).map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        "Correlation ID need to be a valid UUID".to_string(),
                    )
                })?;

                Ok(CorrelationId(uuid))
            }
            // Se o cliente não enviou, geramos um novo ID raiz para iniciar o rastreamento
            None => Ok(CorrelationId(Uuid::now_v7())),
        }
    }
}
