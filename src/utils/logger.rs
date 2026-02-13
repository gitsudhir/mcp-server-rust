use tracing::{debug, error, info, warn};

#[derive(Clone)]
pub struct Logger {
    name: String,
}

impl Logger {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
        }
    }

    pub fn info(&self, msg: &str) {
        info!(logger = %self.name, "{}", msg);
    }

    pub fn debug(&self, msg: &str) {
        debug!(logger = %self.name, "{}", msg);
    }

    pub fn warn(&self, msg: &str) {
        warn!(logger = %self.name, "{}", msg);
    }

    pub fn error(&self, msg: &str) {
        error!(logger = %self.name, "{}", msg);
    }

    pub fn info_with_context(&self, msg: &str, context: &str) {
        info!(logger = %self.name, context = %context, "{}", msg);
    }

    pub fn debug_with_context(&self, msg: &str, context: &str) {
        debug!(logger = %self.name, context = %context, "{}", msg);
    }

    pub fn error_with_context(&self, msg: &str, context: &str) {
        error!(logger = %self.name, context = %context, "{}", msg);
    }
}

pub fn init_logger() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            tracing_subscriber::filter::EnvFilter::from_default_env()
                .add_directive("rust_mcp_server=debug".parse().unwrap()),
        )
        .init();
}