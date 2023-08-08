#[macro_export]
macro_rules! log_if_err {
	($e:expr) => {
		if let Err(err) = &$e {
			tracing::error!("{:?}", err);
		}
	};
}
