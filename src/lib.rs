pub mod strategy;

use strategy::Strategy;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt::Error as FmtError;

type CommandResult<T> = Result<T, Box<Error>>;

trait Command<T> {
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

pub struct CircuitBreaker<T> {
    command: Box<Command<T>>,
    strategy: Box<Strategy>
}

impl<T> CircuitBreaker<T> {
    fn new(command: Box<Command<T>>, strategy: Box<Strategy>) -> Self {
        CircuitBreaker {
            command: command,
            strategy: strategy
        }
    }

    fn execute(&self) -> Result<T, Box<Error>> {
        if self.strategy.allow_request() {
            self.command.execute()
        }

        else {
            Err(Box::new(CircutBreakerError::CircuitOpen))
        }
    }
}
