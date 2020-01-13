use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:7"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:7"]
pub mod types_h {
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:7"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::__int32_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:7"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint32_t, __uint64_t};
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:7"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:7"]
pub mod thread_shared_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:16"]
    pub struct __pthread_internal_list {
        pub __prev: *mut __pthread_internal_list,
        pub __next: *mut __pthread_internal_list,
    }
    #[c2rust::src_loc = "82:1"]
    pub type __pthread_list_t = __pthread_internal_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:8"]
    pub struct __pthread_mutex_s {
        pub __lock: libc::c_int,
        pub __count: libc::c_uint,
        pub __owner: libc::c_int,
        pub __nusers: libc::c_uint,
        pub __kind: libc::c_int,
        pub __spins: libc::c_short,
        pub __elision: libc::c_short,
        pub __list: __pthread_list_t,
    }
}
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:7"]
pub mod pthreadtypes_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:9"]
    pub union pthread_mutex_t {
        pub __data: __pthread_mutex_s,
        pub __size: [libc::c_char; 40],
        pub __align: libc::c_long,
    }
    use super::thread_shared_types_h::__pthread_mutex_s;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:7"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:7"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-thread.h:7"]
pub mod k5_thread_h {
    #[c2rust::src_loc = "283:1"]
    pub type k5_os_mutex = pthread_mutex_t;
    /* is pthreads always available? */
    #[c2rust::src_loc = "354:1"]
    pub type k5_mutex_t = k5_os_mutex;
    #[inline]
    #[c2rust::src_loc = "356:1"]
    pub unsafe extern "C" fn k5_mutex_init(mut m: *mut k5_mutex_t)
     -> libc::c_int {
        return k5_os_mutex_init(m);
    }
    #[inline]
    #[c2rust::src_loc = "367:1"]
    pub unsafe extern "C" fn k5_mutex_lock(mut m: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_lock(m);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_lock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          376 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 33],
                                                    &[libc::c_char; 33]>(b"void k5_mutex_lock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    #[inline]
    #[c2rust::src_loc = "379:1"]
    pub unsafe extern "C" fn k5_mutex_unlock(mut m: *mut k5_mutex_t) {
        let mut r: libc::c_int = k5_os_mutex_unlock(m);
        if r != 0 as libc::c_int {
            fprintf(stderr,
                    b"k5_mutex_unlock: Received error %d (%s)\n\x00" as
                        *const u8 as *const libc::c_char, r, strerror(r));
        }
        if r == 0 as libc::c_int {
        } else {
            __assert_fail(b"r == 0\x00" as *const u8 as *const libc::c_char,
                          b"../../include/k5-thread.h\x00" as *const u8 as
                              *const libc::c_char,
                          388 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 35],
                                                    &[libc::c_char; 35]>(b"void k5_mutex_unlock(k5_mutex_t *)\x00")).as_ptr());
        };
    }
    use super::pthreadtypes_h::pthread_mutex_t;
    use super::stdio_h::{fprintf, stderr};
    use super::string_h::strerror;
    use super::assert_h::__assert_fail;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "290:1"]
        pub fn k5_os_mutex_init(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "291:1"]
        pub fn k5_os_mutex_destroy(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "292:1"]
        pub fn k5_os_mutex_lock(m: *mut k5_os_mutex) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "293:1"]
        pub fn k5_os_mutex_unlock(m: *mut k5_os_mutex) -> libc::c_int;
    }
    /* multiple inclusion? */
    /* In time, many of the definitions above should move into the support
   library, and this file should be greatly simplified.  For type
   definitions, that'll take some work, since other data structures
   incorporate mutexes directly, and our mutex type is dependent on
   configuration options and system attributes.  For most functions,
   though, it should be relatively easy.

   For now, plugins should use the exported functions, and not the
   above macros, and use krb5int_mutex_alloc for allocations.  */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-platform.h:7"]
pub mod k5_platform_h {
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "574:5"]
    pub struct C2RustUnnamed {
        pub i: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C, packed)]
    #[c2rust::src_loc = "620:12"]
    pub struct C2RustUnnamed_0 {
        pub i: uint32_t,
    }
    #[inline]
    #[c2rust::src_loc = "567:1"]
    pub unsafe extern "C" fn store_32_be(mut val: libc::c_uint,
                                         mut vp: *mut libc::c_void) {
        let mut p: *mut libc::c_uchar = vp as *mut libc::c_uchar;
        (*(p as *mut C2RustUnnamed)).i = __bswap_32(val);
    }
    #[inline]
    #[c2rust::src_loc = "613:1"]
    pub unsafe extern "C" fn load_32_be(mut cvp: *const libc::c_void)
     -> libc::c_uint {
        let mut p: *const libc::c_uchar = cvp as *const libc::c_uchar;
        return __bswap_32((*(p as *const C2RustUnnamed_0)).i);
    }
    use super::stdint_uintn_h::uint32_t;
    use super::byteswap_h::__bswap_32;
    extern "C" {
        /*
 * Compose two path components, inserting the platform-appropriate path
 * separator if needed.  If path2 is an absolute path, path1 will be discarded
 * and path_out will be a copy of path2.  Returns 0 on success or ENOMEM on
 * allocation failure.
 */
        #[no_mangle]
        #[c2rust::src_loc = "1074:1"]
        pub fn k5_path_join(path1: *const libc::c_char,
                            path2: *const libc::c_char,
                            path_out: *mut *mut libc::c_char) -> libc::c_long;
    }
    /* K5_PLATFORM_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:7"]
pub mod k5_err_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* include/k5-err.h */
/*
 * Copyright 2006, 2007 Massachusetts Institute of Technology.
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
 *
 * Error-message handling
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:8"]
    pub struct errinfo {
        pub code: libc::c_long,
        pub msg: *mut libc::c_char,
    }
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "66:1"]
        pub fn k5_clear_error(ep: *mut errinfo);
    }
    /* K5_ERR_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:7"]
pub mod k5_plugin_h {
    use super::k5_err_h::errinfo;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2006 Massachusetts Institute of Technology.
 * All Rights Reserved.
 *
 * This software is being provided to you, the LICENSEE, by the
 * Massachusetts Institute of Technology (M.I.T.) under the following
 * license.  By obtaining, using and/or copying this software, you agree
 * that you have read, understood, and will comply with these terms and
 * conditions:
 *
 * Export of this software from the United States of America may
 * require a specific license from the United States Government.
 * It is the responsibility of any person or organization contemplating
 * export to obtain such a license before exporting.
 *
 * WITHIN THAT CONSTRAINT, permission to use, copy, modify and distribute
 * this software and its documentation for any purpose and without fee or
 * royalty is hereby granted, provided that you agree to comply with the
 * following copyright notice and statements, including the disclaimer, and
 * that the same appear on ALL copies of the software and documentation,
 * including modifications that you make for internal use or for
 * distribution:
 *
 * THIS SOFTWARE IS PROVIDED "AS IS", AND M.I.T. MAKES NO REPRESENTATIONS
 * OR WARRANTIES, EXPRESS OR IMPLIED.  By way of example, but not
 * limitation, M.I.T. MAKES NO REPRESENTATIONS OR WARRANTIES OF
 * MERCHANTABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF
 * THE LICENSED SOFTWARE OR DOCUMENTATION WILL NOT INFRINGE ANY THIRD PARTY
 * PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.
 *
 * The name of the Massachusetts Institute of Technology or M.I.T. may NOT
 * be used in advertising or publicity pertaining to distribution of the
 * software.  Title to copyright in this software and any associated
 * documentation shall at all times remain with M.I.T., and USER agrees to
 * preserve same.
 *
 * Furthermore if you modify this software you must label
 * your software as modified software and not distribute it in such a
 * fashion that it might be confused with the original M.I.T. software.
 */
        /* Just those definitions which are needed by util/support/plugins.c,
   which gets compiled before util/et is built, which happens before
   we can construct krb5.h, which is included by k5-int.h.

   So, no krb5 types.  */
        /*
 * Plugins normally export fixed symbol names, but when statically
 * linking plugins, we need a different symbol name for each plugin.
 * The first argument to PLUGIN_SYMBOL_NAME acts as the
 * differentiator, and is only used for static plugin linking.
 *
 * Although this macro (and thus this header file) are used in plugins
 * whose code lies inside the krb5 tree, plugins maintained separately
 * from the krb5 tree do not need it; they can just use the fixed
 * symbol name unconditionally.
 */
        #[c2rust::src_loc = "80:8"]
        pub type plugin_file_handle;
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn krb5int_open_plugin(_: *const libc::c_char,
                                   _: *mut *mut plugin_file_handle,
                                   _: *mut errinfo) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "91:1"]
        pub fn krb5int_close_plugin(_: *mut plugin_file_handle);
        #[no_mangle]
        #[c2rust::src_loc = "98:1"]
        pub fn krb5int_get_plugin_func(_: *mut plugin_file_handle,
                                       _: *const libc::c_char,
                                       _:
                                           *mut Option<unsafe extern "C" fn()
                                                           -> ()>,
                                       _: *mut errinfo) -> libc::c_long;
    }
    /* K5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:7"]
pub mod com_err_h {
    /*
 * Copyright 1988, Student Information Processing Board of the
 * Massachusetts Institute of Technology.
 *
 * Copyright 1995 by Cygnus Support.
 *
 * For copyright and distribution info, see the documentation supplied
 * with this package.
 */
    /* Header file for common error description library. */
    #[c2rust::src_loc = "26:1"]
    pub type errcode_t = libc::c_long;
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/profile/prof_int.h:7"]
pub mod prof_int_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "93:8"]
    pub struct _profile_t {
        pub magic: prf_magic_t,
        pub first_file: prf_file_t,
        pub vt: *mut profile_vtable,
        pub cbdata: *mut libc::c_void,
        pub lib_handle: prf_lib_handle_t,
    }
    #[c2rust::src_loc = "86:1"]
    pub type prf_lib_handle_t = *mut _prf_lib_handle_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "80:8"]
    pub struct _prf_lib_handle_t {
        pub lock: k5_mutex_t,
        pub refcount: libc::c_int,
        pub plugin_handle: *mut plugin_file_handle,
    }
    #[c2rust::src_loc = "71:1"]
    pub type prf_file_t = *mut _prf_file_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "65:8"]
    pub struct _prf_file_t {
        pub magic: prf_magic_t,
        pub data: *mut _prf_data_t,
        pub next: *mut _prf_file_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:8"]
    pub struct _prf_data_t {
        pub magic: prf_magic_t,
        pub lock: k5_mutex_t,
        pub root: *mut profile_node,
        pub last_stat: time_t,
        pub timestamp: time_t,
        pub frac_ts: libc::c_ulong,
        pub flags: libc::c_int,
        pub upd_serial: libc::c_int,
        pub fslen: size_t,
        pub pad: C2RustUnnamed_1,
        pub refcount: libc::c_int,
        pub next: *mut _prf_data_t,
        pub filespec: [libc::c_char; 15],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:2"]
    pub union C2RustUnnamed_1 {
        pub d: libc::c_double,
        pub p: *mut libc::c_void,
        pub ll: uint64_t,
        pub m: k5_mutex_t,
    }
    #[c2rust::src_loc = "19:1"]
    pub type prf_magic_t = libc::c_long;
    #[c2rust::src_loc = "62:1"]
    pub type prf_data_t = *mut _prf_data_t;
    use super::profile_h::{profile_vtable, const_profile_filespec_t};
    use super::k5_thread_h::k5_mutex_t;
    use super::k5_plugin_h::plugin_file_handle;
    use super::time_t_h::time_t;
    use super::stddef_h::size_t;
    use super::stdint_uintn_h::uint64_t;
    use super::com_err_h::errcode_t;
    extern "C" {
        #[c2rust::src_loc = "33:9"]
        pub type profile_node;
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn profile_open_file(file: const_profile_filespec_t,
                                 ret_prof: *mut prf_file_t,
                                 ret_modspec: *mut *mut libc::c_char)
         -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "225:1"]
        pub fn profile_flush_file_data(data: prf_data_t) -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "229:1"]
        pub fn profile_flush_file_data_to_file(data: prf_data_t,
                                               outfile: *const libc::c_char)
         -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "232:1"]
        pub fn profile_flush_file_data_to_buffer(data: prf_data_t,
                                                 bufp: *mut *mut libc::c_char)
         -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "235:1"]
        pub fn profile_free_file(profile: prf_file_t);
        #[no_mangle]
        #[c2rust::src_loc = "238:1"]
        pub fn profile_close_file(profile: prf_file_t) -> errcode_t;
        #[no_mangle]
        #[c2rust::src_loc = "241:1"]
        pub fn profile_file_is_writable(profile: prf_file_t) -> libc::c_int;
    }
    /* prof_set.c -- included from profile.h */
    /* Others included from profile.h */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/profile/profile.h:7"]
pub mod profile_h {
    /*
 * profile.h
 */
    /* Used by profile_init_flags(). */
    /* Allow module declaration */
    /*
 * Used by the profile iterator in prof_get.c
 */
    /* __cplusplus */
    /* path as C string */
    /* list of : separated paths, C string */
    /* path as C string */
    /* list of : separated paths, C string */
    /*
 * profile_init_vtable allows a caller to create a profile-compatible object
 * with a different back end.
 */
    /*
 * Mandatory: Look up all of the relations for names, placing the resulting
 * values in *ret_values.  If no relations exist, return PROF_NO_RELATION, or
 * PROF_NO_SECTION to indicate that one of the intermediate names does not
 * exist as a section.  The list will be freed with free_values.
 */
    /* Mandatory: Free a list of strings returned by get_values. */
    /* Optional: Release any data associated with the profile. */
    /*
 * Optional (mandatory if cleanup is defined): Generate a new cbdata pointer
 * for a copy of the profile.  If not implemented, the new profile will receive
 * the same cbdata pointer as the old one.
 */
    /*
 * Optional: Create an iterator handle.
 *
 * If flags contains PROFILE_ITER_LIST_SECTION, iterate over all of the
 * relations and sections within names.  Otherwise, iterate over the relation
 * values for names, or produce a single section result if names is a section.
 *
 * If flags contains PROFILE_ITER_SECTIONS_ONLY, produce only sections.
 *
 * If flags contains PROFILE_ITER_RELATIONS_ONLY, produce only relations.
 */
    /*
 * Optional (mandatory if iterator_create is defined): Produce the next
 * relation or section in an iteration.  If producing a section result, set
 * *ret_value to NULL.  The returned strings will be freed with free_string.
 */
    /*
 * Optional (mandatory if iterator_create is defined): Free the memory for an
 * iterator.
 */
    /* Optional (mandatory if iterator is defined): Free a string value. */
    /*
 * Optional: Determine if a profile is writable.  If not implemented, the
 * profile is never writable.
 */
    /*
 * Optional: Determine if a profile is modified in memory relative to the
 * persistent store.  If not implemented, the profile is assumed to never be
 * modified.
 */
    /*
 * Optional: Change the value of a relation, or remove it if new_value is NULL.
 * If old_value is set and the relation does not have that value, return
 * PROF_NO_RELATION.
 */
    /*
 * Optional: Rename a section to new_name, or remove the section if new_name is
 * NULL.
 */
    /*
 * Optional: Add a new relation, or a new section if new_value is NULL.  Add
 * any intermediate sections as necessary.
 */
    /*
 * Optional: Flush any pending memory updates to the persistent store.  If
 * implemented, this function will be called by profile_release as well as
 * profile_flush, so make sure it's not inefficient to flush an unmodified
 * profile.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "242:8"]
    pub struct profile_vtable {
        pub minor_ver: libc::c_int,
        pub get_values: profile_get_values_fn,
        pub free_values: profile_free_values_fn,
        pub cleanup: profile_cleanup_fn,
        pub copy: profile_copy_fn,
        pub iterator_create: profile_iterator_create_fn,
        pub iterator: profile_iterator_fn,
        pub iterator_free: profile_iterator_free_fn,
        pub free_string: profile_free_string_fn,
        pub writable: profile_writable_fn,
        pub modified: profile_modified_fn,
        pub update_relation: profile_update_relation_fn,
        pub rename_section: profile_rename_section_fn,
        pub add_relation: profile_add_relation_fn,
        pub flush: profile_flush_fn,
    }
    #[c2rust::src_loc = "239:1"]
    pub type profile_flush_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_long>;
    #[c2rust::src_loc = "229:1"]
    pub type profile_add_relation_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *const libc::c_char,
                                    _: *const libc::c_char) -> libc::c_long>;
    #[c2rust::src_loc = "221:1"]
    pub type profile_rename_section_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *const libc::c_char,
                                    _: *const libc::c_char) -> libc::c_long>;
    #[c2rust::src_loc = "213:1"]
    pub type profile_update_relation_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *const libc::c_char,
                                    _: *const libc::c_char,
                                    _: *const libc::c_char) -> libc::c_long>;
    #[c2rust::src_loc = "205:1"]
    pub type profile_modified_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_int)
                   -> libc::c_long>;
    #[c2rust::src_loc = "197:1"]
    pub type profile_writable_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_int)
                   -> libc::c_long>;
    #[c2rust::src_loc = "190:1"]
    pub type profile_free_string_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut libc::c_char) -> ()>;
    #[c2rust::src_loc = "186:1"]
    pub type profile_iterator_free_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "178:1"]
    pub type profile_iterator_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut libc::c_void,
                                    _: *mut *mut libc::c_char,
                                    _: *mut *mut libc::c_char)
                   -> libc::c_long>;
    #[c2rust::src_loc = "169:1"]
    pub type profile_iterator_create_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const *const libc::c_char,
                                    _: libc::c_int, _: *mut *mut libc::c_void)
                   -> libc::c_long>;
    #[c2rust::src_loc = "155:1"]
    pub type profile_copy_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *mut libc::c_void)
                   -> libc::c_long>;
    #[c2rust::src_loc = "147:1"]
    pub type profile_cleanup_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "143:1"]
    pub type profile_free_values_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut *mut libc::c_char) -> ()>;
    #[c2rust::src_loc = "138:1"]
    pub type profile_get_values_fn
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *const *const libc::c_char,
                                    _: *mut *mut *mut libc::c_char)
                   -> libc::c_long>;
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    #[c2rust::src_loc = "40:1"]
    pub type profile_filespec_t = *mut libc::c_char;
    #[c2rust::src_loc = "42:1"]
    pub type const_profile_filespec_t = *const libc::c_char;
    #[c2rust::src_loc = "43:1"]
    pub type const_profile_filespec_list_t = *const libc::c_char;
    #[c2rust::src_loc = "287:1"]
    pub type profile_module_init_fn
        =
        Option<unsafe extern "C" fn(_: *const libc::c_char,
                                    _: *mut profile_vtable,
                                    _: *mut *mut libc::c_void)
                   -> libc::c_long>;
    use super::prof_int_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/string.h:7"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "124:14"]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "166:14"]
        pub fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/bits/byteswap.h:7"]
pub mod byteswap_h {
    #[inline]
    #[c2rust::src_loc = "48:1"]
    pub unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
        return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int |
                   (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int |
                   (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int |
                   (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
    }
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/usr/include/stdlib.h:7"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:7"]
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
    }
}
#[c2rust::header_src = "/usr/include/assert.h:7"]
pub mod assert_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__int32_t, __uint32_t, __uint64_t, __off_t, __off64_t,
                        __time_t};
pub use self::stdint_intn_h::int32_t;
pub use self::stdint_uintn_h::{uint32_t, uint64_t};
pub use self::time_t_h::time_t;
pub use self::thread_shared_types_h::{__pthread_internal_list,
                                      __pthread_list_t, __pthread_mutex_s};
pub use self::pthreadtypes_h::pthread_mutex_t;
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::k5_thread_h::{k5_os_mutex, k5_mutex_t, k5_mutex_init,
                            k5_mutex_lock, k5_mutex_unlock, k5_os_mutex_init,
                            k5_os_mutex_destroy, k5_os_mutex_lock,
                            k5_os_mutex_unlock};
pub use self::k5_platform_h::{C2RustUnnamed, C2RustUnnamed_0, store_32_be,
                              load_32_be, k5_path_join};
pub use self::k5_err_h::{errinfo, k5_clear_error};
use self::k5_plugin_h::{plugin_file_handle, krb5int_open_plugin,
                        krb5int_close_plugin, krb5int_get_plugin_func};
pub use self::com_err_h::errcode_t;
pub use self::prof_int_h::{_profile_t, prf_lib_handle_t, _prf_lib_handle_t,
                           prf_file_t, _prf_file_t, _prf_data_t,
                           C2RustUnnamed_1, prf_magic_t, prf_data_t,
                           profile_node, profile_open_file,
                           profile_flush_file_data,
                           profile_flush_file_data_to_file,
                           profile_flush_file_data_to_buffer,
                           profile_free_file, profile_close_file,
                           profile_file_is_writable};
pub use self::profile_h::{profile_vtable, profile_flush_fn,
                          profile_add_relation_fn, profile_rename_section_fn,
                          profile_update_relation_fn, profile_modified_fn,
                          profile_writable_fn, profile_free_string_fn,
                          profile_iterator_free_fn, profile_iterator_fn,
                          profile_iterator_create_fn, profile_copy_fn,
                          profile_cleanup_fn, profile_free_values_fn,
                          profile_get_values_fn, profile_t,
                          profile_filespec_t, const_profile_filespec_t,
                          const_profile_filespec_list_t,
                          profile_module_init_fn};
use self::string_h::{memset, strncpy, strdup, strchr, strlen, strerror,
                     memcpy};
pub use self::byteswap_h::__bswap_32;
use self::stdlib_h::{malloc, free};
use self::stdio_h::{stderr, fprintf};
use self::assert_h::__assert_fail;
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * prof_init.c --- routines that manipulate the user-visible profile_t
 *      object.
 */
/* Create a vtable profile, possibly with a library handle.  The new profile
 * takes ownership of the handle refcount on success. */
#[c2rust::src_loc = "18:1"]
unsafe extern "C" fn init_module(mut vtable: *mut profile_vtable,
                                 mut cbdata: *mut libc::c_void,
                                 mut handle: prf_lib_handle_t,
                                 mut ret_profile: *mut profile_t)
 -> errcode_t {
    let mut profile: profile_t = 0 as *mut _profile_t;
    let mut vt_copy: *mut profile_vtable = 0 as *mut profile_vtable;
    /* Check that the vtable's minor version is sane and that mandatory methods
     * are implemented. */
    if (*vtable).minor_ver < 1 as libc::c_int ||
           (*vtable).get_values.is_none() || (*vtable).free_values.is_none() {
        return 22 as libc::c_int as errcode_t
    }
    if (*vtable).cleanup.is_some() && (*vtable).copy.is_none() {
        return 22 as libc::c_int as errcode_t
    }
    if (*vtable).iterator_create.is_some() &&
           ((*vtable).iterator.is_none() || (*vtable).iterator_free.is_none()
                || (*vtable).free_string.is_none()) {
        return 22 as libc::c_int as errcode_t
    }
    profile =
        malloc(::std::mem::size_of::<_profile_t>() as libc::c_ulong) as
            profile_t;
    if profile.is_null() { return 12 as libc::c_int as errcode_t }
    memset(profile as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<_profile_t>() as libc::c_ulong);
    vt_copy =
        malloc(::std::mem::size_of::<profile_vtable>() as libc::c_ulong) as
            *mut profile_vtable;
    if vt_copy.is_null() {
        free(profile as *mut libc::c_void);
        return 12 as libc::c_int as errcode_t
    }
    /* It's safe to just copy the caller's vtable for now.  If the minor
     * version is bumped, we'll need to copy individual fields. */
    *vt_copy = *vtable;
    (*profile).vt = vt_copy;
    (*profile).cbdata = cbdata;
    (*profile).lib_handle = handle;
    (*profile).magic = -(1429577710 as libc::c_long);
    *ret_profile = profile;
    return 0 as libc::c_int as errcode_t;
}
/* Parse modspec into the module path and residual string. */
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn parse_modspec(mut modspec: *const libc::c_char,
                                   mut ret_path: *mut *mut libc::c_char,
                                   mut ret_residual: *mut *mut libc::c_char)
 -> errcode_t {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fullpath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut residual: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: errcode_t = 0;
    *ret_residual = 0 as *mut libc::c_char;
    *ret_path = *ret_residual;
    /* Find the separator, skipping a Windows drive letter if present. */
    p =
        if *modspec as libc::c_int != '\u{0}' as i32 &&
               *modspec.offset(1 as libc::c_int as isize) as libc::c_int ==
                   ':' as i32 {
            modspec.offset(2 as libc::c_int as isize)
        } else { modspec };
    p = strchr(p, ':' as i32);
    if p.is_null() { return -(1429577692 as libc::c_long) }
    /* Copy the path. */
    path =
        malloc((p.wrapping_offset_from(modspec) as libc::c_long +
                    1 as libc::c_int as libc::c_long) as libc::c_ulong) as
            *mut libc::c_char;
    if path.is_null() { return 12 as libc::c_int as errcode_t }
    memcpy(path as *mut libc::c_void, modspec as *const libc::c_void,
           p.wrapping_offset_from(modspec) as libc::c_long as libc::c_ulong);
    *path.offset(p.wrapping_offset_from(modspec) as libc::c_long as isize) =
        '\u{0}' as i32 as libc::c_char;
    /* Compose the path with LIBDIR if it's not absolute. */
    ret =
        k5_path_join(b"/usr/local/lib\x00" as *const u8 as
                         *const libc::c_char, path, &mut fullpath);
    free(path as *mut libc::c_void);
    if ret != 0 { return ret }
    residual = strdup(p.offset(1 as libc::c_int as isize));
    if residual.is_null() {
        free(fullpath as *mut libc::c_void);
        return 12 as libc::c_int as errcode_t
    }
    *ret_path = fullpath;
    *ret_residual = residual;
    return 0 as libc::c_int as errcode_t;
}
/* Load a dynamic profile module as specified by modspec and create a vtable
 * profile for it in *ret_profile. */
#[c2rust::src_loc = "99:1"]
unsafe extern "C" fn init_load_module(mut modspec: *const libc::c_char,
                                      mut ret_profile: *mut profile_t)
 -> errcode_t {
    let mut modpath: *mut libc::c_char =
        0 as *mut libc::c_char; /* Set minor_ver to 1, rest null. */
    let mut residual: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut einfo: errinfo =
        {
            let mut init =
                errinfo{code: 0 as libc::c_int as libc::c_long,
                        msg: 0 as *mut libc::c_char,};
            init
        };
    let mut lib_handle: prf_lib_handle_t = 0 as prf_lib_handle_t;
    let mut plhandle: *mut plugin_file_handle = 0 as *mut plugin_file_handle;
    let mut cbdata: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut fptr: Option<unsafe extern "C" fn() -> ()> = None;
    let mut have_lock: libc::c_int = 0 as libc::c_int;
    let mut have_cbdata: libc::c_int = 0 as libc::c_int;
    let mut vtable: profile_vtable =
        {
            let mut init =
                profile_vtable{minor_ver: 1 as libc::c_int,
                               get_values: None,
                               free_values: None,
                               cleanup: None,
                               copy: None,
                               iterator_create: None,
                               iterator: None,
                               iterator_free: None,
                               free_string: None,
                               writable: None,
                               modified: None,
                               update_relation: None,
                               rename_section: None,
                               add_relation: None,
                               flush: None,};
            init
        };
    let mut err: errcode_t = 0;
    let mut initfn: profile_module_init_fn = None;
    err = parse_modspec(modspec, &mut modpath, &mut residual);
    if !(err != 0) {
        /* Allocate a reference-counted library handle container. */
        lib_handle =
            malloc(::std::mem::size_of::<_prf_lib_handle_t>() as
                       libc::c_ulong) as prf_lib_handle_t;
        if !lib_handle.is_null() {
            err = k5_mutex_init(&mut (*lib_handle).lock) as errcode_t;
            if !(err != 0) {
                have_lock = 1 as libc::c_int;
                /* Open the module and get its initializer. */
                err = krb5int_open_plugin(modpath, &mut plhandle, &mut einfo);
                if !(err != 0) {
                    err =
                        krb5int_get_plugin_func(plhandle,
                                                b"profile_module_init\x00" as
                                                    *const u8 as
                                                    *const libc::c_char,
                                                &mut fptr, &mut einfo);
                    if err == 2 as libc::c_int as libc::c_long {
                        err = -(1429577691 as libc::c_long)
                    }
                    if !(err != 0) {
                        /* Get the profile vtable and callback data pointer. */
                        initfn =
                            ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                               -> ()>,
                                                    profile_module_init_fn>(fptr);
                        err =
                            Some(initfn.expect("non-null function pointer")).expect("non-null function pointer")(residual,
                                                                                                                 &mut vtable,
                                                                                                                 &mut cbdata);
                        if !(err != 0) {
                            have_cbdata = 1 as libc::c_int;
                            /* Create a vtable profile with the information obtained. */
                            (*lib_handle).plugin_handle = plhandle;
                            (*lib_handle).refcount = 1 as libc::c_int;
                            err =
                                init_module(&mut vtable, cbdata, lib_handle,
                                            ret_profile)
                        }
                    }
                }
            }
        }
    }
    free(modpath as *mut libc::c_void);
    free(residual as *mut libc::c_void);
    k5_clear_error(&mut einfo);
    if err != 0 {
        if have_cbdata != 0 && vtable.cleanup.is_some() {
            vtable.cleanup.expect("non-null function pointer")(cbdata);
        }
        if have_lock != 0 { k5_os_mutex_destroy(&mut (*lib_handle).lock); }
        free(lib_handle as *mut libc::c_void);
        if !plhandle.is_null() { krb5int_close_plugin(plhandle); }
    }
    return err;
}
#[no_mangle]
#[c2rust::src_loc = "164:1"]
pub unsafe extern "C" fn profile_init_flags(mut files:
                                                *mut const_profile_filespec_t,
                                            mut flags: libc::c_int,
                                            mut ret_profile: *mut profile_t)
 -> libc::c_long {
    let mut fs: *mut const_profile_filespec_t =
        0 as *mut const_profile_filespec_t;
    let mut profile: profile_t = 0 as *mut _profile_t;
    let mut new_file: prf_file_t = 0 as *mut _prf_file_t;
    let mut last: prf_file_t = 0 as prf_file_t;
    let mut retval: errcode_t = 0 as libc::c_int as errcode_t;
    let mut access_retval: errcode_t = 0 as libc::c_int as errcode_t;
    let mut modspec: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut modspec_arg: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    profile =
        malloc(::std::mem::size_of::<_profile_t>() as libc::c_ulong) as
            profile_t;
    if profile.is_null() { return 12 as libc::c_int as libc::c_long }
    memset(profile as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<_profile_t>() as libc::c_ulong);
    (*profile).magic = -(1429577710 as libc::c_long);
    /*
     * If the filenames list is not specified or empty, return an empty
     * profile.
     */
    if !files.is_null() &&
           !((*files).is_null() ||
                 *(*files).offset(0 as libc::c_int as isize) as libc::c_int ==
                     '\u{0}' as i32) {
        fs = files;
        while !((*fs).is_null() ||
                    *(*fs).offset(0 as libc::c_int as isize) as libc::c_int ==
                        '\u{0}' as i32) {
            /* Allow a module declaration if it is permitted by flags and this
             * is the first file parsed. */
            modspec_arg =
                if flags & 0x1 as libc::c_int != 0 && last.is_null() {
                    &mut modspec
                } else { 0 as *mut *mut libc::c_char };
            retval = profile_open_file(*fs, &mut new_file, modspec_arg);
            if retval == -(1429577693 as libc::c_long) && !modspec.is_null() {
                /* Stop parsing files and load a dynamic module instead. */
                free(profile as *mut libc::c_void);
                retval = init_load_module(modspec, ret_profile);
                free(modspec as *mut libc::c_void);
                return retval
            }
            /* if this file is missing, skip to the next */
            if !(retval == 2 as libc::c_int as libc::c_long) {
                /* If we can't read this file, remember it but keep going. */
                if retval == 13 as libc::c_int as libc::c_long ||
                       retval == 1 as libc::c_int as libc::c_long {
                    access_retval = retval
                } else {
                    if retval != 0 { profile_release(profile); return retval }
                    if !last.is_null() {
                        (*last).next = new_file
                    } else { (*profile).first_file = new_file }
                    last = new_file
                }
            }
            fs = fs.offset(1)
        }
        /*
         * If last is still null after the loop, then all the files were
         * missing or unreadable, so return the appropriate error.
         */
        if last.is_null() {
            profile_release(profile);
            return if access_retval != 0 {
                       access_retval
                   } else { 2 as libc::c_int as libc::c_long }
        }
    }
    *ret_profile = profile;
    return 0 as libc::c_int as libc::c_long;
}
#[no_mangle]
#[c2rust::src_loc = "231:1"]
pub unsafe extern "C" fn profile_init(mut files:
                                          *mut const_profile_filespec_t,
                                      mut ret_profile: *mut profile_t)
 -> libc::c_long {
    return profile_init_flags(files, 0 as libc::c_int, ret_profile);
}
#[no_mangle]
#[c2rust::src_loc = "237:1"]
pub unsafe extern "C" fn profile_init_vtable(mut vtable: *mut profile_vtable,
                                             mut cbdata: *mut libc::c_void,
                                             mut ret_profile: *mut profile_t)
 -> libc::c_long {
    return init_module(vtable, cbdata, 0 as prf_lib_handle_t, ret_profile);
}
/* Copy a vtable profile. */
#[c2rust::src_loc = "245:1"]
unsafe extern "C" fn copy_vtable_profile(mut profile: profile_t,
                                         mut ret_new_profile: *mut profile_t)
 -> errcode_t {
    let mut err: errcode_t = 0;
    let mut cbdata: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut new_profile: profile_t = 0 as *mut _profile_t;
    *ret_new_profile = 0 as profile_t;
    if (*(*profile).vt).copy.is_some() {
        /* Make a copy of profile's cbdata for the new profile. */
        err =
            (*(*profile).vt).copy.expect("non-null function pointer")((*profile).cbdata,
                                                                      &mut cbdata);
        if err != 0 { return err }
        err =
            init_module((*profile).vt, cbdata, (*profile).lib_handle,
                        &mut new_profile);
        if err != 0 && (*(*profile).vt).cleanup.is_some() {
            (*(*profile).vt).cleanup.expect("non-null function pointer")(cbdata);
        }
    } else {
        /* Use the same cbdata as the old profile. */
        err =
            init_module((*profile).vt, (*profile).cbdata,
                        (*profile).lib_handle, &mut new_profile)
    }
    if err != 0 { return err }
    /* Increment the refcount on the library handle if there is one. */
    if !(*profile).lib_handle.is_null() {
        k5_mutex_lock(&mut (*(*profile).lib_handle).lock);
        (*(*profile).lib_handle).refcount += 1;
        k5_mutex_unlock(&mut (*(*profile).lib_handle).lock);
    }
    *ret_new_profile = new_profile;
    return 0 as libc::c_int as errcode_t;
}
#[no_mangle]
#[c2rust::src_loc = "293:1"]
pub unsafe extern "C" fn profile_copy(mut old_profile: profile_t,
                                      mut new_profile: *mut profile_t)
 -> errcode_t {
    let mut size: size_t = 0;
    let mut i: size_t = 0;
    let mut files: *mut const_profile_filespec_t =
        0 as *mut const_profile_filespec_t;
    let mut file: prf_file_t = 0 as *mut _prf_file_t;
    let mut err: errcode_t = 0;
    if !(*old_profile).vt.is_null() {
        return copy_vtable_profile(old_profile, new_profile)
    }
    /* The fields we care about are read-only after creation, so
       no locking is needed.  */
    let mut cll_counter: size_t = 0 as libc::c_int as size_t;
    let mut cll_ptr: prf_file_t = (*old_profile).first_file;
    while !cll_ptr.is_null() {
        cll_counter = cll_counter.wrapping_add(1);
        cll_ptr = (*cll_ptr).next
    }
    size = cll_counter;
    files =
        malloc(size.wrapping_add(1 as libc::c_int as
                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<const_profile_filespec_t>()
                                                                     as
                                                                     libc::c_ulong))
            as *mut const_profile_filespec_t;
    if files.is_null() { return 12 as libc::c_int as errcode_t }
    i = 0 as libc::c_int as size_t;
    file = (*old_profile).first_file;
    while i < size {
        let ref mut fresh0 = *files.offset(i as isize);
        *fresh0 = (*(*file).data).filespec.as_ptr();
        i = i.wrapping_add(1);
        file = (*file).next
    }
    let ref mut fresh1 = *files.offset(size as isize);
    *fresh1 = 0 as const_profile_filespec_t;
    err = profile_init(files, new_profile);
    free(files as *mut libc::c_void);
    return err;
}
#[no_mangle]
#[c2rust::src_loc = "318:1"]
pub unsafe extern "C" fn profile_init_path(mut filepath:
                                               const_profile_filespec_list_t,
                                           mut ret_profile: *mut profile_t)
 -> libc::c_long {
    let mut n_entries: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut ent_len: libc::c_uint = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut t: *const libc::c_char = 0 as *const libc::c_char;
    let mut filenames: *mut profile_filespec_t = 0 as *mut profile_filespec_t;
    let mut retval: errcode_t = 0;
    /* count the distinct filename components */
    s = filepath;
    n_entries = 1 as libc::c_int as libc::c_uint;
    while *s != 0 {
        if *s as libc::c_int == ':' as i32 {
            n_entries = n_entries.wrapping_add(1)
        }
        s = s.offset(1)
    }
    /* the array is NULL terminated */
    filenames =
        malloc((n_entries.wrapping_add(1 as libc::c_int as libc::c_uint) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                    as libc::c_ulong)) as
            *mut profile_filespec_t;
    if filenames.is_null() { return 12 as libc::c_int as libc::c_long }
    /* measure, copy, and skip each one */
    s = filepath;
    i = 0 as libc::c_int;
    loop  {
        t = strchr(s, ':' as i32);
        if !(!t.is_null() ||
                 { t = s.offset(strlen(s) as isize); !t.is_null() }) {
            break ;
        }
        ent_len = t.wrapping_offset_from(s) as libc::c_long as libc::c_uint;
        let ref mut fresh2 = *filenames.offset(i as isize);
        *fresh2 =
            malloc(ent_len.wrapping_add(1 as libc::c_int as libc::c_uint) as
                       libc::c_ulong) as *mut libc::c_char;
        if (*filenames.offset(i as isize)).is_null() {
            loop 
                 /* if malloc fails, free the ones that worked */
                 {
                i -= 1;
                if !(i >= 0 as libc::c_int) { break ; }
                free(*filenames.offset(i as isize) as *mut libc::c_void);
            }
            free(filenames as *mut libc::c_void);
            return 12 as libc::c_int as libc::c_long
        }
        strncpy(*filenames.offset(i as isize), s, ent_len as libc::c_ulong);
        *(*filenames.offset(i as isize)).offset(ent_len as isize) =
            0 as libc::c_int as libc::c_char;
        if *t as libc::c_int == 0 as libc::c_int {
            i += 1;
            break ;
        } else { s = t.offset(1 as libc::c_int as isize); i += 1 }
    }
    /* cap the array */
    let ref mut fresh3 = *filenames.offset(i as isize);
    *fresh3 = 0 as profile_filespec_t;
    retval =
        profile_init_flags(filenames as *mut const_profile_filespec_t,
                           0 as libc::c_int, ret_profile);
    loop 
         /* count back down and free the entries */
         {
        i -= 1;
        if !(i >= 0 as libc::c_int) { break ; }
        free(*filenames.offset(i as isize) as *mut libc::c_void);
    }
    free(filenames as *mut libc::c_void);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "370:1"]
pub unsafe extern "C" fn profile_is_writable(mut profile: profile_t,
                                             mut writable: *mut libc::c_int)
 -> libc::c_long {
    if profile.is_null() || (*profile).magic != -(1429577710 as libc::c_long)
       {
        return -(1429577710 as libc::c_long)
    }
    if writable.is_null() { return 22 as libc::c_int as libc::c_long }
    *writable = 0 as libc::c_int;
    if !(*profile).vt.is_null() {
        if (*(*profile).vt).writable.is_some() {
            return (*(*profile).vt).writable.expect("non-null function pointer")((*profile).cbdata,
                                                                                 writable)
        } else { return 0 as libc::c_int as libc::c_long }
    }
    if !(*profile).first_file.is_null() {
        *writable = profile_file_is_writable((*profile).first_file)
    }
    return 0 as libc::c_int as libc::c_long;
}
#[no_mangle]
#[c2rust::src_loc = "393:1"]
pub unsafe extern "C" fn profile_is_modified(mut profile: profile_t,
                                             mut modified: *mut libc::c_int)
 -> libc::c_long {
    if profile.is_null() || (*profile).magic != -(1429577710 as libc::c_long)
       {
        return -(1429577710 as libc::c_long)
    }
    if modified.is_null() { return 22 as libc::c_int as libc::c_long }
    *modified = 0 as libc::c_int;
    if !(*profile).vt.is_null() {
        if (*(*profile).vt).modified.is_some() {
            return (*(*profile).vt).modified.expect("non-null function pointer")((*profile).cbdata,
                                                                                 modified)
        } else { return 0 as libc::c_int as libc::c_long }
    }
    if !(*profile).first_file.is_null() {
        *modified =
            (*(*(*profile).first_file).data).flags & 0x2 as libc::c_int
    }
    return 0 as libc::c_int as libc::c_long;
}
#[no_mangle]
#[c2rust::src_loc = "416:1"]
pub unsafe extern "C" fn profile_flush(mut profile: profile_t)
 -> libc::c_long {
    if profile.is_null() || (*profile).magic != -(1429577710 as libc::c_long)
       {
        return -(1429577710 as libc::c_long)
    }
    if !(*profile).vt.is_null() {
        if (*(*profile).vt).flush.is_some() {
            return (*(*profile).vt).flush.expect("non-null function pointer")((*profile).cbdata)
        }
        return 0 as libc::c_int as libc::c_long
    }
    if !(*profile).first_file.is_null() {
        return if !(*profile).first_file.is_null() &&
                      (*(*profile).first_file).magic ==
                          -(1429577703 as libc::c_long) {
                   profile_flush_file_data((*(*profile).first_file).data)
               } else { -(1429577703 as libc::c_long) }
    }
    return 0 as libc::c_int as libc::c_long;
}
#[no_mangle]
#[c2rust::src_loc = "434:1"]
pub unsafe extern "C" fn profile_flush_to_file(mut profile: profile_t,
                                               mut outfile:
                                                   const_profile_filespec_t)
 -> libc::c_long {
    if profile.is_null() || (*profile).magic != -(1429577710 as libc::c_long)
       {
        return -(1429577710 as libc::c_long)
    }
    if !(*profile).vt.is_null() { return -(1429577695 as libc::c_long) }
    if !(*profile).first_file.is_null() {
        return if !(*profile).first_file.is_null() &&
                      (*(*profile).first_file).magic ==
                          -(1429577703 as libc::c_long) {
                   profile_flush_file_data_to_file((*(*profile).first_file).data,
                                                   outfile)
               } else { -(1429577703 as libc::c_long) }
    }
    return 0 as libc::c_int as libc::c_long;
}
#[no_mangle]
#[c2rust::src_loc = "450:1"]
pub unsafe extern "C" fn profile_flush_to_buffer(mut profile: profile_t,
                                                 mut buf:
                                                     *mut *mut libc::c_char)
 -> libc::c_long {
    if !(*profile).vt.is_null() { return -(1429577695 as libc::c_long) }
    return profile_flush_file_data_to_buffer((*(*profile).first_file).data,
                                             buf);
}
#[no_mangle]
#[c2rust::src_loc = "458:1"]
pub unsafe extern "C" fn profile_free_buffer(mut profile: profile_t,
                                             mut buf: *mut libc::c_char) {
    free(buf as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "464:1"]
pub unsafe extern "C" fn profile_abandon(mut profile: profile_t) {
    let mut p: prf_file_t = 0 as *mut _prf_file_t;
    let mut next: prf_file_t = 0 as *mut _prf_file_t;
    if profile.is_null() || (*profile).magic != -(1429577710 as libc::c_long)
       {
        return
    }
    if !(*profile).vt.is_null() {
        if (*(*profile).vt).cleanup.is_some() {
            (*(*profile).vt).cleanup.expect("non-null function pointer")((*profile).cbdata);
        }
        if !(*profile).lib_handle.is_null() {
            /* Decrement the refcount on the handle and maybe free it. */
            k5_mutex_lock(&mut (*(*profile).lib_handle).lock);
            (*(*profile).lib_handle).refcount -= 1;
            if (*(*profile).lib_handle).refcount == 0 as libc::c_int {
                krb5int_close_plugin((*(*profile).lib_handle).plugin_handle);
                k5_mutex_unlock(&mut (*(*profile).lib_handle).lock);
                k5_os_mutex_destroy(&mut (*(*profile).lib_handle).lock);
                free((*profile).lib_handle as *mut libc::c_void);
            } else { k5_mutex_unlock(&mut (*(*profile).lib_handle).lock); }
        }
        free((*profile).vt as *mut libc::c_void);
    } else {
        p = (*profile).first_file;
        while !p.is_null() {
            next = (*p).next;
            profile_free_file(p);
            p = next
        }
    }
    (*profile).magic = 0 as libc::c_int as prf_magic_t;
    free(profile as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "497:1"]
pub unsafe extern "C" fn profile_release(mut profile: profile_t) {
    let mut p: prf_file_t = 0 as *mut _prf_file_t;
    let mut next: prf_file_t = 0 as *mut _prf_file_t;
    if profile.is_null() || (*profile).magic != -(1429577710 as libc::c_long)
       {
        return
    }
    if !(*profile).vt.is_null() {
        /* Flush the profile and then delegate to profile_abandon. */
        if (*(*profile).vt).flush.is_some() {
            (*(*profile).vt).flush.expect("non-null function pointer")((*profile).cbdata);
        }
        profile_abandon(profile);
        return
    } else {
        p = (*profile).first_file;
        while !p.is_null() {
            next = (*p).next;
            profile_close_file(p);
            p = next
        }
    }
    (*profile).magic = 0 as libc::c_int as prf_magic_t;
    free(profile as *mut libc::c_void);
}
/* prof_init.c -- included from profile.h */
/*
 * Here begins the profile serialization functions.
 */
#[no_mangle]
#[c2rust::src_loc = "524:1"]
pub unsafe extern "C" fn profile_ser_size(mut unused: *const libc::c_char,
                                          mut profile: profile_t,
                                          mut sizep: *mut size_t)
 -> errcode_t {
    let mut required: size_t = 0;
    let mut pfp: prf_file_t = 0 as *mut _prf_file_t;
    required =
        (3 as libc::c_int as
             libc::c_ulong).wrapping_mul(::std::mem::size_of::<int32_t>() as
                                             libc::c_ulong);
    pfp = (*profile).first_file;
    while !pfp.is_null() {
        required =
            (required as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<int32_t>()
                                                 as libc::c_ulong) as size_t
                as size_t;
        required =
            (required as
                 libc::c_ulong).wrapping_add(strlen((*(*pfp).data).filespec.as_ptr()))
                as size_t as size_t;
        pfp = (*pfp).next
    }
    *sizep =
        (*sizep as libc::c_ulong).wrapping_add(required) as size_t as size_t;
    return 0 as libc::c_int as errcode_t;
}
#[c2rust::src_loc = "539:1"]
unsafe extern "C" fn pack_int32(mut oval: int32_t,
                                mut bufpp: *mut *mut libc::c_uchar,
                                mut remainp: *mut size_t) {
    store_32_be(oval as libc::c_uint, *bufpp as *mut libc::c_void);
    *bufpp =
        (*bufpp).offset(::std::mem::size_of::<int32_t>() as libc::c_ulong as
                            isize);
    *remainp =
        (*remainp as
             libc::c_ulong).wrapping_sub(::std::mem::size_of::<int32_t>() as
                                             libc::c_ulong) as size_t as
            size_t;
}
#[no_mangle]
#[c2rust::src_loc = "546:1"]
pub unsafe extern "C" fn profile_ser_externalize(mut unused:
                                                     *const libc::c_char,
                                                 mut profile: profile_t,
                                                 mut bufpp:
                                                     *mut *mut libc::c_uchar,
                                                 mut remainp: *mut size_t)
 -> errcode_t {
    let mut retval: errcode_t = 0;
    let mut required: size_t = 0;
    let mut bp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut remain: size_t = 0;
    let mut pfp: prf_file_t = 0 as *mut _prf_file_t;
    let mut fcount: int32_t = 0;
    let mut slen: int32_t = 0;
    required = 0 as libc::c_int as size_t;
    bp = *bufpp;
    remain = *remainp;
    retval = 22 as libc::c_int as errcode_t;
    if !profile.is_null() {
        retval = 12 as libc::c_int as errcode_t;
        profile_ser_size(unused, profile, &mut required);
        if required <= remain {
            fcount = 0 as libc::c_int;
            pfp = (*profile).first_file;
            while !pfp.is_null() { fcount += 1; pfp = (*pfp).next }
            pack_int32(-(1429577710 as libc::c_long) as int32_t, &mut bp,
                       &mut remain);
            pack_int32(fcount, &mut bp, &mut remain);
            pfp = (*profile).first_file;
            while !pfp.is_null() {
                slen = strlen((*(*pfp).data).filespec.as_ptr()) as int32_t;
                pack_int32(slen, &mut bp, &mut remain);
                if slen != 0 {
                    memcpy(bp as *mut libc::c_void,
                           (*(*pfp).data).filespec.as_ptr() as
                               *const libc::c_void, slen as size_t);
                    bp = bp.offset(slen as isize);
                    remain =
                        (remain as libc::c_ulong).wrapping_sub(slen as size_t)
                            as size_t as size_t
                }
                pfp = (*pfp).next
            }
            pack_int32(-(1429577710 as libc::c_long) as int32_t, &mut bp,
                       &mut remain);
            retval = 0 as libc::c_int as errcode_t;
            *bufpp = bp;
            *remainp = remain
        }
    }
    return retval;
}
#[c2rust::src_loc = "587:1"]
unsafe extern "C" fn unpack_int32(mut intp: *mut int32_t,
                                  mut bufpp: *mut *mut libc::c_uchar,
                                  mut remainp: *mut size_t) -> libc::c_int {
    if *remainp >= ::std::mem::size_of::<int32_t>() as libc::c_ulong {
        *intp = load_32_be(*bufpp as *const libc::c_void) as int32_t;
        *bufpp =
            (*bufpp).offset(::std::mem::size_of::<int32_t>() as libc::c_ulong
                                as isize);
        *remainp =
            (*remainp as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<int32_t>()
                                                 as libc::c_ulong) as size_t
                as size_t;
        return 0 as libc::c_int
    } else { return 1 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "600:1"]
pub unsafe extern "C" fn profile_ser_internalize(mut unused:
                                                     *const libc::c_char,
                                                 mut profilep: *mut profile_t,
                                                 mut bufpp:
                                                     *mut *mut libc::c_uchar,
                                                 mut remainp: *mut size_t)
 -> errcode_t {
    let mut current_block: u64;
    let mut retval: errcode_t = 0;
    let mut bp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut remain: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut fcount: int32_t = 0;
    let mut tmp: int32_t = 0;
    let mut flist: *mut profile_filespec_t = 0 as *mut profile_filespec_t;
    bp = *bufpp;
    remain = *remainp;
    fcount = 0 as libc::c_int;
    if remain >= 12 as libc::c_int as libc::c_ulong {
        unpack_int32(&mut tmp, &mut bp, &mut remain);
    } else { tmp = 0 as libc::c_int }
    if tmp as libc::c_long != -(1429577710 as libc::c_long) {
        retval = 22 as libc::c_int as errcode_t
    } else {
        unpack_int32(&mut fcount, &mut bp, &mut remain);
        retval = 12 as libc::c_int as errcode_t;
        flist =
            malloc((::std::mem::size_of::<profile_filespec_t>() as
                        libc::c_ulong).wrapping_mul((fcount +
                                                         1 as libc::c_int) as
                                                        size_t)) as
                *mut profile_filespec_t;
        if !flist.is_null() {
            memset(flist as *mut libc::c_void, 0 as libc::c_int,
                   (::std::mem::size_of::<*mut libc::c_char>() as
                        libc::c_ulong).wrapping_mul((fcount +
                                                         1 as libc::c_int) as
                                                        size_t));
            i = 0 as libc::c_int;
            loop  {
                if !(i < fcount) {
                    current_block = 15768484401365413375;
                    break ;
                }
                if unpack_int32(&mut tmp, &mut bp, &mut remain) == 0 {
                    let ref mut fresh4 = *flist.offset(i as isize);
                    *fresh4 =
                        malloc((tmp + 1 as libc::c_int) as size_t) as
                            *mut libc::c_char;
                    if (*flist.offset(i as isize)).is_null() {
                        current_block = 14610734243255607494;
                        break ;
                    }
                    memcpy(*flist.offset(i as isize) as *mut libc::c_void,
                           bp as *const libc::c_void, tmp as size_t);
                    *(*flist.offset(i as isize)).offset(tmp as isize) =
                        '\u{0}' as i32 as libc::c_char;
                    bp = bp.offset(tmp as isize);
                    remain =
                        (remain as libc::c_ulong).wrapping_sub(tmp as size_t)
                            as size_t as size_t
                }
                i += 1
            }
            match current_block {
                14610734243255607494 => { }
                _ => {
                    if unpack_int32(&mut tmp, &mut bp, &mut remain) != 0 ||
                           tmp as libc::c_long !=
                               -(1429577710 as libc::c_long) {
                        retval = 22 as libc::c_int as errcode_t
                    } else {
                        retval =
                            profile_init(flist as
                                             *mut const_profile_filespec_t,
                                         profilep);
                        if !(retval != 0) { *bufpp = bp; *remainp = remain }
                    }
                }
            }
        }
    }
    if !flist.is_null() {
        i = 0 as libc::c_int;
        while i < fcount {
            if !(*flist.offset(i as isize)).is_null() {
                free(*flist.offset(i as isize) as *mut libc::c_void);
            }
            i += 1
        }
        free(flist as *mut libc::c_void);
    }
    return retval;
}
