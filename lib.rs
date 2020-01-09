#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(register_tool)]
#![register_tool(c2rust)]

extern crate libc;

pub mod src {
    pub mod disp_com_err_status;
    pub mod disp_major_status;
    pub mod gssapi_err_generic;
    pub mod gssapi_generic;
    pub mod oid_ops;
    pub mod rel_buffer;
    pub mod rel_oid_set;
    pub mod util_buffer;
    pub mod util_buffer_set;
    pub mod util_errmap;
    pub mod util_seqstate;
    pub mod util_set;
    pub mod util_token;
} // mod src
