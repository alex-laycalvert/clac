#[derive(Debug, Copy, Clone)]
enum Operation {
    Number,
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
}

#[derive(Debug, Clone, Copy)]
struct Node<'a> {
    value: &'a str,
    operation: Operation,
    priority: i32,
}

#[derive(Debug, Clone)]
struct Tree<'a>(Node<'a>, Option<Box<Tree<'a>>>, Option<Box<Tree<'a>>>);

fn evaluate_tree(tree: Option<Tree>) -> Option<f64> {
    dbg!(&tree);
    return None;
    match tree {
        Some(Tree(t, left, right)) => match t.operation {
            Operation::Number => t.value.parse().ok(),
            Operation::Add => {
                let left = match left {
                    Some(l) => {
                        if let Some(l) = evaluate_tree(Some(*l)) {
                            l
                        } else {
                            return None;
                        }
                    }
                    None => return None,
                };
                let right = match right {
                    Some(r) => {
                        if let Some(r) = evaluate_tree(Some(*r)) {
                            r
                        } else {
                            return None;
                        }
                    }
                    None => return None,
                };
                Some(left + right)
            }
            Operation::Subtract => {
                let left = match left {
                    Some(l) => {
                        if let Some(l) = evaluate_tree(Some(*l)) {
                            l
                        } else {
                            return None;
                        }
                    }
                    None => return None,
                };
                let right = match right {
                    Some(r) => {
                        if let Some(r) = evaluate_tree(Some(*r)) {
                            r
                        } else {
                            return None;
                        }
                    }
                    None => return None,
                };
                Some(left - right)
            }
            _ => None,
        },
        None => None,
    }
}

fn evaluate_tokens(tokens: &Vec<String>) -> Option<f64> {
    println!("Tokens: {:?}", tokens);
    let mut tree: Option<Tree> = None;
    let current_priority = 0;
    for i in 0..tokens.len() {
        let tree_node = match tokens[i].as_str() {
            "+" => Tree(
                Node {
                    value: &tokens[i],
                    priority: current_priority + 2,
                    operation: Operation::Add,
                },
                None,
                None,
            ),
            "-" => Tree(
                Node {
                    value: &tokens[i],
                    priority: current_priority + 2,
                    operation: Operation::Subtract,
                },
                None,
                None,
            ),
            "*" => Tree(
                Node {
                    value: &tokens[i],
                    priority: current_priority + 1,
                    operation: Operation::Multiply,
                },
                None,
                None,
            ),
            "/" => Tree(
                Node {
                    value: &tokens[i],
                    priority: current_priority + 1,
                    operation: Operation::Divide,
                },
                None,
                None,
            ),
            _ => Tree(
                Node {
                    value: &tokens[i],
                    priority: current_priority,
                    operation: Operation::Number,
                },
                None,
                None,
            ),
        };
        match &tree {
            Some(t) => {
                if tree_node.0.priority >= t.0.priority {
                    tree = Some(Tree(tree_node.0, Some(Box::new(t.clone())), None));
                } else {
                    match t.1 {
                        None => {
                            tree = Some(Tree(t.0, Some(Box::new(tree_node)), t.2.clone()));
                            continue;
                        }
                        _ => (),
                    }
                    match &t.2 {
                        Some(v) => {
                            tree = Some(Tree(
                                t.0,
                                t.1.clone(),
                                Some(Box::new(Tree(tree_node.0, Some(v.clone()), None))),
                            ));
                            continue;
                        }
                        None => {
                            tree = Some(Tree(t.0, t.1.clone(), Some(Box::new(tree_node))));
                            continue;
                        }
                    }
                }
            }
            None => tree = Some(tree_node),
        }
    }
    evaluate_tree(tree)
}

pub fn evaluate(input: String) -> Option<f64> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current_token: String = "".to_string();
    let chars: Vec<char> = input.trim().chars().collect();
    for i in 0..chars.len() {
        match chars[i] {
            ' ' | '\n' => {
                if current_token.len() == 0 {
                    continue;
                }
                tokens.push(current_token.to_string());
                current_token.clear();
            }
            '+' | '/' | '*' | '(' | ')' => {
                if current_token.len() > 0 {
                    tokens.push(current_token.to_string());
                    current_token.clear();
                }
                tokens.push(chars[i].to_string());
            }
            '-' => {
                if i < chars.len() - 1 && chars[i + 1] != ' ' {
                    current_token.push(chars[i]);
                } else {
                    if current_token.len() > 0 {
                        tokens.push(current_token.to_string());
                        current_token.clear();
                    }
                    tokens.push(chars[i].to_string());
                }
            }
            _ => current_token.push(chars[i]),
        }
    }
    if current_token.len() > 0 {
        tokens.push(current_token);
    }
    evaluate_tokens(&tokens.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(Some(0.0), evaluate("0".to_string()));
    }

    #[test]
    fn single_digit_number() {
        assert_eq!(Some(5.0), evaluate("5".to_string()));
    }

    #[test]
    fn negative_number() {
        assert_eq!(Some(-5.0), evaluate("-5".to_string()));
    }

    #[test]
    fn multi_digit_number() {
        assert_eq!(Some(123.0), evaluate("123".to_string()));
    }

    #[test]
    fn multi_digit_negative_number() {
        assert_eq!(Some(-52.0), evaluate("-52".to_string()));
    }

    #[test]
    fn addition() {
        assert_eq!(Some(3.0), evaluate("1 + 2".to_string()));
    }

    #[test]
    fn multi_digit_addition() {
        assert_eq!(Some(27.0), evaluate("13 + 14".to_string()));
    }

    #[test]
    fn subtraction() {
        assert_eq!(Some(5.0), evaluate("7 - 2".to_string()));
    }

    #[test]
    fn negative_number_subtraction() {
        assert_eq!(Some(-5.0), evaluate("2 - 7".to_string()));
    }

    #[test]
    fn complex_equation_1() {
        assert_eq!(Some(-15.0), evaluate("1 + 2 - (3 + 6) * 2".to_string()));
    }
}
