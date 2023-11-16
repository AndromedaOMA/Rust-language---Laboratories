use anyhow::anyhow;
use anyhow::Result;

trait Command {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[&str]) -> Result<()>;
}

struct PingCdommand {}
impl Command for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }
    fn exec(&mut self, args: &[&str]) -> Result<()> {
        if args.len() == 0 {
            return Err(anyhow!("Ping command does not have arguments!"));
        }
        println!("pong!");
        Ok(());
    }
}

struct CountCommand {}
impl Command for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }
    fn exec(&mut self, args: &[&str]) -> Result<()> {
        if args.len() == 0 {
            return Err(anyhow!("Count command does not have arguments!"));
        }
        println!("{}", args.len());
    }
}

struct TimesCommand {
    count: i32,
}
impl Command for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }
    fn exec(&mut self, args: &[&str]) -> Result<()> {
        if args.len() == 0 {
            return Err(anyhow!("Times command does not have arguments!"));
        }
        self.count += 1;
        println!(
            "We called the times command {:?} times until now!",
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
    fn exec(&mut self, args: &[&str]) -> Result<()> {
        if args.len() == 0 {
            return Err(anyhow!("Print command does not have arguments!"));
        }
        println!("We called the command print!");
    }
}

// struct Terminal {
//     commands: Vec<Box<dyn Command>>,
// }

// impl Terminal {
//     fn new() -> Self {
//         Terminal {
//             commands: Vec::new(),
//         }
//     }
//     fn register() {}
//     fn run() {}
// }

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));

    terminal.run();
}
