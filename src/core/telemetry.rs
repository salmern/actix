use actix_web::Error;
use sqlx::PgPool;
use tracing::{info, error, span, Level};
use tracing_actix_web::TracingLogger;

// Initialize tracing subscriber
pub fn init_tracing() {
    tracing_subscriber::fmt::init();
}

// Define a macro for creating spans
macro_rules! telemetry_span {
    ($level:expr, $name:expr) => {
        span!($level, $name)
    };
}

// Macro for entering spans
macro_rules! telemetry_enter {
    ($span:expr) => {
        let _enter = $span.enter();
    };
}

// Function for logging info messages
pub fn log_info(message: &str) {
    info!("{}", message);
}

// Function for logging error messages
pub fn log_error(message: &str) {
    error!("{}", message);
}
