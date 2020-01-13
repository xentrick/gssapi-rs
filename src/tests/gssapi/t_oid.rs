use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:33"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:33"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:37"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssapi/gssapi.h:37"]
pub mod gssapi_h {
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
    pub type gss_OID_set = *mut gss_OID_set_desc_struct;
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
    use super::stdint_uintn_h::uint32_t;
    use super::stddef_h::size_t;
    extern "C" {
        /* input_name */
        #[no_mangle]
        #[c2rust::src_loc = "574:1"]
        pub fn gss_release_buffer(_: *mut OM_uint32, _: gss_buffer_t)
         -> OM_uint32;
        /* buffer */
        #[no_mangle]
        #[c2rust::src_loc = "579:1"]
        pub fn gss_release_oid_set(_: *mut OM_uint32, _: *mut gss_OID_set)
         -> OM_uint32;
        /* context_handle */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "673:1"]
        pub fn gss_release_oid(_: *mut OM_uint32, _: *mut gss_OID)
         -> OM_uint32;
        /* oid */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "679:1"]
        pub fn gss_create_empty_oid_set(_: *mut OM_uint32,
                                        _: *mut gss_OID_set) -> OM_uint32;
        /* oid_set */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "685:1"]
        pub fn gss_add_oid_set_member(_: *mut OM_uint32, _: gss_OID,
                                      _: *mut gss_OID_set) -> OM_uint32;
        /* oid_set */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "692:1"]
        pub fn gss_test_oid_set_member(_: *mut OM_uint32, _: gss_OID,
                                       _: gss_OID_set, _: *mut libc::c_int)
         -> OM_uint32;
        /* present */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "700:1"]
        pub fn gss_str_to_oid(_: *mut OM_uint32, _: gss_buffer_t,
                              _: *mut gss_OID) -> OM_uint32;
        /* oid */
        /* New for V2 */
        #[no_mangle]
        #[c2rust::src_loc = "707:1"]
        pub fn gss_oid_to_str(_: *mut OM_uint32, _: gss_OID, _: gss_buffer_t)
         -> OM_uint32;
    }
    /* _GSSAPI_H_ */
}
#[c2rust::header_src = "/usr/include/stdio.h:33"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:35"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "63:12"]
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/tests/gssapi/common.h:37"]
pub mod common_h {
    use super::gssapi_h::{gss_OID_desc_struct, gss_OID, OM_uint32};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "73:1"]
        pub fn display_oid(tag: *const libc::c_char, oid: gss_OID);
        #[no_mangle]
        #[c2rust::src_loc = "46:1"]
        pub fn check_gsserr(msg: *const libc::c_char, major: OM_uint32,
                            minor: OM_uint32);
    }
    /* COMMON_H */
}
pub use self::stddef_h::size_t;
pub use self::types_h::__uint32_t;
pub use self::stdint_uintn_h::uint32_t;
pub use self::gssapi_h::{gss_uint32, OM_uint32, gss_OID_desc_struct,
                         gss_OID_desc, gss_OID, gss_OID_set_desc_struct,
                         gss_OID_set, gss_buffer_desc_struct, gss_buffer_desc,
                         gss_buffer_t, gss_release_buffer,
                         gss_release_oid_set, gss_release_oid,
                         gss_create_empty_oid_set, gss_add_oid_set_member,
                         gss_test_oid_set_member, gss_str_to_oid,
                         gss_oid_to_str};
use self::stdio_h::printf;
use self::string_h::{memcmp, strlen};
use self::common_h::{display_oid, check_gsserr};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* tests/gssapi/t_oid.c - Test OID manipulation functions */
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
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "39:8"]
pub struct C2RustUnnamed {
    pub canonical: *mut libc::c_char,
    pub variant: *mut libc::c_char,
    pub oid: gss_OID_desc,
}
#[c2rust::src_loc = "43:3"]
static mut tests: [C2RustUnnamed; 27] =
    [{
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 1 2 840 113554 1 2 1 1 }\x00" as *const u8
                                   as *const libc::c_char as
                                   *mut libc::c_char,
                           variant:
                               b"1.2.840.113554.1.2.1.1\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               10 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"*\x86H\x86\xf7\x12\x01\x02\x01\x01\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 1 2 840 113554 1 2 1 2 }\x00" as *const u8
                                   as *const libc::c_char as
                                   *mut libc::c_char,
                           variant:
                               b"1 2 840 113554 1 2 1 2\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               10 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"*\x86H\x86\xf7\x12\x01\x02\x01\x02\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 1 2 840 113554 1 2 1 3 }\x00" as *const u8
                                   as *const libc::c_char as
                                   *mut libc::c_char,
                           variant:
                               b"{1 2 840 113554 1 2 1 3}\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               10 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"*\x86H\x86\xf7\x12\x01\x02\x01\x03\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 1 3 6 1 5 6 2 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               b"{  1  3  6  1  5  6  2  }\x00" as *const u8
                                   as *const libc::c_char as
                                   *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               6 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"+\x06\x01\x05\x06\x02\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 1 3 6 1 5 6 3 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               b"{ 01 03 06 01 05 06 03 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               6 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"+\x06\x01\x05\x06\x03\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 1 2 840 113554 1 2 2 1 }\x00" as *const u8
                                   as *const libc::c_char as
                                   *mut libc::c_char,
                           variant:
                               b" {01 2 840 113554 1 2 2 1  } \x00" as
                                   *const u8 as *const libc::c_char as
                                   *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               10 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"*\x86H\x86\xf7\x12\x01\x02\x02\x01\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 1 2 840 113554 1 2 2 6 }\x00" as *const u8
                                   as *const libc::c_char as
                                   *mut libc::c_char,
                           variant:
                               b" {1.2.840.113554.1.2.2.6} \x00" as *const u8
                                   as *const libc::c_char as
                                   *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               10 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"*\x86H\x86\xf7\x12\x01\x02\x02\x06\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 1 2 840 113554 1 2 2 2 }\x00" as *const u8
                                   as *const libc::c_char as
                                   *mut libc::c_char,
                           variant:
                               b"{1.2.840.113554.1.2.2.2}\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               10 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"*\x86H\x86\xf7\x12\x01\x02\x02\x02\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 1 2 840 113554 1 2 2 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               b"{ 1.2.840.113554.1.2.2 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               9 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"*\x86H\x86\xf7\x12\x01\x02\x02\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 1 3 5 1 5 2 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               b"001 . 003 . 005 . 001 . 005 . 002\x00" as
                                   *const u8 as *const libc::c_char as
                                   *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               5 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"+\x05\x01\x05\x02\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 1 2 840 48018 1 2 2 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               b"1.2.840.48018.1.2.2 trailing garbage\x00" as
                                   *const u8 as *const libc::c_char as
                                   *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               9 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"*\x86H\x82\xf7\x12\x01\x02\x02\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 1 3 6 1 5 2 5 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               b"{ 1 3 6 1 5 2 5 } trailing garbage\x00" as
                                   *const u8 as *const libc::c_char as
                                   *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               6 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"+\x06\x01\x05\x02\x05\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 1 3 6 1 5 5 2 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               b"{1 3 6 1 5 5 2} trailing garbage\x00" as
                                   *const u8 as *const libc::c_char as
                                   *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               6 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"+\x06\x01\x05\x05\x02\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 0 0 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               0 as *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               1 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"\x00\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 0 39 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               0 as *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               1 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"\'\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 1 0 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               0 as *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               1 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"(\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 1 39 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               0 as *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               1 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"O\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 2 0 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               0 as *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               1 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"P\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 2 40 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               0 as *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               1 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"x\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 2 47 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               0 as *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               1 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"\x7f\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 2 48 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               0 as *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               2 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"\x81\x00\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 2 16304 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               0 as *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               3 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"\x81\x80\x00\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 0 0 0 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               0 as *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               2 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"\x00\x00\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 0 0 1 0 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               0 as *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               3 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"\x00\x01\x00\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 0 0 128 0 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               0 as *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               4 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"\x00\x81\x00\x00 \x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 0 0 0 1 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               0 as *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               3 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"\x00\x00\x01\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     },
     {
         let mut init =
             C2RustUnnamed{canonical:
                               b"{ 0 0 128 0 1 0 128 }\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                           variant:
                               0 as *const libc::c_char as *mut libc::c_char,
                           oid:
                               {
                                   let mut init =
                                       gss_OID_desc_struct{length:
                                                               8 as
                                                                   libc::c_int
                                                                   as
                                                                   OM_uint32,
                                                           elements:
                                                               b"\x00\x81\x00\x00\x01\x00\x81\x00 \x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_void,};
                                   init
                               },};
         init
     }];
#[c2rust::src_loc = "102:14"]
static mut invalid_strings: [*mut libc::c_char; 19] =
    [b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"{}\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"{\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"}\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"  \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b" { } \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"x\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"+1 1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"-1.1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"1.+0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"+0.1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"{ 1 garbage }\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"{ 1 }\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"{ 0 40 }\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"{ 1 40 }\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"{ 1 128 }\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"{ 1 1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"{ 1 2 3 4 +5 }\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char,
     b"{ 1.2.-3.4.5 }\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char];
#[c2rust::src_loc = "124:1"]
unsafe extern "C" fn oid_equal(mut o1: gss_OID, mut o2: gss_OID)
 -> libc::c_int {
    return ((*o1).length == (*o2).length &&
                memcmp((*o1).elements, (*o2).elements,
                       (*o1).length as libc::c_ulong) == 0 as libc::c_int) as
               libc::c_int;
}
#[c2rust::src_loc = "131:1"]
unsafe fn main_0() -> libc::c_int {
    let mut i: size_t = 0;
    let mut major: OM_uint32 = 0;
    let mut minor: OM_uint32 = 0;
    let mut buf: gss_buffer_desc =
        gss_buffer_desc{length: 0, value: 0 as *mut libc::c_void,};
    let mut oid: gss_OID = 0 as *mut gss_OID_desc_struct;
    let mut set: gss_OID_set = 0 as *mut gss_OID_set_desc_struct;
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut present: libc::c_int = 0;
    i = 0 as libc::c_int as size_t;
    while i <
              (::std::mem::size_of::<[C2RustUnnamed; 27]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed>()
                                                   as libc::c_ulong) {
        /* Check that this test's OID converts to its canonical string form. */
        major =
            gss_oid_to_str(&mut minor,
                           &mut (*tests.as_mut_ptr().offset(i as isize)).oid,
                           &mut buf);
        check_gsserr(b"gss_oid_to_str\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        if buf.length !=
               strlen(tests[i as
                                usize].canonical).wrapping_add(1 as
                                                                   libc::c_int
                                                                   as
                                                                   libc::c_ulong)
               ||
               memcmp(buf.value,
                      tests[i as usize].canonical as *const libc::c_void,
                      buf.length) != 0 as libc::c_int {
            status = 1 as libc::c_int;
            printf(b"test %d: OID converts to %.*s, wanted %s\n\x00" as
                       *const u8 as *const libc::c_char, i as libc::c_int,
                   buf.length as libc::c_int, buf.value as *mut libc::c_char,
                   tests[i as usize].canonical);
        }
        gss_release_buffer(&mut minor, &mut buf);
        /* Check that this test's canonical string form converts to its OID. */
        buf.value = tests[i as usize].canonical as *mut libc::c_void;
        buf.length = strlen(tests[i as usize].canonical);
        major = gss_str_to_oid(&mut minor, &mut buf, &mut oid);
        check_gsserr(b"gss_str_to_oid\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        if oid_equal(oid, &mut (*tests.as_mut_ptr().offset(i as isize)).oid)
               == 0 {
            status = 1 as libc::c_int;
            printf(b"test %d: %s converts to wrong OID\n\x00" as *const u8 as
                       *const libc::c_char, i as libc::c_int,
                   tests[i as usize].canonical);
            display_oid(b"wanted\x00" as *const u8 as *const libc::c_char,
                        &mut (*tests.as_mut_ptr().offset(i as isize)).oid);
            display_oid(b"actual\x00" as *const u8 as *const libc::c_char,
                        oid);
        }
        gss_release_oid(&mut minor, &mut oid);
        /* Check that this test's variant string form converts to its OID. */
        if !tests[i as usize].variant.is_null() {
            buf.value = tests[i as usize].variant as *mut libc::c_void;
            buf.length = strlen(tests[i as usize].variant);
            major = gss_str_to_oid(&mut minor, &mut buf, &mut oid);
            check_gsserr(b"gss_str_to_oid\x00" as *const u8 as
                             *const libc::c_char, major, minor);
            if oid_equal(oid,
                         &mut (*tests.as_mut_ptr().offset(i as isize)).oid) ==
                   0 {
                status = 1 as libc::c_int;
                printf(b"test %d: %s converts to wrong OID\n\x00" as *const u8
                           as *const libc::c_char, i as libc::c_int,
                       tests[i as usize].variant);
                display_oid(b"wanted\x00" as *const u8 as *const libc::c_char,
                            &mut (*tests.as_mut_ptr().offset(i as
                                                                 isize)).oid);
                display_oid(b"actual\x00" as *const u8 as *const libc::c_char,
                            oid);
            }
            gss_release_oid(&mut minor, &mut oid);
        }
        i = i.wrapping_add(1)
    }
    i = 0 as libc::c_int as size_t;
    while i <
              (::std::mem::size_of::<[*mut libc::c_char; 19]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut libc::c_char>()
                                                   as libc::c_ulong) {
        buf.value = invalid_strings[i as usize] as *mut libc::c_void;
        buf.length = strlen(invalid_strings[i as usize]);
        major = gss_str_to_oid(&mut minor, &mut buf, &mut oid);
        if major == 0 as libc::c_int as libc::c_uint {
            status = 1 as libc::c_int;
            printf(b"invalid %d: %s converted when it should not have\n\x00"
                       as *const u8 as *const libc::c_char, i as libc::c_int,
                   invalid_strings[i as usize]);
            gss_release_oid(&mut minor, &mut oid);
        }
        i = i.wrapping_add(1)
    }
    major = gss_create_empty_oid_set(&mut minor, &mut set);
    check_gsserr(b"gss_create_empty_oid_set\x00" as *const u8 as
                     *const libc::c_char, major, minor);
    i = 0 as libc::c_int as size_t;
    while i <
              (::std::mem::size_of::<[C2RustUnnamed; 27]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed>()
                                                   as libc::c_ulong) {
        major =
            gss_add_oid_set_member(&mut minor,
                                   &mut (*tests.as_mut_ptr().offset(i as
                                                                        isize)).oid,
                                   &mut set);
        check_gsserr(b"gss_add_oid_set_member\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        i = i.wrapping_add(1)
    }
    if (*set).count != i {
        status = 1 as libc::c_int;
        printf(b"oid set has wrong size: wanted %d, actual %d\n\x00" as
                   *const u8 as *const libc::c_char, i as libc::c_int,
               (*set).count as libc::c_int);
    }
    i = 0 as libc::c_int as size_t;
    while i < (*set).count {
        if oid_equal(&mut *(*set).elements.offset(i as isize),
                     &mut (*tests.as_mut_ptr().offset(i as isize)).oid) == 0 {
            status = 1 as libc::c_int;
            printf(b"oid set has wrong element %d\n\x00" as *const u8 as
                       *const libc::c_char, i as libc::c_int);
            display_oid(b"wanted\x00" as *const u8 as *const libc::c_char,
                        &mut (*tests.as_mut_ptr().offset(i as isize)).oid);
            display_oid(b"actual\x00" as *const u8 as *const libc::c_char,
                        &mut *(*set).elements.offset(i as isize));
        }
        major =
            gss_test_oid_set_member(&mut minor,
                                    &mut (*tests.as_mut_ptr().offset(i as
                                                                         isize)).oid,
                                    set, &mut present);
        check_gsserr(b"gss_test_oid_set_member\x00" as *const u8 as
                         *const libc::c_char, major, minor);
        if present == 0 {
            status = 1 as libc::c_int;
            printf(b"oid set does not contain OID %d\n\x00" as *const u8 as
                       *const libc::c_char, i as libc::c_int);
            display_oid(b"wanted\x00" as *const u8 as *const libc::c_char,
                        &mut (*tests.as_mut_ptr().offset(i as isize)).oid);
        }
        i = i.wrapping_add(1)
    }
    gss_release_oid_set(&mut minor, &mut set);
    return status;
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
