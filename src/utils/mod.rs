pub fn is_letter(c: char) -> bool {
  (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || "áéíóúÁÉÍÓÚñÑ_".contains(c)
}
pub fn is_digit(c: char) -> bool {
  c >= '0' && c <= '9'
}

pub fn read_file(path: &str) -> std::io::Result<String> {
  use std::fs::File;
  use std::io::Read;

  let mut file = File::open(path)?;
  let mut content = String::new();

  file.read_to_string(&mut content)?;

  Ok(content)
}
