use std::{io::Write};


use chrono::Local;
use log::{info, Level, SetLoggerError};


use rustyline::{history::FileHistory, DefaultEditor, Editor, ExternalPrinter};
use tokio::{signal, sync::{mpsc, Mutex}};



pub struct Console {
    editor: Mutex<Editor<(), FileHistory>>,
    printer: Mutex<Box<dyn ExternalPrinter + Send + Sync>>,
    logger: ConsoleLogger,
}


impl Console {
    pub fn new(sender: mpsc::Sender<String>) -> rustyline::Result<Self> {
        let mut editor = DefaultEditor::new()?;
        let printer = editor.create_external_printer()?;  
         
        Ok(Console{
            editor : Mutex::new(editor),
            printer : Mutex::new(Box::new(printer)),
            logger : ConsoleLogger::new(sender),
        })
    }

    #[inline]
    async fn print(&self, string: String) {
        let mut printer_lock = self.printer.lock().await;
        if printer_lock.print(string).is_err() {
            panic!("oops!");
        }
    }
    
    #[inline]
    pub async fn println(&self, string: String) {
        self.print(format!("{string}\n")).await;
    }

    pub fn get_logger(&self) -> &ConsoleLogger {
        &self.logger
    }

    pub fn init(&'static self) -> Result<(), SetLoggerError> {
        log::set_logger(self.get_logger())
            .map(|()| log::set_max_level(log::LevelFilter::Debug))
    }

    pub async fn start(&self) -> rustyline::Result<()> {
        loop {
            let mut editor_lock = self.editor.lock().await;
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
                            match crate::STOP_SIGNAL.get() {
                                Some(sig) => {
                                    let _ = sig.send(true);
                                    break;
                                },
                                None => println!("Server is already stopping"),
                            }
                            
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
    sender: mpsc::Sender<String>,
}

impl ConsoleLogger {
    pub fn new(sender: mpsc::Sender<String>) -> Self {
        ConsoleLogger{
            sender: sender,
        }
    }
    pub fn send(&self, message: String) {
        let dt = Local::now();
        let sender_clone = self.sender.clone();
        crate::RUNTIME.spawn(async move { sender_clone.send(format!("[{}] {}", dt.format("%H:%M:%S"), message)).await });
    }
}

impl log::Log for ConsoleLogger {
    #[inline]
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.target().starts_with("rustmcsrv") && metadata.level() <= Level::Debug
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            self.send(
                format!("{}: {}",
                record.level().to_string().to_uppercase(), 
                record.args().to_string())
            );
        }
    }

    fn flush(&self) {
        let _ = std::io::stdout().flush();
    }
}

