use log::trace;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Method {
    GET,
    POST,
    PATCH,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
    ANY,
    MIDDLEWARE,
}

impl Method {
    pub fn parse(method: &str) -> Option<Self> {
        trace!("Parsing method: {}", method);
        match method.to_uppercase().as_str() {
            "GET" => Some(Method::GET),
            "POST" => Some(Method::POST),
            "PATCH" => Some(Method::PATCH),
            "PUT" => Some(Method::PUT),
            "DELETE" => Some(Method::DELETE),
            "HEAD" => Some(Method::HEAD),
            "OPTIONS" => Some(Method::OPTIONS),
            "CONNECT" => Some(Method::CONNECT),
            "TRACE" => Some(Method::TRACE),
            "ANY" => Some(Method::ANY),
            "MIDDLEWARE" => Some(Method::MIDDLEWARE),
            _ => None,
        }
    }
}
