use std::io::{BufReader, BufWriter};

use std::sync::mpsc::{channel, Sender, Receiver};

use log::Level;
use tokio::sync::{Mutex, RwLock};



use rustyline::{history::FileHistory, DefaultEditor, Editor, ExternalPrinter, Result};

pub struct Console {
    editor: Editor<(), FileHistory>,
    logger: ConsoleLogger,
    printer: Box<dyn ExternalPrinter + Send + Sync>,
}


impl Console {
    pub fn new() -> Result<Self> {
        let mut editor = DefaultEditor::new()?;
        let printer = editor.create_external_printer()?;  
         
        Ok(Console{
            editor : editor,
            logger : ConsoleLogger::new(),
            printer : Box::new(printer),
        })
    }

    pub fn get_printer(&mut self) -> &mut Box<dyn ExternalPrinter + Send + Sync> {
        &mut self.printer
    }

    pub fn set_logging_level(&mut self, level: Level) {
        self.logger.set_level(level);
    } 

    pub fn get_logger(&'static self) -> &'static ConsoleLogger {
        &self.logger
    }



    pub fn start() -> Result<()> {
        loop {
            match crate::CONSOLE.blocking_write().editor.readline("# ") {
                Ok(line) => {
                    crate::CONSOLE.blocking_write().editor.add_history_entry(line.as_str())?;
                    match line.to_lowercase().as_str() {
                        "stop" => {
                            crate::THE_SERVER.get_players_iter().for_each(|weak| {
                                match weak.upgrade() {
                                    Some(player) => {
                                        crate::RUNTIME.spawn(async move { 
                                            player.disconnect("Server closed").await
                                        });
                                    },
                                    None => (),
                                }
                            });
                            break;
                        },
                        _ => println!("Unknown command")
                    }
                },
                Err(e)=> Err(e)?,
            }
        }
        Ok(())
    }
}

pub struct ConsoleLogger {
    
    level: Level,
}

impl ConsoleLogger {
    pub fn new() -> Self {
        ConsoleLogger { level : Level::Info }
    }
    pub fn set_level(&mut self, level: Level) {
        self.level = level;
    }
    
}

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    
    
    fn log(&self, record: &log::Record) {
        let the_string: String = record.args().to_string();
        let level_string: String = self.level.as_str().to_uppercase();
        crate::RUNTIME.spawn( async move {
            let mut block_lock = crate::CONSOLE.write().await;
            let _ = block_lock.get_printer().print(format!("{}: {}", level_string, the_string));
        });
    }

    fn flush(&self) {}
}