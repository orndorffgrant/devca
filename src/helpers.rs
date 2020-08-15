pub(crate) fn stringify<T: std::fmt::Display>(err: T) -> String {
    format!("{}", err)
}
