// use anyhow::anyhow;
// use anyhow::Result;
use std::fs;

trait Command {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[&str]) /*-> Result<()>*/;
}

struct PingCommand {}
impl Command for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }
    fn exec(&mut self, args: &[&str]) /*-> Result<()>*/
    {
        if args.len() == 0 {
            // return Err(anyhow!("Err: \"ping\" command does not have arguments! -> "));
            print!("Err: \"ping\" command does not have arguments! -> ");
        }
        println!("pong!");
    }
}

struct CountCommand {}
impl Command for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }
    fn exec(&mut self, args: &[&str]) /*-> Result<()>*/
    {
        if args.len() == 0 {
            // return Err(anyhow!("Err: \"count\" command does not have arguments! -> ));
            print!("Err: \"count\" command does not have arguments! -> ");
        }
        println!(
            "We called the \"count\" command and we counted {:?} right now!",
            args.len()
        );
    }
}

struct TimesCommand {
    count: i32,
}
impl Command for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }
    fn exec(&mut self, args: &[&str]) /*-> Result<()>*/
    {
        if args.len() == 0 {
            // return Err(anyhow!("Err: \"times\" command does not have arguments! -> "));
            print!("Err: \"times\" command does not have arguments! -> ");
        }
        self.count += 1;
        println!(
            "We called the \"times\" command {:?} times until now!",
            self.count
        );
    }
}

// custom command:
struct PrintCommand {}
impl Command for PrintCommand {
    fn get_name(&self) -> &str {
        "print"
    }
    fn exec(&mut self, args: &[&str]) /*-> Result<()>*/
    {
        if args.len() == 0 {
            // return Err(anyhow!("Err: \"print\" command does not have argument! -> "));
            print!("Err: \"print\" command does not have argument! -> ");
        }
        println!("We called the \"print\" command!");
    }
}

//source: https://doc.rust-lang.org/rust-by-example/trait.html
struct Terminal {
    list_of_commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Terminal {
        Terminal {
            list_of_commands: Vec::<Box<dyn Command>>::new(),
        }
    }

    fn register(&mut self, list_of_commands: Box<dyn Command>) {
        self.list_of_commands.push(list_of_commands);
    }

    fn run(&mut self) {
        let terminal_text: String = fs::read_to_string("./src/input.txt").unwrap();
        for line in terminal_text.lines() {
            if line.trim().len() == 0 {
                println!("No command!");
            } else if let Some(entity) = line.split_whitespace().nth(0) {
                // source: https://doc.rust-lang.org/std/iter/trait.Iterator.html
                if entity == "stop" {
                    println!("Terminal stopped!");
                    return;
                } else {
                    for obj in &mut self.list_of_commands {
                        if entity == obj.get_name() {
                            obj.exec(&line.split_whitespace().collect::<Vec<_>>()[1..]);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(PrintCommand {}));

    let _ = terminal.run();
}
