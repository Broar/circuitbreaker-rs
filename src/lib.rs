pub mod strategy;

use strategy::Strategy;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt::Error as FmtError;

pub struct CircuitBreaker<T> {
    command: Box<Command<T>>,
    strategy: Box<Strategy>
}

impl<T> CircuitBreaker<T> {
    pub fn new(command: Box<Command<T>>, strategy: Box<Strategy>) -> Self {
        CircuitBreaker {
            command: command,
            strategy: strategy
        }
    }

    pub fn execute(&mut self) -> CommandResult<T> {
        if self.strategy.allow_request() {
            let result = self.command.execute();
            if result.is_ok() {
                self.strategy.success();
            }

            else {
                self.strategy.failure();
            }

            result
        }

        else {
            Err(Box::new(CircutBreakerError::CircuitOpen))
        }
    }
}

pub type CommandResult<T> = Result<T, Box<Error>>;

pub trait Command<T> {
    fn execute(&self) -> CommandResult<T>;
}

#[derive(Debug, PartialEq, Eq)]
enum CircutBreakerError {
    CircuitOpen
}

impl Display for CircutBreakerError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), FmtError> {
       Display::fmt(self.description(), formatter)
   }
}

impl Error for CircutBreakerError {
    fn description(&self) -> &str {
        match *self {
            CircutBreakerError::CircuitOpen => "Circuit is still open"
        }
    }
}
