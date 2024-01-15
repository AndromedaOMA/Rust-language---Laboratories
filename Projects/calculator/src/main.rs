use std::collections::HashMap;

struct Calculator {
    sign_pr: HashMap<char, i32>,
    asoc: HashMap<char, char>,
    infix: String,
    postfix: String,
    result: String,
}

impl Calculator {
    fn calculator() -> Calculator {
        Calculator {
            sign_pr: HashMap::from([('+', 1), ('-', 1), ('*', 2), ('/', 2), ('^', 3)]),
            asoc: HashMap::from([('+', 'L'), ('-', 'L'), ('*', 'L'), ('/', 'L'), ('^', 'R')]),
            infix: String::new(),
            postfix: String::new(),
            result: String::new(),
        }
    }

    fn store_infix(&mut self, exp: String) -> String {
        self.infix = exp;
        self.infix = self.infix.trim().to_string();

        let text = String::from("The infix expression is: ".to_owned() + &self.infix);
        return text;
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

    fn show_postfix(&mut self) -> String {
        let text = String::from("This is the postfix expression: ".to_owned() + &self.postfix);
        return text;
    }

    fn evaluate_postfix(&mut self) -> String{
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
                    }
                }
            }
        }

        self.result = stack.pop().unwrap().to_string();
        let text = String::from("The result is: ".to_owned() + &self.result);
        return text;
    }
}

slint::slint! {
    import { Button, VerticalBox, LineEdit } from "std-widgets.slint";

    export component AppWindow inherits Window {

    in property <string> result: "";
    callback calculate(string);

        GridLayout{
            padding:30px;
            spacing:25px;
            Row{
                Text{
                    text: "Enter the arithmetic expression";
                    horizontal-alignment: center;
                    font-size: 20pt;
                    font-weight: 450;
                }
            }
            Row{
                input := LineEdit{
                    horizontal-alignment: center;
                    font-size: 25px;
                    placeholder-text:"yay";
                    height: 40px;
                }
            }
            Row{
                Button {
                    text: "Calculate";
                    primary:true;
                    height: 40px;
                    clicked =>{ calculate(input.text) }
                }
            }
            Row{
                VerticalBox {
                    Rectangle{
                        height: 100px;
                        background: #f2f2f2;
                        Text{
                            color: black;
                            font-size: 25px;
                            font-weight: 500;
                            text: root.result;
                        }
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_calculate(move |infix| {
        let ui = ui_handle.unwrap();

        let mut calc = Calculator::calculator();
        let text1: String = calc.store_infix(infix.to_string());
        calc.infix_to_posfix();
        let text2: String = calc.show_postfix();
        let text3: String = calc.evaluate_postfix();

        let result: String = format!("{}\n{}\n{}", text1, text2, text3);
        ui.set_result(result.into());
    });

    ui.run()
}
