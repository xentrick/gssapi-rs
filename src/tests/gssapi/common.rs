use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:33"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:8"]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    #[c2rust::src_loc = "43:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "38:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "37:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "36:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:33"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:35"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:35"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:35"]
pub mod gssapi_h {
    #[c2rust::src_loc = "79:1"]
    pub type gss_name_t = *mut gss_name_struct;
    #[c2rust::src_loc = "82:1"]
    pub type gss_cred_id_t = *mut gss_cred_id_struct;
    #[c2rust::src_loc = "85:1"]
    pub type gss_ctx_id_t = *mut gss_ctx_id_struct;
    /*
 * The following type must be defined as the smallest natural unsigned integer
 * supported by the platform that has at least 32 bits of precision.
 */
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    /* OM_STRING */
    /*
 * We can't use X/Open definitions, so roll our own.
 */
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID_desc = gss_OID_desc_struct;
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;
    /* OM_STRING */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:16"]
    pub struct gss_OID_set_desc_struct {
        pub count: size_t,
        pub elements: gss_OID,
    }
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set_desc = gss_OID_set_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    pub struct gss_buffer_desc_struct {
        pub length: size_t,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_desc = gss_buffer_desc_struct;
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "122:16"]
    pub struct gss_channel_bindings_struct {
        pub initiator_addrtype: OM_uint32,
        pub initiator_address: gss_buffer_desc,
        pub acceptor_addrtype: OM_uint32,
        pub acceptor_address: gss_buffer_desc,
        pub application_data: gss_buffer_desc,
    }
    #[c2rust::src_loc = "122:1"]
    pub type gss_channel_bindings_t = *mut gss_channel_bindings_struct;
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        /* This is the gssapi.h prologue. */
/* no xom.h */
/* End of gssapi.h prologue. */
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
 * Determine platform-dependent configuration.
 */
        /* __cplusplus */
        /*
 * First, include stddef.h to get size_t defined.
 */
        /*
 * POSIX says that sys/types.h is where size_t is defined.
 */
        /*
 * $Id$
 */
        /*
 * First, define the three platform-dependent pointer types.
 */
        #[c2rust::src_loc = "78:8"]
        pub type gss_name_struct;
        #[c2rust::src_loc = "81:8"]
        pub type gss_cred_id_struct;
        #[c2rust::src_loc = "84:8"]
        pub type gss_ctx_id_struct;
        /*
 * Flag bits for context-level services.
 */
        /*
 * Credential usage options
 */
        /*
 * Status code types for gss_display_status
 */
        /*
 * The constant definitions for channel-bindings address families
 */
        /*
 * Various Null values.
 */
        /*
 * Some alternate names for a couple of the above values.  These are defined
 * for V1 compatibility.
 */
        /*
 * Define the default Quality of Protection for per-message services.  Note
 * that an implementation that offers multiple levels of QOP may either reserve
 * a value (for example zero, as assumed here) to mean "default protection", or
 * alternatively may simply equate GSS_C_QOP_DEFAULT to a specific explicit
 * QOP value.  However a value of 0 should always be interpreted by a GSSAPI
 * implementation as a request for the default protection level.
 */
        /*
 * Expiration time of 2^32-1 seconds means infinite lifetime for a
 * credential or security context
 */
        /* Major status codes */
        /*
 * Some "helper" definitions to make the status code macros obvious.
 */
        /*
 * The macros that test status codes for error conditions.  Note that the
 * GSS_ERROR() macro has changed slightly from the V1 GSSAPI so that it now
 * evaluates its argument only once.
 */
        /*
 * Now the actual status code definitions
 */
        /*
 * Calling errors:
 */
        /*
 * Routine errors:
 */
        /*
 * Supplementary info bits:
 */
        /*
 * Finally, function prototypes for the GSSAPI routines.
 */
        /* Reserved static storage for GSS_oids.  Comments are quotes from RFC 2744.
 *
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {10, (void *)"\x2a\x86\x48\x86\xf7\x12\x01\x02\x01\x01"},
 * corresponding to an object-identifier value of
 * {iso(1) member-body(2) United States(840) mit(113554)
 * infosys(1) gssapi(2) generic(1) user_name(1)}.  The constant
 * GSS_C_NT_USER_NAME should be initialized to point
 * to that gss_OID_desc.
 */
        #[no_mangle]
        #[c2rust::src_loc = "336:27"]
        pub static mut GSS_C_NT_USER_NAME: gss_OID;
        /*
 * The implementation must reserve static storage for a
 * gss_OID_desc object containing the value
 * {10, (void *)"\x2a\x86\x48\x86\xf7\x12"
 *              "\x01\x02\x01\x04"}, corresponding to an
 * object-identifier value of {iso(1) member-body(2)
 * Unites States(840) mit(113554) infosys(1) gssapi(2)
 * generic(1) service_name(4)}.  The constant
 * GSS_C_NT_HOSTBASED_SERVICE should be initialized
 * to point to that gss_OID_desc.
 */
        #[no_mangle]
        #[c2rust::src_loc = "392:27"]
        pub static mut GSS_C_NT_HOSTBASED_SERVICE: gss_OID;
        /* time_rec */
        #[no_mangle]
        #[c2rust::src_loc = "432:1"]
        pub fn gss_release_cred(_: *mut OM_uint32, _: *mut gss_cred_id_t)
         -> OM_uint32;
        /* cred_handle */
        #[no_mangle]
        #[c2rust::src_loc = "437:1"]
        pub fn gss_init_sec_context(_: *mut OM_uint32, _: gss_cred_id_t,
                                    _: *mut gss_ctx_id_t, _: gss_name_t,
                                    _: gss_OID, _: OM_uint32, _: OM_uint32,
                                    _: gss_channel_bindings_t,
                                    _: gss_buffer_t, _: *mut gss_OID,
                                    _: gss_buffer_t, _: *mut OM_uint32,
                                    _: *mut OM_uint32) -> OM_uint32;
        /* time_rec */
        #[no_mangle]
        #[c2rust::src_loc = "453:1"]
        pub fn gss_accept_sec_context(_: *mut OM_uint32, _: *mut gss_ctx_id_t,
                                      _: gss_cred_id_t, _: gss_buffer_t,
                                      _: gss_channel_bindings_t,
                                      _: *mut gss_name_t, _: *mut gss_OID,
                                      _: gss_buffer_t, _: *mut OM_uint32,
                                      _: *mut OM_uint32,
                                      _: *mut gss_cred_id_t) -> OM_uint32;
        /* qop_state */
        #[no_mangle]
        #[c2rust::src_loc = "530:1"]
        pub fn gss_display_status(_: *mut OM_uint32, _: OM_uint32,
                                  _: libc::c_int, _: gss_OID,
                                  _: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        /* name_equal */
        #[no_mangle]
        #[c2rust::src_loc = "554:1"]
        pub fn gss_display_name(_: *mut OM_uint32, _: gss_name_t,
                                _: gss_buffer_t, _: *mut gss_OID)
         -> OM_uint32;
        /* output_name_type */
        #[no_mangle]
        #[c2rust::src_loc = "562:1"]
        pub fn gss_import_name(_: *mut OM_uint32, _: gss_buffer_t, _: gss_OID,
                               _: *mut gss_name_t) -> OM_uint32;
        /* output_name */
        #[no_mangle]
        #[c2rust::src_loc = "569:1"]
        pub fn gss_release_name(_: *mut OM_uint32, _: *mut gss_name_t)
         -> OM_uint32;
        /* input_name */
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        /* oid */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "707:1"]
        pub fn gss_oid_to_str(_: *mut OM_uint32, _: gss_OID, _: gss_buffer_t)
         -> OM_uint32;
        /* dest_name */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "784:1"]
        pub fn gss_canonicalize_name(_: *mut OM_uint32, _: gss_name_t,
                                     _: gss_OID, _: *mut gss_name_t)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_ext.h:35"]
pub mod gssapi_ext_h {
    /* acceptor_time_rec */
    /*
 * GGF extensions
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "134:16"]
    pub struct gss_buffer_set_desc_struct {
        pub count: size_t,
        pub elements: *mut gss_buffer_desc,
    }
    #[c2rust::src_loc = "134:1"]
    pub type gss_buffer_set_t = *mut gss_buffer_set_desc_struct;
    use super::stddef_h::size_t;
    use super::gssapi_h::{gss_buffer_desc, OM_uint32, gss_cred_id_struct,
                          gss_cred_id_t, gss_buffer_desc_struct, gss_buffer_t,
                          gss_name_struct, gss_name_t, gss_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "150:1"]
        pub fn gss_release_buffer_set(_: *mut OM_uint32,
                                      _: *mut gss_buffer_set_t) -> OM_uint32;
        /*
 * Export import cred extensions from GGF, but using Heimdal's signatures
 */
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn gss_export_cred(_: *mut OM_uint32, _: gss_cred_id_t,
                               _: gss_buffer_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "180:1"]
        pub fn gss_import_cred(_: *mut OM_uint32, _: gss_buffer_t,
                               _: *mut gss_cred_id_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "444:1"]
        pub fn gss_inquire_name(_: *mut OM_uint32, _: gss_name_t,
                                _: *mut libc::c_int, _: *mut gss_OID,
                                _: *mut gss_buffer_set_t) -> OM_uint32;
        #[no_mangle]
        #[c2rust::src_loc = "453:1"]
        pub fn gss_get_name_attribute(_: *mut OM_uint32, _: gss_name_t,
                                      _: gss_buffer_t, _: *mut libc::c_int,
                                      _: *mut libc::c_int, _: gss_buffer_t,
                                      _: gss_buffer_t, _: *mut libc::c_int)
         -> OM_uint32;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:35"]
pub mod krb5_h {
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    use super::stdint_intn_h::int32_t;
    extern "C" {
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
        #[no_mangle]
        #[c2rust::src_loc = "8023:1"]
        pub fn krb5_get_error_message(ctx: krb5_context,
                                      code: krb5_error_code)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "8032:1"]
        pub fn krb5_free_error_message(ctx: krb5_context,
                                       msg: *const libc::c_char);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/stdio.h:33"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:34"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:35"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_krb5.h:35"]
pub mod gssapi_krb5_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "77:33"]
        pub static GSS_KRB5_NT_ENTERPRISE_NAME: gss_OID;
        #[no_mangle]
        #[c2rust::src_loc = "40:33"]
        pub static GSS_KRB5_NT_PRINCIPAL_NAME: gss_OID;
    }
    /* _GSSAPI_KRB5_H_ */
    /* __cplusplus */
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__int32_t, __uint32_t, __off_t, __off64_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_name_t, gss_cred_id_t, gss_ctx_id_t, gss_uint32,
                         OM_uint32, gss_OID_desc_struct, gss_OID_desc,
                         gss_OID, gss_OID_set_desc_struct, gss_OID_set_desc,
                         gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_channel_bindings_struct,
                         gss_channel_bindings_t, gss_name_struct,
                         gss_cred_id_struct, gss_ctx_id_struct,
                         GSS_C_NT_USER_NAME, GSS_C_NT_HOSTBASED_SERVICE,
                         gss_release_cred, gss_init_sec_context,
                         gss_accept_sec_context, gss_display_status,
                         gss_display_name, gss_import_name, gss_release_name,
                         gss_release_buffer, gss_oid_to_str,
                         gss_canonicalize_name};
pub use self::gssapi_ext_h::{gss_buffer_set_desc_struct, gss_buffer_set_t,
                             gss_release_buffer_set, gss_export_cred,
                             gss_import_cred, gss_inquire_name,
                             gss_get_name_attribute};
pub use self::krb5_h::{krb5_int32, krb5_error_code, krb5_context,
                       _krb5_context, krb5_get_error_message,
                       krb5_free_error_message};
use self::stdio_h::{stderr, fprintf, printf};
use self::string_h::strlen;
use self::stdlib_h::exit;
use self::gssapi_krb5_h::{GSS_KRB5_NT_ENTERPRISE_NAME,
                          GSS_KRB5_NT_PRINCIPAL_NAME};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/gssapi/common.c - Common utility functions for GSSAPI test programs */
/*
 * Copyright (C) 2012 by the Massachusetts Institute of Technology.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * * Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *
 * * Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in
 *   the documentation and/or other materials provided with the
 *   distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */
#[no_mangle]
#[c2rust::src_loc = "37:14"]
pub static mut mech_krb5: gss_OID_desc =
    {
        let mut init =
            gss_OID_desc_struct{length: 9 as libc::c_int as OM_uint32,
                                elements:
                                    b"*\x86H\x86\xf7\x12\x01\x02\x02\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_void,};
        init
    };
#[no_mangle]
#[c2rust::src_loc = "38:14"]
pub static mut mech_spnego: gss_OID_desc =
    {
        let mut init =
            gss_OID_desc_struct{length: 6 as libc::c_int as OM_uint32,
                                elements:
                                    b"+\x06\x01\x05\x05\x02\x00" as *const u8
                                        as *const libc::c_char as
                                        *mut libc::c_void,};
        init
    };
#[no_mangle]
#[c2rust::src_loc = "39:14"]
pub static mut mech_iakerb: gss_OID_desc =
    {
        let mut init =
            gss_OID_desc_struct{length: 6 as libc::c_int as OM_uint32,
                                elements:
                                    b"+\x06\x01\x05\x02\x05\x00" as *const u8
                                        as *const libc::c_char as
                                        *mut libc::c_void,};
        init
    };
#[no_mangle]
#[c2rust::src_loc = "40:18"]
pub static mut mechset_krb5: gss_OID_set_desc =
    unsafe {
        {
            let mut init =
                gss_OID_set_desc_struct{count: 1 as libc::c_int as size_t,
                                        elements:
                                            &mech_krb5 as *const gss_OID_desc
                                                as *mut gss_OID_desc,};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "41:18"]
pub static mut mechset_spnego: gss_OID_set_desc =
    unsafe {
        {
            let mut init =
                gss_OID_set_desc_struct{count: 1 as libc::c_int as size_t,
                                        elements:
                                            &mech_spnego as
                                                *const gss_OID_desc as
                                                *mut gss_OID_desc,};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "42:18"]
pub static mut mechset_iakerb: gss_OID_set_desc =
    unsafe {
        {
            let mut init =
                gss_OID_set_desc_struct{count: 1 as libc::c_int as size_t,
                                        elements:
                                            &mech_iakerb as
                                                *const gss_OID_desc as
                                                *mut gss_OID_desc,};
            init
        }
    };
#[c2rust::src_loc = "44:1"]
unsafe extern "C" fn display_status(mut msg: *const libc::c_char,
                                    mut code: OM_uint32,
                                    mut type_0: libc::c_int) {
    let mut min_stat: OM_uint32 = 0;
    let mut msg_ctx: OM_uint32 = 0 as libc::c_int as OM_uint32;
    let mut buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    loop  {
        gss_display_status(&mut min_stat, code, type_0, 0 as gss_OID,
                           &mut msg_ctx, &mut buf);
        fprintf(stderr, b"%s: %.*s\n\x00" as *const u8 as *const libc::c_char,
                msg, buf.length as libc::c_int,
                buf.value as *mut libc::c_char);
        gss_release_buffer(&mut min_stat, &mut buf);
        if !(msg_ctx != 0 as libc::c_int as libc::c_uint) { break ; }
    };
}
#[no_mangle]
#[c2rust::src_loc = "58:1"]
pub unsafe extern "C" fn check_gsserr(mut msg: *const libc::c_char,
                                      mut major: OM_uint32,
                                      mut minor: OM_uint32) {
    if major &
           ((0o377 as libc::c_ulong as OM_uint32) << 24 as libc::c_int |
                (0o377 as libc::c_ulong as OM_uint32) << 16 as libc::c_int) !=
           0 {
        display_status(msg, major, 1 as libc::c_int);
        display_status(msg, minor, 2 as libc::c_int);
        exit(1 as libc::c_int);
    };
}
#[no_mangle]
#[c2rust::src_loc = "68:1"]
pub unsafe extern "C" fn check_k5err(mut context: krb5_context,
                                     mut msg: *const libc::c_char,
                                     mut code: krb5_error_code) {
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    if code != 0 {
        errmsg = krb5_get_error_message(context, code);
        printf(b"%s: %s\n\x00" as *const u8 as *const libc::c_char, msg,
               errmsg);
        krb5_free_error_message(context, errmsg);
        exit(1 as libc::c_int);
    };
}
#[no_mangle]
#[c2rust::src_loc = "81:1"]
pub unsafe extern "C" fn errout(mut msg: *const libc::c_char) {
    fprintf(stderr, b"%s\n\x00" as *const u8 as *const libc::c_char, msg);
    exit(1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "88:1"]
pub unsafe extern "C" fn import_name(mut str: *const libc::c_char)
 -> gss_name_t {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut name: gss_name_t = 0 as *mut gss_name_struct;
    let mut buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut nametype: gss_OID = 0 as gss_OID;
    if *str as libc::c_int == 'u' as i32 {
        nametype = GSS_C_NT_USER_NAME
    } else if *str as libc::c_int == 'p' as i32 {
        nametype = GSS_KRB5_NT_PRINCIPAL_NAME
    } else if *str as libc::c_int == 'e' as i32 {
        nametype = GSS_KRB5_NT_ENTERPRISE_NAME
    } else if *str as libc::c_int == 'h' as i32 {
        nametype = GSS_C_NT_HOSTBASED_SERVICE
    }
    if nametype.is_null() ||
           *str.offset(1 as libc::c_int as isize) as libc::c_int != ':' as i32
       {
        errout(b"names must begin with u: or p: or e: or h:\x00" as *const u8
                   as *const libc::c_char);
    }
    buf.value =
        (str as *mut libc::c_char).offset(2 as libc::c_int as isize) as
            *mut libc::c_void;
    buf.length = strlen(str).wrapping_sub(2 as libc::c_int as libc::c_ulong);
    major = gss_import_name(&mut minor, &mut buf, nametype, &mut name);
    check_gsserr(b"gss_import_name\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    return name;
}
#[no_mangle]
#[c2rust::src_loc = "113:1"]
pub unsafe extern "C" fn establish_contexts(mut imech: gss_OID,
                                            mut icred: gss_cred_id_t,
                                            mut acred: gss_cred_id_t,
                                            mut tname: gss_name_t,
                                            mut flags: OM_uint32,
                                            mut ictx: *mut gss_ctx_id_t,
                                            mut actx: *mut gss_ctx_id_t,
                                            mut src_name: *mut gss_name_t,
                                            mut amech: *mut gss_OID,
                                            mut deleg_cred:
                                                *mut gss_cred_id_t) {
    let mut minor: OM_uint32 = 0;
    let mut imaj: OM_uint32 = 0;
    let mut amaj: OM_uint32 = 0;
    let mut itok: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut atok: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    *actx = 0 as gss_ctx_id_t;
    *ictx = *actx;
    amaj =
        ((1 as libc::c_int) << 0 as libc::c_int + 0 as libc::c_int) as
            OM_uint32;
    imaj = amaj;
    atok.value = 0 as *mut libc::c_void;
    itok.value = atok.value;
    atok.length = 0 as libc::c_int as size_t;
    itok.length = atok.length;
    loop  {
        gss_release_buffer(&mut minor, &mut itok);
        imaj =
            gss_init_sec_context(&mut minor, icred, ictx, tname, imech, flags,
                                 0xffffffff as libc::c_ulong as OM_uint32,
                                 0 as gss_channel_bindings_t, &mut atok,
                                 0 as *mut gss_OID, &mut itok,
                                 0 as *mut OM_uint32, 0 as *mut OM_uint32);
        check_gsserr(b"gss_init_sec_context\x00" as *const u8 as
                         *const libc::c_char, imaj, minor);
        if amaj == 0 as libc::c_int as libc::c_uint { break ; }
        gss_release_buffer(&mut minor, &mut atok);
        amaj =
            gss_accept_sec_context(&mut minor, actx, acred, &mut itok,
                                   0 as gss_channel_bindings_t, src_name,
                                   amech, &mut atok, 0 as *mut OM_uint32,
                                   0 as *mut OM_uint32, deleg_cred);
        check_gsserr(b"gss_accept_sec_context\x00" as *const u8 as
                         *const libc::c_char, amaj, minor);
        gss_release_buffer(&mut minor, &mut itok);
        if imaj == 0 as libc::c_int as libc::c_uint { break ; }
    }
    if imaj != 0 as libc::c_int as libc::c_uint ||
           amaj != 0 as libc::c_int as libc::c_uint {
        errout(b"One side wants to continue after the other is done\x00" as
                   *const u8 as *const libc::c_char);
    }
    gss_release_buffer(&mut minor, &mut itok);
    gss_release_buffer(&mut minor, &mut atok);
}
#[no_mangle]
#[c2rust::src_loc = "153:1"]
pub unsafe extern "C" fn export_import_cred(mut cred: *mut gss_cred_id_t) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    major = gss_export_cred(&mut minor, *cred, &mut buf);
    check_gsserr(b"gss_export_cred\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    gss_release_cred(&mut minor, cred);
    major = gss_import_cred(&mut minor, &mut buf, cred);
    check_gsserr(b"gss_import_cred\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    gss_release_buffer(&mut minor, &mut buf);
}
#[no_mangle]
#[c2rust::src_loc = "167:1"]
pub unsafe extern "C" fn display_canon_name(mut tag: *const libc::c_char,
                                            mut name: gss_name_t,
                                            mut mech: gss_OID) {
    let mut canon: gss_name_t = 0 as *mut gss_name_struct;
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    major = gss_canonicalize_name(&mut minor, name, mech, &mut canon);
    check_gsserr(b"gss_canonicalize_name\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    major = gss_display_name(&mut minor, canon, &mut buf, 0 as *mut gss_OID);
    check_gsserr(b"gss_display_name\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    printf(b"%s:\t%.*s\n\x00" as *const u8 as *const libc::c_char, tag,
           buf.length as libc::c_int, buf.value as *mut libc::c_char);
    gss_release_name(&mut minor, &mut canon);
    gss_release_buffer(&mut minor, &mut buf);
}
#[no_mangle]
#[c2rust::src_loc = "186:1"]
pub unsafe extern "C" fn display_oid(mut tag: *const libc::c_char,
                                     mut oid: gss_OID) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    major = gss_oid_to_str(&mut minor, oid, &mut buf);
    check_gsserr(b"gss_oid_to_str\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    if !tag.is_null() {
        printf(b"%s:\t\x00" as *const u8 as *const libc::c_char, tag);
    }
    printf(b"%.*s\n\x00" as *const u8 as *const libc::c_char,
           buf.length as libc::c_int, buf.value as *mut libc::c_char);
    gss_release_buffer(&mut minor, &mut buf);
}
#[c2rust::src_loc = "200:1"]
unsafe extern "C" fn dump_attribute(mut name: gss_name_t,
                                    mut attribute: gss_buffer_t,
                                    mut noisy: libc::c_int) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut value: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut display_value: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut authenticated: libc::c_int = 0 as libc::c_int;
    let mut complete: libc::c_int = 0 as libc::c_int;
    let mut more: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_uint = 0;
    while more != 0 as libc::c_int {
        value.value = 0 as *mut libc::c_void;
        display_value.value = 0 as *mut libc::c_void;
        major =
            gss_get_name_attribute(&mut minor, name, attribute,
                                   &mut authenticated, &mut complete,
                                   &mut value, &mut display_value, &mut more);
        check_gsserr(b"gss_get_name_attribute\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        printf(b"Attribute %.*s %s %s\n\n%.*s\n\x00" as *const u8 as
                   *const libc::c_char, (*attribute).length as libc::c_int,
               (*attribute).value as *mut libc::c_char,
               if authenticated != 0 {
                   b"Authenticated\x00" as *const u8 as *const libc::c_char
               } else { b"\x00" as *const u8 as *const libc::c_char },
               if complete != 0 {
                   b"Complete\x00" as *const u8 as *const libc::c_char
               } else { b"\x00" as *const u8 as *const libc::c_char },
               display_value.length as libc::c_int,
               display_value.value as *mut libc::c_char);
        if noisy != 0 {
            i = 0 as libc::c_int as libc::c_uint;
            while (i as libc::c_ulong) < value.length {
                if i.wrapping_rem(32 as libc::c_int as libc::c_uint) ==
                       0 as libc::c_int as libc::c_uint {
                    printf(b"\n\x00" as *const u8 as *const libc::c_char);
                }
                printf(b"%02x\x00" as *const u8 as *const libc::c_char,
                       *(value.value as *mut libc::c_char).offset(i as isize)
                           as libc::c_int & 0xff as libc::c_int);
                i = i.wrapping_add(1)
            }
            printf(b"\n\n\x00" as *const u8 as *const libc::c_char);
        }
        gss_release_buffer(&mut minor, &mut value);
        gss_release_buffer(&mut minor, &mut display_value);
    };
}
#[no_mangle]
#[c2rust::src_loc = "240:1"]
pub unsafe extern "C" fn enumerate_attributes(mut name: gss_name_t,
                                              mut noisy: libc::c_int) {
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut is_mechname: libc::c_int = 0;
    let mut attrs: gss_buffer_set_t = 0 as gss_buffer_set_t;
    let mut i: size_t = 0;
    major =
        gss_inquire_name(&mut minor, name, &mut is_mechname,
                         0 as *mut gss_OID, &mut attrs);
    check_gsserr(b"gss_inquire_name\x00" as *const u8 as *const libc::c_char,
                 major, minor);
    if !attrs.is_null() {
        i = 0 as libc::c_int as size_t;
        while i < (*attrs).count {
            dump_attribute(name, &mut *(*attrs).elements.offset(i as isize),
                           noisy);
            i = i.wrapping_add(1)
        }
    }
    gss_release_buffer_set(&mut minor, &mut attrs);
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/gssapi/common.h - Declarations for GSSAPI test utility functions */
/*
 * Copyright (C) 2012 by the Massachusetts Institute of Technology.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * * Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *
 * * Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in
 *   the documentation and/or other materials provided with the
 *   distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/* Display an error message (containing msg) and exit if major is an error. */
/* Display an error message (containing msg) and exit if code is an error. */
/* Display an error message containing msg and exit. */
/* Import a GSSAPI name based on a string of the form 'u:username',
 * 'p:principalname', or 'h:host@service' (or just 'h:service'). */
/* Establish contexts using gss_init_sec_context and gss_accept_sec_context. */
/* Export *cred to a token, then release *cred and replace it by re-importing
 * the token. */
/* Display name as canonicalized to mech, preceded by tag. */
/* Display oid in printable form, preceded by tag (if not NULL). */
/* Display attributes of name, including hex value if noisy is true. */
/* Display the contents of buf to fp in hex, followed by a newline. */
#[no_mangle]
#[c2rust::src_loc = "259:1"]
pub unsafe extern "C" fn print_hex(mut fp: *mut FILE, mut buf: gss_buffer_t) {
    let mut i: size_t = 0;
    let mut bytes: *const libc::c_uchar =
        (*buf).value as *const libc::c_uchar;
    i = 0 as libc::c_int as size_t;
    while i < (*buf).length {
        printf(b"%02X\x00" as *const u8 as *const libc::c_char,
               *bytes.offset(i as isize) as libc::c_int);
        i = i.wrapping_add(1)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
