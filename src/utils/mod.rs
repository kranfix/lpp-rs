pub fn is_letter(c: char) -> bool {
  (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || "áéíóúÁÉÍÓÚñÑ_".contains(c)
}
pub fn is_digit(c: char) -> bool {
  c >= '0' && c <= '9'
}
