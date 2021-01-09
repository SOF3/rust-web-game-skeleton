/// The common port
pub const DEFAULT_PORT: u16 = 12345;
/// A string representation of the common port.
pub const DEFAULT_PORT_STR: &str = "12345";

/// Checks whether the client name is valid.
pub fn is_valid_name(name: &str) -> bool {
    name.trim().len() >= 3 && name.len() <= 31
}
