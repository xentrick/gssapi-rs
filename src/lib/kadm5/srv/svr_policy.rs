use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:8"]
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
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:8"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:9"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:9"]
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
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
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
    #[c2rust::src_loc = "236:1"]
    pub type krb5_principal = *mut krb5_principal_data;
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    extern "C" {
        #[c2rust::src_loc = "350:8"]
        pub type _krb5_context;
        #[no_mangle]
        #[c2rust::src_loc = "8043:1"]
        pub fn krb5_clear_error_message(ctx: krb5_context);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:9"]
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
    #[c2rust::src_loc = "215:16"]
    pub struct _osa_policy_ent_t {
        pub version: libc::c_int,
        pub name: *mut libc::c_char,
        pub pw_min_life: krb5_ui_4,
        pub pw_max_life: krb5_ui_4,
        pub pw_min_length: krb5_ui_4,
        pub pw_min_classes: krb5_ui_4,
        pub pw_history_num: krb5_ui_4,
        pub policy_refcnt: krb5_ui_4,
        pub pw_max_fail: krb5_ui_4,
        pub pw_failcnt_interval: krb5_ui_4,
        pub pw_lockout_duration: krb5_ui_4,
        pub attributes: krb5_ui_4,
        pub max_life: krb5_ui_4,
        pub max_renewable_life: krb5_ui_4,
        pub allowed_keysalts: *mut libc::c_char,
        pub n_tl_data: krb5_int16,
        pub tl_data: *mut krb5_tl_data,
    }
    #[c2rust::src_loc = "215:1"]
    pub type osa_policy_ent_rec = _osa_policy_ent_t;
    #[c2rust::src_loc = "215:1"]
    pub type osa_policy_ent_t = *mut _osa_policy_ent_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_int16, krb5_ui_2, krb5_octet, krb5_ui_4,
                        krb5_enctype, krb5_int32, _krb5_context, krb5_context,
                        krb5_error_code};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "837:1"]
        pub fn krb5_db_free_policy(kcontext: krb5_context,
                                   policy: osa_policy_ent_t);
        #[no_mangle]
        #[c2rust::src_loc = "833:1"]
        pub fn krb5_db_delete_policy(kcontext: krb5_context,
                                     policy: *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "823:1"]
        pub fn krb5_db_put_policy(kcontext: krb5_context,
                                  policy: osa_policy_ent_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "818:1"]
        pub fn krb5_db_get_policy(kcontext: krb5_context,
                                  name: *mut libc::c_char,
                                  policy: *mut osa_policy_ent_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "814:1"]
        pub fn krb5_db_create_policy(kcontext: krb5_context,
                                     policy: osa_policy_ent_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "596:1"]
        pub fn krb5_db_update_tl_data(context: krb5_context,
                                      n_tl_datap: *mut krb5_int16,
                                      tl_datap: *mut *mut krb5_tl_data,
                                      new_tl_data: *mut krb5_tl_data)
         -> krb5_error_code;
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:9"]
pub mod admin_h {
    #[c2rust::src_loc = "70:1"]
    pub type kadm5_policy_t = *mut libc::c_char;
    #[c2rust::src_loc = "71:1"]
    pub type kadm5_ret_t = libc::c_long;
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
    use super::krb5_h::{krb5_kvno, krb5_deltat, krb5_flags, krb5_int16,
                        krb5_enctype, krb5_timestamp, krb5_int32};
    use super::kdb_h::{krb5_tl_data, krb5_key_salt_tuple};
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "447:1"]
        pub fn kadm5_free_policy_ent(server_handle: *mut libc::c_void,
                                     ent: kadm5_policy_ent_t) -> kadm5_ret_t;
    }
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/server_internal.h:10"]
pub mod server_internal_h {
    #[c2rust::src_loc = "42:1"]
    pub type kadm5_server_handle_t = *mut _kadm5_server_handle_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "42:16"]
    pub struct _kadm5_server_handle_t {
        pub magic_number: krb5_ui_4,
        pub struct_version: krb5_ui_4,
        pub api_version: krb5_ui_4,
        pub context: krb5_context,
        pub current_caller: krb5_principal,
        pub params: kadm5_config_params,
        pub lhandle: *mut _kadm5_server_handle_t,
        pub db_args: *mut *mut libc::c_char,
        pub qual_handles: *mut pwqual_handle,
        pub hook_handles: *mut kadm5_hook_handle,
    }
    #[c2rust::src_loc = "40:1"]
    pub type kadm5_hook_handle = *mut kadm5_hook_handle_st;
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 * $Header$
 */
    /*
 * This header file is used internally by the Admin API server
 * libraries and Admin server.  IF YOU THINK YOU NEED TO USE THIS FILE
 * FOR ANYTHING, YOU'RE ALMOST CERTAINLY WRONG.
 */
    /*
 * This is the history key version for a newly created DB.  We use this value
 * for principals which have no password history yet to avoid having to look up
 * the history key.  Values other than 2 will cause compatibility issues with
 * pre-1.8 libkadm5 code; the older code will reject key changes when it sees
 * an unexpected value of admin_history_kvno.
 */
    /* A pwqual_handle represents a password quality plugin module. */
    #[c2rust::src_loc = "38:1"]
    pub type pwqual_handle = *mut pwqual_handle_st;
    use super::krb5_h::{krb5_ui_4, krb5_context, krb5_principal};
    use super::admin_h::kadm5_config_params;
    extern "C" {
        #[c2rust::src_loc = "40:16"]
        pub type kadm5_hook_handle_st;
        #[c2rust::src_loc = "38:16"]
        pub type pwqual_handle_st;
    }
    /* __KADM5_SERVER_INTERNAL_H__ */
    /* * @}*/
}
#[c2rust::header_src = "/usr/include/stdlib.h:9"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/string.h:10"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin_internal.h:10"]
pub mod admin_internal_h {
    use super::krb5_h::{krb5_boolean, krb5_int32, krb5_error_code};
    use super::kdb_h::krb5_key_salt_tuple;
    extern "C" {
        /*
 * _KADM5_CHECK_HANDLE calls the function _kadm5_check_handle and
 * returns any non-zero error code that function returns.
 * _kadm5_check_handle, in client_handle.c and server_handle.c, exists
 * in both the server- and client- side libraries.  In each library,
 * it calls CHECK_HANDLE, which is defined by the appropriate
 * _internal.h header file to call GENERIC_CHECK_HANDLE as well as
 * CLIENT_CHECK_HANDLE and SERVER_CHECK_HANDLE.
 *
 * _KADM5_CHECK_HANDLE should be used by a function that needs to
 * check the handle but wants to be the same code in both the client
 * and server library; it makes a function call to the right handle
 * checker.  Code that only exists in one library can call the
 * CHECK_HANDLE macro, which inlines the test instead of making
 * another function call.
 *
 * Got that?
 */
        /* this is needed by the alt_prof code I stole.  The functions
   maybe shouldn't be named krb5_*, but they are. */
        #[no_mangle]
        #[c2rust::src_loc = "72:1"]
        pub fn krb5_string_to_keysalts(string: *const libc::c_char,
                                       tupleseps: *const libc::c_char,
                                       ksaltseps: *const libc::c_char,
                                       dups: krb5_boolean,
                                       ksaltp: *mut *mut krb5_key_salt_tuple,
                                       nksaltp: *mut krb5_int32)
         -> krb5_error_code;
    }
    /* __KADM5_ADMIN_INTERNAL_H__ */
}
pub use self::types_h::{__uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t};
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_kvno, krb5_enctype,
                       krb5_flags, krb5_timestamp, krb5_deltat,
                       krb5_error_code, krb5_magic, _krb5_data, krb5_data,
                       krb5_principal_data, krb5_principal, krb5_context,
                       _krb5_context, krb5_clear_error_message};
pub use self::kdb_h::{_krb5_tl_data, krb5_tl_data, _osa_policy_ent_t,
                      osa_policy_ent_rec, osa_policy_ent_t,
                      __krb5_key_salt_tuple, krb5_key_salt_tuple,
                      krb5_db_free_policy, krb5_db_delete_policy,
                      krb5_db_put_policy, krb5_db_get_policy,
                      krb5_db_create_policy, krb5_db_update_tl_data};
pub use self::admin_h::{kadm5_policy_t, kadm5_ret_t, _kadm5_policy_ent_t,
                        kadm5_policy_ent_t, _kadm5_config_params,
                        kadm5_config_params, kadm5_free_policy_ent};
pub use self::server_internal_h::{kadm5_server_handle_t,
                                  _kadm5_server_handle_t, kadm5_hook_handle,
                                  pwqual_handle, kadm5_hook_handle_st,
                                  pwqual_handle_st};
use self::stdlib_h::{malloc, calloc, free};
use self::string_h::{strlen, strchr, memset, memcpy, strdup};
use self::admin_internal_h::krb5_string_to_keysalts;
/* Validate allowed_keysalts. */
#[c2rust::src_loc = "21:1"]
unsafe extern "C" fn validate_allowed_keysalts(mut allowed_keysalts:
                                                   *const libc::c_char)
 -> kadm5_ret_t {
    let mut ret: kadm5_ret_t = 0;
    let mut ks_tuple: *mut krb5_key_salt_tuple =
        0 as *mut krb5_key_salt_tuple;
    let mut n_ks_tuple: krb5_int32 = 0 as libc::c_int;
    if !strchr(allowed_keysalts, '\t' as i32).is_null() {
        return 43787578 as libc::c_long
    }
    ret =
        krb5_string_to_keysalts(allowed_keysalts,
                                b",\x00" as *const u8 as *const libc::c_char,
                                0 as *const libc::c_char,
                                0 as libc::c_int as krb5_boolean,
                                &mut ks_tuple, &mut n_ks_tuple) as
            kadm5_ret_t;
    free(ks_tuple as *mut libc::c_void);
    if ret == 22 as libc::c_int as libc::c_long {
        return 43787578 as libc::c_long
    }
    return ret;
}
/*
 * Function: kadm5_create_policy
 *
 * Purpose: Create Policies in the policy DB.
 *
 * Arguments:
 *      entry   (input) The policy entry to be written out to the DB.
 *      mask    (input) Specifies which fields in entry are to ge written out
 *                      and which get default values.
 *      <return value> 0 if successful otherwise an error code is returned.
 *
 * Requires:
 *      Entry must be a valid principal entry, and mask have a valid value.
 *
 * Effects:
 *      Writes the data to the database, and does a database sync if
 *      successful.
 *
 */
#[no_mangle]
#[c2rust::src_loc = "58:1"]
pub unsafe extern "C" fn kadm5_create_policy(mut server_handle:
                                                 *mut libc::c_void,
                                             mut entry: kadm5_policy_ent_t,
                                             mut mask: libc::c_long)
 -> kadm5_ret_t {
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut pent: osa_policy_ent_rec =
        osa_policy_ent_rec{version: 0,
                           name: 0 as *mut libc::c_char,
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
    let mut ret: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    krb5_clear_error_message((*handle).context);
    if entry.is_null() || (*entry).policy.is_null() {
        return 22 as libc::c_int as kadm5_ret_t
    }
    if strlen((*entry).policy) == 0 as libc::c_int as libc::c_ulong {
        return 43787537 as libc::c_long
    }
    if mask & 0x800 as libc::c_int as libc::c_long == 0 {
        return 43787534 as libc::c_long
    }
    if mask & 0x4000000 as libc::c_int as libc::c_long != 0 &&
           !(*entry).allowed_keysalts.is_null() {
        ret =
            validate_allowed_keysalts((*entry).allowed_keysalts) as
                libc::c_int;
        if ret != 0 { return ret as kadm5_ret_t }
    }
    memset(&mut pent as *mut osa_policy_ent_rec as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<osa_policy_ent_rec>() as libc::c_ulong);
    pent.name = (*entry).policy;
    p = (*entry).policy;
    while *p as libc::c_int != '\u{0}' as i32 {
        if (*p as libc::c_int) < ' ' as i32 || *p as libc::c_int > '~' as i32
           {
            return 43787537 as libc::c_long
        } else { p = p.offset(1) }
    }
    if mask & 0x4000 as libc::c_int as libc::c_long == 0 {
        pent.pw_max_life = 0 as libc::c_int as krb5_ui_4
    } else { pent.pw_max_life = (*entry).pw_max_life as krb5_ui_4 }
    if mask & 0x8000 as libc::c_int as libc::c_long == 0 {
        pent.pw_min_life = 0 as libc::c_int as krb5_ui_4
    } else {
        if mask & 0x4000 as libc::c_int as libc::c_long != 0 {
            if (*entry).pw_min_life > (*entry).pw_max_life &&
                   (*entry).pw_max_life != 0 as libc::c_int as libc::c_long {
                return 43787541 as libc::c_long
            }
        }
        pent.pw_min_life = (*entry).pw_min_life as krb5_ui_4
    }
    if mask & 0x10000 as libc::c_int as libc::c_long == 0 {
        pent.pw_min_length = 1 as libc::c_int as krb5_ui_4
    } else {
        if (*entry).pw_min_length < 1 as libc::c_int as libc::c_long {
            return 43787536 as libc::c_long
        }
        pent.pw_min_length = (*entry).pw_min_length as krb5_ui_4
    }
    if mask & 0x20000 as libc::c_int as libc::c_long == 0 {
        pent.pw_min_classes = 1 as libc::c_int as krb5_ui_4
    } else {
        if (*entry).pw_min_classes > 5 as libc::c_int as libc::c_long ||
               (*entry).pw_min_classes < 1 as libc::c_int as libc::c_long {
            return 43787535 as libc::c_long
        }
        pent.pw_min_classes = (*entry).pw_min_classes as krb5_ui_4
    }
    if mask & 0x40000 as libc::c_int as libc::c_long == 0 {
        pent.pw_history_num = 1 as libc::c_int as krb5_ui_4
    } else if (*entry).pw_history_num < 1 as libc::c_int as libc::c_long {
        return 43787540 as libc::c_long
    } else { pent.pw_history_num = (*entry).pw_history_num as krb5_ui_4 }
    if (*handle).api_version >=
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        if mask & 0x800000 as libc::c_int as libc::c_long == 0 {
            pent.attributes = 0 as libc::c_int as krb5_ui_4
        } else { pent.attributes = (*entry).attributes as krb5_ui_4 }
        if mask & 0x1000000 as libc::c_int as libc::c_long == 0 {
            pent.max_life = 0 as libc::c_int as krb5_ui_4
        } else { pent.max_life = (*entry).max_life as krb5_ui_4 }
        if mask & 0x2000000 as libc::c_int as libc::c_long == 0 {
            pent.max_renewable_life = 0 as libc::c_int as krb5_ui_4
        } else {
            pent.max_renewable_life = (*entry).max_renewable_life as krb5_ui_4
        }
        if mask & 0x4000000 as libc::c_int as libc::c_long == 0 {
            pent.allowed_keysalts = 0 as *mut libc::c_char
        } else { pent.allowed_keysalts = (*entry).allowed_keysalts }
        if mask & 0x8000000 as libc::c_int as libc::c_long == 0 {
            pent.n_tl_data = 0 as libc::c_int as krb5_int16;
            pent.tl_data = 0 as *mut krb5_tl_data
        } else {
            pent.n_tl_data = (*entry).n_tl_data;
            pent.tl_data = (*entry).tl_data
        }
    }
    if (*handle).api_version >=
           (0x12345700 as libc::c_int | 0x3 as libc::c_int) as libc::c_uint {
        if mask & 0x100000 as libc::c_int as libc::c_long == 0 {
            pent.pw_max_fail = 0 as libc::c_int as krb5_ui_4
        } else { pent.pw_max_fail = (*entry).pw_max_fail }
        if mask & 0x200000 as libc::c_int as libc::c_long == 0 {
            pent.pw_failcnt_interval = 0 as libc::c_int as krb5_ui_4
        } else {
            pent.pw_failcnt_interval =
                (*entry).pw_failcnt_interval as krb5_ui_4
        }
        if mask & 0x400000 as libc::c_int as libc::c_long == 0 {
            pent.pw_lockout_duration = 0 as libc::c_int as krb5_ui_4
        } else {
            pent.pw_lockout_duration =
                (*entry).pw_lockout_duration as krb5_ui_4
        }
    }
    ret = krb5_db_create_policy((*handle).context, &mut pent);
    if ret != 0 {
        return ret as kadm5_ret_t
    } else { return 0 as libc::c_int as kadm5_ret_t };
}
#[no_mangle]
#[c2rust::src_loc = "174:1"]
pub unsafe extern "C" fn kadm5_delete_policy(mut server_handle:
                                                 *mut libc::c_void,
                                             mut name: kadm5_policy_t)
 -> kadm5_ret_t {
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut entry: osa_policy_ent_t = 0 as *mut _osa_policy_ent_t;
    let mut ret: libc::c_int = 0;
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    krb5_clear_error_message((*handle).context);
    if name.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    if strlen(name as *const libc::c_char) ==
           0 as libc::c_int as libc::c_ulong {
        return 43787537 as libc::c_long
    }
    ret = krb5_db_get_policy((*handle).context, name, &mut entry);
    if ret as libc::c_long == -(1780008443 as libc::c_long) {
        return 43787533 as libc::c_long
    } else { if ret != 0 { return ret as kadm5_ret_t } }
    krb5_db_free_policy((*handle).context, entry);
    ret = krb5_db_delete_policy((*handle).context, name);
    if ret as libc::c_long == -(1780008404 as libc::c_long) {
        ret = 43787547 as libc::c_long as libc::c_int
    }
    return if ret == 0 as libc::c_int { 0 as libc::c_int } else { ret } as
               kadm5_ret_t;
}
/* Allocate and form a TL data list of a desired size. */
#[c2rust::src_loc = "203:1"]
unsafe extern "C" fn alloc_tl_data(mut n_tl_data: krb5_int16,
                                   mut tldp: *mut *mut krb5_tl_data)
 -> libc::c_int {
    let mut tlp: *mut *mut krb5_tl_data = tldp; /* caller cleans up */
    let mut i: libc::c_int = 0; /* caller cleans up */
    i = 0 as libc::c_int;
    while i < n_tl_data as libc::c_int {
        *tlp =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<krb5_tl_data>() as libc::c_ulong) as
                *mut krb5_tl_data;
        if (*tlp).is_null() { return 12 as libc::c_int }
        memset(*tlp as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<krb5_tl_data>() as libc::c_ulong);
        tlp = &mut (**tlp).tl_data_next;
        i += 1
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "220:1"]
unsafe extern "C" fn copy_tl_data(mut n_tl_data: krb5_int16,
                                  mut tl_data: *mut krb5_tl_data,
                                  mut out: *mut *mut krb5_tl_data)
 -> kadm5_ret_t {
    let mut ret: kadm5_ret_t = 0;
    let mut tl: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut tl_new: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    ret = alloc_tl_data(n_tl_data, out) as kadm5_ret_t;
    if ret != 0 { return ret }
    tl = tl_data;
    tl_new = *out;
    while !tl.is_null() {
        (*tl_new).tl_data_contents =
            malloc((*tl).tl_data_length as libc::c_ulong) as *mut krb5_octet;
        if (*tl_new).tl_data_contents.is_null() {
            return 12 as libc::c_int as kadm5_ret_t
        }
        memcpy((*tl_new).tl_data_contents as *mut libc::c_void,
               (*tl).tl_data_contents as *const libc::c_void,
               (*tl).tl_data_length as libc::c_ulong);
        (*tl_new).tl_data_type = (*tl).tl_data_type;
        (*tl_new).tl_data_length = (*tl).tl_data_length;
        tl = (*tl).tl_data_next;
        tl_new = (*tl_new).tl_data_next
    }
    return 0 as libc::c_int as kadm5_ret_t;
}
#[no_mangle]
#[c2rust::src_loc = "245:1"]
pub unsafe extern "C" fn kadm5_modify_policy(mut server_handle:
                                                 *mut libc::c_void,
                                             mut entry: kadm5_policy_ent_t,
                                             mut mask: libc::c_long)
 -> kadm5_ret_t {
    let mut current_block: u64;
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    let mut tl: *mut krb5_tl_data = 0 as *mut krb5_tl_data;
    let mut p: osa_policy_ent_t = 0 as *mut _osa_policy_ent_t;
    let mut ret: libc::c_int = 0;
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    krb5_clear_error_message((*handle).context);
    if entry.is_null() || (*entry).policy.is_null() {
        return 22 as libc::c_int as kadm5_ret_t
    }
    if strlen((*entry).policy) == 0 as libc::c_int as libc::c_ulong {
        return 43787537 as libc::c_long
    }
    if mask & 0x800 as libc::c_int as libc::c_long != 0 {
        return 43787534 as libc::c_long
    }
    if mask & 0x4000000 as libc::c_int as libc::c_long != 0 &&
           !(*entry).allowed_keysalts.is_null() {
        ret =
            validate_allowed_keysalts((*entry).allowed_keysalts) as
                libc::c_int;
        if ret != 0 { return ret as kadm5_ret_t }
    }
    if mask & 0x8000000 as libc::c_int as libc::c_long != 0 {
        tl = (*entry).tl_data;
        while !tl.is_null() {
            if ((*tl).tl_data_type as libc::c_int) < 256 as libc::c_int {
                return 43787567 as libc::c_long
            }
            tl = (*tl).tl_data_next
        }
    }
    ret = krb5_db_get_policy((*handle).context, (*entry).policy, &mut p);
    if ret as libc::c_long == -(1780008443 as libc::c_long) {
        return 43787533 as libc::c_long
    } else { if ret != 0 { return ret as kadm5_ret_t } }
    if mask & 0x4000 as libc::c_int as libc::c_long != 0 {
        (*p).pw_max_life = (*entry).pw_max_life as krb5_ui_4
    }
    if mask & 0x8000 as libc::c_int as libc::c_long != 0 {
        if (*entry).pw_min_life > (*p).pw_max_life as libc::c_long &&
               (*p).pw_max_life != 0 as libc::c_int as libc::c_uint {
            krb5_db_free_policy((*handle).context, p);
            return 43787541 as libc::c_long
        }
        (*p).pw_min_life = (*entry).pw_min_life as krb5_ui_4
    }
    if mask & 0x10000 as libc::c_int as libc::c_long != 0 {
        if (*entry).pw_min_length < 1 as libc::c_int as libc::c_long {
            krb5_db_free_policy((*handle).context, p);
            return 43787536 as libc::c_long
        }
        (*p).pw_min_length = (*entry).pw_min_length as krb5_ui_4
    }
    if mask & 0x20000 as libc::c_int as libc::c_long != 0 {
        if (*entry).pw_min_classes > 5 as libc::c_int as libc::c_long ||
               (*entry).pw_min_classes < 1 as libc::c_int as libc::c_long {
            krb5_db_free_policy((*handle).context, p);
            return 43787535 as libc::c_long
        }
        (*p).pw_min_classes = (*entry).pw_min_classes as krb5_ui_4
    }
    if mask & 0x40000 as libc::c_int as libc::c_long != 0 {
        if (*entry).pw_history_num < 1 as libc::c_int as libc::c_long {
            krb5_db_free_policy((*handle).context, p);
            return 43787540 as libc::c_long
        }
        (*p).pw_history_num = (*entry).pw_history_num as krb5_ui_4
    }
    if (*handle).api_version >=
           (0x12345700 as libc::c_int | 0x3 as libc::c_int) as libc::c_uint {
        if mask & 0x100000 as libc::c_int as libc::c_long != 0 {
            (*p).pw_max_fail = (*entry).pw_max_fail
        }
        if mask & 0x200000 as libc::c_int as libc::c_long != 0 {
            (*p).pw_failcnt_interval =
                (*entry).pw_failcnt_interval as krb5_ui_4
        }
        if mask & 0x400000 as libc::c_int as libc::c_long != 0 {
            (*p).pw_lockout_duration =
                (*entry).pw_lockout_duration as krb5_ui_4
        }
    }
    if (*handle).api_version >=
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        if mask & 0x800000 as libc::c_int as libc::c_long != 0 {
            (*p).attributes = (*entry).attributes as krb5_ui_4
        }
        if mask & 0x1000000 as libc::c_int as libc::c_long != 0 {
            (*p).max_life = (*entry).max_life as krb5_ui_4
        }
        if mask & 0x2000000 as libc::c_int as libc::c_long != 0 {
            (*p).max_renewable_life = (*entry).max_renewable_life as krb5_ui_4
        }
        if mask & 0x4000000 as libc::c_int as libc::c_long != 0 {
            free((*p).allowed_keysalts as *mut libc::c_void);
            (*p).allowed_keysalts = 0 as *mut libc::c_char;
            if !(*entry).allowed_keysalts.is_null() {
                (*p).allowed_keysalts = strdup((*entry).allowed_keysalts);
                if (*p).allowed_keysalts.is_null() {
                    ret = 12 as libc::c_int;
                    current_block = 8048257192757430128;
                } else { current_block = 6471821049853688503; }
            } else { current_block = 6471821049853688503; }
        } else { current_block = 6471821049853688503; }
        match current_block {
            8048257192757430128 => { }
            _ => {
                if mask & 0x8000000 as libc::c_int as libc::c_long != 0 {
                    tl = (*entry).tl_data;
                    loop  {
                        if tl.is_null() {
                            current_block = 13349765058737954042;
                            break ;
                        }
                        ret =
                            krb5_db_update_tl_data((*handle).context,
                                                   &mut (*p).n_tl_data,
                                                   &mut (*p).tl_data, tl);
                        if ret != 0 {
                            current_block = 8048257192757430128;
                            break ;
                        }
                        tl = (*tl).tl_data_next
                    }
                } else { current_block = 13349765058737954042; }
            }
        }
    } else { current_block = 13349765058737954042; }
    match current_block {
        13349765058737954042 => {
            ret = krb5_db_put_policy((*handle).context, p)
        }
        _ => { }
    }
    krb5_db_free_policy((*handle).context, p);
    return ret as kadm5_ret_t;
}
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/kadm5/admin.h */
/*
 * Copyright 2001, 2008 by the Massachusetts Institute of Technology.
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
 * Copyright 1993 OpenVision Technologies, Inc., All Rights Reserved
 *
 * $Header$
 */
/*
 * This API is not considered as stable as the main krb5 API.
 *
 * - We may make arbitrary incompatible changes between feature
 *   releases (e.g. from 1.7 to 1.8).
 * - We will make some effort to avoid making incompatible changes for
 *   bugfix releases, but will make them if necessary.
 */
/*
 * Successful return code
 */
/*
 * Field masks
 */
/* kadm5_principal_ent_t */
/* version 2 masks */
/* Novell */
/* all but KEY_DATA, TL_DATA, LOAD */
/* kadm5_policy_ent_t */
/* kadm5_config_params */
/*#define KADM5_CONFIG_ADMIN_KEYTAB       0x00000080*/
/*
 * permission bits
 */
/*
 * API versioning constants
 */
/* version 2 fields */
/* no longer used */
/* version 3 fields */
/* version 4 fields */
/*
 * Data structure returned by kadm5_get_config_params()
 */
/* Novell */ /* ABI change? */
/* Deprecated except for db2 backwards compatibility.  Don't add
       new uses except as fallbacks for parameters that should be
       specified in the database module section of the config
       file.  */
/*    char *            iprop_server;*/
/*
 * functions
 */
/*
 * For all initialization functions, the caller must first initialize
 * a context with kadm5_init_krb5_context which will survive as long
 * as the resulting handle.  The caller should free the context with
 * krb5_free_context.
 */
#[no_mangle]
#[c2rust::src_loc = "357:1"]
pub unsafe extern "C" fn kadm5_get_policy(mut server_handle:
                                              *mut libc::c_void,
                                          mut name: kadm5_policy_t,
                                          mut entry: kadm5_policy_ent_t)
 -> kadm5_ret_t {
    let mut current_block: u64;
    let mut t: osa_policy_ent_t = 0 as *mut _osa_policy_ent_t;
    let mut ret: kadm5_ret_t = 0;
    let mut handle: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    memset(entry as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<_kadm5_policy_ent_t>() as libc::c_ulong);
    let mut srvr: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if srvr.is_null() { return 43787551 as libc::c_long }
    if (*srvr).magic_number != 0x12345800 as libc::c_int as libc::c_uint {
        return 43787551 as libc::c_long
    }
    if (*srvr).struct_version & 0xffffff00 as libc::c_uint !=
           0x12345600 as libc::c_int as libc::c_uint {
        return 43787552 as libc::c_long
    }
    if (*srvr).struct_version <
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787553 as libc::c_long
    }
    if (*srvr).struct_version >
           (0x12345600 as libc::c_int | 0x1 as libc::c_int) as libc::c_uint {
        return 43787554 as libc::c_long
    }
    if (*srvr).api_version & 0xffffff00 as libc::c_uint !=
           0x12345700 as libc::c_int as libc::c_uint {
        return 43787555 as libc::c_long
    }
    if (*srvr).api_version <
           (0x12345700 as libc::c_int | 0x2 as libc::c_int) as libc::c_uint {
        return 43787557 as libc::c_long
    }
    if (*srvr).api_version >
           (0x12345700 as libc::c_int | 0x4 as libc::c_int) as libc::c_uint {
        return 43787559 as libc::c_long
    }
    let mut srvr_0: kadm5_server_handle_t =
        server_handle as kadm5_server_handle_t;
    if (*srvr_0).current_caller.is_null() { return 43787551 as libc::c_long }
    if (*srvr_0).lhandle.is_null() { return 43787551 as libc::c_long }
    krb5_clear_error_message((*handle).context);
    if name.is_null() { return 22 as libc::c_int as kadm5_ret_t }
    if strlen(name as *const libc::c_char) ==
           0 as libc::c_int as libc::c_ulong {
        return 43787537 as libc::c_long
    }
    ret = krb5_db_get_policy((*handle).context, name, &mut t) as kadm5_ret_t;
    if ret == -(1780008443 as libc::c_long) {
        return 43787533 as libc::c_long
    } else { if ret != 0 { return ret } }
    (*entry).policy = strdup((*t).name);
    if (*entry).policy.is_null() {
        ret = 12 as libc::c_int as kadm5_ret_t
    } else {
        (*entry).pw_min_life = (*t).pw_min_life as libc::c_long;
        (*entry).pw_max_life = (*t).pw_max_life as libc::c_long;
        (*entry).pw_min_length = (*t).pw_min_length as libc::c_long;
        (*entry).pw_min_classes = (*t).pw_min_classes as libc::c_long;
        (*entry).pw_history_num = (*t).pw_history_num as libc::c_long;
        if (*handle).api_version >=
               (0x12345700 as libc::c_int | 0x3 as libc::c_int) as
                   libc::c_uint {
            (*entry).pw_max_fail = (*t).pw_max_fail;
            (*entry).pw_failcnt_interval =
                (*t).pw_failcnt_interval as krb5_deltat;
            (*entry).pw_lockout_duration =
                (*t).pw_lockout_duration as krb5_deltat
        }
        if (*handle).api_version >=
               (0x12345700 as libc::c_int | 0x4 as libc::c_int) as
                   libc::c_uint {
            (*entry).attributes = (*t).attributes as krb5_flags;
            (*entry).max_life = (*t).max_life as krb5_deltat;
            (*entry).max_renewable_life =
                (*t).max_renewable_life as krb5_deltat;
            if !(*t).allowed_keysalts.is_null() {
                (*entry).allowed_keysalts = strdup((*t).allowed_keysalts);
                if (*entry).allowed_keysalts.is_null() {
                    ret = 12 as libc::c_int as kadm5_ret_t;
                    current_block = 11002422739775295273;
                } else { current_block = 13619784596304402172; }
            } else { current_block = 13619784596304402172; }
            match current_block {
                11002422739775295273 => { }
                _ => {
                    ret =
                        copy_tl_data((*t).n_tl_data, (*t).tl_data,
                                     &mut (*entry).tl_data);
                    if ret != 0 {
                        current_block = 11002422739775295273;
                    } else {
                        (*entry).n_tl_data = (*t).n_tl_data;
                        current_block = 17747245473264231573;
                    }
                }
            }
        } else { current_block = 17747245473264231573; }
        match current_block {
            11002422739775295273 => { }
            _ => { ret = 0 as libc::c_int as kadm5_ret_t }
        }
    }
    if ret != 0 { kadm5_free_policy_ent(handle as *mut libc::c_void, entry); }
    krb5_db_free_policy((*handle).context, t);
    return ret;
}
