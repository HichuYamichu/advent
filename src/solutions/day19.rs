pub fn a(_input_string: &str) -> String {
  String::from("")
}

pub fn b(_input_string: &str) -> String {
  String::from("")
}

#[test]
pub fn tests_a() {
  assert_eq!(a(""), "");
}

#[test]
fn tests_b() {
  assert_eq!(b(""), "");
}
