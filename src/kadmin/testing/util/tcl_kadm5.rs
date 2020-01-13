use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:3"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:3"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = libc::c_ushort;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:3"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:3"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/tcl.h:6"]
pub mod tcl_h {
    #[c2rust::src_loc = "340:2"]
    pub type ClientData = *mut libc::c_void;
    #[c2rust::src_loc = "415:1"]
    pub type Tcl_WideInt = libc::c_longlong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "493:16"]
    pub struct Tcl_Interp {
        pub resultDontUse: *mut libc::c_char,
        pub freeProcDontUse: Option<unsafe extern "C" fn(_: *mut libc::c_char)
                                        -> ()>,
        pub errorLineDontUse: libc::c_int,
    }
    #[c2rust::src_loc = "530:1"]
    pub type Tcl_Command = *mut Tcl_Command_;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "804:16"]
    pub struct Tcl_Obj {
        pub refCount: libc::c_int,
        pub bytes: *mut libc::c_char,
        pub length: libc::c_int,
        pub typePtr: *const Tcl_ObjType,
        pub internalRep: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "823:5"]
    pub union C2RustUnnamed {
        pub longValue: libc::c_long,
        pub doubleValue: libc::c_double,
        pub otherValuePtr: *mut libc::c_void,
        pub wideValue: Tcl_WideInt,
        pub twoPtrValue: C2RustUnnamed_1,
        pub ptrAndLongRep: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "837:2"]
    pub struct C2RustUnnamed_0 {
        pub ptr: *mut libc::c_void,
        pub value: libc::c_ulong,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "829:2"]
    pub struct C2RustUnnamed_1 {
        pub ptr1: *mut libc::c_void,
        pub ptr2: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "780:16"]
    pub struct Tcl_ObjType {
        pub name: *const libc::c_char,
        pub freeIntRepProc: Option<Tcl_FreeInternalRepProc>,
        pub dupIntRepProc: Option<Tcl_DupInternalRepProc>,
        pub updateStringProc: Option<Tcl_UpdateStringProc>,
        pub setFromAnyProc: Option<Tcl_SetFromAnyProc>,
    }
    #[c2rust::src_loc = "758:1"]
    pub type Tcl_SetFromAnyProc
        =
        unsafe extern "C" fn(_: *mut Tcl_Interp, _: *mut Tcl_Obj)
            -> libc::c_int;
    #[c2rust::src_loc = "759:1"]
    pub type Tcl_UpdateStringProc
        =
        unsafe extern "C" fn(_: *mut Tcl_Obj) -> ();
    #[c2rust::src_loc = "729:1"]
    pub type Tcl_DupInternalRepProc
        =
        unsafe extern "C" fn(_: *mut Tcl_Obj, _: *mut Tcl_Obj) -> ();
    #[c2rust::src_loc = "742:1"]
    pub type Tcl_FreeInternalRepProc
        =
        unsafe extern "C" fn(_: *mut Tcl_Obj) -> ();
    #[c2rust::src_loc = "719:1"]
    pub type Tcl_CmdDeleteProc = unsafe extern "C" fn(_: ClientData) -> ();
    #[c2rust::src_loc = "720:1"]
    pub type Tcl_CmdProc
        =
        unsafe extern "C" fn(_: ClientData, _: *mut Tcl_Interp,
                             _: libc::c_int, _: *mut *const libc::c_char)
            -> libc::c_int;
    #[c2rust::src_loc = "743:1"]
    pub type Tcl_FreeProc = unsafe extern "C" fn(_: *mut libc::c_char) -> ();
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "983:16"]
    pub struct Tcl_DString {
        pub string: *mut libc::c_char,
        pub length: libc::c_int,
        pub spaceAvl: libc::c_int,
        pub staticSpace: [libc::c_char; 200],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1228:8"]
    pub struct Tcl_HashKeyType {
        pub version: libc::c_int,
        pub flags: libc::c_int,
        pub hashKeyProc: Option<Tcl_HashKeyProc>,
        pub compareKeysProc: Option<Tcl_CompareHashKeysProc>,
        pub allocEntryProc: Option<Tcl_AllocHashEntryProc>,
        pub freeEntryProc: Option<Tcl_FreeHashEntryProc>,
    }
    #[c2rust::src_loc = "1160:1"]
    pub type Tcl_FreeHashEntryProc
        =
        unsafe extern "C" fn(_: *mut Tcl_HashEntry) -> ();
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1179:8"]
    pub struct Tcl_HashEntry {
        pub nextPtr: *mut Tcl_HashEntry,
        pub tablePtr: *mut Tcl_HashTable,
        pub hash: *mut libc::c_void,
        pub clientData: ClientData,
        pub key: C2RustUnnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1194:5"]
    pub union C2RustUnnamed_2 {
        pub oneWordValue: *mut libc::c_char,
        pub objPtr: *mut Tcl_Obj,
        pub words: [libc::c_int; 1],
        pub string: [libc::c_char; 1],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1273:8"]
    pub struct Tcl_HashTable {
        pub buckets: *mut *mut Tcl_HashEntry,
        pub staticBuckets: [*mut Tcl_HashEntry; 4],
        pub numBuckets: libc::c_int,
        pub numEntries: libc::c_int,
        pub rebuildSize: libc::c_int,
        pub downShift: libc::c_int,
        pub mask: libc::c_int,
        pub keyType: libc::c_int,
        pub findProc: Option<unsafe extern "C" fn(_: *mut Tcl_HashTable,
                                                  _: *const libc::c_char)
                                 -> *mut Tcl_HashEntry>,
        pub createProc: Option<unsafe extern "C" fn(_: *mut Tcl_HashTable,
                                                    _: *const libc::c_char,
                                                    _: *mut libc::c_int)
                                   -> *mut Tcl_HashEntry>,
        pub typePtr: *const Tcl_HashKeyType,
    }
    #[c2rust::src_loc = "1158:1"]
    pub type Tcl_AllocHashEntryProc
        =
        unsafe extern "C" fn(_: *mut Tcl_HashTable, _: *mut libc::c_void)
            -> *mut Tcl_HashEntry;
    #[c2rust::src_loc = "1157:1"]
    pub type Tcl_CompareHashKeysProc
        =
        unsafe extern "C" fn(_: *mut libc::c_void, _: *mut Tcl_HashEntry)
            -> libc::c_int;
    #[c2rust::src_loc = "1156:1"]
    pub type Tcl_HashKeyProc
        =
        unsafe extern "C" fn(_: *mut Tcl_HashTable, _: *mut libc::c_void)
            -> libc::c_uint;
    extern "C" {
        #[c2rust::src_loc = "530:16"]
        pub type Tcl_Command_;
    }
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:11"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:11"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:11"]
pub mod krb5_h {
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "137:1"]
    pub type krb5_int16 = int16_t;
    #[c2rust::src_loc = "138:1"]
    pub type krb5_ui_2 = uint16_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
    #[c2rust::src_loc = "204:1"]
    pub type krb5_error_code = krb5_int32;
    #[c2rust::src_loc = "206:1"]
    pub type krb5_magic = krb5_error_code;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "208:16"]
    pub struct _krb5_data {
        pub magic: krb5_magic,
        pub length: libc::c_uint,
        pub data: *mut libc::c_char,
    }
    #[c2rust::src_loc = "208:1"]
    pub type krb5_data = _krb5_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "228:16"]
    pub struct krb5_principal_data {
        pub magic: krb5_magic,
        pub realm: krb5_data,
        pub data: *mut krb5_data,
        pub length: krb5_int32,
        pub type_0: krb5_int32,
    }
    /* This file is generated, please don't edit it directly.  */
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* General definitions for Kerberos version 5. */
/*
 * Copyright 1989, 1990, 1995, 2001, 2003, 2007, 2011 by the Massachusetts
 * Institute of Technology.  All Rights Reserved.
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
 * Copyright (C) 1998 by the FundsXpress, INC.
 *
 * All rights reserved.
 *
 * Export of this software from the United States of America may require
 * a specific license from the United States Government.  It is the
 * responsibility of any person or organization contemplating export to
 * obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify, and
 * distribute this software and its documentation for any purpose and
 * without fee is hereby granted, provided that the above copyright
 * notice appear in all copies and that both that copyright notice and
 * this permission notice appear in supporting documentation, and that
 * the name of FundsXpress. not be used in advertising or publicity pertaining
 * to distribution of the software without specific, written prior
 * permission.  FundsXpress makes no representations about the suitability of
 * this software for any purpose.  It is provided "as is" without express
 * or implied warranty.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 */
    /* * @defgroup KRB5_H krb5 library API
 * @{
 */
    /* By default, do not expose deprecated interfaces. */
    /* !KRB5_CALLCONV */
    /* !KRB5_CONFIG__ */
    /* for *_MAX */
    /* from profile.h */
    /* typedef struct _profile_t *profile_t; */
    /*
 * begin wordsize.h
 */
    /*
 * Word-size related definition.
 */
    /* this strange form is necessary since - is a unary operator, not a sign
   indicator */
    /* this strange form is necessary since - is a unary operator, not a sign
   indicator */
    /*
 * end wordsize.h
 */
    /*
 * begin "base-defs.h"
 */
    /*
 * Basic definitions for Kerberos V5 library
 */
    /* This may change, later on */
    /* *
 * Represents a timestamp in seconds since the POSIX epoch.  This legacy type
 * is used frequently in the ABI, but cannot represent timestamps after 2038 as
 * a positive number.  Code which uses this type should cast values of it to
 * uint32_t so that negative values are treated as timestamps between 2038 and
 * 2106 on platforms with 64-bit time_t.
 */
    /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
    /* Originally introduced for PKINIT; now unused.  Do not use this. */
    /* Originally used to recognize AFS and default salts.  No longer used. */
    /* *< An array of strings */
    #[c2rust::src_loc = "236:1"]
    pub type krb5_principal = *mut krb5_principal_data;
    #[c2rust::src_loc = "261:1"]
    pub type krb5_const_principal = *const krb5_principal_data;
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "363:16"]
    pub struct _krb5_keyblock {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    #[c2rust::src_loc = "2281:1"]
    pub type krb5_ccache = *mut _krb5_ccache;
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    extern "C" {
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
        #[c2rust::src_loc = "2280:8"]
        pub type _krb5_ccache;
        #[no_mangle]
        #[c2rust::src_loc = "2403:1"]
        pub fn krb5_cc_close(context_0: krb5_context, cache: krb5_ccache)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context_0: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "4425:1"]
        pub fn krb5_cc_default(context_0: krb5_context,
                               ccache: *mut krb5_ccache) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4341:1"]
        pub fn krb5_cc_resolve(context_0: krb5_context,
                               name: *const libc::c_char,
                               cache: *mut krb5_ccache) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3489:1"]
        pub fn krb5_unparse_name(context_0: krb5_context,
                                 principal: krb5_const_principal,
                                 name: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3427:1"]
        pub fn krb5_parse_name(context_0: krb5_context,
                               name: *const libc::c_char,
                               principal_out: *mut krb5_principal)
         -> krb5_error_code;
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:11"]
pub mod com_err_h {
    #[c2rust::src_loc = "26:1"]
    pub type errcode_t = libc::c_long;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn error_message(_: errcode_t) -> *const libc::c_char;
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:11"]
pub mod kdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "167:16"]
    pub struct _krb5_key_data {
        pub key_data_ver: krb5_int16,
        pub key_data_kvno: krb5_ui_2,
        pub key_data_type: [krb5_int16; 2],
        pub key_data_length: [krb5_ui_2; 2],
        pub key_data_contents: [*mut krb5_octet; 2],
    }
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_enctype,
                        krb5_int32};
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:11"]
pub mod admin_h {
    #[c2rust::src_loc = "70:1"]
    pub type kadm5_policy_t = *mut libc::c_char;
    #[c2rust::src_loc = "71:1"]
    pub type kadm5_ret_t = libc::c_long;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "190:16"]
    pub struct _kadm5_principal_ent_t {
        pub principal: krb5_principal,
        pub princ_expire_time: krb5_timestamp,
        pub last_pwd_change: krb5_timestamp,
        pub pw_expiration: krb5_timestamp,
        pub max_life: krb5_deltat,
        pub mod_name: krb5_principal,
        pub mod_date: krb5_timestamp,
        pub attributes: krb5_flags,
        pub kvno: krb5_kvno,
        pub mkvno: krb5_kvno,
        pub policy: *mut libc::c_char,
        pub aux_attributes: libc::c_long,
        pub max_renewable_life: krb5_deltat,
        pub last_success: krb5_timestamp,
        pub last_failed: krb5_timestamp,
        pub fail_auth_count: krb5_kvno,
        pub n_key_data: krb5_int16,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
        pub key_data: *mut krb5_key_data,
    }
    #[c2rust::src_loc = "190:1"]
    pub type kadm5_principal_ent_rec = _kadm5_principal_ent_t;
    #[c2rust::src_loc = "190:1"]
    pub type kadm5_principal_ent_t = *mut _kadm5_principal_ent_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "215:16"]
    pub struct _kadm5_policy_ent_t {
        pub policy: *mut libc::c_char,
        pub pw_min_life: libc::c_long,
        pub pw_max_life: libc::c_long,
        pub pw_min_length: libc::c_long,
        pub pw_min_classes: libc::c_long,
        pub pw_history_num: libc::c_long,
        pub policy_refcnt: libc::c_long,
        pub pw_max_fail: krb5_kvno,
        pub pw_failcnt_interval: krb5_deltat,
        pub pw_lockout_duration: krb5_deltat,
        pub attributes: krb5_flags,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub allowed_keysalts: *mut libc::c_char,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
    }
    #[c2rust::src_loc = "215:1"]
    pub type kadm5_policy_ent_rec = _kadm5_policy_ent_t;
    #[c2rust::src_loc = "215:1"]
    pub type kadm5_policy_ent_t = *mut _kadm5_policy_ent_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "241:16"]
    pub struct _kadm5_config_params {
        pub mask: libc::c_long,
        pub realm: *mut libc::c_char,
        pub kadmind_port: libc::c_int,
        pub kpasswd_port: libc::c_int,
        pub admin_server: *mut libc::c_char,
        pub dbname: *mut libc::c_char,
        pub acl_file: *mut libc::c_char,
        pub dict_file: *mut libc::c_char,
        pub mkey_from_kbd: libc::c_int,
        pub stash_file: *mut libc::c_char,
        pub mkey_name: *mut libc::c_char,
        pub enctype: krb5_enctype,
        pub max_life: krb5_deltat,
        pub max_rlife: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub flags: krb5_flags,
        pub keysalts: *mut krb5_key_salt_tuple,
        pub num_keysalts: krb5_int32,
        pub kvno: krb5_kvno,
        pub iprop_enabled: libc::c_int,
        pub iprop_ulogsize: uint32_t,
        pub iprop_poll_time: krb5_deltat,
        pub iprop_logfile: *mut libc::c_char,
        pub iprop_port: libc::c_int,
        pub iprop_resync_timeout: libc::c_int,
        pub kadmind_listen: *mut libc::c_char,
        pub kpasswd_listen: *mut libc::c_char,
        pub iprop_listen: *mut libc::c_char,
    }
    #[c2rust::src_loc = "241:1"]
    pub type kadm5_config_params = _kadm5_config_params;
    use super::krb5_h::{krb5_principal, krb5_timestamp, krb5_deltat,
                        krb5_flags, krb5_kvno, krb5_int16, krb5_enctype,
                        krb5_int32, krb5_context, krb5_error_code,
                        krb5_principal_data, krb5_keyblock, _krb5_context,
                        _krb5_ccache, krb5_ccache, krb5_ui_4};
    use super::kdb_h::{krb5_tl_data, krb5_key_data, krb5_key_salt_tuple};
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "465:1"]
        pub fn kadm5_init_krb5_context(_: *mut krb5_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "447:1"]
        pub fn kadm5_free_policy_ent(server_handle: *mut libc::c_void,
                                     ent: kadm5_policy_ent_t) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "444:1"]
        pub fn kadm5_free_principal_ent(server_handle: *mut libc::c_void,
                                        ent: kadm5_principal_ent_t)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "437:1"]
        pub fn kadm5_chpass_principal_util(server_handle: *mut libc::c_void,
                                           princ: krb5_principal,
                                           new_pw: *mut libc::c_char,
                                           ret_pw: *mut *mut libc::c_char,
                                           msg_ret: *mut libc::c_char,
                                           msg_len: libc::c_uint)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "434:1"]
        pub fn kadm5_get_privs(server_handle: *mut libc::c_void,
                               privs: *mut libc::c_long) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "431:1"]
        pub fn kadm5_get_policy(server_handle: *mut libc::c_void,
                                policy: kadm5_policy_t,
                                ent: kadm5_policy_ent_t) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "428:1"]
        pub fn kadm5_modify_policy(server_handle: *mut libc::c_void,
                                   ent: kadm5_policy_ent_t,
                                   mask: libc::c_long) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "426:1"]
        pub fn kadm5_delete_policy(server_handle: *mut libc::c_void,
                                   policy: kadm5_policy_t) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "423:1"]
        pub fn kadm5_create_policy(server_handle: *mut libc::c_void,
                                   ent: kadm5_policy_ent_t,
                                   mask: libc::c_long) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "386:1"]
        pub fn kadm5_randkey_principal(server_handle: *mut libc::c_void,
                                       principal: krb5_principal,
                                       keyblocks: *mut *mut krb5_keyblock,
                                       n_keys: *mut libc::c_int)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "377:1"]
        pub fn kadm5_chpass_principal(server_handle: *mut libc::c_void,
                                      principal: krb5_principal,
                                      pass: *mut libc::c_char) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "373:1"]
        pub fn kadm5_get_principal(server_handle: *mut libc::c_void,
                                   principal: krb5_principal,
                                   ent: kadm5_principal_ent_t,
                                   mask: libc::c_long) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "371:1"]
        pub fn kadm5_rename_principal(server_handle: *mut libc::c_void,
                                      _: krb5_principal, _: krb5_principal)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "368:1"]
        pub fn kadm5_modify_principal(server_handle: *mut libc::c_void,
                                      ent: kadm5_principal_ent_t,
                                      mask: libc::c_long) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn kadm5_delete_principal(server_handle: *mut libc::c_void,
                                      principal: krb5_principal)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "357:1"]
        pub fn kadm5_create_principal(server_handle: *mut libc::c_void,
                                      ent: kadm5_principal_ent_t,
                                      mask: libc::c_long,
                                      pass: *mut libc::c_char) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "356:1"]
        pub fn kadm5_destroy(server_handle: *mut libc::c_void) -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "344:1"]
        pub fn kadm5_init_with_creds(context_0: krb5_context,
                                     client_name: *mut libc::c_char,
                                     cc: krb5_ccache,
                                     service_name: *mut libc::c_char,
                                     params: *mut kadm5_config_params,
                                     struct_version: krb5_ui_4,
                                     api_version: krb5_ui_4,
                                     db_args: *mut *mut libc::c_char,
                                     server_handle: *mut *mut libc::c_void)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "312:1"]
        pub fn kadm5_init(context_0: krb5_context,
                          client_name: *mut libc::c_char,
                          pass: *mut libc::c_char,
                          service_name: *mut libc::c_char,
                          params: *mut kadm5_config_params,
                          struct_version: krb5_ui_4, api_version: krb5_ui_4,
                          db_args: *mut *mut libc::c_char,
                          server_handle: *mut *mut libc::c_void)
         -> kadm5_ret_t;
    }
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src = "/usr/include/stdio.h:3"]
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
        #[c2rust::src_loc = "334:12"]
        pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/string.h:4"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "139:12"]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/strings.h:4"]
pub mod strings_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "116:12"]
        pub fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/tclDecls.h:6"]
pub mod tclDecls_h {
    use super::tcl_h::{Tcl_Interp, Tcl_CmdProc, ClientData, Tcl_CmdDeleteProc,
                       Tcl_Command, Tcl_HashEntry, Tcl_DString, Tcl_HashTable,
                       Tcl_FreeProc};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "55:1"]
        pub fn Tcl_Free(ptr: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "157:1"]
        pub fn Tcl_GetInt(interp: *mut Tcl_Interp, src: *const libc::c_char,
                          intPtr: *mut libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "239:1"]
        pub fn Tcl_AppendElement(interp: *mut Tcl_Interp,
                                 element: *const libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "242:1"]
        pub fn Tcl_AppendResult(interp: *mut Tcl_Interp, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "302:1"]
        pub fn Tcl_CreateCommand(interp: *mut Tcl_Interp,
                                 cmdName: *const libc::c_char,
                                 proc_0: Option<Tcl_CmdProc>,
                                 clientData: ClientData,
                                 deleteProc: Option<Tcl_CmdDeleteProc>)
         -> Tcl_Command;
        #[no_mangle]
        #[c2rust::src_loc = "361:1"]
        pub fn Tcl_DeleteHashEntry(entryPtr: *mut Tcl_HashEntry);
        #[no_mangle]
        #[c2rust::src_loc = "382:1"]
        pub fn Tcl_DStringAppend(dsPtr: *mut Tcl_DString,
                                 bytes: *const libc::c_char,
                                 length: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "385:1"]
        pub fn Tcl_DStringAppendElement(dsPtr: *mut Tcl_DString,
                                        element: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "388:1"]
        pub fn Tcl_DStringEndSublist(dsPtr: *mut Tcl_DString);
        #[no_mangle]
        #[c2rust::src_loc = "390:1"]
        pub fn Tcl_DStringFree(dsPtr: *mut Tcl_DString);
        #[no_mangle]
        #[c2rust::src_loc = "395:1"]
        pub fn Tcl_DStringInit(dsPtr: *mut Tcl_DString);
        #[no_mangle]
        #[c2rust::src_loc = "397:1"]
        pub fn Tcl_DStringResult(interp: *mut Tcl_Interp,
                                 dsPtr: *mut Tcl_DString);
        #[no_mangle]
        #[c2rust::src_loc = "402:1"]
        pub fn Tcl_DStringStartSublist(dsPtr: *mut Tcl_DString);
        #[no_mangle]
        #[c2rust::src_loc = "561:1"]
        pub fn Tcl_InitHashTable(tablePtr: *mut Tcl_HashTable,
                                 keyType: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "656:1"]
        pub fn Tcl_ResetResult(interp: *mut Tcl_Interp);
        #[no_mangle]
        #[c2rust::src_loc = "694:1"]
        pub fn Tcl_SetResult(interp: *mut Tcl_Interp,
                             result: *mut libc::c_char,
                             freeProc: Option<Tcl_FreeProc>);
        #[no_mangle]
        #[c2rust::src_loc = "711:1"]
        pub fn Tcl_SetVar2(interp: *mut Tcl_Interp,
                           part1: *const libc::c_char,
                           part2: *const libc::c_char,
                           newValue: *const libc::c_char, flags: libc::c_int)
         -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "721:1"]
        pub fn Tcl_SplitList(interp: *mut Tcl_Interp,
                             listStr: *const libc::c_char,
                             argcPtr: *mut libc::c_int,
                             argvPtr: *mut *mut *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:11"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __off_t, __off64_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::tcl_h::{ClientData, Tcl_WideInt, Tcl_Interp, Tcl_Command,
                      Tcl_Obj, C2RustUnnamed, C2RustUnnamed_0,
                      C2RustUnnamed_1, Tcl_ObjType, Tcl_SetFromAnyProc,
                      Tcl_UpdateStringProc, Tcl_DupInternalRepProc,
                      Tcl_FreeInternalRepProc, Tcl_CmdDeleteProc, Tcl_CmdProc,
                      Tcl_FreeProc, Tcl_DString, Tcl_HashKeyType,
                      Tcl_FreeHashEntryProc, Tcl_HashEntry, C2RustUnnamed_2,
                      Tcl_HashTable, Tcl_AllocHashEntryProc,
                      Tcl_CompareHashKeysProc, Tcl_HashKeyProc, Tcl_Command_};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_kvno, krb5_enctype, krb5_flags,
                       krb5_timestamp, krb5_deltat, krb5_error_code,
                       krb5_magic, _krb5_data, krb5_data, krb5_principal_data,
                       krb5_principal, krb5_const_principal, krb5_context,
                       _krb5_keyblock, krb5_keyblock, krb5_ccache,
                       _krb5_context, _krb5_ccache, krb5_cc_close,
                       krb5_free_principal, krb5_cc_default, krb5_cc_resolve,
                       krb5_unparse_name, krb5_parse_name};
pub use self::com_err_h::{errcode_t, error_message};
pub use self::kdb_h::{_krb5_tl_data, krb5_tl_data, _krb5_key_data,
                      krb5_key_data, __krb5_key_salt_tuple,
                      krb5_key_salt_tuple};
pub use self::admin_h::{kadm5_policy_t, kadm5_ret_t, _kadm5_principal_ent_t,
                        kadm5_principal_ent_rec, kadm5_principal_ent_t,
                        _kadm5_policy_ent_t, kadm5_policy_ent_rec,
                        kadm5_policy_ent_t, _kadm5_config_params,
                        kadm5_config_params, kadm5_init_krb5_context,
                        kadm5_free_policy_ent, kadm5_free_principal_ent,
                        kadm5_chpass_principal_util, kadm5_get_privs,
                        kadm5_get_policy, kadm5_modify_policy,
                        kadm5_delete_policy, kadm5_create_policy,
                        kadm5_randkey_principal, kadm5_chpass_principal,
                        kadm5_get_principal, kadm5_rename_principal,
                        kadm5_modify_principal, kadm5_delete_principal,
                        kadm5_create_principal, kadm5_destroy,
                        kadm5_init_with_creds, kadm5_init};
use self::stdio_h::{stderr, fprintf, sprintf};
use self::string_h::{memset, strncmp, strdup, strlen};
use self::strings_h::strcasecmp;
use self::tclDecls_h::{Tcl_Free, Tcl_GetInt, Tcl_AppendElement,
                       Tcl_AppendResult, Tcl_CreateCommand,
                       Tcl_DeleteHashEntry, Tcl_DStringAppend,
                       Tcl_DStringAppendElement, Tcl_DStringEndSublist,
                       Tcl_DStringFree, Tcl_DStringInit, Tcl_DStringResult,
                       Tcl_DStringStartSublist, Tcl_InitHashTable,
                       Tcl_ResetResult, Tcl_SetResult, Tcl_SetVar2,
                       Tcl_SplitList};
use self::stdlib_h::{malloc, free, exit};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "18:8"]
pub struct flagval {
    pub name: *mut libc::c_char,
    pub val: krb5_flags,
}
#[c2rust::src_loc = "1569:1"]
pub type init_type = libc::c_uint;
#[c2rust::src_loc = "1569:40"]
pub const INIT_CREDS: init_type = 2;
#[c2rust::src_loc = "1569:29"]
pub const INIT_PASS: init_type = 1;
#[c2rust::src_loc = "1569:18"]
pub const INIT_NONE: init_type = 0;
/* XXX This should probably be in the hash table like server_handle */
#[c2rust::src_loc = "24:21"]
static mut context: krb5_context =
    0 as *const _krb5_context as *mut _krb5_context;
#[c2rust::src_loc = "26:23"]
static mut krb5_flags_array: [flagval; 12] =
    [{
         let mut init =
             flagval{name:
                         b"KRB5_KDB_DISALLOW_POSTDATED\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KRB5_KDB_DISALLOW_FORWARDABLE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x2 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KRB5_KDB_DISALLOW_TGT_BASED\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x4 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KRB5_KDB_DISALLOW_RENEWABLE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x8 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KRB5_KDB_DISALLOW_PROXIABLE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x10 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KRB5_KDB_DISALLOW_DUP_SKEY\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x20 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KRB5_KDB_DISALLOW_ALL_TIX\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x40 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KRB5_KDB_REQUIRES_PRE_AUTH\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x80 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KRB5_KDB_REQUIRES_HW_AUTH\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x100 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KRB5_KDB_REQUIRES_PWCHANGE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x200 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KRB5_KDB_DISALLOW_SVR\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x1000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KRB5_KDB_PWCHANGE_SERVICE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x2000 as libc::c_int,};
         init
     }];
#[c2rust::src_loc = "41:23"]
static mut aux_attributes: [flagval; 1] =
    [{
         let mut init =
             flagval{name:
                         b"KADM5_POLICY\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x800 as libc::c_int,};
         init
     }];
#[c2rust::src_loc = "45:23"]
static mut principal_mask_flags: [flagval; 20] =
    [{
         let mut init =
             flagval{name:
                         b"KADM5_PRINCIPAL\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_PRINC_EXPIRE_TIME\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x2 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_PW_EXPIRATION\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x4 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_LAST_PWD_CHANGE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x8 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_ATTRIBUTES\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x10 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_MAX_LIFE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x20 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_MOD_TIME\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x40 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_MOD_NAME\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x80 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_KVNO\x00" as *const u8 as *const libc::c_char
                             as *mut libc::c_char,
                     val: 0x100 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_MKVNO\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x200 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_AUX_ATTRIBUTES\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x400 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_POLICY\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x800 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_POLICY_CLR\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x1000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_MAX_RLIFE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x2000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_LAST_SUCCESS\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x4000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_LAST_FAILED\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x8000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_FAIL_AUTH_COUNT\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x10000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_KEY_DATA\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x20000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_TL_DATA\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x40000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_PRINCIPAL_NORMAL_MASK\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x41ffff as libc::c_int,};
         init
     }];
#[c2rust::src_loc = "68:23"]
static mut policy_mask_flags: [flagval; 10] =
    [{
         let mut init =
             flagval{name:
                         b"KADM5_POLICY\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x800 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_PW_MAX_LIFE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x4000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_PW_MIN_LIFE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x8000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_PW_MIN_LENGTH\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x10000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_PW_MIN_CLASSES\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x20000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_PW_HISTORY_NUM\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x40000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_REF_COUNT\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x80000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_PW_MAX_FAILURE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x100000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_PW_FAILURE_COUNT_INTERVAL\x00" as *const u8
                             as *const libc::c_char as *mut libc::c_char,
                     val: 0x200000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_PW_LOCKOUT_DURATION\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x400000 as libc::c_int,};
         init
     }];
#[c2rust::src_loc = "81:23"]
static mut config_mask_flags: [flagval; 17] =
    [{
         let mut init =
             flagval{name:
                         b"KADM5_CONFIG_REALM\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_CONFIG_DBNAME\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x2 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_CONFIG_MKEY_NAME\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x4 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_CONFIG_MAX_LIFE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x8 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_CONFIG_MAX_RLIFE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x10 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_CONFIG_EXPIRATION\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x20 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_CONFIG_FLAGS\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x40 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_CONFIG_STASH_FILE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x100 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_CONFIG_ENCTYPE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x200 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_CONFIG_ADBNAME\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x400 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_CONFIG_ADB_LOCKFILE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x800 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_CONFIG_ACL_FILE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x2000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_CONFIG_KADMIND_PORT\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x4000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_CONFIG_ENCTYPES\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x8000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_CONFIG_ADMIN_SERVER\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x10000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_CONFIG_DICT_FILE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x20000 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_CONFIG_MKEY_FROM_KBD\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x40000 as libc::c_int,};
         init
     }];
#[c2rust::src_loc = "101:23"]
static mut priv_flags: [flagval; 4] =
    [{
         let mut init =
             flagval{name:
                         b"KADM5_PRIV_GET\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x1 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_PRIV_ADD\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x2 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_PRIV_MODIFY\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x4 as libc::c_int,};
         init
     },
     {
         let mut init =
             flagval{name:
                         b"KADM5_PRIV_DELETE\x00" as *const u8 as
                             *const libc::c_char as *mut libc::c_char,
                     val: 0x8 as libc::c_int,};
         init
     }];
#[c2rust::src_loc = "109:14"]
static mut arg_error: *mut libc::c_char =
    b"wrong # args\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[c2rust::src_loc = "111:23"]
static mut struct_table: *mut Tcl_HashTable =
    0 as *const Tcl_HashTable as *mut Tcl_HashTable;
#[c2rust::src_loc = "113:1"]
unsafe extern "C" fn put_server_handle(mut interp: *mut Tcl_Interp,
                                       mut handle: *mut libc::c_void,
                                       mut name: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut newPtr: libc::c_int = 0 as libc::c_int;
    static mut buf: [libc::c_char; 20] = [0; 20];
    let mut entry: *mut Tcl_HashEntry = 0 as *mut Tcl_HashEntry;
    if struct_table.is_null() {
        struct_table =
            malloc(::std::mem::size_of::<Tcl_HashTable>() as libc::c_ulong) as
                *mut Tcl_HashTable;
        if struct_table.is_null() {
            fprintf(stderr,
                    b"Out of memory!\n\x00" as *const u8 as
                        *const libc::c_char);
            exit(1 as libc::c_int);
            /* XXX */
        }
        Tcl_InitHashTable(struct_table, 0 as libc::c_int);
    }
    loop  {
        sprintf(buf.as_mut_ptr(),
                b"kadm5_handle%d\x00" as *const u8 as *const libc::c_char, i);
        entry =
            Some((*struct_table).createProc.expect("non-null function pointer")).expect("non-null function pointer")(struct_table,
                                                                                                                     buf.as_mut_ptr()
                                                                                                                         as
                                                                                                                         *const libc::c_char,
                                                                                                                     &mut newPtr);
        i += 1;
        if !(newPtr == 0) { break ; }
    }
    (*entry).clientData = handle;
    *name = buf.as_mut_ptr();
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "141:1"]
unsafe extern "C" fn get_server_handle(mut interp: *mut Tcl_Interp,
                                       mut name: *const libc::c_char,
                                       mut handle: *mut *mut libc::c_void)
 -> libc::c_int {
    let mut entry: *mut Tcl_HashEntry = 0 as *mut Tcl_HashEntry;
    if strcasecmp(name, b"null\x00" as *const u8 as *const libc::c_char) == 0
       {
        *handle = 0 as *mut libc::c_void
    } else {
        if !(!struct_table.is_null() &&
                 {
                     entry =
                         Some((*struct_table).findProc.expect("non-null function pointer")).expect("non-null function pointer")(struct_table,
                                                                                                                                name);
                     !entry.is_null()
                 }) {
            Tcl_AppendResult(interp,
                             b"unknown server handle \x00" as *const u8 as
                                 *const libc::c_char, name, 0 as libc::c_int);
            return 1 as libc::c_int
        }
        *handle = (*entry).clientData
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "159:1"]
unsafe extern "C" fn remove_server_handle(mut interp: *mut Tcl_Interp,
                                          mut name: *const libc::c_char)
 -> libc::c_int {
    let mut entry: *mut Tcl_HashEntry = 0 as *mut Tcl_HashEntry;
    if !(!struct_table.is_null() &&
             {
                 entry =
                     Some((*struct_table).findProc.expect("non-null function pointer")).expect("non-null function pointer")(struct_table,
                                                                                                                            name);
                 !entry.is_null()
             }) {
        Tcl_AppendResult(interp,
                         b"unknown server handle \x00" as *const u8 as
                             *const libc::c_char, name, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    (*entry).clientData = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "190:1"]
unsafe extern "C" fn create_flag_table(mut flags: *mut flagval,
                                       mut size: libc::c_int)
 -> *mut Tcl_HashTable {
    let mut table: *mut Tcl_HashTable = 0 as *mut Tcl_HashTable;
    let mut entry: *mut Tcl_HashEntry = 0 as *mut Tcl_HashEntry;
    let mut i: libc::c_int = 0;
    table =
        malloc(::std::mem::size_of::<Tcl_HashTable>() as libc::c_ulong) as
            *mut Tcl_HashTable;
    if table.is_null() {
        fprintf(stderr,
                b"Out of memory!\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
        /* XXX */
    }
    Tcl_InitHashTable(table, 0 as libc::c_int);
    i = 0 as libc::c_int;
    while i < size {
        let mut newPtr: libc::c_int = 0;
        entry =
            Some((*table).createProc.expect("non-null function pointer")).expect("non-null function pointer")(table,
                                                                                                              (*flags.offset(i
                                                                                                                                 as
                                                                                                                                 isize)).name
                                                                                                                  as
                                                                                                                  *const libc::c_char,
                                                                                                              &mut newPtr);
        if entry.is_null() {
            fprintf(stderr,
                    b"Out of memory!\n\x00" as *const u8 as
                        *const libc::c_char);
            exit(1 as libc::c_int);
            /* XXX */
        }
        (*entry).clientData =
            &mut (*flags.offset(i as isize)).val as *mut krb5_flags as
                ClientData;
        i += 1
    }
    return table;
}
#[c2rust::src_loc = "218:1"]
unsafe extern "C" fn unparse_str(mut in_str: *mut libc::c_char)
 -> *mut Tcl_DString {
    let mut str: *mut Tcl_DString = 0 as *mut Tcl_DString;
    str =
        malloc(::std::mem::size_of::<Tcl_DString>() as libc::c_ulong) as
            *mut Tcl_DString;
    if str.is_null() {
        fprintf(stderr,
                b"Out of memory!\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
        /* XXX */
    }
    Tcl_DStringInit(str);
    if in_str.is_null() {
        Tcl_DStringAppend(str,
                          b"null\x00" as *const u8 as *const libc::c_char,
                          -(1 as libc::c_int));
    } else { Tcl_DStringAppend(str, in_str, -(1 as libc::c_int)); }
    return str;
}
#[c2rust::src_loc = "241:1"]
unsafe extern "C" fn parse_str(mut interp: *mut Tcl_Interp,
                               mut in_str: *const libc::c_char,
                               mut out_str: *mut *mut libc::c_char)
 -> libc::c_int {
    if in_str.is_null() {
        *out_str = 0 as *mut libc::c_char
    } else if strcasecmp(in_str,
                         b"null\x00" as *const u8 as *const libc::c_char) == 0
     {
        *out_str = 0 as *mut libc::c_char
    } else { *out_str = in_str as *mut libc::c_char }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "256:1"]
unsafe extern "C" fn set_ok(mut interp: *mut Tcl_Interp,
                            mut string: *mut libc::c_char) {
    Tcl_SetResult(interp,
                  b"OK\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, None);
    Tcl_AppendElement(interp,
                      b"KADM5_OK\x00" as *const u8 as *const libc::c_char);
    Tcl_AppendElement(interp, string);
}
#[c2rust::src_loc = "265:1"]
unsafe extern "C" fn unparse_err(mut code: kadm5_ret_t) -> *mut Tcl_DString {
    let mut code_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut error_string: *const libc::c_char = 0 as *const libc::c_char;
    let mut dstring: *mut Tcl_DString = 0 as *mut Tcl_DString;
    match code {
        43787520 => {
            code_string =
                b"KADM5_FAILURE\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787521 => {
            code_string =
                b"KADM5_AUTH_GET\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787522 => {
            code_string =
                b"KADM5_AUTH_ADD\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787523 => {
            code_string =
                b"KADM5_AUTH_MODIFY\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        43787524 => {
            code_string =
                b"KADM5_AUTH_DELETE\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        43787525 => {
            code_string =
                b"KADM5_AUTH_INSUFFICIENT\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787526 => {
            code_string =
                b"KADM5_BAD_DB\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787527 => {
            code_string =
                b"KADM5_DUP\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787528 => {
            code_string =
                b"KADM5_RPC_ERROR\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787529 => {
            code_string =
                b"KADM5_NO_SRV\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787530 => {
            code_string =
                b"KADM5_BAD_HIST_KEY\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        43787531 => {
            code_string =
                b"KADM5_NOT_INIT\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787548 => {
            code_string =
                b"KADM5_INIT\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787549 => {
            code_string =
                b"KADM5_BAD_PASSWORD\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        43787532 => {
            code_string =
                b"KADM5_UNK_PRINC\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787533 => {
            code_string =
                b"KADM5_UNK_POLICY\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787534 => {
            code_string =
                b"KADM5_BAD_MASK\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787535 => {
            code_string =
                b"KADM5_BAD_CLASS\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787536 => {
            code_string =
                b"KADM5_BAD_LENGTH\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787537 => {
            code_string =
                b"KADM5_BAD_POLICY\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787540 => {
            code_string =
                b"KADM5_BAD_HISTORY\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        43787538 => {
            code_string =
                b"KADM5_BAD_PRINCIPAL\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        43787539 => {
            code_string =
                b"KADM5_BAD_AUX_ATTR\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        43787542 => {
            code_string =
                b"KADM5_PASS_Q_TOOSHORT\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787543 => {
            code_string =
                b"KADM5_PASS_Q_CLASS\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        43787544 => {
            code_string =
                b"KADM5_PASS_Q_DICT\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        43787545 => {
            code_string =
                b"KADM5_PASS_REUSE\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787546 => {
            code_string =
                b"KADM5_PASS_TOOSOON\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        43787547 => {
            code_string =
                b"KADM5_POLICY_REF\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787550 => {
            code_string =
                b"KADM5_PROTECT_PRINCIPAL\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787551 => {
            code_string =
                b"KADM5_BAD_SERVER_HANDLE\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787552 => {
            code_string =
                b"KADM5_BAD_STRUCT_VERSION\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787553 => {
            code_string =
                b"KADM5_OLD_STRUCT_VERSION\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787554 => {
            code_string =
                b"KADM5_NEW_STRUCT_VERSION\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787555 => {
            code_string =
                b"KADM5_BAD_API_VERSION\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787556 => {
            code_string =
                b"KADM5_OLD_LIB_API_VERSION\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787557 => {
            code_string =
                b"KADM5_OLD_SERVER_API_VERSION\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787558 => {
            code_string =
                b"KADM5_NEW_LIB_API_VERSION\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787559 => {
            code_string =
                b"KADM5_NEW_SERVER_API_VERSION\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787560 => {
            code_string =
                b"KADM5_SECURE_PRINC_MISSING\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787561 => {
            code_string =
                b"KADM5_NO_RENAME_SALT\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787562 => {
            code_string =
                b"KADM5_BAD_CLIENT_PARAMS\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787563 => {
            code_string =
                b"KADM5_BAD_SERVER_PARAMS\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787564 => {
            code_string =
                b"KADM5_AUTH_LIST\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787565 => {
            code_string =
                b"KADM5_AUTH_CHANGEPW\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        43787566 => {
            code_string =
                b"KADM5_GSS_ERROR\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        43787567 => {
            code_string =
                b"KADM5_BAD_TL_TYPE\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        43787568 => {
            code_string =
                b"KADM5_MISSING_CONF_PARAMS\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787569 => {
            code_string =
                b"KADM5_BAD_SERVER_NAME\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787574 => {
            code_string =
                b"KADM5_MISSING_KRB5_CONF_PARAMS\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        43787575 => {
            code_string =
                b"KADM5_XDR_FAILURE\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        43787576 => {
            code_string =
                b"KADM5_CANT_RESOLVE\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        28810241 => {
            code_string =
                b"OSA_ADB_DUP\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        28810242 => {
            code_string =
                b"ENOENT\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        28810243 => {
            code_string =
                b"OSA_ADB_DBINIT\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        28810244 => {
            code_string =
                b"Bad policy name\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        28810245 => {
            code_string =
                b"Bad principal name\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        28810246 => {
            code_string =
                b"Invalid database.\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        28810247 => {
            code_string =
                b"OSA_ADB_XDR_FAILURE\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        28810249 => {
            code_string =
                b"OSA_ADB_BADLOCKMODE\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        28810250 => {
            code_string =
                b"OSA_ADB_CANTLOCK_DB\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        28810251 => {
            code_string =
                b"OSA_ADB_NOTLOCKED\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        28810252 => {
            code_string =
                b"OSA_ADB_NOLOCKFILE\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        28810253 => {
            code_string =
                b"OSA_ADB_NOEXCL_PERM\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        -1780008447 => {
            code_string =
                b"KRB5_KDB_INUSE\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        -1780008446 => {
            code_string =
                b"KRB5_KDB_UK_SERROR\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        -1780008445 => {
            code_string =
                b"KRB5_KDB_UK_RERROR\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        -1780008444 => {
            code_string =
                b"KRB5_KDB_UNAUTH\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        -1780008443 => {
            code_string =
                b"KRB5_KDB_NOENTRY\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        -1780008442 => {
            code_string =
                b"KRB5_KDB_ILL_WILDCARD\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -1780008441 => {
            code_string =
                b"KRB5_KDB_DB_INUSE\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        -1780008440 => {
            code_string =
                b"KRB5_KDB_DB_CHANGED\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        -1780008439 => {
            code_string =
                b"KRB5_KDB_TRUNCATED_RECORD\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -1780008438 => {
            code_string =
                b"KRB5_KDB_RECURSIVELOCK\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -1780008437 => {
            code_string =
                b"KRB5_KDB_NOTLOCKED\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        -1780008436 => {
            code_string =
                b"KRB5_KDB_BADLOCKMODE\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -1780008435 => {
            code_string =
                b"KRB5_KDB_DBNOTINITED\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -1780008434 => {
            code_string =
                b"KRB5_KDB_DBINITED\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        -1780008433 => {
            code_string =
                b"KRB5_KDB_ILLDIRECTION\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -1780008432 => {
            code_string =
                b"KRB5_KDB_NOMASTERKEY\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -1780008431 => {
            code_string =
                b"KRB5_KDB_BADMASTERKEY\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -1780008430 => {
            code_string =
                b"KRB5_KDB_INVALIDKEYSIZE\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -1780008429 => {
            code_string =
                b"KRB5_KDB_CANTREAD_STORED\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -1780008428 => {
            code_string =
                b"KRB5_KDB_BADSTORED_MKEY\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -1780008424 => {
            code_string =
                b"KRB5_KDB_CANTLOCK_DB\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -1780008423 => {
            code_string =
                b"KRB5_KDB_DB_CORRUPT\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        -1765328251 => {
            code_string =
                b"KRB5_PARSE_ILLCHAR\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        -1765328250 => {
            code_string =
                b"KRB5_PARSE_MALFORMED\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -1765328377 => {
            code_string =
                b"KRB5KDC_ERR_S_PRINCIPAL_UNKNOWN\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -1765328230 => {
            code_string =
                b"KRB5_REALM_UNKNOWN\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
        }
        -1765328228 => {
            code_string =
                b"KRB5_KDC_UNREACH\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        -1765328237 => {
            code_string =
                b"KRB5_KDCREP_MODIFIED\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -1765328353 => {
            code_string =
                b"KRB5KRB_AP_ERR_BAD_INTEGRITY\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -1765328378 => {
            code_string =
                b"KRB5KDC_ERR_C_PRINCIPAL_UNKNOWN\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -1765328248 => {
            code_string =
                b"KRB5_CONFIG_BADFORMAT\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char
        }
        -1765328243 => {
            code_string =
                b"KRB5_CC_NOTFOUND\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        -1765328189 => {
            code_string =
                b"KRB5_FCC_NOFILE\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        22 => {
            code_string =
                b"EINVAL\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        2 => {
            code_string =
                b"ENOENT\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
        _ => {
            fprintf(stderr,
                    b"**** CODE %ld (%s) ***\n\x00" as *const u8 as
                        *const libc::c_char, code, error_message(code));
            code_string =
                b"UNKNOWN\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
    }
    error_string = error_message(code);
    dstring =
        malloc(::std::mem::size_of::<Tcl_DString>() as libc::c_ulong) as
            *mut Tcl_DString;
    if dstring.is_null() {
        fprintf(stderr,
                b"Out of memory!\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
        /* XXX Do we really want to exit?  Ok if this is */
        /* just a test program, but what about if it gets */
        /* used for other things later? */
    }
    Tcl_DStringInit(dstring);
    if !(!Tcl_DStringAppendElement(dstring,
                                   b"ERROR\x00" as *const u8 as
                                       *const libc::c_char).is_null() &&
             !Tcl_DStringAppendElement(dstring, code_string).is_null() &&
             !Tcl_DStringAppendElement(dstring, error_string).is_null()) {
        fprintf(stderr,
                b"Out of memory!\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
        /* XXX */
    }
    return dstring;
}
#[c2rust::src_loc = "444:1"]
unsafe extern "C" fn stash_error(mut interp: *mut Tcl_Interp,
                                 mut code: krb5_error_code) {
    let mut dstring: *mut Tcl_DString = unparse_err(code as kadm5_ret_t);
    Tcl_DStringResult(interp, dstring);
    Tcl_DStringFree(dstring);
    free(dstring as *mut libc::c_void);
}
#[c2rust::src_loc = "452:1"]
unsafe extern "C" fn unparse_key_data(mut key_data: *mut krb5_key_data,
                                      mut n_key_data: libc::c_int)
 -> *mut Tcl_DString {
    let mut str: *mut Tcl_DString = 0 as *mut Tcl_DString;
    let mut buf: [libc::c_char; 2048] = [0; 2048];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    str =
        malloc(::std::mem::size_of::<Tcl_DString>() as libc::c_ulong) as
            *mut Tcl_DString;
    if str.is_null() {
        fprintf(stderr,
                b"Out of memory!\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
        /* XXX */
    }
    Tcl_DStringInit(str);
    i = 0 as libc::c_int;
    while i < n_key_data {
        let mut key: *mut krb5_key_data =
            &mut *key_data.offset(i as isize) as *mut krb5_key_data;
        Tcl_DStringStartSublist(str);
        sprintf(buf.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char,
                (*key).key_data_type[0 as libc::c_int as usize] as
                    libc::c_int);
        Tcl_DStringAppendElement(str, buf.as_mut_ptr());
        sprintf(buf.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char,
                if (*key).key_data_ver as libc::c_int > 1 as libc::c_int {
                    (*key).key_data_type[1 as libc::c_int as usize] as
                        libc::c_int
                } else { -(1 as libc::c_int) });
        Tcl_DStringAppendElement(str, buf.as_mut_ptr());
        if !(*key).key_data_contents[0 as libc::c_int as usize].is_null() {
            sprintf(buf.as_mut_ptr(),
                    b"0x\x00" as *const u8 as *const libc::c_char);
            j = 0 as libc::c_int;
            while j <
                      (*key).key_data_length[0 as libc::c_int as usize] as
                          libc::c_int {
                sprintf(buf.as_mut_ptr().offset((2 as libc::c_int *
                                                     (j + 1 as libc::c_int))
                                                    as isize),
                        b"%02x\x00" as *const u8 as *const libc::c_char,
                        *(*key).key_data_contents[0 as libc::c_int as
                                                      usize].offset(j as
                                                                        isize)
                            as libc::c_int);
                j += 1
            }
        } else { *buf.as_mut_ptr() = '\u{0}' as i32 as libc::c_char }
        Tcl_DStringAppendElement(str, buf.as_mut_ptr());
        Tcl_DStringEndSublist(str);
        i += 1
    }
    return str;
}
#[c2rust::src_loc = "487:1"]
unsafe extern "C" fn unparse_tl_data(mut tl_data: *mut krb5_tl_data,
                                     mut n_tl_data: libc::c_int)
 -> *mut Tcl_DString {
    let mut str: *mut Tcl_DString = 0 as *mut Tcl_DString;
    let mut buf: [libc::c_char; 2048] = [0; 2048];
    str =
        malloc(::std::mem::size_of::<Tcl_DString>() as libc::c_ulong) as
            *mut Tcl_DString;
    if str.is_null() {
        fprintf(stderr,
                b"Out of memory!\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
        /* XXX */
    }
    Tcl_DStringInit(str);
    Tcl_DStringStartSublist(str);
    while !tl_data.is_null() {
        Tcl_DStringStartSublist(str);
        sprintf(buf.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char,
                (*tl_data).tl_data_type as libc::c_int);
        Tcl_DStringAppendElement(str, buf.as_mut_ptr());
        sprintf(buf.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const libc::c_char,
                (*tl_data).tl_data_length as libc::c_int);
        Tcl_DStringAppendElement(str, buf.as_mut_ptr());
        Tcl_DStringAppend(str, b" \x00" as *const u8 as *const libc::c_char,
                          1 as libc::c_int);
        Tcl_DStringAppend(str,
                          (*tl_data).tl_data_contents as *mut libc::c_char,
                          (*tl_data).tl_data_length as libc::c_int);
        Tcl_DStringEndSublist(str);
        tl_data = (*tl_data).tl_data_next
    }
    Tcl_DStringEndSublist(str);
    return str;
}
#[c2rust::src_loc = "515:1"]
unsafe extern "C" fn unparse_flags(mut array: *mut flagval,
                                   mut size: libc::c_int,
                                   mut flags: krb5_int32)
 -> *mut Tcl_DString {
    let mut i: libc::c_int = 0;
    let mut str: *mut Tcl_DString = 0 as *mut Tcl_DString;
    str =
        malloc(::std::mem::size_of::<Tcl_DString>() as libc::c_ulong) as
            *mut Tcl_DString;
    if str.is_null() {
        fprintf(stderr,
                b"Out of memory!\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
        /* XXX */
    }
    Tcl_DStringInit(str);
    i = 0 as libc::c_int;
    while i < size {
        if flags & (*array.offset(i as isize)).val != 0 {
            Tcl_DStringAppendElement(str, (*array.offset(i as isize)).name);
        }
        i += 1
    }
    return str;
}
#[c2rust::src_loc = "538:1"]
unsafe extern "C" fn parse_flags(mut interp: *mut Tcl_Interp,
                                 mut table: *mut Tcl_HashTable,
                                 mut array: *mut flagval,
                                 mut size: libc::c_int,
                                 mut str: *const libc::c_char,
                                 mut flags: *mut krb5_flags) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut argc: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut retcode: libc::c_int = 0 as libc::c_int;
    let mut argv: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut entry: *mut Tcl_HashEntry = 0 as *mut Tcl_HashEntry;
    if Tcl_GetInt(interp, str, &mut tmp) == 0 as libc::c_int {
        *flags = tmp;
        return 0 as libc::c_int
    }
    Tcl_ResetResult(interp);
    if Tcl_SplitList(interp, str, &mut argc, &mut argv) != 0 as libc::c_int {
        return 1 as libc::c_int
    }
    if table.is_null() { table = create_flag_table(array, size) }
    *flags = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < argc {
        entry =
            Some((*table).findProc.expect("non-null function pointer")).expect("non-null function pointer")(table,
                                                                                                            *argv.offset(i
                                                                                                                             as
                                                                                                                             isize));
        if entry.is_null() {
            Tcl_AppendResult(interp,
                             b"unknown krb5 flag \x00" as *const u8 as
                                 *const libc::c_char,
                             *argv.offset(i as isize), 0 as libc::c_int);
            retcode = 1 as libc::c_int;
            break ;
        } else { *flags |= *((*entry).clientData as *mut krb5_flags); i += 1 }
    }
    Tcl_Free(argv as *mut libc::c_char);
    return retcode;
}
#[c2rust::src_loc = "575:1"]
unsafe extern "C" fn unparse_privs(mut flags: krb5_flags)
 -> *mut Tcl_DString {
    return unparse_flags(priv_flags.as_mut_ptr(),
                         (::std::mem::size_of::<[flagval; 4]>() as
                              libc::c_ulong).wrapping_div(::std::mem::size_of::<flagval>()
                                                              as
                                                              libc::c_ulong)
                             as libc::c_int, flags);
}
#[c2rust::src_loc = "582:1"]
unsafe extern "C" fn unparse_krb5_flags(mut flags: krb5_flags)
 -> *mut Tcl_DString {
    return unparse_flags(krb5_flags_array.as_mut_ptr(),
                         (::std::mem::size_of::<[flagval; 12]>() as
                              libc::c_ulong).wrapping_div(::std::mem::size_of::<flagval>()
                                                              as
                                                              libc::c_ulong)
                             as libc::c_int, flags);
}
#[c2rust::src_loc = "588:1"]
unsafe extern "C" fn parse_krb5_flags(mut interp: *mut Tcl_Interp,
                                      mut str: *const libc::c_char,
                                      mut flags: *mut krb5_flags)
 -> libc::c_int {
    let mut tmp: krb5_flags = 0;
    static mut table: *mut Tcl_HashTable =
        0 as *const Tcl_HashTable as *mut Tcl_HashTable;
    let mut tcl_ret: libc::c_int = 0;
    tcl_ret =
        parse_flags(interp, table, krb5_flags_array.as_mut_ptr(),
                    (::std::mem::size_of::<[flagval; 12]>() as
                         libc::c_ulong).wrapping_div(::std::mem::size_of::<flagval>()
                                                         as libc::c_ulong) as
                        libc::c_int, str, &mut tmp);
    if tcl_ret != 0 as libc::c_int { return tcl_ret }
    *flags = tmp;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "606:1"]
unsafe extern "C" fn unparse_aux_attributes(mut flags: krb5_int32)
 -> *mut Tcl_DString {
    return unparse_flags(aux_attributes.as_mut_ptr(),
                         (::std::mem::size_of::<[flagval; 1]>() as
                              libc::c_ulong).wrapping_div(::std::mem::size_of::<flagval>()
                                                              as
                                                              libc::c_ulong)
                             as libc::c_int, flags);
}
#[c2rust::src_loc = "613:1"]
unsafe extern "C" fn parse_aux_attributes(mut interp: *mut Tcl_Interp,
                                          mut str: *const libc::c_char,
                                          mut flags: *mut libc::c_long)
 -> libc::c_int {
    let mut tmp: krb5_flags = 0;
    static mut table: *mut Tcl_HashTable =
        0 as *const Tcl_HashTable as *mut Tcl_HashTable;
    let mut tcl_ret: libc::c_int = 0;
    tcl_ret =
        parse_flags(interp, table, aux_attributes.as_mut_ptr(),
                    (::std::mem::size_of::<[flagval; 1]>() as
                         libc::c_ulong).wrapping_div(::std::mem::size_of::<flagval>()
                                                         as libc::c_ulong) as
                        libc::c_int, str, &mut tmp);
    if tcl_ret != 0 as libc::c_int { return tcl_ret }
    *flags = tmp as libc::c_long;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "631:1"]
unsafe extern "C" fn parse_principal_mask(mut interp: *mut Tcl_Interp,
                                          mut str: *const libc::c_char,
                                          mut flags: *mut krb5_int32)
 -> libc::c_int {
    let mut tmp: krb5_flags = 0;
    static mut table: *mut Tcl_HashTable =
        0 as *const Tcl_HashTable as *mut Tcl_HashTable;
    let mut tcl_ret: libc::c_int = 0;
    tcl_ret =
        parse_flags(interp, table, principal_mask_flags.as_mut_ptr(),
                    (::std::mem::size_of::<[flagval; 20]>() as
                         libc::c_ulong).wrapping_div(::std::mem::size_of::<flagval>()
                                                         as libc::c_ulong) as
                        libc::c_int, str, &mut tmp);
    if tcl_ret != 0 as libc::c_int { return tcl_ret }
    *flags = tmp;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "649:1"]
unsafe extern "C" fn parse_policy_mask(mut interp: *mut Tcl_Interp,
                                       mut str: *const libc::c_char,
                                       mut flags: *mut krb5_int32)
 -> libc::c_int {
    let mut tmp: krb5_flags = 0;
    static mut table: *mut Tcl_HashTable =
        0 as *const Tcl_HashTable as *mut Tcl_HashTable;
    let mut tcl_ret: libc::c_int = 0;
    tcl_ret =
        parse_flags(interp, table, policy_mask_flags.as_mut_ptr(),
                    (::std::mem::size_of::<[flagval; 10]>() as
                         libc::c_ulong).wrapping_div(::std::mem::size_of::<flagval>()
                                                         as libc::c_ulong) as
                        libc::c_int, str, &mut tmp);
    if tcl_ret != 0 as libc::c_int { return tcl_ret }
    *flags = tmp;
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "668:1"]
unsafe extern "C" fn unparse_principal_ent(mut princ: kadm5_principal_ent_t,
                                           mut mask: krb5_int32)
 -> *mut Tcl_DString {
    let mut str: *mut Tcl_DString = 0 as *mut Tcl_DString;
    let mut tmp_dstring: *mut Tcl_DString = 0 as *mut Tcl_DString;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 20] = [0; 20];
    let mut krb5_ret: krb5_error_code = 0;
    str =
        malloc(::std::mem::size_of::<Tcl_DString>() as libc::c_ulong) as
            *mut Tcl_DString;
    if str.is_null() {
        fprintf(stderr,
                b"Out of memory!\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
        /* XXX */
    } /* It looks to me from looking at the library source */
    Tcl_DStringInit(str);
    tmp = 0 as *mut libc::c_char;
    /* code for krb5_parse_name that the pointer passed into */
    /* it should be initialized to 0 if I want it do be */
    /* allocated automatically. */
    if mask & 0x1 as libc::c_int != 0 {
        krb5_ret =
            krb5_unparse_name(context,
                              (*princ).principal as krb5_const_principal,
                              &mut tmp);
        if krb5_ret != 0 {
            /* XXX Do we want to return an error?  Not sure. */
            Tcl_DStringAppendElement(str,
                                     b"[unparseable principal]\x00" as
                                         *const u8 as *const libc::c_char);
        } else {
            Tcl_DStringAppendElement(str, tmp);
            free(tmp as *mut libc::c_void);
        }
    } else {
        Tcl_DStringAppendElement(str,
                                 b"null\x00" as *const u8 as
                                     *const libc::c_char);
    }
    sprintf(buf.as_mut_ptr(), b"%u\x00" as *const u8 as *const libc::c_char,
            (*princ).princ_expire_time as libc::c_uint);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    sprintf(buf.as_mut_ptr(), b"%u\x00" as *const u8 as *const libc::c_char,
            (*princ).last_pwd_change as libc::c_uint);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    sprintf(buf.as_mut_ptr(), b"%u\x00" as *const u8 as *const libc::c_char,
            (*princ).pw_expiration as libc::c_uint);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    sprintf(buf.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            (*princ).max_life);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    tmp = 0 as *mut libc::c_char;
    if mask & 0x80 as libc::c_int != 0 {
        krb5_ret =
            krb5_unparse_name(context,
                              (*princ).mod_name as krb5_const_principal,
                              &mut tmp);
        if krb5_ret != 0 {
            /* XXX */
            Tcl_DStringAppendElement(str,
                                     b"[unparseable principal]\x00" as
                                         *const u8 as *const libc::c_char);
        } else {
            Tcl_DStringAppendElement(str, tmp);
            free(tmp as *mut libc::c_void);
        }
    } else {
        Tcl_DStringAppendElement(str,
                                 b"null\x00" as *const u8 as
                                     *const libc::c_char);
    }
    sprintf(buf.as_mut_ptr(), b"%u\x00" as *const u8 as *const libc::c_char,
            (*princ).mod_date as libc::c_uint);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    if mask & 0x10 as libc::c_int != 0 {
        tmp_dstring = unparse_krb5_flags((*princ).attributes);
        Tcl_DStringAppendElement(str, (*tmp_dstring).string);
        Tcl_DStringFree(tmp_dstring);
        free(tmp_dstring as *mut libc::c_void);
    } else {
        Tcl_DStringAppendElement(str,
                                 b"null\x00" as *const u8 as
                                     *const libc::c_char);
    }
    sprintf(buf.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            (*princ).kvno);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    sprintf(buf.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            (*princ).mkvno);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    /* XXX This may be dangerous, because the contents of the policy */
    /* field are undefined if the POLICY bit isn't set.  However, I */
    /* think it's a bug for the field not to be null in that case */
    /* anyway, so we should assume that it will be null so that we'll */
    /* catch it if it isn't. */
    tmp_dstring = unparse_str((*princ).policy);
    Tcl_DStringAppendElement(str, (*tmp_dstring).string);
    Tcl_DStringFree(tmp_dstring);
    free(tmp_dstring as *mut libc::c_void);
    tmp_dstring =
        unparse_aux_attributes((*princ).aux_attributes as krb5_int32);
    Tcl_DStringAppendElement(str, (*tmp_dstring).string);
    Tcl_DStringFree(tmp_dstring);
    free(tmp_dstring as *mut libc::c_void);
    sprintf(buf.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            (*princ).max_renewable_life);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    sprintf(buf.as_mut_ptr(), b"%u\x00" as *const u8 as *const libc::c_char,
            (*princ).last_success as libc::c_uint);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    sprintf(buf.as_mut_ptr(), b"%u\x00" as *const u8 as *const libc::c_char,
            (*princ).last_failed as libc::c_uint);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    sprintf(buf.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            (*princ).fail_auth_count);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    sprintf(buf.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            (*princ).n_key_data as libc::c_int);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    sprintf(buf.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            (*princ).n_tl_data as libc::c_int);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    tmp_dstring =
        unparse_key_data((*princ).key_data,
                         (*princ).n_key_data as libc::c_int);
    Tcl_DStringAppendElement(str, (*tmp_dstring).string);
    Tcl_DStringFree(tmp_dstring);
    free(tmp_dstring as *mut libc::c_void);
    tmp_dstring =
        unparse_tl_data((*princ).tl_data, (*princ).n_tl_data as libc::c_int);
    Tcl_DStringAppendElement(str, (*tmp_dstring).string);
    Tcl_DStringFree(tmp_dstring);
    free(tmp_dstring as *mut libc::c_void);
    return str;
}
#[c2rust::src_loc = "789:1"]
unsafe extern "C" fn parse_keysalts(mut interp: *mut Tcl_Interp,
                                    mut list: *const libc::c_char,
                                    mut keysalts:
                                        *mut *mut krb5_key_salt_tuple,
                                    mut num_keysalts: libc::c_int)
 -> libc::c_int {
    let mut argv: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut argv1: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut argc: libc::c_int = 0;
    let mut argc1: libc::c_int = 0;
    let mut retcode: libc::c_int = 0;
    *keysalts = 0 as *mut krb5_key_salt_tuple;
    if list.is_null() { return 0 as libc::c_int }
    retcode = Tcl_SplitList(interp, list, &mut argc, &mut argv);
    if retcode != 0 as libc::c_int { return retcode }
    if argc != num_keysalts {
        Tcl_SetResult(interp,
                      b"wrong number of keysalts\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, None);
        retcode = 1 as libc::c_int
    } else {
        *keysalts =
            malloc((::std::mem::size_of::<krb5_key_salt_tuple>() as
                        libc::c_ulong).wrapping_mul(num_keysalts as
                                                        libc::c_ulong)) as
                *mut krb5_key_salt_tuple;
        i = 0 as libc::c_int;
        while i < num_keysalts {
            retcode =
                Tcl_SplitList(interp, *argv.offset(i as isize), &mut argc1,
                              &mut argv1);
            if retcode != 0 as libc::c_int { break ; }
            if argc1 != 2 as libc::c_int {
                Tcl_SetResult(interp,
                              b"wrong # of fields in keysalt\x00" as *const u8
                                  as *const libc::c_char as *mut libc::c_char,
                              None);
                retcode = 1 as libc::c_int;
                break ;
            } else {
                /* XXX this used to be argv1[1] too! */
                retcode =
                    Tcl_GetInt(interp,
                               *argv1.offset(0 as libc::c_int as isize),
                               &mut tmp);
                if retcode != 0 as libc::c_int {
                    Tcl_AppendElement(interp,
                                      b"while parsing ks_enctype\x00" as
                                          *const u8 as *const libc::c_char);
                    retcode = 1 as libc::c_int;
                    break ;
                } else {
                    (*(*keysalts).offset(i as isize)).ks_enctype = tmp;
                    retcode =
                        Tcl_GetInt(interp,
                                   *argv1.offset(1 as libc::c_int as isize),
                                   &mut tmp);
                    if retcode != 0 as libc::c_int {
                        Tcl_AppendElement(interp,
                                          b"while parsing ks_salttype\x00" as
                                              *const u8 as
                                              *const libc::c_char);
                        break ;
                    } else {
                        (*(*keysalts).offset(i as isize)).ks_salttype = tmp;
                        Tcl_Free(argv1 as *mut libc::c_char);
                        argv1 = 0 as *mut *const libc::c_char;
                        i += 1
                    }
                }
            }
        }
    }
    if !argv1.is_null() { Tcl_Free(argv1 as *mut libc::c_char); }
    Tcl_Free(argv as *mut libc::c_char);
    return retcode;
}
#[c2rust::src_loc = "847:1"]
unsafe extern "C" fn parse_key_data(mut interp: *mut Tcl_Interp,
                                    mut list: *const libc::c_char,
                                    mut key_data: *mut *mut krb5_key_data,
                                    mut n_key_data: libc::c_int)
 -> libc::c_int {
    let mut argv: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut argc: libc::c_int = 0;
    let mut retcode: libc::c_int = 0;
    *key_data = 0 as *mut krb5_key_data;
    if list.is_null() {
        if n_key_data != 0 as libc::c_int {
            Tcl_SetResult(interp,
                          b"wrong number of key_datas\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char, None);
            retcode = 1 as libc::c_int
        } else { return 0 as libc::c_int }
    } else {
        retcode = Tcl_SplitList(interp, list, &mut argc, &mut argv);
        if retcode != 0 as libc::c_int { return retcode }
        if argc != n_key_data {
            Tcl_SetResult(interp,
                          b"wrong number of key_datas\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char, None);
            retcode = 1 as libc::c_int
        } else if argc != 0 as libc::c_int {
            Tcl_SetResult(interp,
                          b"cannot parse key_data yet\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char, None);
            retcode = 1 as libc::c_int
        }
    }
    Tcl_Free(argv as *mut libc::c_char);
    return retcode;
}
#[c2rust::src_loc = "884:1"]
unsafe extern "C" fn parse_tl_data(mut interp: *mut Tcl_Interp,
                                   mut list: *const libc::c_char,
                                   mut tlp: *mut *mut krb5_tl_data,
                                   mut n_tl_data: libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut tl: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut tl2: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut argv: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut argv1: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut argc: libc::c_int = 0;
    let mut argc1: libc::c_int = 0;
    let mut retcode: libc::c_int = 0;
    *tlp = 0 as *mut krb5_tl_data;
    if list.is_null() {
        if n_tl_data != 0 as libc::c_int {
            Tcl_SetResult(interp,
                          b"wrong number of tl_datas\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char, None);
            retcode = 1 as libc::c_int
        } else { return 0 as libc::c_int }
    } else {
        retcode = Tcl_SplitList(interp, list, &mut argc, &mut argv);
        if retcode != 0 as libc::c_int { return retcode }
        if argc != n_tl_data {
            Tcl_SetResult(interp,
                          b"wrong number of tl_datas\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char, None);
            retcode = 1 as libc::c_int
        } else {
            tl2 = 0 as *mut krb5_tl_data;
            tl = tl2;
            i = 0 as libc::c_int;
            while i < n_tl_data {
                tl2 =
                    malloc(::std::mem::size_of::<krb5_tl_data>() as
                               libc::c_ulong) as *mut krb5_tl_data;
                memset(tl2 as *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<krb5_tl_data>() as
                           libc::c_ulong);
                (*tl2).tl_data_next = tl;
                tl = tl2;
                i += 1
            }
            tl2 = tl;
            i = 0 as libc::c_int;
            loop  {
                if !(i < n_tl_data) {
                    current_block = 15004371738079956865;
                    break ;
                }
                retcode =
                    Tcl_SplitList(interp, *argv.offset(i as isize),
                                  &mut argc1, &mut argv1);
                if retcode != 0 as libc::c_int {
                    current_block = 14955125204043203155;
                    break ;
                }
                if argc1 != 3 as libc::c_int {
                    Tcl_SetResult(interp,
                                  b"wrong # of fields in tl_data\x00" as
                                      *const u8 as *const libc::c_char as
                                      *mut libc::c_char, None);
                    retcode = 1 as libc::c_int;
                    current_block = 14955125204043203155;
                    break ;
                } else {
                    retcode =
                        Tcl_GetInt(interp,
                                   *argv1.offset(0 as libc::c_int as isize),
                                   &mut tmp);
                    if retcode != 0 as libc::c_int {
                        Tcl_AppendElement(interp,
                                          b"while parsing tl_data_type\x00" as
                                              *const u8 as
                                              *const libc::c_char);
                        retcode = 1 as libc::c_int;
                        current_block = 14955125204043203155;
                        break ;
                    } else {
                        (*tl).tl_data_type = tmp as krb5_int16;
                        retcode =
                            Tcl_GetInt(interp,
                                       *argv1.offset(1 as libc::c_int as
                                                         isize), &mut tmp);
                        if retcode != 0 as libc::c_int {
                            Tcl_AppendElement(interp,
                                              b"while parsing tl_data_length\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                            retcode = 1 as libc::c_int;
                            current_block = 14955125204043203155;
                            break ;
                        } else {
                            (*tl).tl_data_length = tmp as krb5_ui_2;
                            if (*tl).tl_data_length as libc::c_ulong !=
                                   strlen(*argv1.offset(2 as libc::c_int as
                                                            isize)) {
                                Tcl_SetResult(interp,
                                              b"length != string length\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char, None);
                                retcode = 1 as libc::c_int;
                                current_block = 14955125204043203155;
                                break ;
                            } else {
                                (*tl).tl_data_contents =
                                    strdup(*argv1.offset(2 as libc::c_int as
                                                             isize)) as
                                        *mut krb5_octet;
                                Tcl_Free(argv1 as *mut libc::c_char);
                                argv1 = 0 as *mut *const libc::c_char;
                                tl = (*tl).tl_data_next;
                                i += 1
                            }
                        }
                    }
                }
            }
            match current_block {
                14955125204043203155 => { }
                _ => {
                    if !tl.is_null() {
                        Tcl_SetResult(interp,
                                      b"tl is not NULL!\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char, None);
                        retcode = 1 as libc::c_int
                    } else { *tlp = tl2 }
                }
            }
        }
    }
    if !argv1.is_null() { Tcl_Free(argv1 as *mut libc::c_char); }
    Tcl_Free(argv as *mut libc::c_char);
    return retcode;
}
#[c2rust::src_loc = "970:1"]
unsafe extern "C" fn parse_config_params(mut interp: *mut Tcl_Interp,
                                         mut list: *mut libc::c_char,
                                         mut params: *mut kadm5_config_params)
 -> libc::c_int {
    static mut table: *mut Tcl_HashTable =
        0 as *const Tcl_HashTable as *mut Tcl_HashTable;
    let mut argv: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut argc: libc::c_int = 0;
    let mut retcode: libc::c_int = 0;
    memset(params as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<kadm5_config_params>() as libc::c_ulong);
    if list.is_null() { return 0 as libc::c_int }
    retcode = Tcl_SplitList(interp, list, &mut argc, &mut argv);
    if retcode != 0 as libc::c_int { return retcode }
    if argc != 20 as libc::c_int {
        Tcl_SetResult(interp,
                      b"wrong # args in config params structure\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char, None);
        retcode = 1 as libc::c_int
    } else {
        retcode =
            parse_flags(interp, table, config_mask_flags.as_mut_ptr(),
                        (::std::mem::size_of::<[flagval; 17]>() as
                             libc::c_ulong).wrapping_div(::std::mem::size_of::<flagval>()
                                                             as libc::c_ulong)
                            as libc::c_int,
                        *argv.offset(0 as libc::c_int as isize), &mut tmp);
        if !(retcode != 0 as libc::c_int) {
            (*params).mask = tmp as libc::c_long;
            retcode =
                parse_str(interp, *argv.offset(1 as libc::c_int as isize),
                          &mut (*params).realm);
            if retcode != 0 as libc::c_int {
                Tcl_AppendElement(interp,
                                  b"while parsing realm name\x00" as *const u8
                                      as *const libc::c_char);
                retcode = 1 as libc::c_int
            } else {
                retcode =
                    Tcl_GetInt(interp,
                               *argv.offset(2 as libc::c_int as isize),
                               &mut tmp);
                if retcode != 0 as libc::c_int {
                    Tcl_AppendElement(interp,
                                      b"while parsing kadmind_port\x00" as
                                          *const u8 as *const libc::c_char);
                    retcode = 1 as libc::c_int
                } else {
                    (*params).kadmind_port = tmp;
                    retcode =
                        parse_str(interp,
                                  *argv.offset(3 as libc::c_int as isize),
                                  &mut (*params).admin_server);
                    if retcode != 0 as libc::c_int {
                        Tcl_AppendElement(interp,
                                          b"while parsing profile name\x00" as
                                              *const u8 as
                                              *const libc::c_char);
                        retcode = 1 as libc::c_int
                    } else {
                        retcode =
                            parse_str(interp,
                                      *argv.offset(4 as libc::c_int as isize),
                                      &mut (*params).dbname);
                        if retcode != 0 as libc::c_int {
                            Tcl_AppendElement(interp,
                                              b"while parsing profile name\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                            retcode = 1 as libc::c_int
                        } else {
                            /* Ignore argv[5], which used to set the admin_dbname field.  */
    /* Ignore argv[6], which used to set the admin_lockfile field.  */
    /* Ignore argv[7], which used to set the admin_keytab field.  */
                            retcode =
                                parse_str(interp,
                                          *argv.offset(8 as libc::c_int as
                                                           isize),
                                          &mut (*params).acl_file);
                            if retcode != 0 as libc::c_int {
                                Tcl_AppendElement(interp,
                                                  b"while parsing acl_file name\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                retcode = 1 as libc::c_int
                            } else {
                                retcode =
                                    parse_str(interp,
                                              *argv.offset(9 as libc::c_int as
                                                               isize),
                                              &mut (*params).dict_file);
                                if retcode != 0 as libc::c_int {
                                    Tcl_AppendElement(interp,
                                                      b"while parsing dict_file name\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                    retcode = 1 as libc::c_int
                                } else {
                                    retcode =
                                        Tcl_GetInt(interp,
                                                   *argv.offset(10 as
                                                                    libc::c_int
                                                                    as isize),
                                                   &mut tmp);
                                    if retcode != 0 as libc::c_int {
                                        Tcl_AppendElement(interp,
                                                          b"while parsing mkey_from_kbd\x00"
                                                              as *const u8 as
                                                              *const libc::c_char);
                                        retcode = 1 as libc::c_int
                                    } else {
                                        (*params).mkey_from_kbd = tmp;
                                        retcode =
                                            parse_str(interp,
                                                      *argv.offset(11 as
                                                                       libc::c_int
                                                                       as
                                                                       isize),
                                                      &mut (*params).stash_file);
                                        if retcode != 0 as libc::c_int {
                                            Tcl_AppendElement(interp,
                                                              b"while parsing stash_file name\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char);
                                            retcode = 1 as libc::c_int
                                        } else {
                                            retcode =
                                                parse_str(interp,
                                                          *argv.offset(12 as
                                                                           libc::c_int
                                                                           as
                                                                           isize),
                                                          &mut (*params).mkey_name);
                                            if retcode != 0 as libc::c_int {
                                                Tcl_AppendElement(interp,
                                                                  b"while parsing mkey_name name\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char);
                                                retcode = 1 as libc::c_int
                                            } else {
                                                retcode =
                                                    Tcl_GetInt(interp,
                                                               *argv.offset(13
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize),
                                                               &mut tmp);
                                                if retcode != 0 as libc::c_int
                                                   {
                                                    Tcl_AppendElement(interp,
                                                                      b"while parsing enctype\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char);
                                                    retcode = 1 as libc::c_int
                                                } else {
                                                    (*params).enctype = tmp;
                                                    retcode =
                                                        Tcl_GetInt(interp,
                                                                   *argv.offset(14
                                                                                    as
                                                                                    libc::c_int
                                                                                    as
                                                                                    isize),
                                                                   &mut tmp);
                                                    if retcode !=
                                                           0 as libc::c_int {
                                                        Tcl_AppendElement(interp,
                                                                          b"while parsing max_life\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char);
                                                        retcode =
                                                            1 as libc::c_int
                                                    } else {
                                                        (*params).max_life =
                                                            tmp;
                                                        retcode =
                                                            Tcl_GetInt(interp,
                                                                       *argv.offset(15
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize),
                                                                       &mut tmp);
                                                        if retcode !=
                                                               0 as
                                                                   libc::c_int
                                                           {
                                                            Tcl_AppendElement(interp,
                                                                              b"while parsing max_rlife\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char);
                                                            retcode =
                                                                1 as
                                                                    libc::c_int
                                                        } else {
                                                            (*params).max_rlife
                                                                = tmp;
                                                            retcode =
                                                                Tcl_GetInt(interp,
                                                                           *argv.offset(16
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize),
                                                                           &mut tmp);
                                                            if retcode !=
                                                                   0 as
                                                                       libc::c_int
                                                               {
                                                                Tcl_AppendElement(interp,
                                                                                  b"while parsing expiration\x00"
                                                                                      as
                                                                                      *const u8
                                                                                      as
                                                                                      *const libc::c_char);
                                                                retcode =
                                                                    1 as
                                                                        libc::c_int
                                                            } else {
                                                                (*params).expiration
                                                                    = tmp;
                                                                retcode =
                                                                    parse_krb5_flags(interp,
                                                                                     *argv.offset(17
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      isize),
                                                                                     &mut tmp);
                                                                if retcode !=
                                                                       0 as
                                                                           libc::c_int
                                                                   {
                                                                    Tcl_AppendElement(interp,
                                                                                      b"while parsing flags\x00"
                                                                                          as
                                                                                          *const u8
                                                                                          as
                                                                                          *const libc::c_char);
                                                                    retcode =
                                                                        1 as
                                                                            libc::c_int
                                                                } else {
                                                                    (*params).flags
                                                                        = tmp;
                                                                    retcode =
                                                                        Tcl_GetInt(interp,
                                                                                   *argv.offset(18
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize),
                                                                                   &mut tmp);
                                                                    if retcode
                                                                           !=
                                                                           0
                                                                               as
                                                                               libc::c_int
                                                                       {
                                                                        Tcl_AppendElement(interp,
                                                                                          b"while parsing num_keysalts\x00"
                                                                                              as
                                                                                              *const u8
                                                                                              as
                                                                                              *const libc::c_char);
                                                                        retcode
                                                                            =
                                                                            1
                                                                                as
                                                                                libc::c_int
                                                                    } else {
                                                                        (*params).num_keysalts
                                                                            =
                                                                            tmp;
                                                                        retcode
                                                                            =
                                                                            parse_keysalts(interp,
                                                                                           *argv.offset(19
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            isize),
                                                                                           &mut (*params).keysalts,
                                                                                           (*params).num_keysalts);
                                                                        if retcode
                                                                               !=
                                                                               0
                                                                                   as
                                                                                   libc::c_int
                                                                           {
                                                                            Tcl_AppendElement(interp,
                                                                                              b"while parsing keysalts\x00"
                                                                                                  as
                                                                                                  *const u8
                                                                                                  as
                                                                                                  *const libc::c_char);
                                                                            retcode
                                                                                =
                                                                                1
                                                                                    as
                                                                                    libc::c_int
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return retcode;
}
#[c2rust::src_loc = "1106:1"]
unsafe extern "C" fn parse_principal_ent(mut interp: *mut Tcl_Interp,
                                         mut list: *mut libc::c_char,
                                         mut out_princ:
                                             *mut kadm5_principal_ent_t)
 -> libc::c_int {
    let mut princ: kadm5_principal_ent_t = 0 as kadm5_principal_ent_t;
    let mut krb5_ret: krb5_error_code = 0;
    let mut tcl_ret: libc::c_int = 0;
    let mut argc: libc::c_int = 0;
    let mut argv: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut retcode: libc::c_int = 0 as libc::c_int;
    tcl_ret = Tcl_SplitList(interp, list, &mut argc, &mut argv);
    if tcl_ret != 0 as libc::c_int { return tcl_ret }
    if argc != 12 as libc::c_int && argc != 20 as libc::c_int {
        Tcl_SetResult(interp,
                      b"wrong # args in principal structure\x00" as *const u8
                          as *const libc::c_char as *mut libc::c_char, None);
        retcode = 1 as libc::c_int
    } else {
        princ =
            malloc(::std::mem::size_of::<_kadm5_principal_ent_t>() as
                       libc::c_ulong) as kadm5_principal_ent_t;
        if princ.is_null() {
            fprintf(stderr,
                    b"Out of memory!\n\x00" as *const u8 as
                        *const libc::c_char);
            exit(1 as libc::c_int);
            /* XXX */
        }
        memset(princ as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<_kadm5_principal_ent_t>() as
                   libc::c_ulong);
        krb5_ret =
            krb5_parse_name(context, *argv.offset(0 as libc::c_int as isize),
                            &mut (*princ).principal);
        if krb5_ret != 0 as libc::c_int {
            stash_error(interp, krb5_ret);
            Tcl_AppendElement(interp,
                              b"while parsing principal\x00" as *const u8 as
                                  *const libc::c_char);
            retcode = 1 as libc::c_int
        } else {
            /*
     * All of the numerical values parsed here are parsed into an
     * "int" and then assigned into the structure in case the actual
     * width of the field in the Kerberos structure is different from
     * the width of an integer.
     */
            tcl_ret =
                Tcl_GetInt(interp, *argv.offset(1 as libc::c_int as isize),
                           &mut tmp);
            if tcl_ret != 0 as libc::c_int {
                Tcl_AppendElement(interp,
                                  b"while parsing princ_expire_time\x00" as
                                      *const u8 as *const libc::c_char);
                retcode = 1 as libc::c_int
            } else {
                (*princ).princ_expire_time = tmp;
                tcl_ret =
                    Tcl_GetInt(interp,
                               *argv.offset(2 as libc::c_int as isize),
                               &mut tmp);
                if tcl_ret != 0 as libc::c_int {
                    Tcl_AppendElement(interp,
                                      b"while parsing last_pwd_change\x00" as
                                          *const u8 as *const libc::c_char);
                    retcode = 1 as libc::c_int
                } else {
                    (*princ).last_pwd_change = tmp;
                    tcl_ret =
                        Tcl_GetInt(interp,
                                   *argv.offset(3 as libc::c_int as isize),
                                   &mut tmp);
                    if tcl_ret != 0 as libc::c_int {
                        Tcl_AppendElement(interp,
                                          b"while parsing pw_expiration\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                        retcode = 1 as libc::c_int
                    } else {
                        (*princ).pw_expiration = tmp;
                        tcl_ret =
                            Tcl_GetInt(interp,
                                       *argv.offset(4 as libc::c_int as
                                                        isize), &mut tmp);
                        if tcl_ret != 0 as libc::c_int {
                            Tcl_AppendElement(interp,
                                              b"while parsing max_life\x00" as
                                                  *const u8 as
                                                  *const libc::c_char);
                            retcode = 1 as libc::c_int
                        } else {
                            (*princ).max_life = tmp;
                            krb5_ret =
                                krb5_parse_name(context,
                                                *argv.offset(5 as libc::c_int
                                                                 as isize),
                                                &mut (*princ).mod_name);
                            if krb5_ret != 0 as libc::c_int {
                                stash_error(interp, krb5_ret);
                                Tcl_AppendElement(interp,
                                                  b"while parsing mod_name\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                retcode = 1 as libc::c_int
                            } else {
                                tcl_ret =
                                    Tcl_GetInt(interp,
                                               *argv.offset(6 as libc::c_int
                                                                as isize),
                                               &mut tmp);
                                if tcl_ret != 0 as libc::c_int {
                                    Tcl_AppendElement(interp,
                                                      b"while parsing mod_date\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                    retcode = 1 as libc::c_int
                                } else {
                                    (*princ).mod_date = tmp;
                                    tcl_ret =
                                        parse_krb5_flags(interp,
                                                         *argv.offset(7 as
                                                                          libc::c_int
                                                                          as
                                                                          isize),
                                                         &mut (*princ).attributes);
                                    if tcl_ret != 0 as libc::c_int {
                                        Tcl_AppendElement(interp,
                                                          b"while parsing attributes\x00"
                                                              as *const u8 as
                                                              *const libc::c_char);
                                        retcode = 1 as libc::c_int
                                    } else {
                                        tcl_ret =
                                            Tcl_GetInt(interp,
                                                       *argv.offset(8 as
                                                                        libc::c_int
                                                                        as
                                                                        isize),
                                                       &mut tmp);
                                        if tcl_ret != 0 as libc::c_int {
                                            Tcl_AppendElement(interp,
                                                              b"while parsing kvno\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char);
                                            retcode = 1 as libc::c_int
                                        } else {
                                            (*princ).kvno = tmp as krb5_kvno;
                                            tcl_ret =
                                                Tcl_GetInt(interp,
                                                           *argv.offset(9 as
                                                                            libc::c_int
                                                                            as
                                                                            isize),
                                                           &mut tmp);
                                            if tcl_ret != 0 as libc::c_int {
                                                Tcl_AppendElement(interp,
                                                                  b"while parsing mkvno\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char);
                                                retcode = 1 as libc::c_int
                                            } else {
                                                (*princ).mkvno =
                                                    tmp as krb5_kvno;
                                                tcl_ret =
                                                    parse_str(interp,
                                                              *argv.offset(10
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               isize),
                                                              &mut (*princ).policy);
                                                if tcl_ret != 0 as libc::c_int
                                                   {
                                                    Tcl_AppendElement(interp,
                                                                      b"while parsing policy\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char);
                                                    retcode = 1 as libc::c_int
                                                } else {
                                                    if !(*princ).policy.is_null()
                                                       {
                                                        (*princ).policy =
                                                            strdup((*princ).policy);
                                                        if (*princ).policy.is_null()
                                                           {
                                                            fprintf(stderr,
                                                                    b"Out of memory!\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char);
                                                            exit(1 as
                                                                     libc::c_int);
                                                        }
                                                    }
                                                    tcl_ret =
                                                        parse_aux_attributes(interp,
                                                                             *argv.offset(11
                                                                                              as
                                                                                              libc::c_int
                                                                                              as
                                                                                              isize),
                                                                             &mut (*princ).aux_attributes);
                                                    if tcl_ret !=
                                                           0 as libc::c_int {
                                                        Tcl_AppendElement(interp,
                                                                          b"while parsing aux_attributes\x00"
                                                                              as
                                                                              *const u8
                                                                              as
                                                                              *const libc::c_char);
                                                        retcode =
                                                            1 as libc::c_int
                                                    } else if !(argc ==
                                                                    12 as
                                                                        libc::c_int)
                                                     {
                                                        tcl_ret =
                                                            Tcl_GetInt(interp,
                                                                       *argv.offset(12
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        isize),
                                                                       &mut tmp);
                                                        if tcl_ret !=
                                                               0 as
                                                                   libc::c_int
                                                           {
                                                            Tcl_AppendElement(interp,
                                                                              b"while parsing max_renewable_life\x00"
                                                                                  as
                                                                                  *const u8
                                                                                  as
                                                                                  *const libc::c_char);
                                                            retcode =
                                                                1 as
                                                                    libc::c_int
                                                        } else {
                                                            (*princ).max_renewable_life
                                                                = tmp;
                                                            tcl_ret =
                                                                Tcl_GetInt(interp,
                                                                           *argv.offset(13
                                                                                            as
                                                                                            libc::c_int
                                                                                            as
                                                                                            isize),
                                                                           &mut tmp);
                                                            if tcl_ret !=
                                                                   0 as
                                                                       libc::c_int
                                                               {
                                                                Tcl_AppendElement(interp,
                                                                                  b"while parsing last_success\x00"
                                                                                      as
                                                                                      *const u8
                                                                                      as
                                                                                      *const libc::c_char);
                                                                retcode =
                                                                    1 as
                                                                        libc::c_int
                                                            } else {
                                                                (*princ).last_success
                                                                    = tmp;
                                                                tcl_ret =
                                                                    Tcl_GetInt(interp,
                                                                               *argv.offset(14
                                                                                                as
                                                                                                libc::c_int
                                                                                                as
                                                                                                isize),
                                                                               &mut tmp);
                                                                if tcl_ret !=
                                                                       0 as
                                                                           libc::c_int
                                                                   {
                                                                    Tcl_AppendElement(interp,
                                                                                      b"while parsing last_failed\x00"
                                                                                          as
                                                                                          *const u8
                                                                                          as
                                                                                          *const libc::c_char);
                                                                    retcode =
                                                                        1 as
                                                                            libc::c_int
                                                                } else {
                                                                    (*princ).last_failed
                                                                        = tmp;
                                                                    tcl_ret =
                                                                        Tcl_GetInt(interp,
                                                                                   *argv.offset(15
                                                                                                    as
                                                                                                    libc::c_int
                                                                                                    as
                                                                                                    isize),
                                                                                   &mut tmp);
                                                                    if tcl_ret
                                                                           !=
                                                                           0
                                                                               as
                                                                               libc::c_int
                                                                       {
                                                                        Tcl_AppendElement(interp,
                                                                                          b"while parsing fail_auth_count\x00"
                                                                                              as
                                                                                              *const u8
                                                                                              as
                                                                                              *const libc::c_char);
                                                                        retcode
                                                                            =
                                                                            1
                                                                                as
                                                                                libc::c_int
                                                                    } else {
                                                                        (*princ).fail_auth_count
                                                                            =
                                                                            tmp
                                                                                as
                                                                                krb5_kvno;
                                                                        tcl_ret
                                                                            =
                                                                            Tcl_GetInt(interp,
                                                                                       *argv.offset(16
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                        as
                                                                                                        isize),
                                                                                       &mut tmp);
                                                                        if tcl_ret
                                                                               !=
                                                                               0
                                                                                   as
                                                                                   libc::c_int
                                                                           {
                                                                            Tcl_AppendElement(interp,
                                                                                              b"while parsing n_key_data\x00"
                                                                                                  as
                                                                                                  *const u8
                                                                                                  as
                                                                                                  *const libc::c_char);
                                                                            retcode
                                                                                =
                                                                                1
                                                                                    as
                                                                                    libc::c_int
                                                                        } else {
                                                                            (*princ).n_key_data
                                                                                =
                                                                                tmp
                                                                                    as
                                                                                    krb5_int16;
                                                                            tcl_ret
                                                                                =
                                                                                Tcl_GetInt(interp,
                                                                                           *argv.offset(17
                                                                                                            as
                                                                                                            libc::c_int
                                                                                                            as
                                                                                                            isize),
                                                                                           &mut tmp);
                                                                            if tcl_ret
                                                                                   !=
                                                                                   0
                                                                                       as
                                                                                       libc::c_int
                                                                               {
                                                                                Tcl_AppendElement(interp,
                                                                                                  b"while parsing n_tl_data\x00"
                                                                                                      as
                                                                                                      *const u8
                                                                                                      as
                                                                                                      *const libc::c_char);
                                                                                retcode
                                                                                    =
                                                                                    1
                                                                                        as
                                                                                        libc::c_int
                                                                            } else {
                                                                                (*princ).n_tl_data
                                                                                    =
                                                                                    tmp
                                                                                        as
                                                                                        krb5_int16;
                                                                                tcl_ret
                                                                                    =
                                                                                    parse_key_data(interp,
                                                                                                   *argv.offset(18
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    isize),
                                                                                                   &mut (*princ).key_data,
                                                                                                   (*princ).n_key_data
                                                                                                       as
                                                                                                       libc::c_int);
                                                                                if tcl_ret
                                                                                       !=
                                                                                       0
                                                                                           as
                                                                                           libc::c_int
                                                                                   {
                                                                                    Tcl_AppendElement(interp,
                                                                                                      b"while parsing key_data\x00"
                                                                                                          as
                                                                                                          *const u8
                                                                                                          as
                                                                                                          *const libc::c_char);
                                                                                    retcode
                                                                                        =
                                                                                        1
                                                                                            as
                                                                                            libc::c_int
                                                                                } else {
                                                                                    tcl_ret
                                                                                        =
                                                                                        parse_tl_data(interp,
                                                                                                      *argv.offset(19
                                                                                                                       as
                                                                                                                       libc::c_int
                                                                                                                       as
                                                                                                                       isize),
                                                                                                      &mut (*princ).tl_data,
                                                                                                      (*princ).n_tl_data
                                                                                                          as
                                                                                                          libc::c_int);
                                                                                    if tcl_ret
                                                                                           !=
                                                                                           0
                                                                                               as
                                                                                               libc::c_int
                                                                                       {
                                                                                        Tcl_AppendElement(interp,
                                                                                                          b"while parsing tl_data\x00"
                                                                                                              as
                                                                                                              *const u8
                                                                                                              as
                                                                                                              *const libc::c_char);
                                                                                        retcode
                                                                                            =
                                                                                            1
                                                                                                as
                                                                                                libc::c_int
                                                                                    } else {
                                                                                        (*princ).n_tl_data
                                                                                            =
                                                                                            tmp
                                                                                                as
                                                                                                krb5_int16
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Tcl_Free(argv as *mut libc::c_char);
    *out_princ = princ;
    return retcode;
}
#[c2rust::src_loc = "1311:1"]
unsafe extern "C" fn free_principal_ent(mut princ:
                                            *mut kadm5_principal_ent_t) {
    krb5_free_principal(context, (**princ).principal);
    krb5_free_principal(context, (**princ).mod_name);
    free((**princ).policy as *mut libc::c_void);
    free(*princ as *mut libc::c_void);
    *princ = 0 as kadm5_principal_ent_t;
}
#[c2rust::src_loc = "1320:1"]
unsafe extern "C" fn unparse_policy_ent(mut policy: kadm5_policy_ent_t)
 -> *mut Tcl_DString {
    let mut str: *mut Tcl_DString = 0 as *mut Tcl_DString;
    let mut tmp_dstring: *mut Tcl_DString = 0 as *mut Tcl_DString;
    let mut buf: [libc::c_char; 20] = [0; 20];
    str =
        malloc(::std::mem::size_of::<Tcl_DString>() as libc::c_ulong) as
            *mut Tcl_DString;
    if str.is_null() {
        fprintf(stderr,
                b"Out of memory!\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
        /* XXX */
    }
    Tcl_DStringInit(str);
    tmp_dstring = unparse_str((*policy).policy);
    Tcl_DStringAppendElement(str, (*tmp_dstring).string);
    Tcl_DStringFree(tmp_dstring);
    free(tmp_dstring as *mut libc::c_void);
    sprintf(buf.as_mut_ptr(), b"%ld\x00" as *const u8 as *const libc::c_char,
            (*policy).pw_min_life);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    sprintf(buf.as_mut_ptr(), b"%ld\x00" as *const u8 as *const libc::c_char,
            (*policy).pw_max_life);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    sprintf(buf.as_mut_ptr(), b"%ld\x00" as *const u8 as *const libc::c_char,
            (*policy).pw_min_length);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    sprintf(buf.as_mut_ptr(), b"%ld\x00" as *const u8 as *const libc::c_char,
            (*policy).pw_min_classes);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    sprintf(buf.as_mut_ptr(), b"%ld\x00" as *const u8 as *const libc::c_char,
            (*policy).pw_history_num);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    sprintf(buf.as_mut_ptr(), b"%ld\x00" as *const u8 as *const libc::c_char,
            (*policy).policy_refcnt);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    sprintf(buf.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            (*policy).pw_max_fail);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    sprintf(buf.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            (*policy).pw_failcnt_interval);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    sprintf(buf.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            (*policy).pw_lockout_duration);
    Tcl_DStringAppendElement(str, buf.as_mut_ptr());
    return str;
}
#[c2rust::src_loc = "1369:1"]
unsafe extern "C" fn parse_policy_ent(mut interp: *mut Tcl_Interp,
                                      mut list: *mut libc::c_char,
                                      mut out_policy: *mut kadm5_policy_ent_t)
 -> libc::c_int {
    let mut policy: kadm5_policy_ent_t = 0 as kadm5_policy_ent_t;
    let mut tcl_ret: libc::c_int = 0;
    let mut argc: libc::c_int = 0;
    let mut argv: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut retcode: libc::c_int = 0 as libc::c_int;
    tcl_ret = Tcl_SplitList(interp, list, &mut argc, &mut argv);
    if tcl_ret != 0 as libc::c_int { return tcl_ret }
    if argc != 7 as libc::c_int && argc != 10 as libc::c_int {
        Tcl_SetResult(interp,
                      b"wrong # args in policy structure\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, None);
        retcode = 1 as libc::c_int
    } else {
        policy =
            malloc(::std::mem::size_of::<_kadm5_policy_ent_t>() as
                       libc::c_ulong) as kadm5_policy_ent_t;
        if policy.is_null() {
            fprintf(stderr,
                    b"Out of memory!\n\x00" as *const u8 as
                        *const libc::c_char);
            exit(1 as libc::c_int);
            /* XXX */
        }
        tcl_ret =
            parse_str(interp, *argv.offset(0 as libc::c_int as isize),
                      &mut (*policy).policy);
        if tcl_ret != 0 as libc::c_int {
            Tcl_AppendElement(interp,
                              b"while parsing policy name\x00" as *const u8 as
                                  *const libc::c_char);
            retcode = 1 as libc::c_int
        } else {
            if !(*policy).policy.is_null() {
                (*policy).policy = strdup((*policy).policy);
                if (*policy).policy.is_null() {
                    fprintf(stderr,
                            b"Out of memory!\n\x00" as *const u8 as
                                *const libc::c_char);
                    exit(1 as libc::c_int);
                    /* XXX */
                }
            }
            /*
     * All of the numerical values parsed here are parsed into an
     * "int" and then assigned into the structure in case the actual
     * width of the field in the Kerberos structure is different from
     * the width of an integer.
     */
            tcl_ret =
                Tcl_GetInt(interp, *argv.offset(1 as libc::c_int as isize),
                           &mut tmp);
            if tcl_ret != 0 as libc::c_int {
                Tcl_AppendElement(interp,
                                  b"while parsing pw_min_life\x00" as
                                      *const u8 as *const libc::c_char);
                retcode = 1 as libc::c_int
            } else {
                (*policy).pw_min_life = tmp as libc::c_long;
                tcl_ret =
                    Tcl_GetInt(interp,
                               *argv.offset(2 as libc::c_int as isize),
                               &mut tmp);
                if tcl_ret != 0 as libc::c_int {
                    Tcl_AppendElement(interp,
                                      b"while parsing pw_max_life\x00" as
                                          *const u8 as *const libc::c_char);
                    retcode = 1 as libc::c_int
                } else {
                    (*policy).pw_max_life = tmp as libc::c_long;
                    tcl_ret =
                        Tcl_GetInt(interp,
                                   *argv.offset(3 as libc::c_int as isize),
                                   &mut tmp);
                    if tcl_ret != 0 as libc::c_int {
                        Tcl_AppendElement(interp,
                                          b"while parsing pw_min_length\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                        retcode = 1 as libc::c_int
                    } else {
                        (*policy).pw_min_length = tmp as libc::c_long;
                        tcl_ret =
                            Tcl_GetInt(interp,
                                       *argv.offset(4 as libc::c_int as
                                                        isize), &mut tmp);
                        if tcl_ret != 0 as libc::c_int {
                            Tcl_AppendElement(interp,
                                              b"while parsing pw_min_classes\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                            retcode = 1 as libc::c_int
                        } else {
                            (*policy).pw_min_classes = tmp as libc::c_long;
                            tcl_ret =
                                Tcl_GetInt(interp,
                                           *argv.offset(5 as libc::c_int as
                                                            isize), &mut tmp);
                            if tcl_ret != 0 as libc::c_int {
                                Tcl_AppendElement(interp,
                                                  b"while parsing pw_history_num\x00"
                                                      as *const u8 as
                                                      *const libc::c_char);
                                retcode = 1 as libc::c_int
                            } else {
                                (*policy).pw_history_num =
                                    tmp as libc::c_long;
                                tcl_ret =
                                    Tcl_GetInt(interp,
                                               *argv.offset(6 as libc::c_int
                                                                as isize),
                                               &mut tmp);
                                if tcl_ret != 0 as libc::c_int {
                                    Tcl_AppendElement(interp,
                                                      b"while parsing policy_refcnt\x00"
                                                          as *const u8 as
                                                          *const libc::c_char);
                                    retcode = 1 as libc::c_int
                                } else {
                                    (*policy).policy_refcnt =
                                        tmp as libc::c_long;
                                    if !(argc == 7 as libc::c_int) {
                                        tcl_ret =
                                            Tcl_GetInt(interp,
                                                       *argv.offset(7 as
                                                                        libc::c_int
                                                                        as
                                                                        isize),
                                                       &mut tmp);
                                        if tcl_ret != 0 as libc::c_int {
                                            Tcl_AppendElement(interp,
                                                              b"while parsing pw_max_fail\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char);
                                            retcode = 1 as libc::c_int
                                        } else {
                                            (*policy).pw_max_fail =
                                                tmp as krb5_kvno;
                                            tcl_ret =
                                                Tcl_GetInt(interp,
                                                           *argv.offset(8 as
                                                                            libc::c_int
                                                                            as
                                                                            isize),
                                                           &mut tmp);
                                            if tcl_ret != 0 as libc::c_int {
                                                Tcl_AppendElement(interp,
                                                                  b"while parsing pw_failcnt_interval\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char);
                                                retcode = 1 as libc::c_int
                                            } else {
                                                (*policy).pw_failcnt_interval
                                                    = tmp;
                                                tcl_ret =
                                                    Tcl_GetInt(interp,
                                                               *argv.offset(9
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                isize),
                                                               &mut tmp);
                                                if tcl_ret != 0 as libc::c_int
                                                   {
                                                    Tcl_AppendElement(interp,
                                                                      b"while parsing pw_lockout_duration\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char);
                                                    retcode = 1 as libc::c_int
                                                } else {
                                                    (*policy).pw_lockout_duration
                                                        = tmp
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Tcl_Free(argv as *mut libc::c_char);
    *out_policy = policy;
    return retcode;
}
#[c2rust::src_loc = "1495:1"]
unsafe extern "C" fn free_policy_ent(mut policy: *mut kadm5_policy_ent_t) {
    free((**policy).policy as *mut libc::c_void);
    free(*policy as *mut libc::c_void);
    *policy = 0 as kadm5_policy_ent_t;
}
#[c2rust::src_loc = "1502:1"]
unsafe extern "C" fn unparse_keytype(mut enctype: krb5_enctype)
 -> *mut Tcl_DString {
    let mut str: *mut Tcl_DString = 0 as *mut Tcl_DString;
    let mut buf: [libc::c_char; 50] = [0; 50];
    str =
        malloc(::std::mem::size_of::<Tcl_DString>() as libc::c_ulong) as
            *mut Tcl_DString;
    if str.is_null() {
        fprintf(stderr,
                b"Out of memory!\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
        /* XXX */
    }
    Tcl_DStringInit(str);
    match enctype {
        0 => {
            /* XXX is this right? */
            Tcl_DStringAppend(str,
                              b"ENCTYPE_NULL\x00" as *const u8 as
                                  *const libc::c_char, -(1 as libc::c_int));
        }
        _ => {
            sprintf(buf.as_mut_ptr(),
                    b"UNKNOWN KEYTYPE (0x%x)\x00" as *const u8 as
                        *const libc::c_char, enctype);
            Tcl_DStringAppend(str, buf.as_mut_ptr(), -(1 as libc::c_int));
        }
    }
    return str;
}
#[c2rust::src_loc = "1527:1"]
unsafe extern "C" fn unparse_keyblocks(mut keyblocks: *mut krb5_keyblock,
                                       mut num_keys: libc::c_int)
 -> *mut Tcl_DString {
    let mut str: *mut Tcl_DString = 0 as *mut Tcl_DString;
    let mut keytype: *mut Tcl_DString = 0 as *mut Tcl_DString;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_int = 0;
    str =
        malloc(::std::mem::size_of::<Tcl_DString>() as libc::c_ulong) as
            *mut Tcl_DString;
    if str.is_null() {
        fprintf(stderr,
                b"Out of memory!\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
        /* XXX */
    }
    Tcl_DStringInit(str);
    j = 0 as libc::c_int;
    while j < num_keys {
        let mut keyblock: *mut krb5_keyblock =
            &mut *keyblocks.offset(j as isize) as *mut krb5_keyblock;
        Tcl_DStringStartSublist(str);
        keytype = unparse_keytype((*keyblock).enctype);
        Tcl_DStringAppendElement(str, (*keytype).string);
        Tcl_DStringFree(keytype);
        free(keytype as *mut libc::c_void);
        if (*keyblock).length == 0 as libc::c_int as libc::c_uint {
            Tcl_DStringAppendElement(str,
                                     b"0x00\x00" as *const u8 as
                                         *const libc::c_char);
        } else {
            Tcl_DStringAppendElement(str,
                                     b"0x\x00" as *const u8 as
                                         *const libc::c_char);
            i = 0 as libc::c_int as libc::c_uint;
            while i < (*keyblock).length {
                let mut buf: [libc::c_char; 3] = [0; 3];
                sprintf(buf.as_mut_ptr(),
                        b"%02x\x00" as *const u8 as *const libc::c_char,
                        *(*keyblock).contents.offset(i as isize) as
                            libc::c_int);
                Tcl_DStringAppend(str, buf.as_mut_ptr(), -(1 as libc::c_int));
                i = i.wrapping_add(1)
            }
        }
        Tcl_DStringEndSublist(str);
        j += 1
    }
    return str;
}
#[c2rust::src_loc = "1571:1"]
unsafe extern "C" fn _tcl_kadm5_init_any(mut init_type: init_type,
                                         mut clientData: ClientData,
                                         mut interp: *mut Tcl_Interp,
                                         mut argc: libc::c_int,
                                         mut argv: *mut *const libc::c_char)
 -> libc::c_int {
    let mut ret: kadm5_ret_t = 0;
    let mut client_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pass: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut service_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tcl_ret: libc::c_int = 0;
    let mut struct_version: krb5_ui_4 = 0;
    let mut api_version: krb5_ui_4 = 0;
    let mut handle_var: *const libc::c_char = 0 as *const libc::c_char;
    let mut server_handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut handle_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut params_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut whoami: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    let mut params: kadm5_config_params =
        kadm5_config_params{mask: 0,
                            realm: 0 as *mut libc::c_char,
                            kadmind_port: 0,
                            kpasswd_port: 0,
                            admin_server: 0 as *mut libc::c_char,
                            dbname: 0 as *mut libc::c_char,
                            acl_file: 0 as *mut libc::c_char,
                            dict_file: 0 as *mut libc::c_char,
                            mkey_from_kbd: 0,
                            stash_file: 0 as *mut libc::c_char,
                            mkey_name: 0 as *mut libc::c_char,
                            enctype: 0,
                            max_life: 0,
                            max_rlife: 0,
                            expiration: 0,
                            flags: 0,
                            keysalts: 0 as *mut krb5_key_salt_tuple,
                            num_keysalts: 0,
                            kvno: 0,
                            iprop_enabled: 0,
                            iprop_ulogsize: 0,
                            iprop_poll_time: 0,
                            iprop_logfile: 0 as *mut libc::c_char,
                            iprop_port: 0,
                            iprop_resync_timeout: 0,
                            kadmind_listen: 0 as *mut libc::c_char,
                            kpasswd_listen: 0 as *mut libc::c_char,
                            iprop_listen: 0 as *mut libc::c_char,};
    argv = argv.offset(1);
    argc -= 1;
    kadm5_init_krb5_context(&mut context);
    if argc != 7 as libc::c_int {
        Tcl_AppendResult(interp, whoami,
                         b": \x00" as *const u8 as *const libc::c_char,
                         arg_error, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    tcl_ret =
        parse_str(interp, *argv.offset(0 as libc::c_int as isize),
                  &mut client_name);
    if tcl_ret != 0 as libc::c_int ||
           {
               tcl_ret =
                   parse_str(interp, *argv.offset(1 as libc::c_int as isize),
                             &mut pass);
               (tcl_ret) != 0 as libc::c_int
           } ||
           {
               tcl_ret =
                   parse_str(interp, *argv.offset(2 as libc::c_int as isize),
                             &mut service_name);
               (tcl_ret) != 0 as libc::c_int
           } ||
           {
               tcl_ret =
                   parse_str(interp, *argv.offset(3 as libc::c_int as isize),
                             &mut params_str);
               (tcl_ret) != 0 as libc::c_int
           } ||
           {
               tcl_ret = parse_config_params(interp, params_str, &mut params);
               (tcl_ret) != 0 as libc::c_int
           } ||
           {
               tcl_ret =
                   Tcl_GetInt(interp, *argv.offset(4 as libc::c_int as isize),
                              &mut struct_version as *mut krb5_ui_4 as
                                  *mut libc::c_int);
               (tcl_ret) != 0 as libc::c_int
           } ||
           {
               tcl_ret =
                   Tcl_GetInt(interp, *argv.offset(5 as libc::c_int as isize),
                              &mut api_version as *mut krb5_ui_4 as
                                  *mut libc::c_int);
               (tcl_ret) != 0 as libc::c_int
           } {
        return tcl_ret
    }
    handle_var = *argv.offset(6 as libc::c_int as isize);
    if !(!handle_var.is_null() && *handle_var as libc::c_int != 0) {
        Tcl_SetResult(interp,
                      b"must specify server handle variable name\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char, None);
        return 1 as libc::c_int
    }
    if init_type as libc::c_uint == INIT_CREDS as libc::c_int as libc::c_uint
       {
        let mut cc: krb5_ccache = 0 as *mut _krb5_ccache;
        if pass.is_null() {
            ret = krb5_cc_default(context, &mut cc) as kadm5_ret_t;
            if ret != 0 {
                stash_error(interp, ret as krb5_error_code);
                return 1 as libc::c_int
            }
        } else {
            ret = krb5_cc_resolve(context, pass, &mut cc) as kadm5_ret_t;
            if ret != 0 {
                stash_error(interp, ret as krb5_error_code);
                return 1 as libc::c_int
            }
        }
        ret =
            kadm5_init_with_creds(context, client_name, cc, service_name,
                                  &mut params, struct_version, api_version,
                                  0 as *mut *mut libc::c_char,
                                  &mut server_handle);
        krb5_cc_close(context, cc);
    } else {
        ret =
            kadm5_init(context, client_name, pass, service_name, &mut params,
                       struct_version, api_version,
                       0 as *mut *mut libc::c_char, &mut server_handle)
    }
    /* The string fields of params are aliases into argv[3], but
     * params.keysalts is allocated, so clean it up. */
    free(params.keysalts as *mut libc::c_void);
    if ret != 0 as libc::c_int as libc::c_long {
        stash_error(interp, ret as krb5_error_code);
        return 1 as libc::c_int
    }
    tcl_ret = put_server_handle(interp, server_handle, &mut handle_name);
    if tcl_ret != 0 as libc::c_int { return tcl_ret }
    if Tcl_SetVar2(interp, handle_var, 0 as *const libc::c_char, handle_name,
                   0x200 as libc::c_int).is_null() {
        return 1 as libc::c_int
    }
    set_ok(interp,
           b"KADM5 API initialized.\x00" as *const u8 as *const libc::c_char
               as *mut libc::c_char);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1660:1"]
unsafe extern "C" fn tcl_kadm5_init(mut clientData: ClientData,
                                    mut interp: *mut Tcl_Interp,
                                    mut argc: libc::c_int,
                                    mut argv: *mut *const libc::c_char)
 -> libc::c_int {
    return _tcl_kadm5_init_any(INIT_PASS, clientData, interp, argc, argv);
}
#[c2rust::src_loc = "1666:1"]
unsafe extern "C" fn tcl_kadm5_init_with_creds(mut clientData: ClientData,
                                               mut interp: *mut Tcl_Interp,
                                               mut argc: libc::c_int,
                                               mut argv:
                                                   *mut *const libc::c_char)
 -> libc::c_int {
    return _tcl_kadm5_init_any(INIT_CREDS, clientData, interp, argc, argv);
}
#[c2rust::src_loc = "1672:1"]
unsafe extern "C" fn tcl_kadm5_destroy(mut clientData: ClientData,
                                       mut interp: *mut Tcl_Interp,
                                       mut argc: libc::c_int,
                                       mut argv: *mut *const libc::c_char)
 -> libc::c_int {
    let mut ret: kadm5_ret_t = 0;
    let mut tcl_ret: libc::c_int = 0;
    let mut server_handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut whoami: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    argv = argv.offset(1);
    argc -= 1;
    if argc != 0 as libc::c_int + 1 as libc::c_int {
        Tcl_AppendResult(interp, whoami,
                         b": \x00" as *const u8 as *const libc::c_char,
                         arg_error, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    let mut ltcl_ret: libc::c_int = 0;
    ltcl_ret =
        get_server_handle(interp, *argv.offset(0 as libc::c_int as isize),
                          &mut server_handle);
    if ltcl_ret != 0 as libc::c_int { return ltcl_ret }
    argv = argv.offset(1);
    argc -= 1;
    ret = kadm5_destroy(server_handle);
    if ret != 0 as libc::c_int as libc::c_long {
        stash_error(interp, ret as krb5_error_code);
        return 1 as libc::c_int
    }
    tcl_ret =
        remove_server_handle(interp,
                             *argv.offset(-(1 as libc::c_int) as isize));
    if tcl_ret != 0 as libc::c_int { return tcl_ret }
    set_ok(interp,
           b"KADM5 API deinitialized.\x00" as *const u8 as *const libc::c_char
               as *mut libc::c_char);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "1695:1"]
unsafe extern "C" fn tcl_kadm5_create_principal(mut clientData: ClientData,
                                                mut interp: *mut Tcl_Interp,
                                                mut argc: libc::c_int,
                                                mut argv:
                                                    *mut *const libc::c_char)
 -> libc::c_int {
    let mut tcl_ret: libc::c_int = 0;
    let mut ret: kadm5_ret_t = 0;
    let mut retcode: libc::c_int = 0 as libc::c_int;
    let mut princ_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut princ: kadm5_principal_ent_t = 0 as kadm5_principal_ent_t;
    let mut mask: krb5_int32 = 0;
    let mut pw: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut server_handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut whoami: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    argv = argv.offset(1);
    argc -= 1;
    if argc != 3 as libc::c_int + 1 as libc::c_int {
        Tcl_AppendResult(interp, whoami,
                         b": \x00" as *const u8 as *const libc::c_char,
                         arg_error, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    let mut ltcl_ret: libc::c_int = 0;
    ltcl_ret =
        get_server_handle(interp, *argv.offset(0 as libc::c_int as isize),
                          &mut server_handle);
    if ltcl_ret != 0 as libc::c_int { return ltcl_ret }
    argv = argv.offset(1);
    argc -= 1;
    tcl_ret =
        parse_str(interp, *argv.offset(0 as libc::c_int as isize),
                  &mut princ_string);
    if tcl_ret != 0 as libc::c_int {
        Tcl_AppendElement(interp,
                          b"while parsing principal\x00" as *const u8 as
                              *const libc::c_char);
        return tcl_ret
    }
    if !princ_string.is_null() &&
           {
               tcl_ret =
                   parse_principal_ent(interp, princ_string, &mut princ);
               (tcl_ret) != 0 as libc::c_int
           } {
        return tcl_ret
    }
    tcl_ret =
        parse_principal_mask(interp, *argv.offset(1 as libc::c_int as isize),
                             &mut mask);
    if tcl_ret != 0 as libc::c_int {
        retcode = tcl_ret
    } else {
        tcl_ret =
            parse_str(interp, *argv.offset(2 as libc::c_int as isize),
                      &mut pw);
        if tcl_ret != 0 as libc::c_int {
            retcode = tcl_ret
        } else {
            ret =
                kadm5_create_principal(server_handle, princ,
                                       mask as libc::c_long, pw);
            if ret != 0 as libc::c_int as libc::c_long {
                stash_error(interp, ret as krb5_error_code);
                retcode = 1 as libc::c_int
            } else {
                set_ok(interp,
                       b"Principal created.\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
            }
        }
    }
    if !princ.is_null() { free_principal_ent(&mut princ); }
    return retcode;
}
#[c2rust::src_loc = "1765:1"]
unsafe extern "C" fn tcl_kadm5_delete_principal(mut clientData: ClientData,
                                                mut interp: *mut Tcl_Interp,
                                                mut argc: libc::c_int,
                                                mut argv:
                                                    *mut *const libc::c_char)
 -> libc::c_int {
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut krb5_ret: krb5_error_code = 0;
    let mut ret: kadm5_ret_t = 0;
    let mut tcl_ret: libc::c_int = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut server_handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut whoami: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    argv = argv.offset(1);
    argc -= 1;
    if argc != 1 as libc::c_int + 1 as libc::c_int {
        Tcl_AppendResult(interp, whoami,
                         b": \x00" as *const u8 as *const libc::c_char,
                         arg_error, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    let mut ltcl_ret: libc::c_int = 0;
    ltcl_ret =
        get_server_handle(interp, *argv.offset(0 as libc::c_int as isize),
                          &mut server_handle);
    if ltcl_ret != 0 as libc::c_int { return ltcl_ret }
    argv = argv.offset(1);
    argc -= 1;
    tcl_ret =
        parse_str(interp, *argv.offset(0 as libc::c_int as isize), &mut name);
    if tcl_ret != 0 as libc::c_int { return tcl_ret }
    if !name.is_null() {
        krb5_ret = krb5_parse_name(context, name, &mut princ);
        if krb5_ret != 0 {
            stash_error(interp, krb5_ret);
            Tcl_AppendElement(interp,
                              b"while parsing principal\x00" as *const u8 as
                                  *const libc::c_char);
            return 1 as libc::c_int
        }
    } else { princ = 0 as krb5_principal }
    ret = kadm5_delete_principal(server_handle, princ);
    if !princ.is_null() { krb5_free_principal(context, princ); }
    if ret != 0 as libc::c_int as libc::c_long {
        stash_error(interp, ret as krb5_error_code);
        return 1 as libc::c_int
    } else {
        set_ok(interp,
               b"Principal deleted.\x00" as *const u8 as *const libc::c_char
                   as *mut libc::c_char);
        return 0 as libc::c_int
    };
}
#[c2rust::src_loc = "1803:1"]
unsafe extern "C" fn tcl_kadm5_modify_principal(mut clientData: ClientData,
                                                mut interp: *mut Tcl_Interp,
                                                mut argc: libc::c_int,
                                                mut argv:
                                                    *mut *const libc::c_char)
 -> libc::c_int {
    let mut princ_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut princ: kadm5_principal_ent_t = 0 as kadm5_principal_ent_t;
    let mut tcl_ret: libc::c_int = 0;
    let mut mask: krb5_int32 = 0;
    let mut retcode: libc::c_int = 0 as libc::c_int;
    let mut ret: kadm5_ret_t = 0;
    let mut server_handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut whoami: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    argv = argv.offset(1);
    argc -= 1;
    if argc != 2 as libc::c_int + 1 as libc::c_int {
        Tcl_AppendResult(interp, whoami,
                         b": \x00" as *const u8 as *const libc::c_char,
                         arg_error, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    let mut ltcl_ret: libc::c_int = 0;
    ltcl_ret =
        get_server_handle(interp, *argv.offset(0 as libc::c_int as isize),
                          &mut server_handle);
    if ltcl_ret != 0 as libc::c_int { return ltcl_ret }
    argv = argv.offset(1);
    argc -= 1;
    tcl_ret =
        parse_str(interp, *argv.offset(0 as libc::c_int as isize),
                  &mut princ_string);
    if tcl_ret != 0 as libc::c_int {
        Tcl_AppendElement(interp,
                          b"while parsing principal\x00" as *const u8 as
                              *const libc::c_char);
        return tcl_ret
    }
    if !princ_string.is_null() &&
           {
               tcl_ret =
                   parse_principal_ent(interp, princ_string, &mut princ);
               (tcl_ret) != 0 as libc::c_int
           } {
        return tcl_ret
    }
    tcl_ret =
        parse_principal_mask(interp, *argv.offset(1 as libc::c_int as isize),
                             &mut mask);
    if tcl_ret != 0 as libc::c_int {
        retcode = 1 as libc::c_int
    } else {
        ret =
            kadm5_modify_principal(server_handle, princ,
                                   mask as libc::c_long);
        if ret != 0 as libc::c_int as libc::c_long {
            stash_error(interp, ret as krb5_error_code);
            retcode = 1 as libc::c_int
        } else {
            set_ok(interp,
                   b"Principal modified.\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char);
        }
    }
    if !princ.is_null() { free_principal_ent(&mut princ); }
    return retcode;
}
#[c2rust::src_loc = "1850:1"]
unsafe extern "C" fn tcl_kadm5_rename_principal(mut clientData: ClientData,
                                                mut interp: *mut Tcl_Interp,
                                                mut argc: libc::c_int,
                                                mut argv:
                                                    *mut *const libc::c_char)
 -> libc::c_int {
    let mut source: krb5_principal = 0 as *mut krb5_principal_data;
    let mut target: krb5_principal = 0 as *mut krb5_principal_data;
    let mut krb5_ret: krb5_error_code = 0;
    let mut ret: kadm5_ret_t = 0;
    let mut retcode: libc::c_int = 0 as libc::c_int;
    let mut server_handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut whoami: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    argv = argv.offset(1);
    argc -= 1;
    if argc != 2 as libc::c_int + 1 as libc::c_int {
        Tcl_AppendResult(interp, whoami,
                         b": \x00" as *const u8 as *const libc::c_char,
                         arg_error, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    let mut ltcl_ret: libc::c_int = 0;
    ltcl_ret =
        get_server_handle(interp, *argv.offset(0 as libc::c_int as isize),
                          &mut server_handle);
    if ltcl_ret != 0 as libc::c_int { return ltcl_ret }
    argv = argv.offset(1);
    argc -= 1;
    krb5_ret =
        krb5_parse_name(context, *argv.offset(0 as libc::c_int as isize),
                        &mut source);
    if krb5_ret != 0 as libc::c_int {
        stash_error(interp, krb5_ret);
        Tcl_AppendElement(interp,
                          b"while parsing source\x00" as *const u8 as
                              *const libc::c_char);
        return 1 as libc::c_int
    }
    krb5_ret =
        krb5_parse_name(context, *argv.offset(1 as libc::c_int as isize),
                        &mut target);
    if krb5_ret != 0 as libc::c_int {
        stash_error(interp, krb5_ret);
        Tcl_AppendElement(interp,
                          b"while parsing target\x00" as *const u8 as
                              *const libc::c_char);
        krb5_free_principal(context, source);
        return 1 as libc::c_int
    }
    ret = kadm5_rename_principal(server_handle, source, target);
    if ret == 0 as libc::c_int as libc::c_long {
        set_ok(interp,
               b"Principal renamed.\x00" as *const u8 as *const libc::c_char
                   as *mut libc::c_char);
    } else {
        stash_error(interp, ret as krb5_error_code);
        retcode = 1 as libc::c_int
    }
    krb5_free_principal(context, source);
    krb5_free_principal(context, target);
    return retcode;
}
#[c2rust::src_loc = "1891:1"]
unsafe extern "C" fn tcl_kadm5_chpass_principal(mut clientData: ClientData,
                                                mut interp: *mut Tcl_Interp,
                                                mut argc: libc::c_int,
                                                mut argv:
                                                    *mut *const libc::c_char)
 -> libc::c_int {
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut pw: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut krb5_ret: krb5_error_code = 0;
    let mut retcode: libc::c_int = 0 as libc::c_int;
    let mut ret: kadm5_ret_t = 0;
    let mut server_handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut whoami: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    argv = argv.offset(1);
    argc -= 1;
    if argc != 2 as libc::c_int + 1 as libc::c_int {
        Tcl_AppendResult(interp, whoami,
                         b": \x00" as *const u8 as *const libc::c_char,
                         arg_error, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    let mut ltcl_ret: libc::c_int = 0;
    ltcl_ret =
        get_server_handle(interp, *argv.offset(0 as libc::c_int as isize),
                          &mut server_handle);
    if ltcl_ret != 0 as libc::c_int { return ltcl_ret }
    argv = argv.offset(1);
    argc -= 1;
    krb5_ret =
        krb5_parse_name(context, *argv.offset(0 as libc::c_int as isize),
                        &mut princ);
    if krb5_ret != 0 as libc::c_int {
        stash_error(interp, krb5_ret);
        Tcl_AppendElement(interp,
                          b"while parsing principal name\x00" as *const u8 as
                              *const libc::c_char);
        return 1 as libc::c_int
    }
    if parse_str(interp, *argv.offset(1 as libc::c_int as isize), &mut pw) !=
           0 as libc::c_int {
        Tcl_AppendElement(interp,
                          b"while parsing password\x00" as *const u8 as
                              *const libc::c_char);
        retcode = 1 as libc::c_int
    } else {
        ret = kadm5_chpass_principal(server_handle, princ, pw);
        if ret == 0 as libc::c_int as libc::c_long {
            set_ok(interp,
                   b"Password changed.\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char);
        } else {
            stash_error(interp, ret as krb5_error_code);
            retcode = 1 as libc::c_int
        }
    }
    krb5_free_principal(context, princ);
    return retcode;
}
#[c2rust::src_loc = "1947:1"]
unsafe extern "C" fn tcl_kadm5_chpass_principal_util(mut clientData:
                                                         ClientData,
                                                     mut interp:
                                                         *mut Tcl_Interp,
                                                     mut argc: libc::c_int,
                                                     mut argv:
                                                         *mut *const libc::c_char)
 -> libc::c_int {
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut new_pw: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pw_ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pw_ret_var: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut msg_ret: [libc::c_char; 1024] = [0; 1024];
    let mut msg_ret_var: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut krb5_ret: krb5_error_code = 0;
    let mut ret: kadm5_ret_t = 0;
    let mut retcode: libc::c_int = 0 as libc::c_int;
    let mut server_handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut whoami: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    argv = argv.offset(1);
    argc -= 1;
    if argc != 4 as libc::c_int + 1 as libc::c_int {
        Tcl_AppendResult(interp, whoami,
                         b": \x00" as *const u8 as *const libc::c_char,
                         arg_error, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    let mut ltcl_ret: libc::c_int = 0;
    ltcl_ret =
        get_server_handle(interp, *argv.offset(0 as libc::c_int as isize),
                          &mut server_handle);
    if ltcl_ret != 0 as libc::c_int { return ltcl_ret }
    argv = argv.offset(1);
    argc -= 1;
    krb5_ret =
        krb5_parse_name(context, *argv.offset(0 as libc::c_int as isize),
                        &mut princ);
    if krb5_ret != 0 as libc::c_int {
        stash_error(interp, krb5_ret);
        Tcl_AppendElement(interp,
                          b"while parsing principal name\x00" as *const u8 as
                              *const libc::c_char);
        return 1 as libc::c_int
    }
    if parse_str(interp, *argv.offset(1 as libc::c_int as isize), &mut new_pw)
           != 0 as libc::c_int {
        Tcl_AppendElement(interp,
                          b"while parsing new password\x00" as *const u8 as
                              *const libc::c_char);
        retcode = 1 as libc::c_int
    } else if parse_str(interp, *argv.offset(3 as libc::c_int as isize),
                        &mut pw_ret_var) != 0 as libc::c_int {
        Tcl_AppendElement(interp,
                          b"while parsing pw_ret variable name\x00" as
                              *const u8 as *const libc::c_char);
        retcode = 1 as libc::c_int
    } else if parse_str(interp, *argv.offset(4 as libc::c_int as isize),
                        &mut msg_ret_var) != 0 as libc::c_int {
        Tcl_AppendElement(interp,
                          b"while parsing msg_ret variable name\x00" as
                              *const u8 as *const libc::c_char);
        retcode = 1 as libc::c_int
    } else {
        ret =
            kadm5_chpass_principal_util(server_handle, princ, new_pw,
                                        if !pw_ret_var.is_null() {
                                            &mut pw_ret
                                        } else {
                                            0 as *mut *mut libc::c_char
                                        },
                                        if !msg_ret_var.is_null() {
                                            msg_ret.as_mut_ptr()
                                        } else { 0 as *mut libc::c_char },
                                        if !msg_ret_var.is_null() {
                                            ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                as libc::c_ulong
                                        } else {
                                            0 as libc::c_int as libc::c_ulong
                                        } as libc::c_uint);
        if ret == 0 as libc::c_int as libc::c_long {
            if !pw_ret_var.is_null() &&
                   Tcl_SetVar2(interp, pw_ret_var, 0 as *const libc::c_char,
                               pw_ret, 0x200 as libc::c_int).is_null() {
                Tcl_AppendElement(interp,
                                  b"while setting pw_ret variable\x00" as
                                      *const u8 as *const libc::c_char);
                retcode = 1 as libc::c_int
            } else if !msg_ret_var.is_null() &&
                          Tcl_SetVar2(interp, msg_ret_var,
                                      0 as *const libc::c_char,
                                      msg_ret.as_mut_ptr(),
                                      0x200 as libc::c_int).is_null() {
                Tcl_AppendElement(interp,
                                  b"while setting msg_ret variable\x00" as
                                      *const u8 as *const libc::c_char);
                retcode = 1 as libc::c_int
            } else {
                set_ok(interp,
                       b"Password changed.\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
            }
        } else {
            stash_error(interp, ret as krb5_error_code);
            retcode = 1 as libc::c_int
        }
    }
    krb5_free_principal(context, princ);
    return retcode;
}
#[c2rust::src_loc = "2032:1"]
unsafe extern "C" fn tcl_kadm5_randkey_principal(mut clientData: ClientData,
                                                 mut interp: *mut Tcl_Interp,
                                                 mut argc: libc::c_int,
                                                 mut argv:
                                                     *mut *const libc::c_char)
 -> libc::c_int {
    let mut current_block: u64;
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut keyblocks: *mut krb5_keyblock = 0 as *mut krb5_keyblock;
    let mut num_keys: libc::c_int = 0;
    let mut keyblock_var: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num_var: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 50] = [0; 50];
    let mut keyblock_dstring: *mut Tcl_DString = 0 as *mut Tcl_DString;
    let mut krb5_ret: krb5_error_code = 0;
    let mut ret: kadm5_ret_t = 0;
    let mut retcode: libc::c_int = 0 as libc::c_int;
    let mut server_handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut whoami: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    argv = argv.offset(1);
    argc -= 1;
    if argc != 3 as libc::c_int + 1 as libc::c_int {
        Tcl_AppendResult(interp, whoami,
                         b": \x00" as *const u8 as *const libc::c_char,
                         arg_error, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    let mut ltcl_ret: libc::c_int = 0;
    ltcl_ret =
        get_server_handle(interp, *argv.offset(0 as libc::c_int as isize),
                          &mut server_handle);
    if ltcl_ret != 0 as libc::c_int { return ltcl_ret }
    argv = argv.offset(1);
    argc -= 1;
    krb5_ret =
        krb5_parse_name(context, *argv.offset(0 as libc::c_int as isize),
                        &mut princ);
    if krb5_ret != 0 as libc::c_int {
        stash_error(interp, krb5_ret);
        Tcl_AppendElement(interp,
                          b"while parsing principal name\x00" as *const u8 as
                              *const libc::c_char);
        return 1 as libc::c_int
    }
    if parse_str(interp, *argv.offset(1 as libc::c_int as isize),
                 &mut keyblock_var) != 0 as libc::c_int {
        Tcl_AppendElement(interp,
                          b"while parsing keyblock variable name\x00" as
                              *const u8 as *const libc::c_char);
        retcode = 1 as libc::c_int
    } else if parse_str(interp, *argv.offset(2 as libc::c_int as isize),
                        &mut num_var) != 0 as libc::c_int {
        Tcl_AppendElement(interp,
                          b"while parsing keyblock variable name\x00" as
                              *const u8 as *const libc::c_char);
        retcode = 1 as libc::c_int
    } else {
        ret =
            kadm5_randkey_principal(server_handle, princ,
                                    if !keyblock_var.is_null() {
                                        &mut keyblocks
                                    } else { 0 as *mut *mut krb5_keyblock },
                                    &mut num_keys);
        if ret == 0 as libc::c_int as libc::c_long {
            if !keyblock_var.is_null() {
                keyblock_dstring = unparse_keyblocks(keyblocks, num_keys);
                if Tcl_SetVar2(interp, keyblock_var, 0 as *const libc::c_char,
                               (*keyblock_dstring).string,
                               0x200 as libc::c_int).is_null() {
                    Tcl_AppendElement(interp,
                                      b"while setting keyblock variable\x00"
                                          as *const u8 as
                                          *const libc::c_char);
                    retcode = 1 as libc::c_int;
                    current_block = 450520887190808148;
                } else { current_block = 1118134448028020070; }
            } else { current_block = 1118134448028020070; }
            match current_block {
                450520887190808148 => { }
                _ => {
                    if !num_var.is_null() {
                        sprintf(buf.as_mut_ptr(),
                                b"%d\x00" as *const u8 as *const libc::c_char,
                                num_keys);
                        if Tcl_SetVar2(interp, num_var,
                                       0 as *const libc::c_char,
                                       buf.as_mut_ptr(),
                                       0x200 as libc::c_int).is_null() {
                            Tcl_AppendElement(interp,
                                              b"while setting num_keys variable\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                        }
                    }
                    set_ok(interp,
                           b"Key randomized.\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char);
                }
            }
        } else {
            stash_error(interp, ret as krb5_error_code);
            retcode = 1 as libc::c_int
        }
    }
    krb5_free_principal(context, princ);
    if !keyblock_dstring.is_null() {
        Tcl_DStringFree(keyblock_dstring);
        free(keyblock_dstring as *mut libc::c_void);
    }
    return retcode;
}
#[c2rust::src_loc = "2106:1"]
unsafe extern "C" fn tcl_kadm5_get_principal(mut clientData: ClientData,
                                             mut interp: *mut Tcl_Interp,
                                             mut argc: libc::c_int,
                                             mut argv:
                                                 *mut *const libc::c_char)
 -> libc::c_int {
    let mut princ: krb5_principal = 0 as *mut krb5_principal_data;
    let mut ent: kadm5_principal_ent_rec =
        kadm5_principal_ent_rec{principal: 0 as *mut krb5_principal_data,
                                princ_expire_time: 0,
                                last_pwd_change: 0,
                                pw_expiration: 0,
                                max_life: 0,
                                mod_name: 0 as *mut krb5_principal_data,
                                mod_date: 0,
                                attributes: 0,
                                kvno: 0,
                                mkvno: 0,
                                policy: 0 as *mut libc::c_char,
                                aux_attributes: 0,
                                max_renewable_life: 0,
                                last_success: 0,
                                last_failed: 0,
                                fail_auth_count: 0,
                                n_key_data: 0,
                                n_tl_data: 0,
                                tl_data: 0 as *mut krb5_tl_data,
                                key_data: 0 as *mut krb5_key_data,};
    let mut ent_dstring: *mut Tcl_DString = 0 as *mut Tcl_DString;
    let mut ent_var: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut krb5_ret: krb5_error_code = 0;
    let mut tcl_ret: libc::c_int = 0;
    let mut ret: kadm5_ret_t = -(1 as libc::c_int) as kadm5_ret_t;
    let mut mask: krb5_int32 = 0;
    let mut retcode: libc::c_int = 0 as libc::c_int;
    let mut server_handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut whoami: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    argv = argv.offset(1);
    argc -= 1;
    if argc != 3 as libc::c_int + 1 as libc::c_int {
        Tcl_AppendResult(interp, whoami,
                         b": \x00" as *const u8 as *const libc::c_char,
                         arg_error, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    let mut ltcl_ret: libc::c_int = 0;
    ltcl_ret =
        get_server_handle(interp, *argv.offset(0 as libc::c_int as isize),
                          &mut server_handle);
    if ltcl_ret != 0 as libc::c_int { return ltcl_ret }
    argv = argv.offset(1);
    argc -= 1;
    tcl_ret =
        parse_str(interp, *argv.offset(0 as libc::c_int as isize), &mut name);
    if tcl_ret != 0 as libc::c_int { return tcl_ret }
    if !name.is_null() {
        krb5_ret = krb5_parse_name(context, name, &mut princ);
        if krb5_ret != 0 as libc::c_int {
            stash_error(interp, krb5_ret);
            Tcl_AppendElement(interp,
                              b"while parsing principal name\x00" as *const u8
                                  as *const libc::c_char);
            return 1 as libc::c_int
        }
    } else { princ = 0 as krb5_principal }
    tcl_ret =
        parse_str(interp, *argv.offset(1 as libc::c_int as isize),
                  &mut ent_var);
    if tcl_ret != 0 as libc::c_int {
        Tcl_AppendElement(interp,
                          b"while parsing entry variable name\x00" as
                              *const u8 as *const libc::c_char);
        retcode = 1 as libc::c_int
    } else {
        tcl_ret =
            parse_principal_mask(interp,
                                 *argv.offset(2 as libc::c_int as isize),
                                 &mut mask);
        if tcl_ret != 0 as libc::c_int {
            Tcl_AppendElement(interp,
                              b"while parsing principal mask\x00" as *const u8
                                  as *const libc::c_char);
            retcode = 1 as libc::c_int
        } else {
            ret =
                kadm5_get_principal(server_handle, princ,
                                    if !ent_var.is_null() {
                                        &mut ent
                                    } else {
                                        0 as *mut kadm5_principal_ent_rec
                                    }, mask as libc::c_long);
            if ret == 0 as libc::c_int as libc::c_long {
                if !ent_var.is_null() {
                    ent_dstring = unparse_principal_ent(&mut ent, mask);
                    if Tcl_SetVar2(interp, ent_var, 0 as *const libc::c_char,
                                   (*ent_dstring).string,
                                   0x200 as libc::c_int).is_null() {
                        Tcl_AppendElement(interp,
                                          b"while setting entry variable\x00"
                                              as *const u8 as
                                              *const libc::c_char);
                        retcode = 1 as libc::c_int
                    } else {
                        set_ok(interp,
                               b"Principal retrieved.\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char);
                    }
                }
            } else {
                stash_error(interp, ret as krb5_error_code);
                retcode = 1 as libc::c_int
            }
        }
    }
    if !ent_dstring.is_null() {
        Tcl_DStringFree(ent_dstring);
        free(ent_dstring as *mut libc::c_void);
    }
    if !princ.is_null() { krb5_free_principal(context, princ); }
    if ret == 0 as libc::c_int as libc::c_long && !ent_var.is_null() &&
           {
               ret = kadm5_free_principal_ent(server_handle, &mut ent);
               (ret) != 0
           } && retcode == 0 as libc::c_int {
        stash_error(interp, ret as krb5_error_code);
        retcode = 1 as libc::c_int
    }
    return retcode;
}
#[c2rust::src_loc = "2180:1"]
unsafe extern "C" fn tcl_kadm5_create_policy(mut clientData: ClientData,
                                             mut interp: *mut Tcl_Interp,
                                             mut argc: libc::c_int,
                                             mut argv:
                                                 *mut *const libc::c_char)
 -> libc::c_int {
    let mut tcl_ret: libc::c_int = 0;
    let mut ret: kadm5_ret_t = 0;
    let mut retcode: libc::c_int = 0 as libc::c_int;
    let mut policy_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut policy: kadm5_policy_ent_t = 0 as kadm5_policy_ent_t;
    let mut mask: krb5_int32 = 0;
    let mut server_handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut whoami: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    argv = argv.offset(1);
    argc -= 1;
    if argc != 2 as libc::c_int + 1 as libc::c_int {
        Tcl_AppendResult(interp, whoami,
                         b": \x00" as *const u8 as *const libc::c_char,
                         arg_error, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    let mut ltcl_ret: libc::c_int = 0;
    ltcl_ret =
        get_server_handle(interp, *argv.offset(0 as libc::c_int as isize),
                          &mut server_handle);
    if ltcl_ret != 0 as libc::c_int { return ltcl_ret }
    argv = argv.offset(1);
    argc -= 1;
    tcl_ret =
        parse_str(interp, *argv.offset(0 as libc::c_int as isize),
                  &mut policy_string);
    if tcl_ret != 0 as libc::c_int {
        Tcl_AppendElement(interp,
                          b"while parsing policy\x00" as *const u8 as
                              *const libc::c_char);
        return tcl_ret
    }
    if !policy_string.is_null() &&
           {
               tcl_ret = parse_policy_ent(interp, policy_string, &mut policy);
               (tcl_ret) != 0 as libc::c_int
           } {
        return tcl_ret
    }
    tcl_ret =
        parse_policy_mask(interp, *argv.offset(1 as libc::c_int as isize),
                          &mut mask);
    if tcl_ret != 0 as libc::c_int {
        retcode = tcl_ret
    } else {
        ret =
            kadm5_create_policy(server_handle, policy, mask as libc::c_long);
        if ret != 0 as libc::c_int as libc::c_long {
            stash_error(interp, ret as krb5_error_code);
            retcode = 1 as libc::c_int
        } else {
            set_ok(interp,
                   b"Policy created.\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char);
        }
    }
    if !policy.is_null() { free_policy_ent(&mut policy); }
    return retcode;
}
#[c2rust::src_loc = "2228:1"]
unsafe extern "C" fn tcl_kadm5_delete_policy(mut clientData: ClientData,
                                             mut interp: *mut Tcl_Interp,
                                             mut argc: libc::c_int,
                                             mut argv:
                                                 *mut *const libc::c_char)
 -> libc::c_int {
    let mut ret: kadm5_ret_t = 0;
    let mut policy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut server_handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut whoami: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    argv = argv.offset(1);
    argc -= 1;
    if argc != 1 as libc::c_int + 1 as libc::c_int {
        Tcl_AppendResult(interp, whoami,
                         b": \x00" as *const u8 as *const libc::c_char,
                         arg_error, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    let mut ltcl_ret: libc::c_int = 0;
    ltcl_ret =
        get_server_handle(interp, *argv.offset(0 as libc::c_int as isize),
                          &mut server_handle);
    if ltcl_ret != 0 as libc::c_int { return ltcl_ret }
    argv = argv.offset(1);
    argc -= 1;
    if parse_str(interp, *argv.offset(0 as libc::c_int as isize), &mut policy)
           != 0 as libc::c_int {
        Tcl_AppendElement(interp,
                          b"while parsing policy name\x00" as *const u8 as
                              *const libc::c_char);
        return 1 as libc::c_int
    }
    ret = kadm5_delete_policy(server_handle, policy);
    if ret != 0 as libc::c_int as libc::c_long {
        stash_error(interp, ret as krb5_error_code);
        return 1 as libc::c_int
    } else {
        set_ok(interp,
               b"Policy deleted.\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char);
        return 0 as libc::c_int
    };
}
#[c2rust::src_loc = "2255:1"]
unsafe extern "C" fn tcl_kadm5_modify_policy(mut clientData: ClientData,
                                             mut interp: *mut Tcl_Interp,
                                             mut argc: libc::c_int,
                                             mut argv:
                                                 *mut *const libc::c_char)
 -> libc::c_int {
    let mut policy_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut policy: kadm5_policy_ent_t = 0 as kadm5_policy_ent_t;
    let mut tcl_ret: libc::c_int = 0;
    let mut mask: krb5_int32 = 0;
    let mut retcode: libc::c_int = 0 as libc::c_int;
    let mut ret: kadm5_ret_t = 0;
    let mut server_handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut whoami: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    argv = argv.offset(1);
    argc -= 1;
    if argc != 2 as libc::c_int + 1 as libc::c_int {
        Tcl_AppendResult(interp, whoami,
                         b": \x00" as *const u8 as *const libc::c_char,
                         arg_error, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    let mut ltcl_ret: libc::c_int = 0;
    ltcl_ret =
        get_server_handle(interp, *argv.offset(0 as libc::c_int as isize),
                          &mut server_handle);
    if ltcl_ret != 0 as libc::c_int { return ltcl_ret }
    argv = argv.offset(1);
    argc -= 1;
    tcl_ret =
        parse_str(interp, *argv.offset(0 as libc::c_int as isize),
                  &mut policy_string);
    if tcl_ret != 0 as libc::c_int {
        Tcl_AppendElement(interp,
                          b"while parsing policy\x00" as *const u8 as
                              *const libc::c_char);
        return tcl_ret
    }
    if !policy_string.is_null() &&
           {
               tcl_ret = parse_policy_ent(interp, policy_string, &mut policy);
               (tcl_ret) != 0 as libc::c_int
           } {
        return tcl_ret
    }
    tcl_ret =
        parse_policy_mask(interp, *argv.offset(1 as libc::c_int as isize),
                          &mut mask);
    if tcl_ret != 0 as libc::c_int {
        retcode = 1 as libc::c_int
    } else {
        ret =
            kadm5_modify_policy(server_handle, policy, mask as libc::c_long);
        if ret != 0 as libc::c_int as libc::c_long {
            stash_error(interp, ret as krb5_error_code);
            retcode = 1 as libc::c_int
        } else {
            set_ok(interp,
                   b"Policy modified.\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char);
        }
    }
    if !policy.is_null() { free_policy_ent(&mut policy); }
    return retcode;
}
#[c2rust::src_loc = "2301:1"]
unsafe extern "C" fn tcl_kadm5_get_policy(mut clientData: ClientData,
                                          mut interp: *mut Tcl_Interp,
                                          mut argc: libc::c_int,
                                          mut argv: *mut *const libc::c_char)
 -> libc::c_int {
    let mut ent: kadm5_policy_ent_rec =
        kadm5_policy_ent_rec{policy: 0 as *mut libc::c_char,
                             pw_min_life: 0,
                             pw_max_life: 0,
                             pw_min_length: 0,
                             pw_min_classes: 0,
                             pw_history_num: 0,
                             policy_refcnt: 0,
                             pw_max_fail: 0,
                             pw_failcnt_interval: 0,
                             pw_lockout_duration: 0,
                             attributes: 0,
                             max_life: 0,
                             max_renewable_life: 0,
                             allowed_keysalts: 0 as *mut libc::c_char,
                             n_tl_data: 0,
                             tl_data: 0 as *mut krb5_tl_data,};
    let mut ent_dstring: *mut Tcl_DString = 0 as *mut Tcl_DString;
    let mut policy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ent_var: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: kadm5_ret_t = 0;
    let mut retcode: libc::c_int = 0 as libc::c_int;
    let mut server_handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut whoami: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    argv = argv.offset(1);
    argc -= 1;
    if argc != 2 as libc::c_int + 1 as libc::c_int {
        Tcl_AppendResult(interp, whoami,
                         b": \x00" as *const u8 as *const libc::c_char,
                         arg_error, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    let mut ltcl_ret: libc::c_int = 0;
    ltcl_ret =
        get_server_handle(interp, *argv.offset(0 as libc::c_int as isize),
                          &mut server_handle);
    if ltcl_ret != 0 as libc::c_int { return ltcl_ret }
    argv = argv.offset(1);
    argc -= 1;
    if parse_str(interp, *argv.offset(0 as libc::c_int as isize), &mut policy)
           != 0 as libc::c_int {
        Tcl_AppendElement(interp,
                          b"while parsing policy name\x00" as *const u8 as
                              *const libc::c_char);
        return 1 as libc::c_int
    }
    if parse_str(interp, *argv.offset(1 as libc::c_int as isize),
                 &mut ent_var) != 0 as libc::c_int {
        Tcl_AppendElement(interp,
                          b"while parsing entry variable name\x00" as
                              *const u8 as *const libc::c_char);
        return 1 as libc::c_int
    }
    ret =
        kadm5_get_policy(server_handle, policy,
                         if !ent_var.is_null() {
                             &mut ent
                         } else { 0 as *mut kadm5_policy_ent_rec });
    if ret == 0 as libc::c_int as libc::c_long {
        if !ent_var.is_null() {
            ent_dstring = unparse_policy_ent(&mut ent);
            if Tcl_SetVar2(interp, ent_var, 0 as *const libc::c_char,
                           (*ent_dstring).string,
                           0x200 as libc::c_int).is_null() {
                Tcl_AppendElement(interp,
                                  b"while setting entry variable\x00" as
                                      *const u8 as *const libc::c_char);
                retcode = 1 as libc::c_int
            } else {
                set_ok(interp,
                       b"Policy retrieved.\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char);
            }
        }
    } else {
        stash_error(interp, ret as krb5_error_code);
        retcode = 1 as libc::c_int
    }
    if !ent_dstring.is_null() {
        Tcl_DStringFree(ent_dstring);
        free(ent_dstring as *mut libc::c_void);
    }
    if !ent_var.is_null() && ret == 0 as libc::c_int as libc::c_long &&
           {
               ret = kadm5_free_policy_ent(server_handle, &mut ent);
               (ret) != 0
           } && retcode == 0 as libc::c_int {
        stash_error(interp, ret as krb5_error_code);
        retcode = 1 as libc::c_int
    }
    return retcode;
}
#[c2rust::src_loc = "2359:1"]
unsafe extern "C" fn tcl_kadm5_free_principal_ent(mut clientData: ClientData,
                                                  mut interp: *mut Tcl_Interp,
                                                  mut argc: libc::c_int,
                                                  mut argv:
                                                      *mut *const libc::c_char)
 -> libc::c_int {
    let mut ent_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ent: kadm5_principal_ent_t = 0 as *mut _kadm5_principal_ent_t;
    let mut ret: kadm5_ret_t = 0;
    let mut server_handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut whoami: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    argv = argv.offset(1);
    argc -= 1;
    if argc != 1 as libc::c_int + 1 as libc::c_int {
        Tcl_AppendResult(interp, whoami,
                         b": \x00" as *const u8 as *const libc::c_char,
                         arg_error, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    let mut ltcl_ret: libc::c_int = 0;
    ltcl_ret =
        get_server_handle(interp, *argv.offset(0 as libc::c_int as isize),
                          &mut server_handle);
    if ltcl_ret != 0 as libc::c_int { return ltcl_ret }
    argv = argv.offset(1);
    argc -= 1;
    if parse_str(interp, *argv.offset(0 as libc::c_int as isize),
                 &mut ent_name) != 0 as libc::c_int {
        Tcl_AppendElement(interp,
                          b"while parsing entry name\x00" as *const u8 as
                              *const libc::c_char);
        return 1 as libc::c_int
    }
    if ent_name.is_null() &&
           {
               ret =
                   kadm5_free_principal_ent(server_handle,
                                            0 as kadm5_principal_ent_t);
               (ret) != 0
           } {
        stash_error(interp, ret as krb5_error_code);
        return 1 as libc::c_int
    } else {
        let mut entry: *mut Tcl_HashEntry = 0 as *mut Tcl_HashEntry;
        if strncmp(ent_name,
                   b"principal\x00" as *const u8 as *const libc::c_char,
                   (::std::mem::size_of::<[libc::c_char; 10]>() as
                        libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                        libc::c_ulong)) != 0 {
            Tcl_AppendResult(interp,
                             b"invalid principal handle \"\x00" as *const u8
                                 as *const libc::c_char, ent_name,
                             b"\"\x00" as *const u8 as *const libc::c_char,
                             0 as libc::c_int);
            return 1 as libc::c_int
        }
        if struct_table.is_null() {
            struct_table =
                malloc(::std::mem::size_of::<Tcl_HashTable>() as
                           libc::c_ulong) as *mut Tcl_HashTable;
            if struct_table.is_null() {
                fprintf(stderr,
                        b"Out of memory!\n\x00" as *const u8 as
                            *const libc::c_char);
                exit(1 as libc::c_int);
                /* XXX */
            }
            Tcl_InitHashTable(struct_table, 0 as libc::c_int);
        }
        entry =
            Some((*struct_table).findProc.expect("non-null function pointer")).expect("non-null function pointer")(struct_table,
                                                                                                                   ent_name
                                                                                                                       as
                                                                                                                       *const libc::c_char);
        if entry.is_null() {
            Tcl_AppendResult(interp,
                             b"principal handle \"\x00" as *const u8 as
                                 *const libc::c_char, ent_name,
                             b"\" not found\x00" as *const u8 as
                                 *const libc::c_char, 0 as libc::c_int);
            return 1 as libc::c_int
        }
        ent = (*entry).clientData as kadm5_principal_ent_t;
        ret = kadm5_free_principal_ent(server_handle, ent);
        if ret != 0 as libc::c_int as libc::c_long {
            stash_error(interp, ret as krb5_error_code);
            return 1 as libc::c_int
        }
        Tcl_DeleteHashEntry(entry);
    }
    set_ok(interp,
           b"Principal freed.\x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "2415:1"]
unsafe extern "C" fn tcl_kadm5_free_policy_ent(mut clientData: ClientData,
                                               mut interp: *mut Tcl_Interp,
                                               mut argc: libc::c_int,
                                               mut argv:
                                                   *mut *const libc::c_char)
 -> libc::c_int {
    let mut ent_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ent: kadm5_policy_ent_t = 0 as *mut _kadm5_policy_ent_t;
    let mut ret: kadm5_ret_t = 0;
    let mut server_handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut whoami: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    argv = argv.offset(1);
    argc -= 1;
    if argc != 1 as libc::c_int + 1 as libc::c_int {
        Tcl_AppendResult(interp, whoami,
                         b": \x00" as *const u8 as *const libc::c_char,
                         arg_error, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    let mut ltcl_ret: libc::c_int = 0;
    ltcl_ret =
        get_server_handle(interp, *argv.offset(0 as libc::c_int as isize),
                          &mut server_handle);
    if ltcl_ret != 0 as libc::c_int { return ltcl_ret }
    argv = argv.offset(1);
    argc -= 1;
    if parse_str(interp, *argv.offset(0 as libc::c_int as isize),
                 &mut ent_name) != 0 as libc::c_int {
        Tcl_AppendElement(interp,
                          b"while parsing entry name\x00" as *const u8 as
                              *const libc::c_char);
        return 1 as libc::c_int
    }
    if ent_name.is_null() &&
           {
               ret =
                   kadm5_free_policy_ent(server_handle,
                                         0 as kadm5_policy_ent_t);
               (ret) != 0
           } {
        stash_error(interp, ret as krb5_error_code);
        return 1 as libc::c_int
    } else {
        let mut entry: *mut Tcl_HashEntry = 0 as *mut Tcl_HashEntry;
        if strncmp(ent_name,
                   b"policy\x00" as *const u8 as *const libc::c_char,
                   (::std::mem::size_of::<[libc::c_char; 7]>() as
                        libc::c_ulong).wrapping_sub(1 as libc::c_int as
                                                        libc::c_ulong)) != 0 {
            Tcl_AppendResult(interp,
                             b"invalid principal handle \"\x00" as *const u8
                                 as *const libc::c_char, ent_name,
                             b"\"\x00" as *const u8 as *const libc::c_char,
                             0 as libc::c_int);
            return 1 as libc::c_int
        }
        if struct_table.is_null() {
            struct_table =
                malloc(::std::mem::size_of::<Tcl_HashTable>() as
                           libc::c_ulong) as *mut Tcl_HashTable;
            if struct_table.is_null() {
                fprintf(stderr,
                        b"Out of memory!\n\x00" as *const u8 as
                            *const libc::c_char);
                exit(1 as libc::c_int);
                /* XXX */
            }
            Tcl_InitHashTable(struct_table, 0 as libc::c_int);
        }
        entry =
            Some((*struct_table).findProc.expect("non-null function pointer")).expect("non-null function pointer")(struct_table,
                                                                                                                   ent_name
                                                                                                                       as
                                                                                                                       *const libc::c_char);
        if entry.is_null() {
            Tcl_AppendResult(interp,
                             b"policy handle \"\x00" as *const u8 as
                                 *const libc::c_char, ent_name,
                             b"\" not found\x00" as *const u8 as
                                 *const libc::c_char, 0 as libc::c_int);
            return 1 as libc::c_int
        }
        ent = (*entry).clientData as kadm5_policy_ent_t;
        ret = kadm5_free_policy_ent(server_handle, ent);
        if ret != 0 as libc::c_int as libc::c_long {
            stash_error(interp, ret as krb5_error_code);
            return 1 as libc::c_int
        }
        Tcl_DeleteHashEntry(entry);
    }
    set_ok(interp,
           b"Policy freed.\x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char);
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "2470:1"]
unsafe extern "C" fn tcl_kadm5_get_privs(mut clientData: ClientData,
                                         mut interp: *mut Tcl_Interp,
                                         mut argc: libc::c_int,
                                         mut argv: *mut *const libc::c_char)
 -> libc::c_int {
    let mut set_ret: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: kadm5_ret_t = 0;
    let mut priv_var: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut privs: libc::c_long = 0;
    let mut server_handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut whoami: *const libc::c_char =
        *argv.offset(0 as libc::c_int as isize);
    argv = argv.offset(1);
    argc -= 1;
    if argc != 1 as libc::c_int + 1 as libc::c_int {
        Tcl_AppendResult(interp, whoami,
                         b": \x00" as *const u8 as *const libc::c_char,
                         arg_error, 0 as libc::c_int);
        return 1 as libc::c_int
    }
    let mut ltcl_ret: libc::c_int = 0;
    ltcl_ret =
        get_server_handle(interp, *argv.offset(0 as libc::c_int as isize),
                          &mut server_handle);
    if ltcl_ret != 0 as libc::c_int { return ltcl_ret }
    argv = argv.offset(1);
    argc -= 1;
    if parse_str(interp, *argv.offset(0 as libc::c_int as isize),
                 &mut priv_var) != 0 as libc::c_int {
        Tcl_AppendElement(interp,
                          b"while parsing privs variable name\x00" as
                              *const u8 as *const libc::c_char);
        return 1 as libc::c_int
    }
    ret =
        kadm5_get_privs(server_handle,
                        if !priv_var.is_null() {
                            &mut privs
                        } else { 0 as *mut libc::c_long });
    if ret == 0 as libc::c_int as libc::c_long {
        if !priv_var.is_null() {
            let mut str: *mut Tcl_DString =
                unparse_privs(privs as krb5_flags);
            set_ret =
                Tcl_SetVar2(interp, priv_var, 0 as *const libc::c_char,
                            (*str).string, 0x200 as libc::c_int);
            Tcl_DStringFree(str);
            free(str as *mut libc::c_void);
            if set_ret.is_null() {
                Tcl_AppendElement(interp,
                                  b"while setting priv variable\x00" as
                                      *const u8 as *const libc::c_char);
                return 1 as libc::c_int
            }
        }
        set_ok(interp,
               b"Privileges retrieved.\x00" as *const u8 as
                   *const libc::c_char as *mut libc::c_char);
        return 0 as libc::c_int
    } else {
        stash_error(interp, ret as krb5_error_code);
        return 1 as libc::c_int
    };
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
#[no_mangle]
#[c2rust::src_loc = "2509:1"]
pub unsafe extern "C" fn Tcl_kadm5_init(mut interp: *mut Tcl_Interp) {
    let mut buf: [libc::c_char; 20] = [0; 20];
    Tcl_SetVar2(interp,
                b"KADM5_ADMIN_SERVICE\x00" as *const u8 as
                    *const libc::c_char, 0 as *const libc::c_char,
                b"kadmin/admin\x00" as *const u8 as *const libc::c_char,
                1 as libc::c_int);
    Tcl_SetVar2(interp,
                b"KADM5_CHANGEPW_SERVICE\x00" as *const u8 as
                    *const libc::c_char, 0 as *const libc::c_char,
                b"kadmin/changepw\x00" as *const u8 as *const libc::c_char,
                1 as libc::c_int);
    sprintf(buf.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            0x12345600 as libc::c_int | 0x1 as libc::c_int);
    Tcl_SetVar2(interp,
                b"KADM5_STRUCT_VERSION\x00" as *const u8 as
                    *const libc::c_char, 0 as *const libc::c_char,
                buf.as_mut_ptr(), 1 as libc::c_int);
    sprintf(buf.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            0x12345700 as libc::c_int | 0x2 as libc::c_int);
    Tcl_SetVar2(interp,
                b"KADM5_API_VERSION_2\x00" as *const u8 as
                    *const libc::c_char, 0 as *const libc::c_char,
                buf.as_mut_ptr(), 1 as libc::c_int);
    sprintf(buf.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            0x12345700 as libc::c_int | 0x3 as libc::c_int);
    Tcl_SetVar2(interp,
                b"KADM5_API_VERSION_3\x00" as *const u8 as
                    *const libc::c_char, 0 as *const libc::c_char,
                buf.as_mut_ptr(), 1 as libc::c_int);
    sprintf(buf.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            0x12345700 as libc::c_int | 0x4 as libc::c_int);
    Tcl_SetVar2(interp,
                b"KADM5_API_VERSION_4\x00" as *const u8 as
                    *const libc::c_char, 0 as *const libc::c_char,
                buf.as_mut_ptr(), 1 as libc::c_int);
    sprintf(buf.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            0x12345700 as libc::c_int);
    Tcl_SetVar2(interp,
                b"KADM5_API_VERSION_MASK\x00" as *const u8 as
                    *const libc::c_char, 0 as *const libc::c_char,
                buf.as_mut_ptr(), 1 as libc::c_int);
    sprintf(buf.as_mut_ptr(), b"%d\x00" as *const u8 as *const libc::c_char,
            0x12345600 as libc::c_int);
    Tcl_SetVar2(interp,
                b"KADM5_STRUCT_VERSION_MASK\x00" as *const u8 as
                    *const libc::c_char, 0 as *const libc::c_char,
                buf.as_mut_ptr(), 1 as libc::c_int);
    Tcl_CreateCommand(interp,
                      b"kadm5_init\x00" as *const u8 as *const libc::c_char,
                      Some(tcl_kadm5_init as
                               unsafe extern "C" fn(_: ClientData,
                                                    _: *mut Tcl_Interp,
                                                    _: libc::c_int,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> libc::c_int), 0 as ClientData, None);
    Tcl_CreateCommand(interp,
                      b"kadm5_init_with_creds\x00" as *const u8 as
                          *const libc::c_char,
                      Some(tcl_kadm5_init_with_creds as
                               unsafe extern "C" fn(_: ClientData,
                                                    _: *mut Tcl_Interp,
                                                    _: libc::c_int,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> libc::c_int), 0 as ClientData, None);
    Tcl_CreateCommand(interp,
                      b"kadm5_destroy\x00" as *const u8 as
                          *const libc::c_char,
                      Some(tcl_kadm5_destroy as
                               unsafe extern "C" fn(_: ClientData,
                                                    _: *mut Tcl_Interp,
                                                    _: libc::c_int,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> libc::c_int), 0 as ClientData, None);
    Tcl_CreateCommand(interp,
                      b"kadm5_create_principal\x00" as *const u8 as
                          *const libc::c_char,
                      Some(tcl_kadm5_create_principal as
                               unsafe extern "C" fn(_: ClientData,
                                                    _: *mut Tcl_Interp,
                                                    _: libc::c_int,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> libc::c_int), 0 as ClientData, None);
    Tcl_CreateCommand(interp,
                      b"kadm5_delete_principal\x00" as *const u8 as
                          *const libc::c_char,
                      Some(tcl_kadm5_delete_principal as
                               unsafe extern "C" fn(_: ClientData,
                                                    _: *mut Tcl_Interp,
                                                    _: libc::c_int,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> libc::c_int), 0 as ClientData, None);
    Tcl_CreateCommand(interp,
                      b"kadm5_modify_principal\x00" as *const u8 as
                          *const libc::c_char,
                      Some(tcl_kadm5_modify_principal as
                               unsafe extern "C" fn(_: ClientData,
                                                    _: *mut Tcl_Interp,
                                                    _: libc::c_int,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> libc::c_int), 0 as ClientData, None);
    Tcl_CreateCommand(interp,
                      b"kadm5_rename_principal\x00" as *const u8 as
                          *const libc::c_char,
                      Some(tcl_kadm5_rename_principal as
                               unsafe extern "C" fn(_: ClientData,
                                                    _: *mut Tcl_Interp,
                                                    _: libc::c_int,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> libc::c_int), 0 as ClientData, None);
    Tcl_CreateCommand(interp,
                      b"kadm5_chpass_principal\x00" as *const u8 as
                          *const libc::c_char,
                      Some(tcl_kadm5_chpass_principal as
                               unsafe extern "C" fn(_: ClientData,
                                                    _: *mut Tcl_Interp,
                                                    _: libc::c_int,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> libc::c_int), 0 as ClientData, None);
    Tcl_CreateCommand(interp,
                      b"kadm5_chpass_principal_util\x00" as *const u8 as
                          *const libc::c_char,
                      Some(tcl_kadm5_chpass_principal_util as
                               unsafe extern "C" fn(_: ClientData,
                                                    _: *mut Tcl_Interp,
                                                    _: libc::c_int,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> libc::c_int), 0 as ClientData, None);
    Tcl_CreateCommand(interp,
                      b"kadm5_randkey_principal\x00" as *const u8 as
                          *const libc::c_char,
                      Some(tcl_kadm5_randkey_principal as
                               unsafe extern "C" fn(_: ClientData,
                                                    _: *mut Tcl_Interp,
                                                    _: libc::c_int,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> libc::c_int), 0 as ClientData, None);
    Tcl_CreateCommand(interp,
                      b"kadm5_get_principal\x00" as *const u8 as
                          *const libc::c_char,
                      Some(tcl_kadm5_get_principal as
                               unsafe extern "C" fn(_: ClientData,
                                                    _: *mut Tcl_Interp,
                                                    _: libc::c_int,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> libc::c_int), 0 as ClientData, None);
    Tcl_CreateCommand(interp,
                      b"kadm5_create_policy\x00" as *const u8 as
                          *const libc::c_char,
                      Some(tcl_kadm5_create_policy as
                               unsafe extern "C" fn(_: ClientData,
                                                    _: *mut Tcl_Interp,
                                                    _: libc::c_int,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> libc::c_int), 0 as ClientData, None);
    Tcl_CreateCommand(interp,
                      b"kadm5_delete_policy\x00" as *const u8 as
                          *const libc::c_char,
                      Some(tcl_kadm5_delete_policy as
                               unsafe extern "C" fn(_: ClientData,
                                                    _: *mut Tcl_Interp,
                                                    _: libc::c_int,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> libc::c_int), 0 as ClientData, None);
    Tcl_CreateCommand(interp,
                      b"kadm5_modify_policy\x00" as *const u8 as
                          *const libc::c_char,
                      Some(tcl_kadm5_modify_policy as
                               unsafe extern "C" fn(_: ClientData,
                                                    _: *mut Tcl_Interp,
                                                    _: libc::c_int,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> libc::c_int), 0 as ClientData, None);
    Tcl_CreateCommand(interp,
                      b"kadm5_get_policy\x00" as *const u8 as
                          *const libc::c_char,
                      Some(tcl_kadm5_get_policy as
                               unsafe extern "C" fn(_: ClientData,
                                                    _: *mut Tcl_Interp,
                                                    _: libc::c_int,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> libc::c_int), 0 as ClientData, None);
    Tcl_CreateCommand(interp,
                      b"kadm5_free_principal_ent\x00" as *const u8 as
                          *const libc::c_char,
                      Some(tcl_kadm5_free_principal_ent as
                               unsafe extern "C" fn(_: ClientData,
                                                    _: *mut Tcl_Interp,
                                                    _: libc::c_int,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> libc::c_int), 0 as ClientData, None);
    Tcl_CreateCommand(interp,
                      b"kadm5_free_policy_ent\x00" as *const u8 as
                          *const libc::c_char,
                      Some(tcl_kadm5_free_policy_ent as
                               unsafe extern "C" fn(_: ClientData,
                                                    _: *mut Tcl_Interp,
                                                    _: libc::c_int,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> libc::c_int), 0 as ClientData, None);
    Tcl_CreateCommand(interp,
                      b"kadm5_get_privs\x00" as *const u8 as
                          *const libc::c_char,
                      Some(tcl_kadm5_get_privs as
                               unsafe extern "C" fn(_: ClientData,
                                                    _: *mut Tcl_Interp,
                                                    _: libc::c_int,
                                                    _:
                                                        *mut *const libc::c_char)
                                   -> libc::c_int), 0 as ClientData, None);
}
