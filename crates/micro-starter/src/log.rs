#[cfg(debug_assertions)]
pub(crate) fn setup_logging() {
	use tracing_subscriber::EnvFilter;

	tracing_subscriber::fmt()
		.with_env_filter(
			EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn,micro=info")),
		)
		.init();
}

#[cfg(not(debug_assertions))]
pub(crate) fn setup_logging() -> tracing_appender::non_blocking::WorkerGuard {
	use tracing_subscriber::EnvFilter;

	use crate::dirs::project_dirs;

	let logs_dir = project_dirs()
		.map(|project_dirs| project_dirs.data_dir().to_path_buf())
		.unwrap_or(
			std::env::current_exe()
				.expect("could not get path of executable")
				.parent()
				.unwrap()
				.to_path_buf(),
		)
		.join("logs");

	let file_appender = tracing_appender::rolling::hourly(logs_dir, "log");
	let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
	tracing_subscriber::fmt()
		.with_env_filter(
			EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn")),
		)
		.with_ansi(false)
		.with_writer(non_blocking)
		.init();
	guard
}
