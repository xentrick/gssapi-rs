use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:27"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "145:1"]
    pub type __dev_t = libc::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = libc::c_ulong;
    #[c2rust::src_loc = "150:1"]
    pub type __mode_t = libc::c_uint;
    #[c2rust::src_loc = "151:1"]
    pub type __nlink_t = libc::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "174:1"]
    pub type __blksize_t = libc::c_long;
    #[c2rust::src_loc = "179:1"]
    pub type __blkcnt_t = libc::c_long;
    #[c2rust::src_loc = "196:1"]
    pub type __syscall_slong_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:27"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "9:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__time_t, __syscall_slong_t};
}
#[c2rust::header_src = "/usr/include/bits/stat.h:27"]
pub mod stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "46:8"]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: libc::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    use super::types_h::{__dev_t, __ino_t, __nlink_t, __mode_t, __uid_t,
                         __gid_t, __off_t, __blksize_t, __blkcnt_t,
                         __syscall_slong_t};
    use super::struct_timespec_h::timespec;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:28"]
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
        #[c2rust::src_loc = "51:1"]
        pub fn k5_set_error(ep: *mut errinfo, code: libc::c_long,
                            fmt: *const libc::c_char, _: ...);
    }
    /* K5_ERR_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:28"]
pub mod k5_plugin_h {
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
    /* opaque */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "82:8"]
    pub struct plugin_dir_handle {
        pub files: *mut *mut plugin_file_handle,
    }
    use super::plugin_file_handle;
    /* K5_PLUGIN_H */
}
#[c2rust::header_src = "/usr/include/dirent.h:441"]
pub mod include_dirent_h {
    #[c2rust::src_loc = "127:1"]
    pub type DIR = __dirstream;
    use super::dirent_h::dirent;
    extern "C" {
        #[c2rust::src_loc = "127:16"]
        pub type __dirstream;
        #[no_mangle]
        #[c2rust::src_loc = "134:1"]
        pub fn opendir(__name: *const libc::c_char) -> *mut DIR;
        #[no_mangle]
        #[c2rust::src_loc = "149:1"]
        pub fn closedir(__dirp: *mut DIR) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "162:1"]
        pub fn readdir(__dirp: *mut DIR) -> *mut dirent;
    }
}
#[c2rust::header_src = "/usr/include/bits/dirent.h:441"]
pub mod dirent_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "22:8"]
    pub struct dirent {
        pub d_ino: __ino_t,
        pub d_off: __off_t,
        pub d_reclen: libc::c_ushort,
        pub d_type: libc::c_uchar,
        pub d_name: [libc::c_char; 256],
    }
    use super::types_h::{__ino_t, __off_t};
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "384:15"]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:27"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:27"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/dlfcn.h:30"]
pub mod dlfcn_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "56:1"]
        pub fn dlopen(__file: *const libc::c_char, __mode: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "60:1"]
        pub fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "64:1"]
        pub fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "82:1"]
        pub fn dlerror() -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:34"]
pub mod sys_stat_h {
    use super::stat_h::stat;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "205:1"]
        pub fn stat(__file: *const libc::c_char, __buf: *mut stat)
         -> libc::c_int;
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__dev_t, __uid_t, __gid_t, __ino_t, __mode_t,
                        __nlink_t, __off_t, __time_t, __blksize_t, __blkcnt_t,
                        __syscall_slong_t};
pub use self::struct_timespec_h::timespec;
pub use self::stat_h::stat;
pub use self::k5_err_h::{errinfo, k5_set_error};
pub use self::k5_plugin_h::plugin_dir_handle;
pub use self::include_dirent_h::{DIR, __dirstream, opendir, closedir,
                                 readdir};
pub use self::dirent_h::dirent;
use self::string_h::{strlen, strerror, strcmp};
use self::stdlib_h::{calloc, realloc, free};
use self::stdio_h::asprintf;
use self::libintl_h::dgettext;
use self::errno_h::__errno_location;
use self::dlfcn_h::{dlopen, dlclose, dlsym, dlerror};
use self::sys_stat_h::stat;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "93:8"]
pub struct plugin_file_handle {
    pub dlhandle: *mut libc::c_void,
}
/*
 * glibc bug 11941, fixed in release 2.25, can cause an assertion failure in
 * dlclose() on process exit.  Our workaround is to leak dlopen() handles
 * (which doesn't typically manifest in leak detection tools because the
 * handles are still reachable via a global table in libdl).  Because we
 * dlopen() with RTLD_NODELETE, we weren't going to unload the plugin objects
 * anyway.
 */
#[c2rust::src_loc = "83:1"]
unsafe extern "C" fn Tprintf(mut fmt: *const libc::c_char, mut args: ...) { }
#[no_mangle]
#[c2rust::src_loc = "189:1"]
pub unsafe extern "C" fn krb5int_open_plugin(mut filepath:
                                                 *const libc::c_char,
                                             mut h:
                                                 *mut *mut plugin_file_handle,
                                             mut ep: *mut errinfo)
 -> libc::c_long {
    let mut err: libc::c_long =
        0 as libc::c_int as
            libc::c_long; /* calloc initializes ptrs to NULL */
    let mut statbuf: stat =
        stat{st_dev: 0,
             st_ino: 0,
             st_nlink: 0,
             st_mode: 0,
             st_uid: 0,
             st_gid: 0,
             __pad0: 0,
             st_rdev: 0,
             st_size: 0,
             st_blksize: 0,
             st_blocks: 0,
             st_atim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_mtim: timespec{tv_sec: 0, tv_nsec: 0,},
             st_ctim: timespec{tv_sec: 0, tv_nsec: 0,},
             __glibc_reserved: [0; 3],};
    let mut htmp: *mut plugin_file_handle = 0 as *mut plugin_file_handle;
    let mut got_plugin: libc::c_int = 0 as libc::c_int;
    if err == 0 {
        if stat(filepath, &mut statbuf) < 0 as libc::c_int {
            err = *__errno_location() as libc::c_long;
            Tprintf(b"stat(%s): %s\n\x00" as *const u8 as *const libc::c_char,
                    filepath, strerror(err as libc::c_int));
            k5_set_error(ep, err,
                         dgettext(b"mit-krb5\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"unable to find plugin [%s]: %s\x00" as
                                      *const u8 as *const libc::c_char),
                         filepath, strerror(err as libc::c_int));
        }
    }
    if err == 0 {
        htmp =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<plugin_file_handle>() as
                       libc::c_ulong) as *mut plugin_file_handle;
        if htmp.is_null() { err = 12 as libc::c_int as libc::c_long }
    }
    if err == 0 &&
           statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
               0o100000 as libc::c_int as libc::c_uint {
        let mut handle: *mut libc::c_void = 0 as *mut libc::c_void;
        /* USE_CFBUNDLE */
        if err == 0 {
            handle =
                dlopen(filepath,
                       0x2 as libc::c_int | 0 as libc::c_int |
                           0 as libc::c_int |
                           0x1000 as libc::c_int); /* XXX */
            if handle.is_null() {
                let mut e: *const libc::c_char = dlerror();
                if e.is_null() {
                    e =
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"unknown failure\x00" as *const u8 as
                                     *const libc::c_char)
                }
                Tprintf(b"dlopen(%s): %s\n\x00" as *const u8 as
                            *const libc::c_char, filepath, e);
                err = 2 as libc::c_int as libc::c_long;
                k5_set_error(ep, err,
                             dgettext(b"mit-krb5\x00" as *const u8 as
                                          *const libc::c_char,
                                      b"unable to load plugin [%s]: %s\x00" as
                                          *const u8 as *const libc::c_char),
                             filepath, e);
            }
        }
        if err == 0 {
            got_plugin = 1 as libc::c_int;
            (*htmp).dlhandle = handle;
            handle = 0 as *mut libc::c_void
        }
        if !handle.is_null() { dlclose(handle); }
    }
    /* USE_DLOPEN */
    if err == 0 && got_plugin == 0 {
        err =
            2 as libc::c_int as
                libc::c_long; /* no plugin or no way to load plugins */
        k5_set_error(ep, err,
                     dgettext(b"mit-krb5\x00" as *const u8 as
                                  *const libc::c_char,
                              b"plugin unavailable: %s\x00" as *const u8 as
                                  *const libc::c_char),
                     strerror(err as libc::c_int));
    }
    if err == 0 {
        *h = htmp;
        htmp = 0 as *mut plugin_file_handle
        /* h takes ownership */
    }
    free(htmp as *mut libc::c_void);
    return err;
}
#[c2rust::src_loc = "344:1"]
unsafe extern "C" fn krb5int_get_plugin_sym(mut h: *mut plugin_file_handle,
                                            mut csymname: *const libc::c_char,
                                            mut isfunc: libc::c_int,
                                            mut ptr: *mut *mut libc::c_void,
                                            mut ep: *mut errinfo)
 -> libc::c_long {
    let mut err: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut sym: *mut libc::c_void = 0 as *mut libc::c_void;
    if err == 0 && sym.is_null() && !(*h).dlhandle.is_null() {
        /* XXX Do we need to add a leading "_" to the symbol name on any
           modern platforms?  */
        sym = dlsym((*h).dlhandle, csymname); /* XXX copy and save away */
        if sym.is_null() {
            let mut e: *const libc::c_char = dlerror(); /* XXX */
            if e.is_null() {
                e = b"unknown failure\x00" as *const u8 as *const libc::c_char
            }
            Tprintf(b"dlsym(%s): %s\n\x00" as *const u8 as
                        *const libc::c_char, csymname, e);
            err = 2 as libc::c_int as libc::c_long;
            k5_set_error(ep, err,
                         b"%s\x00" as *const u8 as *const libc::c_char, e);
        }
    }
    if err == 0 && sym.is_null() {
        err = 2 as libc::c_int as libc::c_long
        /* unimplemented */
    }
    if err == 0 { *ptr = sym }
    return err;
}
#[no_mangle]
#[c2rust::src_loc = "407:1"]
pub unsafe extern "C" fn krb5int_get_plugin_data(mut h:
                                                     *mut plugin_file_handle,
                                                 mut csymname:
                                                     *const libc::c_char,
                                                 mut ptr:
                                                     *mut *mut libc::c_void,
                                                 mut ep: *mut errinfo)
 -> libc::c_long {
    return krb5int_get_plugin_sym(h, csymname, 0 as libc::c_int, ptr, ep);
}
#[no_mangle]
#[c2rust::src_loc = "414:1"]
pub unsafe extern "C" fn krb5int_get_plugin_func(mut h:
                                                     *mut plugin_file_handle,
                                                 mut csymname:
                                                     *const libc::c_char,
                                                 mut ptr:
                                                     *mut Option<unsafe extern "C" fn()
                                                                     -> ()>,
                                                 mut ep: *mut errinfo)
 -> libc::c_long {
    let mut dptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut err: libc::c_long =
        krb5int_get_plugin_sym(h, csymname, 1 as libc::c_int, &mut dptr, ep);
    if err == 0 {
        /* Cast function pointers to avoid code duplication */
        *ptr =
            ::std::mem::transmute::<*mut libc::c_void,
                                    Option<unsafe extern "C" fn()
                                               -> ()>>(dptr)
    }
    return err;
}
#[no_mangle]
#[c2rust::src_loc = "427:1"]
pub unsafe extern "C" fn krb5int_close_plugin(mut h:
                                                  *mut plugin_file_handle) {
    if !(*h).dlhandle.is_null() { dlclose((*h).dlhandle); }
    free(h as *mut libc::c_void);
}
/* autoconf docs suggest using this preference order */
#[c2rust::src_loc = "459:1"]
unsafe extern "C" fn krb5int_plugin_file_handle_array_init(mut harray:
                                                               *mut *mut *mut plugin_file_handle)
 -> libc::c_long {
    let mut err: libc::c_long =
        0 as libc::c_int as libc::c_long; /* calloc initializes to NULL */
    *harray =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<*mut plugin_file_handle>() as
                   libc::c_ulong) as
            *mut *mut plugin_file_handle; /* +1 for NULL */
    if (*harray).is_null() { err = 12 as libc::c_int as libc::c_long }
    return err;
}
#[c2rust::src_loc = "470:1"]
unsafe extern "C" fn krb5int_plugin_file_handle_array_add(mut harray:
                                                              *mut *mut *mut plugin_file_handle,
                                                          mut count:
                                                              *mut size_t,
                                                          mut p:
                                                              *mut plugin_file_handle)
 -> libc::c_long {
    let mut err: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut newharray: *mut *mut plugin_file_handle =
        0 as *mut *mut plugin_file_handle;
    let mut newcount: size_t =
        (*count).wrapping_add(1 as libc::c_int as libc::c_ulong);
    newharray =
        realloc(*harray as *mut libc::c_void,
                newcount.wrapping_add(1 as libc::c_int as
                                          libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut plugin_file_handle>()
                                                                          as
                                                                          libc::c_ulong))
            as *mut *mut plugin_file_handle;
    if newharray.is_null() {
        err = 12 as libc::c_int as libc::c_long
    } else {
        let ref mut fresh0 =
            *newharray.offset(newcount.wrapping_sub(1 as libc::c_int as
                                                        libc::c_ulong) as
                                  isize);
        *fresh0 = p;
        let ref mut fresh1 = *newharray.offset(newcount as isize);
        *fresh1 = 0 as *mut plugin_file_handle;
        *count = newcount;
        *harray = newharray
    }
    return err;
}
#[c2rust::src_loc = "491:1"]
unsafe extern "C" fn krb5int_plugin_file_handle_array_free(mut harray:
                                                               *mut *mut plugin_file_handle) {
    if !harray.is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while !(*harray.offset(i as isize)).is_null() {
            krb5int_close_plugin(*harray.offset(i as isize));
            i += 1
        }
        free(harray as *mut libc::c_void);
    };
}
#[c2rust::src_loc = "512:1"]
unsafe extern "C" fn krb5int_free_plugin_filenames(mut filenames:
                                                       *mut *mut libc::c_char) {
    if !filenames.is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while !(*filenames.offset(i as isize)).is_null() {
            free(*filenames.offset(i as isize) as *mut libc::c_void);
            i += 1
        }
        free(filenames as *mut libc::c_void);
    };
}
#[c2rust::src_loc = "525:1"]
unsafe extern "C" fn krb5int_get_plugin_filenames(mut filebases:
                                                      *const *const libc::c_char,
                                                  mut filenames:
                                                      *mut *mut *mut libc::c_char)
 -> libc::c_long {
    let mut err: libc::c_long = 0 as libc::c_int as libc::c_long;
    static mut fileexts: [*const libc::c_char; 3] =
        [b"\x00" as *const u8 as *const libc::c_char,
         b".so\x00" as *const u8 as *const libc::c_char,
         0 as *const libc::c_char];
    let mut tempnames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut bases_count: size_t = 0 as libc::c_int as size_t;
    let mut exts_count: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    if filebases.is_null() { err = 22 as libc::c_int as libc::c_long }
    if filenames.is_null() { err = 22 as libc::c_int as libc::c_long }
    if err == 0 {
        i = 0 as libc::c_int as size_t;
        while !(*filebases.offset(i as isize)).is_null() {
            bases_count = bases_count.wrapping_add(1);
            i = i.wrapping_add(1)
        }
        i = 0 as libc::c_int as size_t;
        while !fileexts[i as usize].is_null() {
            exts_count = exts_count.wrapping_add(1);
            i = i.wrapping_add(1)
        }
        tempnames =
            calloc(bases_count.wrapping_mul(exts_count).wrapping_add(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong),
                   ::std::mem::size_of::<*mut libc::c_char>() as
                       libc::c_ulong) as *mut *mut libc::c_char;
        if tempnames.is_null() { err = 12 as libc::c_int as libc::c_long }
    }
    if err == 0 {
        let mut j: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while err == 0 && !(*filebases.offset(i as isize)).is_null() {
            j = 0 as libc::c_int as size_t;
            while err == 0 && !fileexts[j as usize].is_null() {
                if asprintf(&mut *tempnames.offset(i.wrapping_mul(exts_count).wrapping_add(j)
                                                       as isize) as
                                *mut *mut libc::c_char,
                            b"%s%s\x00" as *const u8 as *const libc::c_char,
                            *filebases.offset(i as isize),
                            fileexts[j as usize]) < 0 as libc::c_int {
                    let ref mut fresh2 =
                        *tempnames.offset(i.wrapping_mul(exts_count).wrapping_add(j)
                                              as isize);
                    *fresh2 = 0 as *mut libc::c_char;
                    err = 12 as libc::c_int as libc::c_long
                }
                j = j.wrapping_add(1)
            }
            i = i.wrapping_add(1)
        }
        let ref mut fresh3 =
            *tempnames.offset(bases_count.wrapping_mul(exts_count) as isize);
        *fresh3 = 0 as *mut libc::c_char
        /* NUL-terminate */
    }
    if err == 0 {
        *filenames = tempnames;
        tempnames = 0 as *mut *mut libc::c_char
    }
    krb5int_free_plugin_filenames(tempnames);
    return err;
}
/* Takes a NULL-terminated list of directories.  If filebases is NULL, filebases is ignored
 * all plugins in the directories are loaded.  If filebases is a NULL-terminated array of names,
 * only plugins in the directories with those name (plus any platform extension) are loaded. */
#[no_mangle]
#[c2rust::src_loc = "574:1"]
pub unsafe extern "C" fn krb5int_open_plugin_dirs(mut dirnames:
                                                      *const *const libc::c_char,
                                                  mut filebases:
                                                      *const *const libc::c_char,
                                                  mut dirhandle:
                                                      *mut plugin_dir_handle,
                                                  mut ep: *mut errinfo)
 -> libc::c_long {
    let mut err: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut h: *mut *mut plugin_file_handle =
        0 as *mut *mut plugin_file_handle;
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut filenames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if err == 0 { err = krb5int_plugin_file_handle_array_init(&mut h) }
    if err == 0 && !filebases.is_null() {
        err = krb5int_get_plugin_filenames(filebases, &mut filenames)
    }
    i = 0 as libc::c_int;
    while err == 0 && !(*dirnames.offset(i as isize)).is_null() {
        if !filenames.is_null() {
            /* load plugins with names from filenames from each directory */
            let mut j: libc::c_int = 0;
            j = 0 as libc::c_int;
            while err == 0 && !(*filenames.offset(j as isize)).is_null() {
                let mut handle: *mut plugin_file_handle =
                    0 as *mut plugin_file_handle;
                let mut filepath: *mut libc::c_char = 0 as *mut libc::c_char;
                if err == 0 {
                    if asprintf(&mut filepath as *mut *mut libc::c_char,
                                b"%s/%s\x00" as *const u8 as
                                    *const libc::c_char,
                                *dirnames.offset(i as isize),
                                *filenames.offset(j as isize)) <
                           0 as libc::c_int {
                        filepath = 0 as *mut libc::c_char;
                        err = 12 as libc::c_int as libc::c_long
                    }
                }
                if err == 0 &&
                       krb5int_open_plugin(filepath, &mut handle, ep) ==
                           0 as libc::c_int as libc::c_long {
                    err =
                        krb5int_plugin_file_handle_array_add(&mut h,
                                                             &mut count,
                                                             handle);
                    if err == 0 { handle = 0 as *mut plugin_file_handle }
                    /* h takes ownership */
                }
                free(filepath as *mut libc::c_void);
                if !handle.is_null() { krb5int_close_plugin(handle); }
                j += 1
            }
        } else {
            /* load all plugins in each directory */
            let mut dir: *mut DIR = opendir(*dirnames.offset(i as isize));
            while !dir.is_null() && err == 0 {
                let mut d: *mut dirent = 0 as *mut dirent;
                let mut filepath_0: *mut libc::c_char =
                    0 as *mut libc::c_char;
                let mut handle_0: *mut plugin_file_handle =
                    0 as *mut plugin_file_handle;
                d = readdir(dir);
                if d.is_null() { break ; }
                if strcmp((*d).d_name.as_mut_ptr(),
                          b".\x00" as *const u8 as *const libc::c_char) ==
                       0 as libc::c_int ||
                       strcmp((*d).d_name.as_mut_ptr(),
                              b"..\x00" as *const u8 as *const libc::c_char)
                           == 0 as libc::c_int {
                    continue ;
                }
                if err == 0 {
                    let mut len: libc::c_int =
                        strlen((*d).d_name.as_mut_ptr()) as libc::c_int;
                    if asprintf(&mut filepath_0 as *mut *mut libc::c_char,
                                b"%s/%*s\x00" as *const u8 as
                                    *const libc::c_char,
                                *dirnames.offset(i as isize), len,
                                (*d).d_name.as_mut_ptr()) < 0 as libc::c_int {
                        filepath_0 = 0 as *mut libc::c_char;
                        err = 12 as libc::c_int as libc::c_long
                    }
                }
                if err == 0 {
                    if krb5int_open_plugin(filepath_0, &mut handle_0, ep) ==
                           0 as libc::c_int as libc::c_long {
                        err =
                            krb5int_plugin_file_handle_array_add(&mut h,
                                                                 &mut count,
                                                                 handle_0);
                        if err == 0 {
                            handle_0 = 0 as *mut plugin_file_handle
                        }
                        /* h takes ownership */
                    }
                }
                free(filepath_0 as *mut libc::c_void);
                if !handle_0.is_null() { krb5int_close_plugin(handle_0); }
            }
            if !dir.is_null() { closedir(dir); }
        }
        i += 1
    }
    if err == 2 as libc::c_int as libc::c_long {
        err = 0 as libc::c_int as libc::c_long
        /* ran out of plugins -- do nothing */
    }
    if err == 0 {
        (*dirhandle).files = h;
        h = 0 as *mut *mut plugin_file_handle
        /* dirhandle->files takes ownership */
    }
    if !filenames.is_null() { krb5int_free_plugin_filenames(filenames); }
    if !h.is_null() { krb5int_plugin_file_handle_array_free(h); }
    return err;
}
#[no_mangle]
#[c2rust::src_loc = "674:1"]
pub unsafe extern "C" fn krb5int_close_plugin_dirs(mut dirhandle:
                                                       *mut plugin_dir_handle) {
    if !(*dirhandle).files.is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while !(*(*dirhandle).files.offset(i as isize)).is_null() {
            krb5int_close_plugin(*(*dirhandle).files.offset(i as isize));
            i += 1
        }
        free((*dirhandle).files as *mut libc::c_void);
        (*dirhandle).files = 0 as *mut *mut plugin_file_handle
    };
}
#[no_mangle]
#[c2rust::src_loc = "687:1"]
pub unsafe extern "C" fn krb5int_free_plugin_dir_data(mut ptrs:
                                                          *mut *mut libc::c_void) {
    /* Nothing special to be done per pointer.  */
    free(ptrs as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "694:1"]
pub unsafe extern "C" fn krb5int_get_plugin_dir_data(mut dirhandle:
                                                         *mut plugin_dir_handle,
                                                     mut symname:
                                                         *const libc::c_char,
                                                     mut ptrs:
                                                         *mut *mut *mut libc::c_void,
                                                     mut ep: *mut errinfo)
 -> libc::c_long {
    let mut err: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut p: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut count: size_t = 0 as libc::c_int as size_t;
    /* XXX Do we need to add a leading "_" to the symbol name on any
       modern platforms?  */
    Tprintf(b"get_plugin_data_sym(%s)\n\x00" as *const u8 as
                *const libc::c_char,
            symname); /* calloc initializes to NULL */
    if err == 0 {
        p =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<*mut libc::c_void>() as
                       libc::c_ulong) as
                *mut *mut libc::c_void; /* +1 for NULL */
        if p.is_null() { err = 12 as libc::c_int as libc::c_long }
    }
    if err == 0 && !dirhandle.is_null() && !(*dirhandle).files.is_null() {
        let mut i: libc::c_int = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while err == 0 && !(*(*dirhandle).files.offset(i as isize)).is_null()
              {
            let mut sym: *mut libc::c_void = 0 as *mut libc::c_void;
            if krb5int_get_plugin_data(*(*dirhandle).files.offset(i as isize),
                                       symname, &mut sym, ep) ==
                   0 as libc::c_int as libc::c_long {
                let mut newp: *mut *mut libc::c_void =
                    0 as *mut *mut libc::c_void;
                count = count.wrapping_add(1);
                newp =
                    realloc(p as *mut libc::c_void,
                            count.wrapping_add(1 as libc::c_int as
                                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_void>()
                                                                                   as
                                                                                   libc::c_ulong))
                        as *mut *mut libc::c_void;
                if newp.is_null() {
                    err = 12 as libc::c_int as libc::c_long
                } else {
                    p = newp;
                    let ref mut fresh4 =
                        *p.offset(count.wrapping_sub(1 as libc::c_int as
                                                         libc::c_ulong) as
                                      isize);
                    *fresh4 = sym;
                    let ref mut fresh5 = *p.offset(count as isize);
                    *fresh5 = 0 as *mut libc::c_void
                }
            }
            i += 1
        }
    }
    if err == 0 {
        *ptrs = p;
        p = 0 as *mut *mut libc::c_void
        /* ptrs takes ownership */
    }
    free(p as *mut libc::c_void);
    return err;
}
#[no_mangle]
#[c2rust::src_loc = "746:1"]
pub unsafe extern "C" fn krb5int_free_plugin_dir_func(mut ptrs:
                                                          *mut Option<unsafe extern "C" fn()
                                                                          ->
                                                                              ()>) {
    /* Nothing special to be done per pointer.  */
    free(ptrs as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "753:1"]
pub unsafe extern "C" fn krb5int_get_plugin_dir_func(mut dirhandle:
                                                         *mut plugin_dir_handle,
                                                     mut symname:
                                                         *const libc::c_char,
                                                     mut ptrs:
                                                         *mut *mut Option<unsafe extern "C" fn()
                                                                              ->
                                                                                  ()>,
                                                     mut ep: *mut errinfo)
 -> libc::c_long {
    let mut err: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut p: *mut Option<unsafe extern "C" fn() -> ()> =
        0 as *mut Option<unsafe extern "C" fn() -> ()>;
    let mut count: size_t = 0 as libc::c_int as size_t;
    /* XXX Do we need to add a leading "_" to the symbol name on any
       modern platforms?  */
    Tprintf(b"get_plugin_data_sym(%s)\n\x00" as *const u8 as
                *const libc::c_char,
            symname); /* calloc initializes to NULL */
    if err == 0 {
        p =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<Option<unsafe extern "C" fn()
                                                    -> ()>>() as
                       libc::c_ulong) as
                *mut Option<unsafe extern "C" fn() -> ()>; /* +1 for NULL */
        if p.is_null() { err = 12 as libc::c_int as libc::c_long }
    }
    if err == 0 && !dirhandle.is_null() && !(*dirhandle).files.is_null() {
        let mut i: libc::c_int = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while err == 0 && !(*(*dirhandle).files.offset(i as isize)).is_null()
              {
            let mut sym: Option<unsafe extern "C" fn() -> ()> = None;
            if krb5int_get_plugin_func(*(*dirhandle).files.offset(i as isize),
                                       symname, &mut sym, ep) ==
                   0 as libc::c_int as libc::c_long {
                let mut newp: *mut Option<unsafe extern "C" fn() -> ()> =
                    0 as *mut Option<unsafe extern "C" fn() -> ()>;
                count = count.wrapping_add(1);
                newp =
                    realloc(p as *mut libc::c_void,
                            count.wrapping_add(1 as libc::c_int as
                                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<Option<unsafe extern "C" fn()
                                                                                                                ->
                                                                                                                    ()>>()
                                                                                   as
                                                                                   libc::c_ulong))
                        as *mut Option<unsafe extern "C" fn() -> ()>;
                if newp.is_null() {
                    err = 12 as libc::c_int as libc::c_long
                } else {
                    p = newp;
                    let ref mut fresh6 =
                        *p.offset(count.wrapping_sub(1 as libc::c_int as
                                                         libc::c_ulong) as
                                      isize);
                    *fresh6 = sym;
                    let ref mut fresh7 = *p.offset(count as isize);
                    *fresh7 = None
                }
            }
            i += 1
        }
    }
    if err == 0 {
        *ptrs = p as *mut Option<unsafe extern "C" fn() -> ()>;
        p = 0 as *mut Option<unsafe extern "C" fn() -> ()>
        /* ptrs takes ownership */
    }
    free(p as *mut libc::c_void);
    return err;
}
