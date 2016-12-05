#![deny(missing_docs, warnings)]

//! Request logging middleware for Iron

extern crate iron;
#[macro_use]
extern crate log;
extern crate time;
#[macro_use]
extern crate error_chain;

use iron::{AfterMiddleware, IronResult, IronError, Request, Response};

use std::error::Error as StdError;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}
use errors::*;

pub mod format;

/// `Middleware` for logging request and response info to the terminal.
pub struct ErrorLogger { }

impl ErrorLogger {
    /// Create a new ErrorLogger AfterMiddleware
    pub fn new() -> ErrorLogger {
        ErrorLogger {}
    }
}

impl ErrorLogger {
    fn log_err(&self, err: &IronError) -> IronResult<()> {
        if let Some(ref err) = err.error.downcast::<Error>() {
            // Error logging from brson's error chain template
            // http://brson.github.io/2016/11/30/starting-with-error-chain
            error!("{}", err);

            for e in err.iter().skip(1) {
                error!("caused by: {}", e);
            }

            // The backtrace is not always generated. Try to run this example
            // with `RUST_BACKTRACE=1`.
            if let Some(backtrace) = err.backtrace() {
                error!("backtrace: {:?}", backtrace);
            }
        } else {
            warn!("{}", err.description());
            if let Some(cause) = err.cause() {
                warn!("{:?}", cause);
            }
        }

        Ok(())
    }
}

impl AfterMiddleware for ErrorLogger {
    fn after(&self, _: &mut Request, res: Response) -> IronResult<Response> {
        Ok(res)
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        try!(self.log_err(&err));
        Err(err)
    }
}
