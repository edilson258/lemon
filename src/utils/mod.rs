pub mod range;
pub mod report;
pub mod source;

pub fn match_number(character: char) -> bool {
  "1234567890.".contains(character)
}
