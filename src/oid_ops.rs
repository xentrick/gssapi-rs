use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:50"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:50"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = u32;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:50"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:50"]
pub mod gssapi_h {
    #[c2rust::src_loc = "91:1"]
    pub type gss_uint32 = uint32_t;
    #[c2rust::src_loc = "104:1"]
    pub type OM_uint32 = gss_uint32;

    #[repr(C)]
    #[c2rust::src_loc = "106:16"]
    #[derive(Copy, Clone)]
    pub struct gss_OID_desc_struct {
        pub length: OM_uint32,
        pub elements: *mut libc::c_void,
    }
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID_desc = gss_OID_desc_struct;
    #[c2rust::src_loc = "106:1"]
    pub type gss_OID = *mut gss_OID_desc_struct;

    #[repr(C)]
    #[c2rust::src_loc = "112:16"]
    #[derive(Copy, Clone)]
    pub struct gss_OID_set_desc_struct {
        pub count: usize,
        pub elements: gss_OID,
    }
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set_desc = gss_OID_set_desc_struct;
    #[c2rust::src_loc = "112:1"]
    pub type gss_OID_set = *mut gss_OID_set_desc_struct;

    #[repr(C)]
    #[c2rust::src_loc = "117:16"]
    #[derive(Copy, Clone)]
    pub struct gss_buffer_desc_struct {
        pub length: usize,
        pub value: *mut libc::c_void,
    }
    #[c2rust::src_loc = "117:1"]
    pub type gss_buffer_t = *mut gss_buffer_desc_struct;
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
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
         * {10, (void *)"\x2a\x86\x48\x86\xf7\x12\x01\x02\x01\x02"},
         * corresponding to an object-identifier value of
         * {iso(1) member-body(2) United States(840) mit(113554)
         * infosys(1) gssapi(2) generic(1) machine_uid_name(2)}.
         * The constant GSS_C_NT_MACHINE_UID_NAME should be
         * initialized to point to that gss_OID_desc.
         */
        #[no_mangle]
        #[c2rust::src_loc = "348:27"]
        pub static mut GSS_C_NT_MACHINE_UID_NAME: gss_OID;
        /*
         * The implementation must reserve static storage for a
         * gss_OID_desc object containing the value
         * {10, (void *)"\x2a\x86\x48\x86\xf7\x12\x01\x02\x01\x03"},
         * corresponding to an object-identifier value of
         * {iso(1) member-body(2) United States(840) mit(113554)
         * infosys(1) gssapi(2) generic(1) string_uid_name(3)}.
         * The constant GSS_C_NT_STRING_UID_NAME should be
         * initialized to point to that gss_OID_desc.
         */
        #[no_mangle]
        #[c2rust::src_loc = "360:27"]
        pub static mut GSS_C_NT_STRING_UID_NAME: gss_OID;
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
        /*
         * The implementation must reserve static storage for a
         * gss_OID_desc object containing the value
         * {6, (void *)"\x2b\x06\01\x05\x06\x03"},
         * corresponding to an object identifier value of
         * {1(iso), 3(org), 6(dod), 1(internet), 5(security),
         * 6(nametypes), 3(gss-anonymous-name)}.  The constant
         * and GSS_C_NT_ANONYMOUS should be initialized to point
         * to that gss_OID_desc.
         */
        #[no_mangle]
        #[c2rust::src_loc = "404:27"]
        pub static mut GSS_C_NT_ANONYMOUS: gss_OID;
        /*
         * The implementation must reserve static storage for a
         * gss_OID_desc object containing the value
         * {6, (void *)"\x2b\x06\x01\x05\x06\x04"},
         * corresponding to an object-identifier value of
         * {1(iso), 3(org), 6(dod), 1(internet), 5(security),
         * 6(nametypes), 4(gss-api-exported-name)}.  The constant
         * GSS_C_NT_EXPORT_NAME should be initialized to point
         * to that gss_OID_desc.
         */
        #[no_mangle]
        #[c2rust::src_loc = "417:27"]
        pub static mut GSS_C_NT_EXPORT_NAME: gss_OID;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/k5-buf.h:50"]
pub mod k5_buf_h {
    #[c2rust::src_loc = "48:1"]
    pub type k5buftype = u32;
    #[c2rust::src_loc = "48:59"]
    pub const K5BUF_DYNAMIC_ZAP: k5buftype = 3;
    #[c2rust::src_loc = "48:44"]
    pub const K5BUF_DYNAMIC: k5buftype = 2;
    #[c2rust::src_loc = "48:31"]
    pub const K5BUF_FIXED: k5buftype = 1;
    #[c2rust::src_loc = "48:18"]
    pub const K5BUF_ERROR: k5buftype = 0;

    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    #[derive(Copy, Clone)]
    pub struct k5buf {
        pub buftype: k5buftype,
        pub data: *mut libc::c_void,
        pub space: usize,
        pub len: usize,
    }
    use super::stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "100:1"]
        pub fn k5_buf_status(buf: *mut k5buf) -> i32;
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn k5_buf_add_fmt(buf: *mut k5buf, fmt: *const i8, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn k5_buf_add_len(buf: *mut k5buf, data: *const libc::c_void, len: usize);
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn k5_buf_add(buf: *mut k5buf, data: *const i8);
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn k5_buf_init_dynamic(buf: *mut k5buf);
    }
    /* K5_BUF_H */
}
#[c2rust::header_src = "/usr/include/ctype.h:59"]
pub mod ctype_h {
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed_0 = 8192;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed_0 = 2048;
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed_0 = u32;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed_0 = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed_0 = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed_0 = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed_0 = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed_0 = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed_0 = 16384;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed_0 = 4096;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed_0 = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed_0 = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed_0 = 256;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const u16;
    }
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapiP_generic.h:50"]
pub mod gssapiP_generic_h {
    #[inline]
    #[c2rust::src_loc = "270:1"]
    pub unsafe extern "C" fn k5buf_to_gss(
        mut minor: *mut OM_uint32,
        mut input_k5buf: *mut k5buf,
        mut output_buffer: gss_buffer_t,
    ) -> OM_uint32 {
        let mut status = 0u32;
        if k5_buf_status(input_k5buf) != 0i32 {
            *minor = 12u32;
            return (13u32) << 16i32;
        }
        (*output_buffer).length = (*input_k5buf).len;
        (*output_buffer).value = (*input_k5buf).data;
        memset(
            input_k5buf as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<k5buf>(),
        );
        return status;
    }

    use super::gssapi_h::gss_OID_set;
    use super::gssapi_h::gss_buffer_desc_struct;
    use super::gssapi_h::gss_buffer_t;
    use super::gssapi_h::OM_uint32;
    use super::k5_buf_h::k5_buf_status;
    use super::k5_buf_h::k5buf;
    use super::stddef_h::size_t;
    use super::string_h::memset;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn generic_gss_release_oid_set(_: *mut OM_uint32, _: *mut gss_OID_set) -> OM_uint32;
    }
    /* _GSSAPIP_GENERIC_H_ */
}
#[c2rust::header_src = "/usr/include/assert.h:50"]
pub mod assert_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(
            __assertion: *const i8,
            __file: *const i8,
            __line: u32,
            __function: *const i8,
        ) -> !;
    }
}
#[c2rust::header_src = "/usr/include/string.h:50"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: usize) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: i32, _: usize) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: usize) -> i32;
    }
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapi_generic.h:50"]
pub mod gssapi_generic_h {
    use super::gssapi_h::gss_OID;
    use super::gssapi_h::gss_OID_desc_struct;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "54:27"]
        pub static mut gss_nt_service_name: gss_OID;
    }
    /* _GSSAPI_GENERIC_H_ */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/lib/gssapi/generic/gssapi_ext.h:50"]
pub mod gssapi_ext_h {
    use super::gssapi_h::gss_OID;
    use super::gssapi_h::gss_OID_desc_struct;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "434:27"]
        pub static mut GSS_C_NT_COMPOSITE_EXPORT: gss_OID;
    }
    /* GSSAPI_EXT_H_ */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi_alloc.h:50"]
pub mod gssapi_alloc_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    /* To the extent possible under law, Painless Security, LLC has waived
     * all copyright and related or neighboring rights to GSS-API Memory
     * Management Header. This work is published from: United States.
     */
    /* not _WIN32 or DEBUG_GSSALLOC */
    /* Normal Unix case, just use free/malloc/calloc/realloc. */
    #[inline]
    #[c2rust::src_loc = "93:1"]
    pub unsafe extern "C" fn gssalloc_free(mut value: *mut libc::c_void) {
        free(value);
    }
    #[inline]
    #[c2rust::src_loc = "99:1"]
    pub unsafe extern "C" fn gssalloc_malloc(mut size: usize) -> *mut libc::c_void {
        return malloc(size);
    }
    #[inline]
    #[c2rust::src_loc = "105:1"]
    pub unsafe extern "C" fn gssalloc_calloc(
        mut count: usize,
        mut size: usize,
    ) -> *mut libc::c_void {
        return calloc(count, size);
    }
    use super::stddef_h::size_t;
    use super::stdlib_h::calloc;
    use super::stdlib_h::free;
    use super::stdlib_h::malloc;
}
#[c2rust::header_src = "/usr/include/stdlib.h:50"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: usize, _: usize) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: usize) -> *mut libc::c_void;
    }
}
pub use self::stddef_h::size_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::types_h::__uint32_t;

use self::assert_h::__assert_fail;
pub use self::ctype_h::C2RustUnnamed_0;
pub use self::ctype_h::_ISalnum;
pub use self::ctype_h::_ISalpha;
pub use self::ctype_h::_ISblank;
pub use self::ctype_h::_IScntrl;
pub use self::ctype_h::_ISdigit;
pub use self::ctype_h::_ISgraph;
pub use self::ctype_h::_ISlower;
pub use self::ctype_h::_ISprint;
pub use self::ctype_h::_ISpunct;
pub use self::ctype_h::_ISspace;
pub use self::ctype_h::_ISupper;
pub use self::ctype_h::_ISxdigit;
pub use self::ctype_h::__ctype_b_loc;
pub use self::gssapiP_generic_h::generic_gss_release_oid_set;
pub use self::gssapiP_generic_h::k5buf_to_gss;
use self::gssapi_ext_h::GSS_C_NT_COMPOSITE_EXPORT;
use self::gssapi_generic_h::gss_nt_service_name;
pub use self::gssapi_h::gss_OID;
pub use self::gssapi_h::gss_OID_desc;
pub use self::gssapi_h::gss_OID_desc_struct;
pub use self::gssapi_h::gss_OID_set;
pub use self::gssapi_h::gss_OID_set_desc;
pub use self::gssapi_h::gss_OID_set_desc_struct;
pub use self::gssapi_h::gss_buffer_desc_struct;
pub use self::gssapi_h::gss_buffer_t;
pub use self::gssapi_h::gss_uint32;
pub use self::gssapi_h::OM_uint32;
pub use self::gssapi_h::GSS_C_NT_ANONYMOUS;
pub use self::gssapi_h::GSS_C_NT_EXPORT_NAME;
pub use self::gssapi_h::GSS_C_NT_HOSTBASED_SERVICE;
pub use self::gssapi_h::GSS_C_NT_MACHINE_UID_NAME;
pub use self::gssapi_h::GSS_C_NT_STRING_UID_NAME;
pub use self::gssapi_h::GSS_C_NT_USER_NAME;
pub use self::k5_buf_h::k5_buf_add;
pub use self::k5_buf_h::k5_buf_add_fmt;
pub use self::k5_buf_h::k5_buf_add_len;
pub use self::k5_buf_h::k5_buf_init_dynamic;
pub use self::k5_buf_h::k5_buf_status;
pub use self::k5_buf_h::k5buf;
pub use self::k5_buf_h::k5buftype;
pub use self::k5_buf_h::K5BUF_DYNAMIC;
pub use self::k5_buf_h::K5BUF_DYNAMIC_ZAP;
pub use self::k5_buf_h::K5BUF_ERROR;
pub use self::k5_buf_h::K5BUF_FIXED;
use self::string_h::memcmp;
use self::string_h::memcpy;
use self::string_h::memset;

pub use self::gssapi_alloc_h::gssalloc_calloc;
pub use self::gssapi_alloc_h::gssalloc_free;
pub use self::gssapi_alloc_h::gssalloc_malloc;
use self::stdlib_h::calloc;
use self::stdlib_h::free;
use self::stdlib_h::malloc;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/gssapi/generic/oid_ops.c */
/*
 * Copyright 1995 by the Massachusetts Institute of Technology.
 * All Rights Reserved.
 *
 * Export of this software from the United States of America may
 *   require a specific license from the United States Government.
 *   It is the responsibility of any person or organization contemplating
 *   export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of M.I.T. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 * M.I.T. makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 */
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
/* GSS-API V2 interfaces to manipulate OIDs */
/*
 * The functions for allocating and releasing individual OIDs use malloc and
 * free instead of the gssalloc wrappers, because the mechglue currently mixes
 * generic_gss_copy_oid() with hand-freeing of OIDs.  We do not need to free
 * free OIDs allocated by mechanisms, so this should not be a problem.
 */
#[no_mangle]
#[c2rust::src_loc = "68:1"]
pub unsafe extern "C" fn generic_gss_release_oid(
    mut minor_status: *mut OM_uint32,
    mut oid: *mut gss_OID,
) -> OM_uint32 {
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if oid.is_null() || (*oid).is_null() {
        return 0u32;
    }
    /*
     * The V2 API says the following!
     *
     * gss_release_oid[()] will recognize any of the GSSAPI's own OID values,
     * and will silently ignore attempts to free these OIDs; for other OIDs
     * it will call the C free() routine for both the OID data and the
     * descriptor.  This allows applications to freely mix their own heap-
     * allocated OID values with OIDs returned by GSS-API.
     */
    /*
     * We use the official OID definitions instead of the unofficial OID
     * definitions. But we continue to support the unofficial OID
     * gss_nt_service_name just in case if some gss applications use
     * the old OID.
     */
    if *oid != GSS_C_NT_USER_NAME
        && *oid != GSS_C_NT_MACHINE_UID_NAME
        && *oid != GSS_C_NT_STRING_UID_NAME
        && *oid != GSS_C_NT_HOSTBASED_SERVICE
        && *oid != GSS_C_NT_ANONYMOUS
        && *oid != GSS_C_NT_EXPORT_NAME
        && *oid != GSS_C_NT_COMPOSITE_EXPORT
        && *oid != gss_nt_service_name
    {
        free((**oid).elements);
        free(*oid as *mut libc::c_void);
    }
    *oid = 0 as gss_OID;
    return 0u32;
}
#[no_mangle]
#[c2rust::src_loc = "109:1"]
pub unsafe extern "C" fn generic_gss_copy_oid(
    mut minor_status: *mut OM_uint32,
    oid: *const gss_OID_desc,
    mut new_oid: *mut gss_OID,
) -> OM_uint32 {
    let mut p = 0 as *mut gss_OID_desc_struct;
    *minor_status = 0u32;
    p = malloc(::std::mem::size_of::<gss_OID_desc>()) as gss_OID;
    if p.is_null() {
        *minor_status = 12u32;
        return (13u32) << 16i32;
    }
    (*p).length = (*oid).length;
    (*p).elements = malloc((*p).length as usize);
    if (*p).elements.is_null() {
        free(p as *mut libc::c_void);
        return (13u32) << 16i32;
    }
    memcpy((*p).elements, (*oid).elements, (*p).length as usize);
    *new_oid = p;
    return 0u32;
}
#[no_mangle]
#[c2rust::src_loc = "135:1"]
pub unsafe extern "C" fn generic_gss_create_empty_oid_set(
    mut minor_status: *mut OM_uint32,
    mut oid_set: *mut gss_OID_set,
) -> OM_uint32 {
    *minor_status = 0u32;
    if oid_set.is_null() {
        return (2u32) << 24i32;
    }
    *oid_set = gssalloc_malloc(::std::mem::size_of::<gss_OID_set_desc>()) as gss_OID_set;
    if !(*oid_set).is_null() {
        memset(
            *oid_set as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<gss_OID_set_desc>(),
        );
        return 0u32;
    } else {
        *minor_status = 12u32;
        return (13u32) << 16i32;
    };
}
#[no_mangle]
#[c2rust::src_loc = "153:1"]
pub unsafe extern "C" fn generic_gss_add_oid_set_member(
    mut minor_status: *mut OM_uint32,
    member_oid: *const gss_OID_desc,
    mut oid_set: *mut gss_OID_set,
) -> OM_uint32 {
    let mut elist = 0 as *mut gss_OID_desc_struct;
    let mut lastel = 0 as *mut gss_OID_desc_struct;
    *minor_status = 0u32;
    if member_oid.is_null() || (*member_oid).length == 0u32 || (*member_oid).elements.is_null() {
        return (1u32) << 24i32;
    }
    if oid_set.is_null() {
        return (2u32) << 24i32;
    }
    elist = (**oid_set).elements;
    /* Get an enlarged copy of the array */
    (**oid_set).elements = gssalloc_malloc(
        (**oid_set)
            .count
            .wrapping_add(1usize)
            .wrapping_mul(::std::mem::size_of::<gss_OID_desc>()),
    ) as gss_OID;
    if !(**oid_set).elements.is_null() {
        /* Copy in the old junk */
        if !elist.is_null() {
            memcpy(
                (**oid_set).elements as *mut libc::c_void,
                elist as *const libc::c_void,
                (**oid_set)
                    .count
                    .wrapping_mul(::std::mem::size_of::<gss_OID_desc>()),
            );
        }
        /* Duplicate the input element */
        lastel = &mut *(**oid_set).elements.offset((**oid_set).count as isize)
            as *mut gss_OID_desc_struct;
        (*lastel).elements = gssalloc_malloc((*member_oid).length as usize);
        if !(*lastel).elements.is_null() {
            /* Success - copy elements */
            memcpy(
                (*lastel).elements,
                (*member_oid).elements,
                (*member_oid).length as usize,
            );
            /* Set length */
            (*lastel).length = (*member_oid).length;
            /* Update count */
            (**oid_set).count = (**oid_set).count.wrapping_add(1);
            if !elist.is_null() {
                gssalloc_free(elist as *mut libc::c_void);
            }
            *minor_status = 0u32;
            return 0u32;
        } else {
            gssalloc_free((**oid_set).elements as *mut libc::c_void);
        }
    }
    /* Failure - restore old contents of list */
    (**oid_set).elements = elist;
    *minor_status = 12u32;
    return (13u32) << 16i32;
}
#[no_mangle]
#[c2rust::src_loc = "206:1"]
pub unsafe extern "C" fn generic_gss_test_oid_set_member(
    mut minor_status: *mut OM_uint32,
    member: *const gss_OID_desc,
    mut set: gss_OID_set,
    mut present: *mut i32,
) -> OM_uint32 {
    let mut i = 0;
    let mut result = 0;
    *minor_status = 0u32;
    if member.is_null() || set.is_null() {
        return (1u32) << 24i32;
    }
    if present.is_null() {
        return (2u32) << 24i32;
    }
    result = 0i32;
    i = 0u32;
    while (i as usize) < (*set).count {
        if (*(*set).elements.offset(i as isize)).length == (*member).length
            && memcmp(
                (*(*set).elements.offset(i as isize)).elements,
                (*member).elements,
                (*member).length as usize,
            ) == 0
        {
            result = 1i32;
            break;
        } else {
            i = i.wrapping_add(1)
        }
    }
    *present = result;
    return 0u32;
}
#[no_mangle]
#[c2rust::src_loc = "237:1"]
pub unsafe extern "C" fn generic_gss_oid_to_str(
    mut minor_status: *mut OM_uint32,
    oid: *const gss_OID_desc,
    mut oid_str: gss_buffer_t,
) -> OM_uint32 {
    let mut number = 0;
    let mut n = 0;
    let mut i = 0;
    let mut first = 0;
    let mut cp = 0 as *mut u8;
    let mut buf = k5buf {
        buftype: K5BUF_ERROR,
        data: 0 as *mut libc::c_void,
        space: 0,
        len: 0,
    };
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !oid_str.is_null() {
        (*oid_str).length = 0usize;
        (*oid_str).value = 0 as *mut libc::c_void
    }
    if oid.is_null() || (*oid).length == 0u32 || (*oid).elements.is_null() {
        return (1u32) << 24i32;
    }
    if oid_str.is_null() {
        return (2u32) << 24i32;
    }
    /* Decoded according to krb5/gssapi_krb5.c */
    cp = (*oid).elements as *mut u8;
    number = *cp.offset(0isize) as usize;
    k5_buf_init_dynamic(&mut buf);
    k5_buf_add(&mut buf, b"{ \x00" as *const u8 as *const i8);
    number = 0usize;
    cp = (*oid).elements as *mut u8;
    first = 1i32;
    i = 0u32;
    while i < (*oid).length {
        number = number << 7i32 | (*cp.offset(i as isize) as i32 & 0x7fi32) as usize;
        if *cp.offset(i as isize) as i32 & 0x80i32 == 0i32 {
            if first != 0 {
                n = if number < 40usize {
                    0i32
                } else if number < 80usize {
                    1i32
                } else {
                    2i32
                } as usize;
                k5_buf_add_fmt(
                    &mut buf as *mut k5buf,
                    b"%lu %lu \x00" as *const u8 as *const i8,
                    n,
                    number.wrapping_sub(n.wrapping_mul(40usize)),
                );
                first = 0i32
            } else {
                k5_buf_add_fmt(
                    &mut buf as *mut k5buf,
                    b"%lu \x00" as *const u8 as *const i8,
                    number,
                );
            }
            number = 0usize
        }
        i = i.wrapping_add(1)
    }
    k5_buf_add_len(
        &mut buf,
        b"}\x00\x00" as *const u8 as *const libc::c_void,
        2usize,
    );
    return k5buf_to_gss(minor_status, &mut buf, oid_str);
}
/* Return the length of a DER OID subidentifier encoding. */
#[c2rust::src_loc = "289:1"]
unsafe extern "C" fn arc_encoded_length(mut arc: usize) -> usize {
    let mut len = 1usize;
    arc >>= 7i32;
    while arc != 0 {
        len = len.wrapping_add(1);
        arc >>= 7i32
    }
    return len;
}
/* Encode a subidentifier into *bufp and advance it to the encoding's end. */
#[c2rust::src_loc = "300:1"]
unsafe extern "C" fn arc_encode(mut arc: usize, mut bufp: *mut *mut u8) {
    let mut p = 0 as *mut u8;
    /* Advance to the end and encode backwards. */
    *bufp = (*bufp).offset(arc_encoded_length(arc) as isize);
    p = *bufp;
    p = p.offset(-1);
    *p = (arc & 0x7fusize) as u8;
    arc >>= 7i32;
    while arc != 0 {
        p = p.offset(-1);
        *p = (arc & 0x7fusize | 0x80usize) as u8;
        arc >>= 7i32
    }
}
/* Fetch an arc value from *bufp and advance past it and any following spaces
 * or periods.  Return 1 on success, 0 if *bufp is not at a valid arc value. */
#[c2rust::src_loc = "314:1"]
unsafe extern "C" fn get_arc(
    mut bufp: *mut *const u8,
    mut end: *const u8,
    mut arc_out: *mut usize,
) -> i32 {
    let mut p = *bufp;
    let mut arc = 0usize;
    let mut newval = 0;
    if p == end
        || *(*__ctype_b_loc()).offset(*p as i32 as isize) as i32 & _ISdigit as u16 as i32 == 0
    {
        return 0i32;
    }
    while p < end
        && *(*__ctype_b_loc()).offset(*p as i32 as isize) as i32 & _ISdigit as u16 as i32 != 0
    {
        newval = arc
            .wrapping_mul(10usize)
            .wrapping_add((*p as i32 - '0' as i32) as usize);
        if newval < arc {
            return 0i32;
        }
        arc = newval;
        p = p.offset(1)
    }
    while p < end
        && (*(*__ctype_b_loc()).offset(*p as i32 as isize) as i32 & _ISspace as u16 as i32 != 0
            || *p as i32 == '.' as i32)
    {
        p = p.offset(1)
    }
    *bufp = p;
    *arc_out = arc;
    return 1i32;
}
/*
 * Convert a sequence of two or more decimal arc values into a DER-encoded OID.
 * The values may be separated by any combination of whitespace and period
 * characters, and may be optionally surrounded with braces.  Leading
 * whitespace and trailing garbage is allowed.  The first arc value must be 0,
 * 1, or 2, and the second value must be less than 40 if the first value is not
 * 2.
 */
#[no_mangle]
#[c2rust::src_loc = "344:1"]
pub unsafe extern "C" fn generic_gss_str_to_oid(
    mut minor_status: *mut OM_uint32,
    mut oid_str: gss_buffer_t,
    mut oid_out: *mut gss_OID,
) -> OM_uint32 {
    let mut p = 0 as *const u8;
    let mut end = 0 as *const u8;
    let mut arc3_start = 0 as *const u8;
    let mut out = 0 as *mut u8;

    let mut arc = 0;
    let mut arc1 = 0;
    let mut arc2 = 0;
    let mut nbytes = 0;
    let mut brace = 0i32;
    let mut oid = 0 as *mut gss_OID_desc_struct;
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !oid_out.is_null() {
        *oid_out = 0 as gss_OID
    }
    if oid_str.is_null() || (*oid_str).value.is_null() || (*oid_str).length == 0usize {
        return (1u32) << 24i32;
    }
    if oid_out.is_null() {
        return (2u32) << 24i32;
    }
    /* Skip past initial spaces and, optionally, an open brace. */
    brace = 0i32;
    p = (*oid_str).value as *const u8;
    end = p.offset((*oid_str).length as isize);
    while p < end
        && *(*__ctype_b_loc()).offset(*p as i32 as isize) as i32 & _ISspace as u16 as i32 != 0
    {
        p = p.offset(1)
    }
    if p < end && *p as i32 == '{' as i32 {
        brace = 1i32;
        p = p.offset(1)
    }
    while p < end
        && *(*__ctype_b_loc()).offset(*p as i32 as isize) as i32 & _ISspace as u16 as i32 != 0
    {
        p = p.offset(1)
    }
    /* Get the first two arc values, to be encoded as one subidentifier. */
    if get_arc(&mut p, end, &mut arc1) == 0 || get_arc(&mut p, end, &mut arc2) == 0 {
        return (13u32) << 16i32;
    }
    if arc1 > 2usize
        || arc1 < 2usize && arc2 > 39usize
        || arc2
            > (9223372036854775807 as usize)
                .wrapping_mul(2usize)
                .wrapping_add(1usize)
                .wrapping_sub(80usize)
    {
        return (13u32) << 16i32;
    }
    arc3_start = p;
    /* Compute the total length of the encoding while checking syntax. */
    nbytes = arc_encoded_length(arc1.wrapping_mul(40usize).wrapping_add(arc2));
    while get_arc(&mut p, end, &mut arc) != 0 {
        nbytes = (nbytes).wrapping_add(arc_encoded_length(arc))
    }
    if brace != 0 && (p == end || *p as i32 != '}' as i32) {
        return (13u32) << 16i32;
    }
    /* Allocate an oid structure. */
    oid = malloc(::std::mem::size_of::<gss_OID_desc_struct>()) as gss_OID;
    if oid.is_null() {
        return (13u32) << 16i32;
    }
    (*oid).elements = malloc(nbytes);
    if (*oid).elements.is_null() {
        free(oid as *mut libc::c_void);
        return (13u32) << 16i32;
    }
    (*oid).length = nbytes as OM_uint32;
    out = (*oid).elements as *mut u8;
    arc_encode(arc1.wrapping_mul(40usize).wrapping_add(arc2), &mut out);
    p = arc3_start;
    while get_arc(&mut p, end, &mut arc) != 0 {
        arc_encode(arc, &mut out);
    }
    if out.offset(-(nbytes as isize)) == (*oid).elements as *mut u8 {
    } else {
        __assert_fail(
            b"out - nbytes == oid->elements\x00" as *const u8 as *const i8,
            b"oid_ops.c\x00" as *const u8 as *const i8,
            411u32,
            (*::std::mem::transmute::<&[u8; 71], &[i8; 71]>(
                b"OM_uint32 generic_gss_str_to_oid(OM_uint32 *, gss_buffer_t, gss_OID *)\x00",
            ))
            .as_ptr(),
        );
    }
    *oid_out = oid;
    return 0u32;
}
/* Compose an OID of a prefix and an integer suffix */
#[no_mangle]
#[c2rust::src_loc = "417:1"]
pub unsafe extern "C" fn generic_gss_oid_compose(
    mut minor_status: *mut OM_uint32,
    mut prefix: *const i8,
    mut prefix_len: usize,
    mut suffix: i32,
    mut oid: *mut gss_OID_desc,
) -> OM_uint32 {
    let mut osuffix = 0;
    let mut i = 0;
    let mut nbytes = 0;
    let mut op = 0 as *mut u8;
    if oid.is_null() {
        *minor_status = 22u32;
        return (13u32) << 16i32;
    }
    if ((*oid).length as usize) < prefix_len {
        *minor_status = 34u32;
        return (13u32) << 16i32;
    }
    memcpy((*oid).elements, prefix as *const libc::c_void, prefix_len);
    nbytes = 0usize;
    osuffix = suffix;
    while suffix != 0 {
        nbytes = nbytes.wrapping_add(1);
        suffix >>= 7i32
    }
    suffix = osuffix;
    if ((*oid).length as usize) < prefix_len.wrapping_add(nbytes) {
        *minor_status = 34u32;
        return (13u32) << 16i32;
    }
    op = ((*oid).elements as *mut u8)
        .offset(prefix_len as isize)
        .offset(nbytes as isize);
    i = -(1i32);
    while suffix != 0 {
        *op.offset(i as isize) = (suffix as u8 as i32 & 0x7fi32) as u8;
        if i != -(1i32) {
            let ref mut fresh0 = *op.offset(i as isize);
            *fresh0 = (*fresh0 as i32 | 0x80i32) as u8
        }
        i -= 1;
        suffix >>= 7i32
    }
    (*oid).length = prefix_len.wrapping_add(nbytes) as OM_uint32;
    *minor_status = 0u32;
    return 0u32;
}
#[no_mangle]
#[c2rust::src_loc = "468:1"]
pub unsafe extern "C" fn generic_gss_oid_decompose(
    mut minor_status: *mut OM_uint32,
    mut prefix: *const i8,
    mut prefix_len: usize,
    mut oid: *mut gss_OID_desc,
    mut suffix: *mut i32,
) -> OM_uint32 {
    let mut i = 0;
    let mut slen = 0;
    let mut op = 0 as *mut u8;
    if ((*oid).length as usize) < prefix_len
        || memcmp((*oid).elements, prefix as *const libc::c_void, prefix_len) != 0i32
    {
        return (1u32) << 16i32;
    }
    op = ((*oid).elements as *mut u8).offset(prefix_len as isize);
    *suffix = 0i32;
    slen = ((*oid).length as usize).wrapping_sub(prefix_len);
    i = 0usize;
    while i < slen {
        *suffix = *suffix << 7i32 | *op.offset(i as isize) as i32 & 0x7fi32;
        if i.wrapping_add(1usize) != slen && *op.offset(i as isize) as i32 & 0x80i32 == 0i32 {
            *minor_status = 22u32;
            return (13u32) << 16i32;
        }
        i = i.wrapping_add(1)
    }
    return 0u32;
}
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
#[no_mangle]
#[c2rust::src_loc = "500:1"]
pub unsafe extern "C" fn generic_gss_copy_oid_set(
    mut minor_status: *mut OM_uint32,
    oidset: *const gss_OID_set_desc,
    mut new_oidset: *mut gss_OID_set,
) -> OM_uint32 {
    let mut current_block: u64;
    let mut copy = 0 as *mut gss_OID_set_desc;
    let mut minor = 0u32;
    let mut major = 0u32;
    let mut i = 0;
    if !minor_status.is_null() {
        *minor_status = 0u32
    }
    if !new_oidset.is_null() {
        *new_oidset = 0 as gss_OID_set
    }
    if oidset.is_null() {
        return (1u32) << 24i32;
    }
    if new_oidset.is_null() {
        return (2u32) << 24i32;
    }
    copy =
        gssalloc_calloc(1usize, ::std::mem::size_of::<gss_OID_set_desc>()) as *mut gss_OID_set_desc;
    if copy.is_null() {
        major = (13u32) << 16i32
    } else {
        (*copy).elements = gssalloc_calloc(
            (*oidset).count,
            ::std::mem::size_of::<gss_OID_desc_struct>(),
        ) as *mut gss_OID_desc;
        if (*copy).elements.is_null() {
            major = (13u32) << 16i32
        } else {
            (*copy).count = (*oidset).count;
            i = 0u32;
            loop {
                if !((i as usize) < (*copy).count) {
                    current_block = 5634871135123216486;
                    break;
                }

                let mut out = &mut *(*copy).elements.offset(i as isize) as *mut gss_OID_desc_struct;
                let mut in_0 =
                    &mut *(*oidset).elements.offset(i as isize) as *mut gss_OID_desc_struct;
                (*out).elements = gssalloc_malloc((*in_0).length as usize);
                if (*out).elements.is_null() {
                    major = (13u32) << 16i32;
                    current_block = 1186425362246550222;
                    break;
                } else {
                    memcpy((*out).elements, (*in_0).elements, (*in_0).length as usize);
                    (*out).length = (*in_0).length;
                    i = i.wrapping_add(1)
                }
            }
            match current_block {
                1186425362246550222 => {}
                _ => *new_oidset = copy,
            }
        }
    }
    if major != 0u32 {
        generic_gss_release_oid_set(&mut minor, &mut copy);
    }
    return major;
}
