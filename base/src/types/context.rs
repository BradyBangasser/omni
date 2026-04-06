use crate::types::route::RoutePlan;

struct Header {}
struct ConnectionState {}
struct Param {}

#[repr(C)]
pub struct RequestContextCold {
    pub body_l: u32,
    pub headers_l: u16,
    pub params_l: u16,
    pub body: *const u8,
    pub headers: *const Header,
    pub params: *const Param,
}

/*
* Default cache line size is 64 bytes (512 bits)
*/
#[repr(C, align(64))]
pub struct RequestContextHot {
    pub method: u8,                     // 008
    pub flags: u8,                      // 016
    pub status: u16,                    // 032
    pub path_l: u32,                    // 064
    pub res_buf_l: u32,                 // 096
    pub stream_id: u32,                 // 128
    pub path: *const u8,                // 196
    pub res_buf: *mut u8,               // 256
    pub conn_ptr: *mut ConnectionState, // 320
    pub req: *mut RequestContextCold,   // 384
    pub _route: *const RoutePlan,       // 448
}
