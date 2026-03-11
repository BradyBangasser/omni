pub mod context;
pub mod http;
pub mod route;

pub enum Types {
    OmniContext,
    OmniRequest,
    OmniResponse,
    OmniHeader,
    OmniBody,
}
