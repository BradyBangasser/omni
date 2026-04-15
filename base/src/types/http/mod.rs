use bitflags::bitflags;
use std::fmt;
use std::str::FromStr;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Method: u32 {
        const GET      = 1 << 0;
        const POST     = 1 << 1;
        const PATCH    = 1 << 2;
        const PUT      = 1 << 3;
        const DELETE   = 1 << 4;
        const HEAD     = 1 << 5;
        const OPTIONS  = 1 << 6;
        const CONNECT  = 1 << 7;
        const TRACE    = 1 << 8;
    }
}

impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PATCH" => Ok(Method::PATCH),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "HEAD" => Ok(Method::HEAD),
            "OPTIONS" => Ok(Method::OPTIONS),
            "CONNECT" => Ok(Method::CONNECT),
            "TRACE" => Ok(Method::TRACE),
            _ => Err(format!("Unknown HTTP method: {}", s)),
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
