use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:28"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:28"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = u32;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:28"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:28"]
pub mod gssapi_h {
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]#[derive(Copy, Clone)]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID_desc = gss_OID_desc_struct;
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    
    #[repr(C)]
    #[c2rust::src_loc = "112:16"]#[derive(Copy, Clone)]
    pub struct gss_OID_set_desc_struct {
        pub count: size_t,
        pub elements: gss_OID,
    }
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set_desc = gss_OID_set_desc_struct;
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set = *mut gss_OID_set_desc_struct;
    
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]#[derive(Copy, Clone)]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    #[c2rust::src_loc = "850:1"]
    pub type gss_const_OID = *const gss_OID_desc;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/string.h:28"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: usize) -> i32;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:28"]
pub mod gssapiP_generic_h {
    use super::gssapi_h::{gss_buffer_desc_struct, gss_buffer_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "152:1"]
        pub fn gssint_g_make_string_buffer(str: *const i8,
                                           buffer: gss_buffer_t)
         -> i32;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
pub use self::stddef_h::size_t;
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_uint32, OM_uint32, gss_OID_desc_struct,
                         gss_OID_desc, gss_OID, gss_OID_set_desc_struct,
                         gss_OID_set_desc, gss_OID_set,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_const_OID};
use self::string_h::memcmp;
use self::gssapiP_generic_h::gssint_g_make_string_buffer;

#[repr(C)]
#[c2rust::src_loc = "242:15"]#[derive(Copy, Clone)]
pub struct mech_attr_info_desc {
    pub mech_attr: gss_OID,
    pub name: *const i8,
    pub short_desc: *const i8,
    pub long_desc: *const i8,
}
#[c2rust::src_loc = "38:27"]
static mut const_oids: [gss_OID_desc; 40] =
    [{
         let mut init =
             gss_OID_desc_struct{length: 10 as i32 as OM_uint32,
                                 elements:
                                     b"*\x86H\x86\xf7\x12\x01\x02\x01\x01\x00"
                                         as *const u8 as *const i8
                                         as *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 10 as i32 as OM_uint32,
                                 elements:
                                     b"*\x86H\x86\xf7\x12\x01\x02\x01\x02\x00"
                                         as *const u8 as *const i8
                                         as *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 10 as i32 as OM_uint32,
                                 elements:
                                     b"*\x86H\x86\xf7\x12\x01\x02\x01\x03\x00"
                                         as *const u8 as *const i8
                                         as *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 6 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x06\x02\x00" as *const u8
                                         as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 10 as i32 as OM_uint32,
                                 elements:
                                     b"*\x86H\x86\xf7\x12\x01\x02\x01\x04\x00"
                                         as *const u8 as *const i8
                                         as *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 6 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x06\x03\x00" as *const u8
                                         as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 6 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x06\x04\x00" as *const u8
                                         as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 6 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x06\x06\x00" as *const u8
                                         as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 11 as i32 as OM_uint32,
                                 elements:
                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x05\x00"
                                         as *const u8 as *const i8
                                         as *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 11 as i32 as OM_uint32,
                                 elements:
                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x10\x00"
                                         as *const u8 as *const i8
                                         as *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 11 as i32 as OM_uint32,
                                 elements:
                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x11\x00"
                                         as *const u8 as *const i8
                                         as *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x01\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x02\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x03\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x04\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x05\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x06\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x07\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x08\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\t\x00" as *const u8
                                         as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\n\x00" as *const u8
                                         as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x0b\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x0c\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\r\x00" as *const u8
                                         as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x0e\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x0f\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x10\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x11\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x12\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x13\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x14\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x15\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x16\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x17\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x18\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x19\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x1a\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 7 as i32 as OM_uint32,
                                 elements:
                                     b"+\x06\x01\x05\x05\r\x1b\x00" as
                                         *const u8 as *const i8 as
                                         *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 11 as i32 as OM_uint32,
                                 elements:
                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x12\x00"
                                         as *const u8 as *const i8
                                         as *mut libc::c_void,};
         init
     },
     {
         let mut init =
             gss_OID_desc_struct{length: 11 as i32 as OM_uint32,
                                 elements:
                                     b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x0f\x00"
                                         as *const u8 as *const i8
                                         as *mut libc::c_void,};
         init
     }];
/* Here are the constants which point to the static structure above.
 *
 * Constants of the form GSS_C_NT_* are specified by rfc 2744.
 *
 * Constants of the form gss_nt_* are the original MIT krb5 names
 * found in gssapi_generic.h.  They are provided for compatibility. */
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "182:20"]
pub static mut GSS_C_NT_USER_NAME: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "183:20"]
pub static mut gss_nt_user_name: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "185:20"]
pub static mut GSS_C_NT_MACHINE_UID_NAME: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "186:20"]
pub static mut gss_nt_machine_uid_name: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "188:20"]
pub static mut GSS_C_NT_STRING_UID_NAME: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "189:20"]
pub static mut gss_nt_string_uid_name: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "191:20"]
pub static mut GSS_C_NT_HOSTBASED_SERVICE_X: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "192:9"]
pub static mut gss_nt_service_name_v2: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "194:20"]
pub static mut GSS_C_NT_HOSTBASED_SERVICE: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "195:20"]
pub static mut gss_nt_service_name: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "197:20"]
pub static mut GSS_C_NT_ANONYMOUS: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "199:20"]
pub static mut GSS_C_NT_EXPORT_NAME: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "200:9"]
pub static mut gss_nt_exported_name: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "202:20"]
pub static mut GSS_C_NT_COMPOSITE_EXPORT: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "203:20"]
pub static mut GSS_C_INQ_SSPI_SESSION_KEY: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "204:20"]
pub static mut GSS_C_INQ_NEGOEX_KEY: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "205:20"]
pub static mut GSS_C_INQ_NEGOEX_VERIFY_KEY: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "207:26"]
pub static mut GSS_C_MA_MECH_CONCRETE: gss_const_OID =
    0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "208:26"]
pub static mut GSS_C_MA_MECH_PSEUDO: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "209:26"]
pub static mut GSS_C_MA_MECH_COMPOSITE: gss_const_OID =
    0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "210:26"]
pub static mut GSS_C_MA_MECH_NEGO: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "211:26"]
pub static mut GSS_C_MA_MECH_GLUE: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "212:26"]
pub static mut GSS_C_MA_NOT_MECH: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "213:26"]
pub static mut GSS_C_MA_DEPRECATED: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "214:26"]
pub static mut GSS_C_MA_NOT_DFLT_MECH: gss_const_OID =
    0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "215:26"]
pub static mut GSS_C_MA_ITOK_FRAMED: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "216:26"]
pub static mut GSS_C_MA_AUTH_INIT: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "217:26"]
pub static mut GSS_C_MA_AUTH_TARG: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "218:26"]
pub static mut GSS_C_MA_AUTH_INIT_INIT: gss_const_OID =
    0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "219:26"]
pub static mut GSS_C_MA_AUTH_TARG_INIT: gss_const_OID =
    0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "220:26"]
pub static mut GSS_C_MA_AUTH_INIT_ANON: gss_const_OID =
    0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "221:26"]
pub static mut GSS_C_MA_AUTH_TARG_ANON: gss_const_OID =
    0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "222:26"]
pub static mut GSS_C_MA_DELEG_CRED: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "223:26"]
pub static mut GSS_C_MA_INTEG_PROT: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "224:26"]
pub static mut GSS_C_MA_CONF_PROT: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "225:26"]
pub static mut GSS_C_MA_MIC: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "226:26"]
pub static mut GSS_C_MA_WRAP: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "227:26"]
pub static mut GSS_C_MA_PROT_READY: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "228:26"]
pub static mut GSS_C_MA_REPLAY_DET: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "229:26"]
pub static mut GSS_C_MA_OOS_DET: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "230:26"]
pub static mut GSS_C_MA_CBINDINGS: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "231:26"]
pub static mut GSS_C_MA_PFS: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "232:26"]
pub static mut GSS_C_MA_COMPRESS: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "233:26"]
pub static mut GSS_C_MA_CTX_TRANS: gss_const_OID = 0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "234:26"]
pub static mut GSS_C_MA_NEGOEX_AND_SPNEGO: gss_const_OID =
    0 as *const gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "236:20"]
pub static mut GSS_C_SEC_CONTEXT_SASL_SSF: gss_OID =
    0 as *const gss_OID_desc_struct as *mut gss_OID_desc_struct;
// Initialized in run_static_initializers
#[c2rust::src_loc = "238:25"]
static mut gss_ma_known_attrs_desc: gss_OID_set_desc =
    gss_OID_set_desc{count: 0,
                     elements:
                         0 as *const gss_OID_desc_struct as
                             *mut gss_OID_desc_struct,};
#[no_mangle]
#[c2rust::src_loc = "240:13"]
pub static mut gss_ma_known_attrs: gss_OID_set =
    unsafe {
        &gss_ma_known_attrs_desc as *const gss_OID_set_desc as
            *mut gss_OID_set_desc
    };
// Initialized in run_static_initializers
#[c2rust::src_loc = "247:3"]
static mut mech_attr_info: [mech_attr_info_desc; 28] =
    [mech_attr_info_desc{mech_attr: 0 as *mut gss_OID_desc_struct,
                         name: 0 as *const i8,
                         short_desc: 0 as *const i8,
                         long_desc: 0 as *const i8,}; 28];
/* -*- mode: c; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 by OpenVision Technologies, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appears in all copies and
 * that both that copyright notice and this permission notice appear in
 * supporting documentation, and that the name of OpenVision not be used
 * in advertising or publicity pertaining to distribution of the software
 * without specific, written prior permission. OpenVision makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 * OPENVISION DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL OPENVISION BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF
 * USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
 * OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
/*
 * $Id$
 */
/* * helper macros **/
/* this code knows that an int on the wire is 32 bits.  The type of
   num should be at least this big, or the extra shifts may do weird
   things */
/* * malloc wrappers; these may actually do something later */
/* * helper functions **/
/* hide names from applications, especially glib applications */
/* flags for g_verify_token_header() */
/* * declarations of internal name mechanism functions **/
/* minor_status */
/* buffer */
/* minor_status */
/* set */
/* minor_status */
/* set */
/* minor_status */
/* oid */
/* new_oid */
/* minor_status */
/* oid_set */
/* minor_status */
/* member_oid */
/* oid_set */
/* minor_status */
/* member */
/* set */
/* present */
/* minor_status */
/* oid */
/* oid_str */
/* minor_status */
/* oid_str */
/* oid */
/* minor_status */
/* prefix */
/* prefix_len */
/* suffix */
/* oid */
/* minor_status */
/*prefix */
/* prefix_len */
/* oid */
/* suffix */
/*
 * Transfer contents of a k5buf to a gss_buffer and invalidate the source
 * On unix, this is a simple pointer copy
 * On windows, memory is reallocated and copied.
 */
/*minor_status*/
/*buffer_set*/
/*minor_status*/
/*member_buffer*/
/*buffer_set*/
/*minor_status*/
/*buffer_set*/
/* minor_status */
/*oidset*/
/*new_oidset*/
#[no_mangle]
#[c2rust::src_loc = "421:1"]
pub unsafe extern "C" fn generic_gss_display_mech_attr(mut minor_status:
                                                           *mut OM_uint32,
                                                       mut mech_attr:
                                                           gss_const_OID,
                                                       mut name: gss_buffer_t,
                                                       mut short_desc:
                                                           gss_buffer_t,
                                                       mut long_desc:
                                                           gss_buffer_t)
 -> OM_uint32 {
    let mut i: size_t = 0;
    if !minor_status.is_null() {
        *minor_status = 0 as i32 as OM_uint32
    }
    if !name.is_null() {
        (*name).length = 0 as i32 as size_t;
        (*name).value = 0 as *mut libc::c_void
    }
    if !short_desc.is_null() {
        (*short_desc).length = 0 as i32 as size_t;
        (*short_desc).value = 0 as *mut libc::c_void
    }
    if !long_desc.is_null() {
        (*long_desc).length = 0 as i32 as size_t;
        (*long_desc).value = 0 as *mut libc::c_void
    }
    if minor_status.is_null() {
        return (2 as usize as OM_uint32) << 24 as i32
    }
    i = 0 as i32 as size_t;
    while i <
              (::std::mem::size_of::<[mech_attr_info_desc; 28]>() as
                   usize).wrapping_div(::std::mem::size_of::<mech_attr_info_desc>()
                                                   as usize) {
        let mut mai: *mut mech_attr_info_desc =
            &mut *mech_attr_info.as_mut_ptr().offset(i as isize) as
                *mut mech_attr_info_desc;
        if (*mech_attr).length == (*(*mai).mech_attr).length &&
               memcmp((*mech_attr).elements, (*(*mai).mech_attr).elements,
                      (*mech_attr).length as usize) ==
                   0 as i32 {
            if !name.is_null() &&
                   gssint_g_make_string_buffer((*mai).name, name) == 0 {
                *minor_status = 12 as i32 as OM_uint32;
                return (13 as usize as OM_uint32) << 16 as i32
            }
            if !short_desc.is_null() &&
                   gssint_g_make_string_buffer((*mai).short_desc, short_desc)
                       == 0 {
                *minor_status = 12 as i32 as OM_uint32;
                return (13 as usize as OM_uint32) << 16 as i32
            }
            if !long_desc.is_null() &&
                   gssint_g_make_string_buffer((*mai).long_desc, long_desc) ==
                       0 {
                *minor_status = 12 as i32 as OM_uint32;
                return (13 as usize as OM_uint32) << 16 as i32
            }
            return 0 as i32 as OM_uint32
        }
        i = i.wrapping_add(1)
    }
    return (19 as usize as OM_uint32) << 16 as i32;
}
// Initialized in run_static_initializers
#[c2rust::src_loc = "473:24"]
static mut const_attrs: [gss_buffer_desc; 1] =
    [gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,}; 1];
// Initialized in run_static_initializers
#[no_mangle]
#[c2rust::src_loc = "478:25"]
pub static mut GSS_C_ATTR_LOCAL_LOGIN_USER: gss_buffer_t =
    0 as *const gss_buffer_desc_struct as *mut gss_buffer_desc_struct;
unsafe extern "C" fn run_static_initializers() {
    GSS_C_NT_USER_NAME =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(0 as i32 as isize);
    gss_nt_user_name =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(0 as i32 as isize);
    GSS_C_NT_MACHINE_UID_NAME =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(1 as i32 as isize);
    gss_nt_machine_uid_name =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(1 as i32 as isize);
    GSS_C_NT_STRING_UID_NAME =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(2 as i32 as isize);
    gss_nt_string_uid_name =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(2 as i32 as isize);
    GSS_C_NT_HOSTBASED_SERVICE_X =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(3 as i32 as isize);
    gss_nt_service_name_v2 =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(3 as i32 as isize);
    GSS_C_NT_HOSTBASED_SERVICE =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(4 as i32 as isize);
    gss_nt_service_name =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(4 as i32 as isize);
    GSS_C_NT_ANONYMOUS =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(5 as i32 as isize);
    GSS_C_NT_EXPORT_NAME =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(6 as i32 as isize);
    gss_nt_exported_name =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(6 as i32 as isize);
    GSS_C_NT_COMPOSITE_EXPORT =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(7 as i32 as isize);
    GSS_C_INQ_SSPI_SESSION_KEY =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(8 as i32 as isize);
    GSS_C_INQ_NEGOEX_KEY =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(9 as i32 as isize);
    GSS_C_INQ_NEGOEX_VERIFY_KEY =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(10 as i32 as isize);
    GSS_C_MA_MECH_CONCRETE =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(11 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_MECH_PSEUDO =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(12 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_MECH_COMPOSITE =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(13 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_MECH_NEGO =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(14 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_MECH_GLUE =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(15 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_NOT_MECH =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(16 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_DEPRECATED =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(17 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_NOT_DFLT_MECH =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(18 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_ITOK_FRAMED =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(19 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_AUTH_INIT =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(20 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_AUTH_TARG =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(21 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_AUTH_INIT_INIT =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(22 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_AUTH_TARG_INIT =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(23 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_AUTH_INIT_ANON =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(24 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_AUTH_TARG_ANON =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(25 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_DELEG_CRED =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(26 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_INTEG_PROT =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(27 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_CONF_PROT =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(28 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_MIC =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(29 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_WRAP =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(30 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_PROT_READY =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(31 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_REPLAY_DET =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(32 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_OOS_DET =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(33 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_CBINDINGS =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(34 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_PFS =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(35 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_COMPRESS =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(36 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_CTX_TRANS =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(37 as i32 as isize) as
            gss_const_OID;
    GSS_C_MA_NEGOEX_AND_SPNEGO =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(38 as i32 as isize) as
            gss_const_OID;
    GSS_C_SEC_CONTEXT_SASL_SSF =
        (const_oids.as_ptr() as
             *mut gss_OID_desc).offset(39 as i32 as isize);
    gss_ma_known_attrs_desc =
        {
            let mut init =
                gss_OID_set_desc_struct{count: 28 as i32 as size_t,
                                        elements:
                                            (const_oids.as_ptr() as
                                                 *mut gss_OID_desc).offset(11
                                                                               as
                                                                               i32
                                                                               as
                                                                               isize),};
            init
        };
    mech_attr_info =
        [{
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(11 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_MECH_CONCRETE\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"concrete-mech\x00" as *const u8 as
                                             *const i8,
                                     long_desc:
                                         b"Mechanism is neither a pseudo-mechanism nor a composite mechanism.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(12 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_MECH_PSEUDO\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"pseudo-mech\x00" as *const u8 as
                                             *const i8,
                                     long_desc:
                                         b"Mechanism is a pseudo-mechanism.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(13 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_MECH_COMPOSITE\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"composite-mech\x00" as *const u8 as
                                             *const i8,
                                     long_desc:
                                         b"Mechanism is a composite of other mechanisms.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(14 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_MECH_NEGO\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"mech-negotiation-mech\x00" as
                                             *const u8 as *const i8,
                                     long_desc:
                                         b"Mechanism negotiates other mechanisms.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(15 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_MECH_GLUE\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"mech-glue\x00" as *const u8 as
                                             *const i8,
                                     long_desc:
                                         b"OID is not a mechanism but the GSS-API itself.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(16 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_NOT_MECH\x00" as *const u8
                                             as *const i8,
                                     short_desc:
                                         b"not-mech\x00" as *const u8 as
                                             *const i8,
                                     long_desc:
                                         b"Known OID but not a mechanism OID.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(17 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_DEPRECATED\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"mech-deprecated\x00" as *const u8
                                             as *const i8,
                                     long_desc:
                                         b"Mechanism is deprecated.\x00" as
                                             *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(18 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_NOT_DFLT_MECH\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"mech-not-default\x00" as *const u8
                                             as *const i8,
                                     long_desc:
                                         b"Mechanism must not be used as a default mechanism.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(19 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_ITOK_FRAMED\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"initial-is-framed\x00" as *const u8
                                             as *const i8,
                                     long_desc:
                                         b"Mechanism\'s initial contexts are properly framed.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(20 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_AUTH_INIT\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"auth-init-princ\x00" as *const u8
                                             as *const i8,
                                     long_desc:
                                         b"Mechanism supports authentication of initiator to acceptor.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(21 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_AUTH_TARG\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"auth-targ-princ\x00" as *const u8
                                             as *const i8,
                                     long_desc:
                                         b"Mechanism supports authentication of acceptor to initiator.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(22 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_AUTH_INIT_INIT\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"auth-init-princ-initial\x00" as
                                             *const u8 as *const i8,
                                     long_desc:
                                         b"Mechanism supports authentication of initiator using initial credentials.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(23 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_AUTH_TARG_INIT\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"auth-target-princ-initial\x00" as
                                             *const u8 as *const i8,
                                     long_desc:
                                         b"Mechanism supports authentication of acceptor using initial credentials.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(24 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_AUTH_INIT_ANON\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"auth-init-princ-anon\x00" as
                                             *const u8 as *const i8,
                                     long_desc:
                                         b"Mechanism supports GSS_C_NT_ANONYMOUS as an initiator name.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(25 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_AUTH_TARG_ANON\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"auth-targ-princ-anon\x00" as
                                             *const u8 as *const i8,
                                     long_desc:
                                         b"Mechanism supports GSS_C_NT_ANONYMOUS as an acceptor name.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(26 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_DELEG_CRED\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"deleg-cred\x00" as *const u8 as
                                             *const i8,
                                     long_desc:
                                         b"Mechanism supports credential delegation.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(27 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_INTEG_PROT\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"integ-prot\x00" as *const u8 as
                                             *const i8,
                                     long_desc:
                                         b"Mechanism supports per-message integrity protection.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(28 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_CONF_PROT\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"conf-prot\x00" as *const u8 as
                                             *const i8,
                                     long_desc:
                                         b"Mechanism supports per-message confidentiality protection.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(29 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_MIC\x00" as *const u8 as
                                             *const i8,
                                     short_desc:
                                         b"mic\x00" as *const u8 as
                                             *const i8,
                                     long_desc:
                                         b"Mechanism supports Message Integrity Code (MIC) tokens.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(30 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_WRAP\x00" as *const u8 as
                                             *const i8,
                                     short_desc:
                                         b"wrap\x00" as *const u8 as
                                             *const i8,
                                     long_desc:
                                         b"Mechanism supports wrap tokens.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(31 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_PROT_READY\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"prot-ready\x00" as *const u8 as
                                             *const i8,
                                     long_desc:
                                         b"Mechanism supports per-message proteciton prior to full context establishment.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(32 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_REPLAY_DET\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"replay-detection\x00" as *const u8
                                             as *const i8,
                                     long_desc:
                                         b"Mechanism supports replay detection.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(33 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_OOS_DET\x00" as *const u8
                                             as *const i8,
                                     short_desc:
                                         b"oos-detection\x00" as *const u8 as
                                             *const i8,
                                     long_desc:
                                         b"Mechanism supports out-of-sequence detection.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(34 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_CBINDINGS\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"channel-bindings\x00" as *const u8
                                             as *const i8,
                                     long_desc:
                                         b"Mechanism supports channel bindings.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(35 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_PFS\x00" as *const u8 as
                                             *const i8,
                                     short_desc:
                                         b"pfs\x00" as *const u8 as
                                             *const i8,
                                     long_desc:
                                         b"Mechanism supports Perfect Forward Security.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(36 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_COMPRESS\x00" as *const u8
                                             as *const i8,
                                     short_desc:
                                         b"compress\x00" as *const u8 as
                                             *const i8,
                                     long_desc:
                                         b"Mechanism supports compression of data inputs to gss_wrap().\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(37 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_CTX_TRANS\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"context-transfer\x00" as *const u8
                                             as *const i8,
                                     long_desc:
                                         b"Mechanism supports security context export/import.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         },
         {
             let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut gss_OID_desc).offset(38 as
                                                                            i32
                                                                            as
                                                                            isize),
                                     name:
                                         b"GSS_C_MA_NEGOEX_AND_SPNEGO\x00" as
                                             *const u8 as *const i8,
                                     short_desc:
                                         b"negoex-only\x00" as *const u8 as
                                             *const i8,
                                     long_desc:
                                         b"NegoEx mechanism should also be negotiable through SPNEGO.\x00"
                                             as *const u8 as
                                             *const i8,};
             init
         }];
    const_attrs =
        [{
             let mut init =
                 gss_buffer_desc_struct{length:
                                            (::std::mem::size_of::<[i8; 17]>()
                                                 as
                                                 usize).wrapping_sub(1
                                                                                 as
                                                                                 i32
                                                                                 as
                                                                                 usize),
                                        value:
                                            b"local-login-user\x00" as
                                                *const u8 as
                                                *const i8 as
                                                *mut libc::c_void,};
             init
         }];
    GSS_C_ATTR_LOCAL_LOGIN_USER =
        &mut *const_attrs.as_mut_ptr().offset(0 as i32 as isize) as
            *mut gss_buffer_desc
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
