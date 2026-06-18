use std::path::PathBuf;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use tracing_appender::non_blocking::WorkerGuard;

static mut LOG_GUARD: Option<WorkerGuard> = None;

pub fn init_logging(log_dir: Option<PathBuf>) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    if let Some(dir) = log_dir {
        let file_appender = tracing_appender::rolling::daily(&dir, "deva.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        // SAFETY: This is only called from single-threaded init context during startup
        unsafe { LOG_GUARD = Some(guard); }

        let _ = tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().with_writer(non_blocking).with_ansi(false))
            .with(fmt::layer().with_writer(std::io::stderr))
            .try_init();
    } else {
        let _ = tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().with_writer(std::io::stderr))
            .try_init();
    }
}
