use ::libc;

/* _GSSAPIP_GENERIC_H_ */
pub use crate::gssapi_h::gss_OID;
pub use crate::gssapi_h::gss_OID_desc;
pub use crate::gssapi_h::gss_OID_desc_struct;
pub use crate::gssapi_h::gss_OID_set;
pub use crate::gssapi_h::gss_OID_set_desc;
pub use crate::gssapi_h::gss_OID_set_desc_struct;
pub use crate::gssapi_h::gss_buffer_desc;
pub use crate::gssapi_h::gss_buffer_desc_struct;
pub use crate::gssapi_h::gss_buffer_t;
pub use crate::gssapi_h::gss_const_OID;
pub use crate::gssapi_h::gss_uint32;
pub use crate::gssapi_h::OM_uint32;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__uint32_t;

pub use crate::stdlib::uint32_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mech_attr_info_desc {
    pub mech_attr: crate::gssapi_h::gss_OID,
    pub name: *const i8,
    pub short_desc: *const i8,
    pub long_desc: *const i8,
}

static mut const_oids: [crate::gssapi_h::gss_OID_desc; 40] = [
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 10u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x01\x01\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 10u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x01\x02\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 10u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x01\x03\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 6u32,
            elements: b"+\x06\x01\x05\x06\x02\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 10u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x01\x04\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 6u32,
            elements: b"+\x06\x01\x05\x06\x03\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 6u32,
            elements: b"+\x06\x01\x05\x06\x04\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 6u32,
            elements: b"+\x06\x01\x05\x06\x06\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 11u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x05\x00" as *const u8
                as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 11u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x10\x00" as *const u8
                as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 11u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x11\x00" as *const u8
                as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x01\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x02\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x03\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x04\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x05\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x06\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x07\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x08\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\t\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\n\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x0b\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x0c\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\r\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x0e\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x0f\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x10\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x11\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x12\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x13\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x14\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x15\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x16\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x17\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x18\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x19\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x1a\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 7u32,
            elements: b"+\x06\x01\x05\x05\r\x1b\x00" as *const u8 as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 11u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x12\x00" as *const u8
                as *mut libc::c_void,
        };
        init
    },
    {
        let mut init = crate::gssapi_h::gss_OID_desc_struct {
            length: 11u32,
            elements: b"*\x86H\x86\xf7\x12\x01\x02\x02\x05\x0f\x00" as *const u8
                as *mut libc::c_void,
        };
        init
    },
];
/* Here are the constants which point to the static structure above.
 *
 * Constants of the form GSS_C_NT_* are specified by rfc 2744.
 *
 * Constants of the form gss_nt_* are the original MIT krb5 names
 * found in gssapi_generic.h.  They are provided for compatibility. */
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_NT_USER_NAME: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut gss_nt_user_name: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_NT_MACHINE_UID_NAME: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut gss_nt_machine_uid_name: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_NT_STRING_UID_NAME: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut gss_nt_string_uid_name: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_NT_HOSTBASED_SERVICE_X: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut gss_nt_service_name_v2: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_NT_HOSTBASED_SERVICE: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut gss_nt_service_name: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_NT_ANONYMOUS: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_NT_EXPORT_NAME: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut gss_nt_exported_name: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_NT_COMPOSITE_EXPORT: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_INQ_SSPI_SESSION_KEY: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_INQ_NEGOEX_KEY: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_INQ_NEGOEX_VERIFY_KEY: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_MECH_CONCRETE: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_MECH_PSEUDO: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_MECH_COMPOSITE: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_MECH_NEGO: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_MECH_GLUE: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_NOT_MECH: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_DEPRECATED: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_NOT_DFLT_MECH: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_ITOK_FRAMED: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_AUTH_INIT: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_AUTH_TARG: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_AUTH_INIT_INIT: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_AUTH_TARG_INIT: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_AUTH_INIT_ANON: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_AUTH_TARG_ANON: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_DELEG_CRED: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_INTEG_PROT: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_CONF_PROT: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_MIC: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_WRAP: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_PROT_READY: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_REPLAY_DET: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_OOS_DET: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_CBINDINGS: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_PFS: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_COMPRESS: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_CTX_TRANS: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_MA_NEGOEX_AND_SPNEGO: crate::gssapi_h::gss_const_OID =
    0 as *const crate::gssapi_h::gss_OID_desc;
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_SEC_CONTEXT_SASL_SSF: crate::gssapi_h::gss_OID =
    0 as *mut crate::gssapi_h::gss_OID_desc_struct;
// Initialized in run_static_initializers

static mut gss_ma_known_attrs_desc: crate::gssapi_h::gss_OID_set_desc =
    crate::gssapi_h::gss_OID_set_desc {
        count: 0,
        elements: 0 as *mut crate::gssapi_h::gss_OID_desc_struct,
    };
#[no_mangle]

pub static mut gss_ma_known_attrs: crate::gssapi_h::gss_OID_set = unsafe {
    &gss_ma_known_attrs_desc as *const crate::gssapi_h::gss_OID_set_desc
        as *mut crate::gssapi_h::gss_OID_set_desc
};
// Initialized in run_static_initializers

static mut mech_attr_info: [mech_attr_info_desc; 28] = [mech_attr_info_desc {
    mech_attr: 0 as *mut crate::gssapi_h::gss_OID_desc_struct,
    name: 0 as *const i8,
    short_desc: 0 as *const i8,
    long_desc: 0 as *const i8,
}; 28];
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

pub unsafe extern "C" fn generic_gss_display_mech_attr(
    mut minor_status: *mut crate::gssapi_h::OM_uint32,
    mut mech_attr: crate::gssapi_h::gss_const_OID,
    mut name: crate::gssapi_h::gss_buffer_t,
    mut short_desc: crate::gssapi_h::gss_buffer_t,
    mut long_desc: crate::gssapi_h::gss_buffer_t,
) -> crate::gssapi_h::OM_uint32 {
    let mut i: crate::stddef_h::size_t = 0;
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !name.is_null() {
        (*name).length = 0usize;
        (*name).value = 0 as *mut libc::c_void
    }
    if !short_desc.is_null() {
        (*short_desc).length = 0usize;
        (*short_desc).value = 0 as *mut libc::c_void
    }
    if !long_desc.is_null() {
        (*long_desc).length = 0usize;
        (*long_desc).value = 0 as *mut libc::c_void
    }
    if minor_status.is_null() {
        return (2u32) << 24i32;
    }
    i = 0usize;
    while i
        < (::std::mem::size_of::<[mech_attr_info_desc; 28]>())
            .wrapping_div(::std::mem::size_of::<mech_attr_info_desc>())
    {
        let mut mai: *mut mech_attr_info_desc =
            &mut *mech_attr_info.as_mut_ptr().offset(i as isize) as *mut mech_attr_info_desc;
        if (*mech_attr).length == (*(*mai).mech_attr).length
            && crate::stdlib::memcmp(
                (*mech_attr).elements,
                (*(*mai).mech_attr).elements,
                (*mech_attr).length as usize,
            ) == 0i32
        {
            if !name.is_null()
                && crate::src::generic::util_buffer::gssint_g_make_string_buffer((*mai).name, name)
                    == 0
            {
                *minor_status = 12u32;
                return (13u32) << 16i32;
            }
            if !short_desc.is_null()
                && crate::src::generic::util_buffer::gssint_g_make_string_buffer(
                    (*mai).short_desc,
                    short_desc,
                ) == 0
            {
                *minor_status = 12u32;
                return (13u32) << 16i32;
            }
            if !long_desc.is_null()
                && crate::src::generic::util_buffer::gssint_g_make_string_buffer(
                    (*mai).long_desc,
                    long_desc,
                ) == 0
            {
                *minor_status = 12u32;
                return (13u32) << 16i32;
            }
            return 0u32;
        }
        i = i.wrapping_add(1)
    }
    return (19u32) << 16i32;
}
// Initialized in run_static_initializers

static mut const_attrs: [crate::gssapi_h::gss_buffer_desc; 1] = [crate::gssapi_h::gss_buffer_desc {
    length: 0,
    value: 0 as *mut libc::c_void,
}; 1];
// Initialized in run_static_initializers
#[no_mangle]

pub static mut GSS_C_ATTR_LOCAL_LOGIN_USER: crate::gssapi_h::gss_buffer_t =
    0 as *mut crate::gssapi_h::gss_buffer_desc_struct;
unsafe extern "C" fn run_static_initializers() {
    GSS_C_NT_USER_NAME = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(0isize);
    gss_nt_user_name = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(0isize);
    GSS_C_NT_MACHINE_UID_NAME =
        (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(1isize);
    gss_nt_machine_uid_name =
        (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(1isize);
    GSS_C_NT_STRING_UID_NAME =
        (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(2isize);
    gss_nt_string_uid_name =
        (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(2isize);
    GSS_C_NT_HOSTBASED_SERVICE_X =
        (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(3isize);
    gss_nt_service_name_v2 =
        (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(3isize);
    GSS_C_NT_HOSTBASED_SERVICE =
        (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(4isize);
    gss_nt_service_name =
        (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(4isize);
    GSS_C_NT_ANONYMOUS = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(5isize);
    GSS_C_NT_EXPORT_NAME =
        (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(6isize);
    gss_nt_exported_name =
        (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(6isize);
    GSS_C_NT_COMPOSITE_EXPORT =
        (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(7isize);
    GSS_C_INQ_SSPI_SESSION_KEY =
        (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(8isize);
    GSS_C_INQ_NEGOEX_KEY =
        (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(9isize);
    GSS_C_INQ_NEGOEX_VERIFY_KEY =
        (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(10isize);
    GSS_C_MA_MECH_CONCRETE = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
        .offset(11isize) as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_MECH_PSEUDO = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
        .offset(12isize) as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_MECH_COMPOSITE = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
        .offset(13isize) as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_MECH_NEGO = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(14isize)
        as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_MECH_GLUE = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(15isize)
        as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_NOT_MECH = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(16isize)
        as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_DEPRECATED = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
        .offset(17isize) as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_NOT_DFLT_MECH = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
        .offset(18isize) as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_ITOK_FRAMED = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
        .offset(19isize) as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_AUTH_INIT = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(20isize)
        as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_AUTH_TARG = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(21isize)
        as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_AUTH_INIT_INIT = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
        .offset(22isize) as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_AUTH_TARG_INIT = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
        .offset(23isize) as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_AUTH_INIT_ANON = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
        .offset(24isize) as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_AUTH_TARG_ANON = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
        .offset(25isize) as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_DELEG_CRED = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
        .offset(26isize) as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_INTEG_PROT = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
        .offset(27isize) as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_CONF_PROT = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(28isize)
        as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_MIC = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(29isize)
        as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_WRAP = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(30isize)
        as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_PROT_READY = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
        .offset(31isize) as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_REPLAY_DET = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
        .offset(32isize) as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_OOS_DET = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(33isize)
        as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_CBINDINGS = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(34isize)
        as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_PFS = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(35isize)
        as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_COMPRESS = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(36isize)
        as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_CTX_TRANS = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(37isize)
        as crate::gssapi_h::gss_const_OID;
    GSS_C_MA_NEGOEX_AND_SPNEGO = (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
        .offset(38isize) as crate::gssapi_h::gss_const_OID;
    GSS_C_SEC_CONTEXT_SASL_SSF =
        (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(39isize);
    gss_ma_known_attrs_desc = {
        let mut init = crate::gssapi_h::gss_OID_set_desc_struct {
            count: 28usize,
            elements: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc).offset(11isize),
        };
        init
    };
    mech_attr_info = [
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(11isize),
                name: b"GSS_C_MA_MECH_CONCRETE\x00" as *const u8 as *const i8,
                short_desc: b"concrete-mech\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism is neither a pseudo-mechanism nor a composite mechanism.\x00"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(12isize),
                name: b"GSS_C_MA_MECH_PSEUDO\x00" as *const u8 as *const i8,
                short_desc: b"pseudo-mech\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism is a pseudo-mechanism.\x00" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(13isize),
                name: b"GSS_C_MA_MECH_COMPOSITE\x00" as *const u8 as *const i8,
                short_desc: b"composite-mech\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism is a composite of other mechanisms.\x00" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(14isize),
                name: b"GSS_C_MA_MECH_NEGO\x00" as *const u8 as *const i8,
                short_desc: b"mech-negotiation-mech\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism negotiates other mechanisms.\x00" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(15isize),
                name: b"GSS_C_MA_MECH_GLUE\x00" as *const u8 as *const i8,
                short_desc: b"mech-glue\x00" as *const u8 as *const i8,
                long_desc: b"OID is not a mechanism but the GSS-API itself.\x00" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(16isize),
                name: b"GSS_C_MA_NOT_MECH\x00" as *const u8 as *const i8,
                short_desc: b"not-mech\x00" as *const u8 as *const i8,
                long_desc: b"Known OID but not a mechanism OID.\x00" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(17isize),
                name: b"GSS_C_MA_DEPRECATED\x00" as *const u8 as *const i8,
                short_desc: b"mech-deprecated\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism is deprecated.\x00" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(18isize),
                name: b"GSS_C_MA_NOT_DFLT_MECH\x00" as *const u8 as *const i8,
                short_desc: b"mech-not-default\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism must not be used as a default mechanism.\x00" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(19isize),
                name: b"GSS_C_MA_ITOK_FRAMED\x00" as *const u8 as *const i8,
                short_desc: b"initial-is-framed\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism\'s initial contexts are properly framed.\x00" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(20isize),
                name: b"GSS_C_MA_AUTH_INIT\x00" as *const u8 as *const i8,
                short_desc: b"auth-init-princ\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism supports authentication of initiator to acceptor.\x00"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(21isize),
                name: b"GSS_C_MA_AUTH_TARG\x00" as *const u8 as *const i8,
                short_desc: b"auth-targ-princ\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism supports authentication of acceptor to initiator.\x00"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(22isize),
                name: b"GSS_C_MA_AUTH_INIT_INIT\x00" as *const u8 as *const i8,
                short_desc: b"auth-init-princ-initial\x00" as *const u8 as *const i8,
                long_desc:
                    b"Mechanism supports authentication of initiator using initial credentials.\x00"
                        as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(23isize),
                name: b"GSS_C_MA_AUTH_TARG_INIT\x00" as *const u8 as *const i8,
                short_desc: b"auth-target-princ-initial\x00" as *const u8 as *const i8,
                long_desc:
                    b"Mechanism supports authentication of acceptor using initial credentials.\x00"
                        as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(24isize),
                name: b"GSS_C_MA_AUTH_INIT_ANON\x00" as *const u8 as *const i8,
                short_desc: b"auth-init-princ-anon\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism supports GSS_C_NT_ANONYMOUS as an initiator name.\x00"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(25isize),
                name: b"GSS_C_MA_AUTH_TARG_ANON\x00" as *const u8 as *const i8,
                short_desc: b"auth-targ-princ-anon\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism supports GSS_C_NT_ANONYMOUS as an acceptor name.\x00"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(26isize),
                name: b"GSS_C_MA_DELEG_CRED\x00" as *const u8 as *const i8,
                short_desc: b"deleg-cred\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism supports credential delegation.\x00" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(27isize),
                name: b"GSS_C_MA_INTEG_PROT\x00" as *const u8 as *const i8,
                short_desc: b"integ-prot\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism supports per-message integrity protection.\x00" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(28isize),
                name: b"GSS_C_MA_CONF_PROT\x00" as *const u8 as *const i8,
                short_desc: b"conf-prot\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism supports per-message confidentiality protection.\x00"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(29isize),
                name: b"GSS_C_MA_MIC\x00" as *const u8 as *const i8,
                short_desc: b"mic\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism supports Message Integrity Code (MIC) tokens.\x00"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(30isize),
                name: b"GSS_C_MA_WRAP\x00" as *const u8 as *const i8,
                short_desc: b"wrap\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism supports wrap tokens.\x00" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init =
                 mech_attr_info_desc{mech_attr:
                                         (const_oids.as_ptr() as
                                              *mut crate::gssapi_h::gss_OID_desc).offset(31isize),
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
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(32isize),
                name: b"GSS_C_MA_REPLAY_DET\x00" as *const u8 as *const i8,
                short_desc: b"replay-detection\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism supports replay detection.\x00" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(33isize),
                name: b"GSS_C_MA_OOS_DET\x00" as *const u8 as *const i8,
                short_desc: b"oos-detection\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism supports out-of-sequence detection.\x00" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(34isize),
                name: b"GSS_C_MA_CBINDINGS\x00" as *const u8 as *const i8,
                short_desc: b"channel-bindings\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism supports channel bindings.\x00" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(35isize),
                name: b"GSS_C_MA_PFS\x00" as *const u8 as *const i8,
                short_desc: b"pfs\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism supports Perfect Forward Security.\x00" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(36isize),
                name: b"GSS_C_MA_COMPRESS\x00" as *const u8 as *const i8,
                short_desc: b"compress\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism supports compression of data inputs to gss_wrap().\x00"
                    as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(37isize),
                name: b"GSS_C_MA_CTX_TRANS\x00" as *const u8 as *const i8,
                short_desc: b"context-transfer\x00" as *const u8 as *const i8,
                long_desc: b"Mechanism supports security context export/import.\x00" as *const u8
                    as *const i8,
            };
            init
        },
        {
            let mut init = mech_attr_info_desc {
                mech_attr: (const_oids.as_ptr() as *mut crate::gssapi_h::gss_OID_desc)
                    .offset(38isize),
                name: b"GSS_C_MA_NEGOEX_AND_SPNEGO\x00" as *const u8 as *const i8,
                short_desc: b"negoex-only\x00" as *const u8 as *const i8,
                long_desc: b"NegoEx mechanism should also be negotiable through SPNEGO.\x00"
                    as *const u8 as *const i8,
            };
            init
        },
    ];
    const_attrs = [{
        let mut init = crate::gssapi_h::gss_buffer_desc_struct {
            length: (::std::mem::size_of::<[i8; 17]>()).wrapping_sub(1usize),
            value: b"local-login-user\x00" as *const u8 as *mut libc::c_void,
        };
        init
    }];
    GSS_C_ATTR_LOCAL_LOGIN_USER =
        &mut *const_attrs.as_mut_ptr().offset(0isize) as *mut crate::gssapi_h::gss_buffer_desc
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
