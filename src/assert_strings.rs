pub fn main() {
  create_a_string();
  can_determine_the_length_of_a_string_with_len();
}

fn create_a_string() {
  // A string can be created by calling new on the String class
  // String::new()
  assert!( string_test( ___() ), "A string has not been created.");

  // Text between doublestrings are not actual string objects in Rust
  // There are a borrowed reference to a str
  assert!( string_test( ___() ), "A string has not been created.");
}

fn can_determine_the_length_of_a_string_with_len() {
  // A string has a length
  let test_string = "test";
  assert!( test_string.len() == 4, "This is not the length you're looking for." );
  
  let new_string = "This is a string";
  assert!( ___(), "This is not the length you're looking for." );
}

fn ___() -> bool {
  false
}

fn string_test(_: String) -> bool {
  true
}
