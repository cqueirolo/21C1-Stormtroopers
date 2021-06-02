use crate::command::cmd_trait::Command;
use crate::errors::run_error::RunError;
use crate::logger::{Loggable, Logger};
use crate::structure_string::StructureString;

use std::sync::Arc;

const INFO_PING_COMMAND: &str = "Run command PING\n";
const RESPONSE_PING_COMMAND: &str = "PONG\n";
const CLIENT_ID: &str = "PingCommand";

pub struct PingCommand {
    logger: Logger<String>,
    id_job: u32,
}

impl PingCommand {
    pub fn new(id_job: u32, logger: Logger<String>) -> PingCommand {
        PingCommand { id_job, logger }
    }
}
impl Command for PingCommand {
    fn run(
        &self,
        _args: Vec<&str>,
        _structure: Arc<StructureString<String>>,
    ) -> Result<String, RunError> {
        let _log_info_res = self.logger.info(self, INFO_PING_COMMAND);

        Ok(String::from(RESPONSE_PING_COMMAND))
    }
}

impl Loggable for PingCommand {
    fn get_id_client(&self) -> &str {
        CLIENT_ID
    }

    fn get_id_thread(&self) -> u32 {
        self.id_job
    }
}

#[cfg(test)]
#[test]
fn test_ping_command_return() {
    let log = Logger::new(String::from("log"), "./".to_string()).unwrap();
    let arc_structure = Arc::new(StructureString::new());

    let ping = PingCommand::new(0, log);
    assert_eq!(
        Command::run(&ping, vec!(""), arc_structure),
        Ok(String::from(RESPONSE_PING_COMMAND))
    );
}