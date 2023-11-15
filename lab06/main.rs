use eval::eval;
use std::fs::{File, read};
use std::io::{self, BufRead}; 
use rusqlite::Connection;
// P1 SI P2
fn main() {
    let mut terminal = Terminal::new();
    terminal.path = String::from("commands.txt");

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(EvalCommand {}));
    terminal.register(Box::new(BMCommand {database : String::from("bookmarks.db")}));

    terminal.run();
}
trait CommandTrait {
    fn get_name(&self) -> &'static str;
    fn exec(&mut self, args: &str);
}

struct PingCommand {}
impl CommandTrait for PingCommand {
    fn exec(&mut self, args: &str) {
        println!("pong");
    }
    fn get_name(&self) -> &'static str {
        return "ping";
    }
}

struct CountCommand {}

impl CommandTrait for CountCommand {
    fn exec(&mut self, args: &str) {
        let cols: Vec<&str> = args.split_whitespace().collect();

        println!("counted {} arguments", cols.len());
    }
    fn get_name(&self) -> &'static str {
        return "count";
    }
}

struct TimesCommand {
    count: u32,
}

impl CommandTrait for TimesCommand {
    fn exec(&mut self, args: &str) {
        println!("{} times", self.count);
        self.count += 1;
    }
    fn get_name(&self) -> &'static str {
        return "times";
    }
}

struct EvalCommand {}
impl CommandTrait for EvalCommand {
    fn exec(&mut self, args: &str) {
        let result = eval(args);
        match result {
            Ok(value) => println!("{} = {}", args, value),
            Err(e) => eprintln!("Error({e}) during evaluation of ({args})"),
        }
    }
    fn get_name(&self) -> &'static str {
        return "eval";
    }
}

struct BMCommand {
    database: String,
}
impl CommandTrait for BMCommand {
    fn exec(&mut self, args: &str) {
        let mut cols = args.splitn(2, char::is_whitespace);

        if let Some(col1) = cols.next() {
            if col1 == "add" {
                let col2: Option<&str>;
                let col3: Option<&str>;

                let rest =  cols.next();
                if rest.is_none() {
                    eprint!("format gresit pt add"); return;
                }
                let mut cols2 = rest.unwrap().splitn(2, char::is_whitespace);
                col2 = cols2.next();
                col3 = cols2.next();
                

                if col2.is_some() && col3.is_some() {
                    let db_ = Connection::open("bookmarks.db");
                    if db_.is_err() {
                        eprintln!("coulnd't open bookmark database");
                        
                    }
                    else {
                        let db = db_.unwrap();

                        let create = r"
                        create table if not exists bookmarks (
                            name text    not null,
                            url  text    not null
                        );
                        ";

                        if let Err(e) = db.execute(create,()){
                            eprintln!("Database error : {e}");
                        }
                        
                        if let Err(e) = db.execute("insert into bookmarks (name, url) values (?1, ?2);", (col2, col3)){
                            eprintln!("Database error : {e}");
                        } 
                    }
                    
                } else {
                    eprintln!("Wrong command format for bm add");
                }
            } else if col1 == "search" {
                let col2: Option<&str>;

                col2 = cols.next();

                if col2.is_some() {
                    let db_ = Connection::open("bookmarks.db");
                    if db_.is_err() {
                        eprintln!("coulnd't open bookmark database");
                        
                    }
                    else {
                        let db = db_.unwrap();                         
                        
                        let mut stmt_ = db.prepare("select * from bookmarks");
                        let mut stmt;
                        if stmt_.is_ok()  {stmt = stmt_.unwrap();}
                        else {eprint!("Error preparing statement"); return;}

                        let iter_ = stmt.query_map([], |row| {
                            Ok(
                                (row.get("name"),row.get("url"))
                            )
                        });
                        let iter;
                        if iter_.is_ok() {
                            iter = iter_.unwrap();
                        }
                        else {
                            eprintln!("error during query_map"); return;
                        }
                        for t in iter {
                            let t = t.unwrap();
                            
                            if t.0.is_ok() && t.1.is_ok() {
                                let name : String = t.0.unwrap();
                                let  url : String = t.1.unwrap();

                                let lookfor = col2.unwrap(); 
                                if name.contains(lookfor) {
                                    println!("{name} {url}");
                                }
                            }                            
                        }
                       
                    }
                } else {
                    eprintln!("Wrong command format for bm add");
                }
            } else {
                eprintln!("Coulnd't recognize bm command");
            }
        } else {
            eprintln!("Wrong command format for bm");
        }
    }
    fn get_name(&self) -> &'static str {
        return "bm";
    }
}

#[derive(Default)]
struct Terminal {
    comands: Vec<Box<dyn CommandTrait>>,
    path: String,
}

impl Terminal {
    fn run(&mut self) {
        let res = File::open(&self.path);

        let input: File;
        match res {
            Ok(file) => {
                input = file;
            }
            Err(err) => {
                eprintln!("Error opening file: {}", err);
                return;
            }
        }

        let buf = io::BufReader::new(input);

        for line_ in buf.lines() {
            if line_.is_ok() {
                let line: String = line_.unwrap();
                let trimmed = line.trim();
                if trimmed == "" {
                    continue;
                } else if trimmed == "stop" {
                    return;
                }

                let mut cols = trimmed.splitn(2, char::is_whitespace);

                if let Some(col1) = cols.next() {
                    let mut findcommand: Option<&mut Box<dyn CommandTrait>> = None;
                    for x in &mut self.comands {
                        if x.get_name() == col1 {
                            findcommand = Some(x);
                        }
                    }
                    match findcommand {
                        Some(command) => {
                            if let Some(col2) = cols.next() {
                                command.exec(col2);
                            } else {
                                let blank = String::from("");
                                command.exec(&blank);
                            }
                        }
                        None => {
                            println!("Couldn't find command {}", col1);
                        }
                    }
                }
            }
        }
    }
    fn new() -> Self {
        return Self {
            ..Default::default()
        };
    }

    fn register(&mut self, item: Box<dyn CommandTrait>) {
        self.comands.push(item);
    }
}
