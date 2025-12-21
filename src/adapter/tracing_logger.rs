use crate::{
    domain::{PokemonEvent, PokemonFailureReason},
    ports::Logger,
};

/// TracingLogger adapter that implements the Logger port using the tracing crate
/// Maps domain events to structured tracing events
pub struct TracingLogger;

impl TracingLogger {
    pub fn new() -> Self {
        Self
    }
}

impl Logger for TracingLogger {
    fn log(&self, event: &PokemonEvent) {
        match event {
            PokemonEvent::FetchRequested { name } => {
                tracing::info!(
                    pokemon.fetch.requested = true,
                    pokemon.name = %name,
                    "Pokemon fetch requested"
                );
            }
            PokemonEvent::FetchSucceeded {
                name,
                habitat,
                is_legendary,
            } => {
                tracing::info!(
                    pokemon.fetch.succeeded = true,
                    pokemon.name = %name,
                    pokemon.habitat = %habitat,
                    pokemon.is_legendary = %is_legendary,
                    "Pokemon fetch succeeded"
                );
            }
            PokemonEvent::FetchFailed { name, reason } => {
                let reason_str = match reason {
                    PokemonFailureReason::NotFound => "not_found",
                    PokemonFailureReason::ServiceUnavailable => "service_unavailable",
                    PokemonFailureReason::DataCorrupted => "data_corrupted",
                };
                tracing::error!(
                    pokemon.fetch.failed = true,
                    pokemon.name = %name,
                    pokemon.failure.reason = reason_str,
                    "Pokemon fetch failed"
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{PokemonEvent, PokemonFailureReason};

    #[test]
    fn test_tracing_logger_logs_domain_events() {
        let logger = TracingLogger::new();

        // Test fetch requested event
        logger.log(&PokemonEvent::FetchRequested {
            name: "pikachu".to_string(),
        });

        // Test fetch succeeded event
        logger.log(&PokemonEvent::FetchSucceeded {
            name: "pikachu".to_string(),
            habitat: "forest".to_string(),
            is_legendary: false,
        });

        // Test fetch failed event
        logger.log(&PokemonEvent::FetchFailed {
            name: "unknown".to_string(),
            reason: PokemonFailureReason::NotFound,
        });
    }
}
