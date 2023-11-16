use std::fs;

#[derive(Debug)]
enum Errors {
    NotValidCommand,
    NoArgs,
    TooManyArgs,
    EmptyLine,
}

trait Command {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[&str]) -> Result<(), Errors>;
}

struct PingCommand {}
struct TimesCommand {
    count: u32,
}
struct CpCommand {}
struct FiboCommand {
    x: u32,
}

impl Command for PingCommand {
    fn get_name(&self) -> &str {
        return "ping";
    }
    fn exec(&mut self, args: &[&str]) -> Result<(), Errors> {
        if args.is_empty() {
            println!("pong!");
        } else {
            return Err(Errors::TooManyArgs);
        }
        Ok(())
    }
}

impl Command for TimesCommand {
    fn get_name(&self) -> &str {
        return "times";
    }
    fn exec(&mut self, args: &[&str]) -> Result<(), Errors> {
        if args.is_empty() {
            self.count += 1;
            println!("{} times called", self.count);
        } else {
            return Err(Errors::TooManyArgs);
        }
        Ok(())
    }
}

impl Command for CpCommand {
    fn get_name(&self) -> &str {
        return "count";
    }
    fn exec(&mut self, args: &[&str]) -> Result<(), Errors> {
        let mut k = 0;
        for _ in args {
            k += 1;
        }
        if k != 0 {
            println!("counted {} args", k);
        } else {
            return Err(Errors::NoArgs);
        }
        Ok(())
    }
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut p = 0;
    let mut c = 1;

    for _ in 2..=n {
        let next = p + c;
        p = c;
        c = next;
    }

    return c;
}

impl Command for FiboCommand {
    fn get_name(&self) -> &str {
        return "fibonacci";
    }
    fn exec(&mut self, args: &[&str]) -> Result<(), Errors> {
        if args.len() == 1 {
            for i in args[0].chars() {
                self.x = self.x * 10 + i.to_digit(10).unwrap();
            }
            print!("First {} numbers of Fibonacci sequence is : ", self.x);
            for i in 0..self.x {
                print!("{:?} ", fibonacci(i));
            }
            println!();
        } else if args.len() == 0 {
            return Err(Errors::NoArgs);
        } else {
            return Err(Errors::TooManyArgs);
        }
        Ok(())
    }
}

struct Terminal {
    commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Terminal {
        let commands = Vec::<Box<dyn Command>>::new();
        return Terminal { commands };
    }
    fn register(&mut self, comm: Box<dyn Command>) {
        self.commands.push(comm);
    }
    fn run(&mut self) {
        let s = fs::read_to_string("src/text.txt").unwrap();

        for i in s.lines() {
            let mut k = 1;
            let mut args = Vec::<&str>::new();
            let mut cmm_name = "";
            let mut ok = false;
            if i.trim() == "" {
                println!("{:?}", Errors::EmptyLine);
                println!();
            } else {
                for j in i.trim().split(' ') {
                    if k == 1 {
                        cmm_name = j;
                    }
                    k += 1;
                    if k > 2 {
                        if j != "" {
                            args.push(j);
                        }
                    }
                }
                let args: &[&str] = &args;
                for t in &mut self.commands {
                    if cmm_name == t.get_name() {
                        println!("Command:{} ", t.get_name());
                        match t.exec(args) {
                            Ok(()) => println!("Successfully"),
                            Err(err) => eprintln!("{:?}", err),
                        }
                        println!();
                        ok = true;
                        break;
                    } else if cmm_name.to_lowercase() == t.get_name() {
                        println!("Suggested command: {}", t.get_name());
                        println!();
                        ok = true;
                        break;
                    }
                }
                if cmm_name == "stop" {
                    println!("Stopped!");
                    println!();
                    break;
                }
                if ok == false {
                    println!("{:?}", Errors::NotValidCommand);
                    println!();
                }
            }
        }
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(CpCommand {}));
    terminal.register(Box::new(FiboCommand { x: 0 }));

    terminal.run();
}
