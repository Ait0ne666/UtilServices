use std::sync::Arc;

use flexi_logger::{Age, Criterion, FileSpec, Logger, LoggerHandle, Naming, writers::FileLogWriter};

use crate::{setup_grpc::App, prelude::Severity};

pub struct FileLogger {
    pub logger: Arc<LoggerHandle>,
}

impl FileLogger {
    pub fn new(logger: Arc<LoggerHandle>) -> Self {
        FileLogger { logger: logger }
    }

    pub fn create_handle() -> LoggerHandle {
        Logger::try_with_str("info")
            .unwrap() // Write all error, warn, and info messages
            .log_to_file(FileSpec::default())
            .rotate(
                // If the program runs long enough,
                Criterion::Age(Age::Day), // - create a new file every day
                Naming::Timestamps,
                flexi_logger::Cleanup::Never,
            )
            .start()
            .unwrap()
    }


    pub fn log(&self, app: App, message: &str, severity: Severity) {
        let t = app.title.clone();

        let res = self.logger.reset_flw(
            &FileLogWriter::builder(
                FileSpec::default()
                    .basename(app.title)
                    .directory(format!("./logs/{}", t)),
            )
            .append()
            .rotate(
                Criterion::Age(Age::Day), 
                Naming::Timestamps,
                flexi_logger::Cleanup::Never,
            )
        );


        match res {
            Ok(_) => {},
            Err(e) => {
                println!("{}", e);
            },
        }


        match severity {
            Severity::Warning => {
                log::warn!("{}", message)
            },
            Severity::Info => {
                log::info!("{}", message)
            },
            Severity::Error => {
                log::error!("{}", message)
            },
        }



    }
}
