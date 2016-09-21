pub mod strategy;

use strategy::{BoxedStrategy, Strategy};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt::Error as FmtError;

pub trait CircuitBreaker<T> {
    fn execute(&mut self) -> CommandResult<T>;
}

pub struct DefaultCircuitBreaker<T> {
    command: BoxedCommand<T>,
    fallback: Option<BoxedCommand<T>>,
    strategy: BoxedStrategy
}

impl<T> DefaultCircuitBreaker<T> {
    pub fn new<C, F, S>(command: C, fallback: Option<F>, strategy: S) -> Self
        where C: Command<T> + 'static, F: Command<T> + 'static, S: Strategy + 'static
    {
        let fallback = if let Some(fallback) = fallback { Some(fallback.boxed()) } else { None };

        DefaultCircuitBreaker {
            command: command.boxed(),
            fallback: fallback,
            strategy: strategy.boxed()
        }
    }
}

impl<T> CircuitBreaker<T> for DefaultCircuitBreaker<T> {
    fn execute(&mut self) -> CommandResult<T> {
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

        else if self.fallback.is_some() {
            let fallback = self.fallback.as_ref().unwrap();
            fallback.execute()
        }

        else {
            Err(Box::new(CircutBreakerError::CircuitOpen))
        }
    }
}

pub type CommandResult<T> = Result<T, Box<Error>>;
pub type BoxedCommand<T> = Box<Command<T>>;

pub trait Command<T>: Send {
    fn execute(&self) -> CommandResult<T>;
    fn boxed(self) -> BoxedCommand<T> where Self: Sized + Send + 'static {
        Box::new(self)
    }
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
