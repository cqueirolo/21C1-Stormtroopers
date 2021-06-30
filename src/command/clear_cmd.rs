use crate::command::cmd_trait::Command;
use crate::server::app_info::AppInfo;

use crate::command::command_parser::ParsedMessage;
use crate::errors::run_error::RunError;
use crate::server::logger::{Loggable, Logger};

const INFO_RUN_COMMAND: &str = "Run command CLEAR\n";
const CLIENT_ID: &str = "ClearCommand";

const MIN_VALID_ARGS: i32 = 0;
const MAX_VALID_ARGS: i32 = 0;

pub struct ClearCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl ClearCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> ClearCommand {
        ClearCommand { id_job, logger }
    }
}

impl Loggable for ClearCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for ClearCommand {
    fn clone(&self) -> ClearCommand {
        ClearCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for ClearCommand {
    fn run(
        &self,
        mut _args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self
            .logger
            .info(self, INFO_RUN_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        ParsedMessage::validate_args(_args.clone(), MIN_VALID_ARGS, MAX_VALID_ARGS)?;

        let mut response = String::from("");

        for _i in 1..51 {
            response.push('\n');
            response.push('\r');
        }

        //response.push_str(&String::from("printf \"\\033c\""));

        //if std::process::Command::new("clear").status().unwrap().success() {}

        Ok(response)
    }
}
