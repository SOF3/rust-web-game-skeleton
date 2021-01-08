/// The common port
pub const DEFAULT_PORT: u16 = 12345;
pub const DEFAULT_PORT_STR: &str = "12345";

pub fn is_valid_name(name: &str) -> bool {
    return name.trim().len() >= 3 && name.len() <= 31;
}
