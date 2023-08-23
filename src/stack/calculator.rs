use super::bracket_checker::all_bracket_with_character;
use super::Stack;
use std::collections::HashMap;

fn priority() -> HashMap<&'static str, i32> {
  let mut prec = HashMap::new();
  prec.insert("(", 1);
  prec.insert(")", 1);
  prec.insert("+", 2);
  prec.insert("-", 2);
  prec.insert("*", 3);
  prec.insert("/", 3);
  prec.insert("%", 3);
  prec
}

#[derive(PartialEq, Debug)]
pub enum CalcError {
  DivideByZero,
  UnknownOperator(String),
  InvalidInputString,
}

fn calc(operator: &str, first_oprand: isize, second_oprand: isize) -> Result<isize, CalcError> {
  match operator {
    "+" => Ok(first_oprand + second_oprand),
    "-" => Ok(first_oprand - second_oprand),
    "*" => Ok(first_oprand * second_oprand),
    "/" => {
      if second_oprand == 0 {
        Err(CalcError::DivideByZero)
      } else {
        Ok(first_oprand / second_oprand)
      }
    }
    "%" => Ok(first_oprand % second_oprand),
    _ => Err(CalcError::UnknownOperator(operator.to_owned())),
  }
}

/**
1. 创建一个名为 op_stack 的空栈以保存运算符。给输出创建一个空列表 postfix。
2. 通过使用字符串方法拆分将输入的中缀字符串转换为标记列表 src_str。
3. 从左到右扫描标记列表。
   1. 如果标记是操作数，将其附加到输出列表的末尾。
   2.如果标记是左括号，将其压到 op_stack 上。
   3. 如果标记是右括号，则弹出 op_stack，直到删除相应左括号，将运算符加入 postfix。
   4. 如果标记是运算符 + - * /，则压入 op_stack。但先弹出 op_stack 中更高或相等优先级的运算符到 postfix。
4. 当输入处理完后，检查 op_stack，仍在栈上的运算符都可弹出到 postfix。
*/
pub fn infix_to_postfix(infix: &str) -> Option<String> {
  if !all_bracket_with_character(infix) {
    return None;
  }

  let priority = priority();
  let mut ops = Stack::new();
  let mut postfix = Vec::new();
  for token in infix.split_whitespace() {
    let num = token.parse::<isize>();
    if num.is_ok() {
      postfix.push(token);
    } else if "(" == token {
      ops.push(token);
    } else if ")" == token {
      let mut top = ops.pop().unwrap();
      while top != "(" {
        postfix.push(top);
        top = ops.pop().unwrap();
      }
    } else {
      while !ops.is_empty() && priority[ops.peek().unwrap()] >= priority[token] {
        postfix.push(ops.pop().unwrap());
      }
      ops.push(token);
    }
  }

  while !ops.is_empty() {
    postfix.push(ops.pop().unwrap())
  }

  let mut postfix_str = "".to_string();
  for c in postfix {
    postfix_str += &c.to_string();
    postfix_str += " ";
  }

  Some(postfix_str)
}

/**
1. 初始化两个栈：运算符栈S1和储存中间结果的栈S2；
2. 从右至左扫描中缀表达式；
3. 遇到操作数时，将其压入S2；
4. 遇到运算符时，比较其与S1栈顶运算符的优先级：
   1. 如果S1为空，或栈顶运算符为右括号“)”，则直接将此运算符入栈；
   2. 否则，若优先级比栈顶运算符的较高或相等，也将运算符压入S1；
   3. 否则，优先级低于栈顶运算符，将S1栈顶的运算符弹出并压入到S2中，再次转到(4-1)与S1中新的栈顶运算符相比较；
5. 遇到括号时：
   1. 如果是右括号“)”，则直接压入S1；
   2. 如果是左括号“(”，则依次弹出S1栈顶的运算符，并压入S2，直到遇到右括号为止，此时将这一对括号丢弃；
6. 重复步骤(2)至(5)，直到表达式的最左边；
7. 将S1中剩余的运算符依次弹出并压入S2；
8. 依次弹出S2中的元素并输出，结果即为中缀表达式对应的前缀表达式。
 */
pub fn infix_to_prefix(infix: &str) -> Option<String> {
  if !all_bracket_with_character(infix) {
    return None;
  }
  let priority = priority();

  let mut ops = Stack::new();
  let mut prefix = Stack::new();
  for token in infix.split_whitespace().rev() {
    if token.parse::<isize>().is_ok() {
      prefix.push(token);
    } else if token == "(" {
      let mut top = ops.pop().unwrap();
      while top != ")" {
        prefix.push(top);
        top = ops.pop().unwrap();
      }
    } else if token == ")" {
      ops.push(token);
    } else {
      while !ops.is_empty() && priority[ops.peek().unwrap()] > priority[token] {
        prefix.push(ops.pop().unwrap());
      }
      ops.push(token);
    }
  }

  while !ops.is_empty() {
    prefix.push(ops.pop().unwrap());
  }

  let mut prefix_str = "".to_string();
  while !prefix.is_empty() {
    prefix_str += prefix.pop().unwrap();
    prefix_str += " ";
  }

  Some(prefix_str)
}

/**
1. 创建一个名为 op_stack 的空栈。
2. 拆分字符串为符号列表。
3. 从左到右扫描符号列表。
   1. 如果符号是操作数，将其从字符串转换为整数，并将值压到 op_stack。
   2. 如果符号是运算符，弹出 op_stack 两次。第一次弹出的是第二个操作数，第二次弹出的是第一个操作数。<br>
      执行算术运算后，将结果压到操作数栈中。
4. 当输入的表达式被完全处理后，结果就在栈上，弹出 op_stack 得到最终运算值。
*/
pub fn postfix_calculator(postfix: &str) -> Result<isize, CalcError> {
  if postfix.len() < 5 {
    return Err(CalcError::InvalidInputString);
  }
  let mut oprands = Stack::new();
  for token in postfix.split_whitespace() {
    let num = token.parse::<isize>();
    if num.is_ok() {
      oprands.push(num.unwrap());
    } else {
      let oprand2 = oprands.pop().unwrap();
      let oprand1 = oprands.pop().unwrap();
      let res = calc(token, oprand1, oprand2);
      match res {
        Ok(val) => oprands.push(val),
        err => return err,
      }
    }
  }
  Ok(oprands.pop().unwrap())
}

/**
1. 创建一个结果栈。
2. 从右至左扫描表达式，
   1. 若为数字，压入结果栈中
   2. 若为运算符，从结果栈中弹出两个操作数，第一次弹出的是第一个操作数，第二次弹出的是第二个操作数。<br>
      运算结果压入结果栈中。
3. 弹出结果栈中的元素得到最终值
*/
pub fn prefix_calculator(prefix: &str) -> Result<isize, CalcError> {
  if prefix.len() < 5 {
    return Err(CalcError::InvalidInputString);
  }
  let mut oprands = Stack::new();
  for token in prefix.split_whitespace().rev() {
    let num = token.parse::<isize>();
    if num.is_ok() {
      oprands.push(num.unwrap());
    } else {
      let oprand1 = oprands.pop().unwrap();
      let oprand2 = oprands.pop().unwrap();
      let res = calc(token, oprand1, oprand2);
      match res {
        Ok(val) => oprands.push(val),
        err => return err,
      }
    }
  }
  Ok(oprands.pop().unwrap())
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  pub fn test_infix_to_postfix() {
    let infix = "( 1000 + 25 ) % ( 2 + 18 - 7 )";
    let postfix = infix_to_postfix(infix);
    assert_eq!(Some("1000 25 + 2 18 + 7 - % ".to_owned()), postfix);
    let infix = "1000 + 24 * 34 + 100 - 3 * ( 4 + 7 )";
    let postfix = infix_to_postfix(infix);
    assert_eq!(
      Some("1000 24 34 * + 100 + 3 4 7 + * - ".to_owned()),
      postfix
    );
    let infix = "1 + 3 * 4 / ( 5 - 1 ) * 3 + 1 - ( 10 + 1 )";
    let postfix = infix_to_postfix(infix);
    assert_eq!(
      Some("1 3 4 * 5 1 - / 3 * + 1 + 10 1 + - ".to_owned()),
      postfix
    );
  }
  #[test]
  pub fn test_infix_to_prefix() {
    let infix = "( 1000 + 25 ) % ( 2 + 18 - 7 )";
    let prefix = infix_to_prefix(infix);
    assert_eq!(Some("% + 1000 25 - + 2 18 7 ".to_owned()), prefix);
    let infix = "1000 + 24 * 34 + 100 - 3 * ( 4 + 7 )";
    let prefix = infix_to_prefix(infix);
    assert_eq!(
      Some("- + + 1000 * 24 34 100 * 3 + 4 7 ".to_string()),
      prefix
    );
    let infix = "1 + 3 * 4 / ( 5 - 1 ) * 3 + 1 - ( 10 + 1 )";
    let prefix = infix_to_prefix(infix);
    assert_eq!(
      Some("- + + 1 * / * 3 4 - 5 1 3 1 + 10 1 ".to_string()),
      prefix
    );
  }
  #[test]
  pub fn test_postfix_calculator() {
    let infix = "( 1000 + 25 ) % ( 2 + 18 - 7 )";
    let postfix = infix_to_postfix(infix);
    assert_eq!(Ok(11), postfix_calculator(&postfix.unwrap()));
    let infix = "1000 + 24 * 34 + 100 - 3 * ( 4 + 7 )";
    let postfix = infix_to_postfix(infix);
    assert_eq!(Ok(1883), postfix_calculator(&postfix.unwrap()));
    let infix = "1 + 3 * 4 / ( 5 - 1 ) * 3 + 1 - ( 10 + 1 )";
    let postfix = infix_to_postfix(infix);
    assert_eq!(Ok(0), postfix_calculator(&postfix.unwrap()));
  }
  #[test]
  pub fn test_prefix_calculator() {
    let infix = "( 1000 + 25 ) % ( 2 + 18 - 7 )";
    let prefix = infix_to_prefix(infix);
    assert_eq!(Ok(11), prefix_calculator(&prefix.unwrap()));
    let infix = "1000 + 24 * 34 + 100 - 3 * ( 4 + 7 )";
    let prefix = infix_to_prefix(infix);
    assert_eq!(Ok(1883), prefix_calculator(&prefix.unwrap()));
    let infix = "1 + 3 * 4 / ( 5 - 1 ) * 3 + 1 - ( 10 + 1 )";
    let prefix = infix_to_prefix(infix);
    assert_eq!(Ok(0), prefix_calculator(&prefix.unwrap()));
  }
}
