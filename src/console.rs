use std::sync::Mutex;

use log::{info, Level, SetLoggerError};


use rustyline::{history::FileHistory, DefaultEditor, Editor, ExternalPrinter};



pub struct Console {
    editor: Mutex<Editor<(), FileHistory>>,
    printer: tokio::sync::Mutex<Box<dyn ExternalPrinter + Send + Sync>>,
    logger: ConsoleLogger,
}


impl Console {
    pub fn new() -> rustyline::Result<Self> {
        let mut editor = DefaultEditor::new()?;
        let printer = editor.create_external_printer()?;  
         
        Ok(Console{
            editor : Mutex::new(editor),
            printer : tokio::sync::Mutex::new(Box::new(printer)),
            logger : ConsoleLogger::new(),
        })
    }

    #[inline]
    fn print(&self, string: String) {
        crate::RUNTIME.spawn(async move {
            let mut printer_lock = crate::CONSOLE.printer.lock().await;
            if printer_lock.print(string).is_err() {
                panic!("oops!");
            }
        });
        
    }
    
    #[inline]
    pub fn println(&self, string: String) {
        self.print(format!("{string}\n"));
    }

    pub fn get_logger(&self) -> &ConsoleLogger {
        &self.logger
    }

    pub fn init(&'static self) -> Result<(), SetLoggerError> {
        log::set_logger(self.get_logger())
            .map(|()| log::set_max_level(log::LevelFilter::Debug))
    }

    pub fn start(&self) -> rustyline::Result<()> {
        loop {
            let mut editor_lock = self.editor.lock().unwrap();
            match editor_lock.readline("# ") {
                Ok(line) => {
                    editor_lock.add_history_entry(line.as_str())?;
                    match line.to_lowercase().as_str() {
                        "stop" => {
                            info!("Stopping server...");
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
                            //TODO: Stuff
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

pub struct ConsoleLogger;

impl ConsoleLogger {
    pub fn new() -> Self {
        ConsoleLogger{}
    }
}

impl log::Log for ConsoleLogger {
    #[inline]
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.target().starts_with("rustmcsrv") && metadata.level() <= Level::Debug
    }

    #[inline]
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            crate::CONSOLE.println(format!("{}: {}", record.level().to_string().to_uppercase(), record.args().to_string()));
        }
    }

    fn flush(&self) {}
}

