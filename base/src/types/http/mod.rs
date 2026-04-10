#[derive(Clone, Debug, PartialEq, Eq, Hash, strum::EnumString)]
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
    BITFIELD(u8),
}
