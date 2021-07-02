use crate::command::cmd_trait::Command;
use crate::command::command_builder::CommandBuilder;
use crate::constants::LINE_BREAK;
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;
use crate::server::logger::{Loggable, Logger};

const INFO_DBSIZE_COMMAND: &str = "Run command DECRBY\n";
const CLIENT_ID: &str = "DecrbyCommmand";
const CONST_CMD: &str = "decrby";

pub struct DecrbyCommand {
    id_job: u32,
    logger: Logger<String>,
}

impl DecrbyCommand {
    pub fn new(id_job: u32, logger: Logger<String>, mut command_builder: CommandBuilder) -> Self {
        let cmd = Self { id_job, logger };
        command_builder.insert(CONST_CMD.to_string(), Box::new(cmd.clone()));
        cmd
    }
}

impl Loggable for DecrbyCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

impl Clone for DecrbyCommand {
    fn clone(&self) -> DecrbyCommand {
        DecrbyCommand {
            id_job: self.id_job,
            logger: self.logger.clone(),
        }
    }
}

impl Command for DecrbyCommand {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        _id_client: usize,
    ) -> Result<String, RunError> {
        let log_info_res = self
            .logger
            .info(self, INFO_DBSIZE_COMMAND, app_info.get_verbose());
        if let Ok(_r) = log_info_res {}

        let db = app_info.get_string_db();

        let rsp = db.decrby(args[0].to_string(), args[1].to_string())?;
        let mut response = rsp.to_string();
        response.push(LINE_BREAK);
        Ok(response)
    }
}