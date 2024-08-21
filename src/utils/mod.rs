pub mod highlight;
pub mod range;
pub fn match_number(character: char) -> bool {
  "1234567890.".contains(character)
}

pub fn match_identifier(character: char) -> bool {
  "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_".contains(character)
}
