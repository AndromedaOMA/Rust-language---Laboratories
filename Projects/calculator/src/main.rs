use std::collections::HashMap;
use std::io::stdin;

struct Calculator {
    sign_pr: HashMap<char, i32>,
    asoc: HashMap<char, char>,
    infix: String,
    postfix: String,
}

impl Calculator {
    fn calculator() -> Calculator {
        Calculator {
            sign_pr: HashMap::from([('+', 1), ('-', 1), ('*', 2), ('/', 2), ('^', 3)]),
            asoc: HashMap::from([('+', 'L'), ('-', 'L'), ('*', 'L'), ('/', 'L'), ('^', 'R')]),
            infix: String::new(),
            postfix: String::new(),
        }
    }

    fn store_infix(&mut self) {
        println!("Enter the infix expression: ");
        stdin()
            .read_line(&mut self.infix)
            .expect("Failed to read line");

        self.infix = self.infix.trim().to_string();
    }

    fn infix_to_posfix(&mut self) {
        let mut stack: Vec<char> = Vec::new();
        for c in self.infix.chars() {
            if c.is_digit(10) {
                self.postfix.push(c);
            } else if c == '(' {
                stack.push(c);
            } else if c == ')' {
                while stack.last().unwrap() != &'(' {
                    self.postfix.push(stack.last().unwrap().clone());
                    stack.pop();
                }
                stack.pop();
            } else {
                while !stack.is_empty()
                    && (self.sign_pr.get(&c) < self.sign_pr.get(stack.last().unwrap())
                        || self.sign_pr.get(&c) == self.sign_pr.get(stack.last().unwrap())
                            && self.asoc.get(&c) == Some(&'L'))
                {
                    self.postfix.push(stack.last().unwrap().clone());
                    stack.pop();
                }
                stack.push(c);
            }
        }
        while !stack.is_empty() {
            self.postfix.push(stack.last().unwrap().clone());
            stack.pop();
        }
    }

    fn show_postfix(&mut self) {
        println!("This is the postfix expression: {:?}", self.postfix);
    }

    fn evaluate_postfix(&mut self) {
        let mut stack: Vec<i32> = Vec::new();

        for c in self.postfix.chars() {
            if c.is_digit(10) {
                stack.push(c.to_digit(10).unwrap() as i32);
            } else {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                match c {
                    '+' => stack.push(a + b),
                    '-' => stack.push(a - b),
                    '*' => stack.push(a * b),
                    '/' => stack.push(a / b),
                    '^' => stack.push(a.pow(b as u32)),
                    _ => {
                        println!("Invalid operator");
                        return;
                    }
                }
            }
        }
        println!("The result is: {:?}", stack.pop().unwrap());
    }
}

fn main() {
    let mut calc = Calculator::calculator();
    calc.store_infix();
    println!("This is the infix expression: {:?}", calc.infix);

    calc.infix_to_posfix();

    calc.show_postfix();

    calc.evaluate_postfix();
}
