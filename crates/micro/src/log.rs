/// Logs a [`Result`] at the error level if the [`Result`] is [`Err`].
#[macro_export]
macro_rules! log_if_err {
	($e:expr) => {
		if let Err(err) = &$e {
			tracing::error!("{:?}", err);
		}
	};
	($e:expr, with_backtrace) => {
		if let Err(err) = &$e {
			tracing::error!("{:?}\n{:?}", err, $crate::backtrace::Backtrace::new());
		}
	};
}
