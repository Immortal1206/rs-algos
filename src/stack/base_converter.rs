use super::Stack;

pub fn convert_to_binary(mut dec_num: u32) -> String {
  let mut rem_stack = Stack::new();

  while dec_num > 0 {
    let rem = dec_num % 2;
    rem_stack.push(rem);
    dec_num /= 2;
  }

  let mut bin_str = "".to_string();
  while !rem_stack.is_empty() {
    let rem = rem_stack.pop().unwrap().to_string();
    bin_str += &rem;
  }

  bin_str
}

const DIGITS: [char; 16] = [
  '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];
pub fn base_converter_stack(mut dec_num: u32, base: u32) -> String {
  let mut rem_stack = Stack::new();
  while dec_num > 0 {
    let rem = dec_num % base;
    rem_stack.push(rem);
    dec_num /= base;
  }

  let mut base_str = "".to_string();
  while !rem_stack.is_empty() {
    let rem = rem_stack.pop().unwrap() as usize;
    base_str += &DIGITS[rem].to_string();
  }

  base_str
}

pub fn base_converter_recursion(dec_num: u32, base: u32) -> String {
  if dec_num < base {
    DIGITS[dec_num as usize].to_string()
  } else {
    base_converter_recursion(dec_num / base, base) + &DIGITS[(dec_num % base) as usize].to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  pub fn test_convert_to_binary() {
    let bin_str: String = convert_to_binary(233);
    assert_eq!(bin_str, "11101001")
  }
  #[test]
  pub fn test_base_converter() {
    let bin_str: String = base_converter_stack(10, 2);
    let hex_str: String = base_converter_stack(43, 16);
    assert_eq!("2B", hex_str);
    assert_eq!("1010", bin_str);
    let bin_str: String = base_converter_recursion(10, 2);
    let hex_str: String = base_converter_recursion(43, 16);
    assert_eq!("2B", hex_str);
    assert_eq!("1010", bin_str);
  }
}
