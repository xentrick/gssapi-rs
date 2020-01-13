use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:25"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:25"]
pub mod types_h {
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = libc::c_int;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/sys/types.h:26"]
pub mod sys_types_h {
    #[c2rust::src_loc = "97:1"]
    pub type pid_t = __pid_t;
    use super::types_h::__pid_t;
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:26"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/verto.h:40"]
pub mod verto_h {
    #[c2rust::src_loc = "37:1"]
    pub type verto_proc = pid_t;
    #[c2rust::src_loc = "38:1"]
    pub type verto_proc_status = libc::c_int;
    #[c2rust::src_loc = "60:9"]
    pub type verto_ev_flag = libc::c_uint;
    #[c2rust::src_loc = "76:5"]
    pub const _VERTO_EV_FLAG_MAX: verto_ev_flag = 256;
    #[c2rust::src_loc = "71:5"]
    pub const _VERTO_EV_FLAG_MUTABLE_MASK: verto_ev_flag = 62;
    #[c2rust::src_loc = "70:5"]
    pub const VERTO_EV_FLAG_REINITIABLE: verto_ev_flag = 64;
    #[c2rust::src_loc = "69:5"]
    pub const VERTO_EV_FLAG_IO_CLOSE_FD: verto_ev_flag = 256;
    #[c2rust::src_loc = "68:5"]
    pub const VERTO_EV_FLAG_IO_ERROR: verto_ev_flag = 128;
    #[c2rust::src_loc = "67:5"]
    pub const VERTO_EV_FLAG_IO_WRITE: verto_ev_flag = 32;
    #[c2rust::src_loc = "66:5"]
    pub const VERTO_EV_FLAG_IO_READ: verto_ev_flag = 16;
    #[c2rust::src_loc = "65:5"]
    pub const VERTO_EV_FLAG_PRIORITY_HIGH: verto_ev_flag = 8;
    #[c2rust::src_loc = "64:5"]
    pub const VERTO_EV_FLAG_PRIORITY_MEDIUM: verto_ev_flag = 4;
    #[c2rust::src_loc = "63:5"]
    pub const VERTO_EV_FLAG_PRIORITY_LOW: verto_ev_flag = 2;
    #[c2rust::src_loc = "62:5"]
    pub const VERTO_EV_FLAG_PERSIST: verto_ev_flag = 1;
    #[c2rust::src_loc = "61:5"]
    pub const VERTO_EV_FLAG_NONE: verto_ev_flag = 0;
    #[c2rust::src_loc = "79:1"]
    pub type verto_callback
        =
        unsafe extern "C" fn(_: *mut verto_ctx, _: *mut verto_ev) -> ();
    #[c2rust::src_loc = "51:9"]
    pub type verto_ev_type = libc::c_uint;
    #[c2rust::src_loc = "57:5"]
    pub const VERTO_EV_TYPE_CHILD: verto_ev_type = 16;
    #[c2rust::src_loc = "56:5"]
    pub const VERTO_EV_TYPE_SIGNAL: verto_ev_type = 8;
    #[c2rust::src_loc = "55:5"]
    pub const VERTO_EV_TYPE_IDLE: verto_ev_type = 4;
    #[c2rust::src_loc = "54:5"]
    pub const VERTO_EV_TYPE_TIMEOUT: verto_ev_type = 2;
    #[c2rust::src_loc = "53:5"]
    pub const VERTO_EV_TYPE_IO: verto_ev_type = 1;
    #[c2rust::src_loc = "52:5"]
    pub const VERTO_EV_TYPE_NONE: verto_ev_type = 0;
    use super::sys_types_h::pid_t;
    use super::{verto_ctx, verto_ev};
    /* VERTO_H_ */
    /* __cplusplus */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/verto/verto-module.h:40"]
pub mod verto_module_h {
    /*
 * Copyright 2011 Red Hat, Inc.
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation files
 * (the "Software"), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge,
 * publish, distribute, sublicense, and/or sell copies of the Software,
 * and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
    /* ** THE FOLLOWING ARE FOR IMPLEMENTATION MODULES ONLY ***/
    #[c2rust::src_loc = "35:1"]
    pub type verto_mod_ev = ();
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "90:9"]
    pub struct verto_module {
        pub vers: libc::c_uint,
        pub name: *const libc::c_char,
        pub symb: *const libc::c_char,
        pub types: verto_ev_type,
        pub funcs: *mut verto_ctx_funcs,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "71:9"]
    pub struct verto_ctx_funcs {
        pub ctx_new: Option<unsafe extern "C" fn() -> *mut libc::c_void>,
        pub ctx_default: Option<unsafe extern "C" fn() -> *mut libc::c_void>,
        pub ctx_free: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                 -> ()>,
        pub ctx_run: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub ctx_run_once: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> ()>,
        pub ctx_break: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                  -> ()>,
        pub ctx_reinitialize: Option<unsafe extern "C" fn(_:
                                                              *mut libc::c_void)
                                         -> ()>,
        pub ctx_set_flags: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                       _: *const verto_ev,
                                                       _: *mut libc::c_void)
                                      -> ()>,
        pub ctx_add: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                 _: *const verto_ev,
                                                 _: *mut verto_ev_flag)
                                -> *mut libc::c_void>,
        pub ctx_del: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                 _: *const verto_ev,
                                                 _: *mut libc::c_void) -> ()>,
    }
    #[c2rust::src_loc = "34:1"]
    pub type verto_mod_ctx = ();
    use super::verto_h::{verto_ev_type, verto_ev_flag};
    use super::verto_ev;
    /* VERTO_MODULE_H_ */
}
#[c2rust::header_src = "/usr/include/stdlib.h:26"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "136:12"]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "225:14"]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:29"]
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
#[c2rust::header_src = "/usr/include/unistd.h:40"]
pub mod unistd_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/verto/module.h:41"]
pub mod module_h {
    extern "C" {
        /* Closes a module.
 *
 * Does nothing if dll is NULL.
 *
 * @param dll The module to close.
 */
        #[no_mangle]
        #[c2rust::src_loc = "52:1"]
        pub fn module_close(dll: *mut libc::c_void);
    }
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__pid_t, __time_t};
pub use self::sys_types_h::pid_t;
pub use self::time_t_h::time_t;
pub use self::verto_h::{verto_proc, verto_proc_status, verto_ev_flag,
                        _VERTO_EV_FLAG_MAX, _VERTO_EV_FLAG_MUTABLE_MASK,
                        VERTO_EV_FLAG_REINITIABLE, VERTO_EV_FLAG_IO_CLOSE_FD,
                        VERTO_EV_FLAG_IO_ERROR, VERTO_EV_FLAG_IO_WRITE,
                        VERTO_EV_FLAG_IO_READ, VERTO_EV_FLAG_PRIORITY_HIGH,
                        VERTO_EV_FLAG_PRIORITY_MEDIUM,
                        VERTO_EV_FLAG_PRIORITY_LOW, VERTO_EV_FLAG_PERSIST,
                        VERTO_EV_FLAG_NONE, verto_callback, verto_ev_type,
                        VERTO_EV_TYPE_CHILD, VERTO_EV_TYPE_SIGNAL,
                        VERTO_EV_TYPE_IDLE, VERTO_EV_TYPE_TIMEOUT,
                        VERTO_EV_TYPE_IO, VERTO_EV_TYPE_NONE};
pub use self::verto_module_h::{verto_mod_ev, verto_module, verto_ctx_funcs,
                               verto_mod_ctx};
use self::stdlib_h::{realloc, free};
use self::string_h::{memset, strcmp, strchr};
use self::assert_h::__assert_fail;
use self::unistd_h::close;
use self::module_h::module_close;
extern "C" {
    /*
 * This symbol can be used when embedding verto.c in a library along with a
 * built-in private module, to preload the module instead of dynamically
 * linking it in later.  Define to <modulename>.
 */
    #[no_mangle]
    #[c2rust::src_loc = "108:21"]
    pub static mut verto_module_table_k5ev: verto_module;
}
/*
 * Copyright 2011 Red Hat, Inc.
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation files
 * (the "Software"), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge,
 * publish, distribute, sublicense, and/or sell copies of the Software,
 * and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
/* Remove flags we can emulate */
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "51:8"]
pub struct verto_ctx {
    pub ref_0: size_t,
    pub ctx: *mut libc::c_void,
    pub module: *const verto_module,
    pub events: *mut verto_ev,
    pub deflt: libc::c_int,
    pub exit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "70:8"]
pub struct verto_ev {
    pub next: *mut verto_ev,
    pub ctx: *mut verto_ctx,
    pub type_0: verto_ev_type,
    pub callback: Option<verto_callback>,
    pub onfree: Option<verto_callback>,
    pub priv_0: *mut libc::c_void,
    pub ev: *mut libc::c_void,
    pub flags: verto_ev_flag,
    pub actual: verto_ev_flag,
    pub depth: size_t,
    pub deleted: libc::c_int,
    pub option: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "82:5"]
pub union C2RustUnnamed {
    pub io: verto_io,
    pub signal: libc::c_int,
    pub interval: time_t,
    pub child: verto_child,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "60:9"]
pub struct verto_child {
    pub proc_0: verto_proc,
    pub status: verto_proc_status,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "65:9"]
pub struct verto_io {
    pub fd: libc::c_int,
    pub state: verto_ev_flag,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "91:8"]
pub struct module_record {
    pub next: *mut module_record,
    pub module: *const verto_module,
    pub dll: *mut libc::c_void,
    pub filename: *mut libc::c_char,
    pub defctx: *mut verto_ctx,
}
#[c2rust::src_loc = "109:22"]
static mut builtin_record: module_record =
    unsafe {
        {
            let mut init =
                module_record{next:
                                  0 as *const module_record as
                                      *mut module_record,
                              module:
                                  &verto_module_table_k5ev as
                                      *const verto_module as
                                      *mut verto_module,
                              dll:
                                  0 as *const libc::c_void as
                                      *mut libc::c_void,
                              filename:
                                  b"\x00" as *const u8 as *const libc::c_char
                                      as *mut libc::c_char,
                              defctx:
                                  0 as *const verto_ctx as *mut verto_ctx,};
            init
        }
    };
#[c2rust::src_loc = "112:23"]
static mut loaded_modules: *mut module_record =
    unsafe { &builtin_record as *const module_record as *mut module_record };
#[c2rust::src_loc = "117:16"]
static mut resize_cb:
       Option<unsafe extern "C" fn(_: *mut libc::c_void, _: size_t)
                  -> *mut libc::c_void> =
    None;
#[c2rust::src_loc = "118:12"]
static mut resize_cb_hierarchical: libc::c_int = 0;
/* HAVE_PTHREAD */
/* HAVE_PTHREAD */
#[c2rust::src_loc = "161:1"]
unsafe extern "C" fn vresize(mut mem: *mut libc::c_void, mut size: size_t)
 -> *mut libc::c_void {
    if resize_cb.is_none() {
        resize_cb =
            Some(realloc as
                     unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: libc::c_ulong)
                         -> *mut libc::c_void)
    }
    if size == 0 as libc::c_int as libc::c_ulong &&
           resize_cb ==
               Some(realloc as
                        unsafe extern "C" fn(_: *mut libc::c_void,
                                             _: libc::c_ulong)
                            -> *mut libc::c_void) {
        /* Avoid memleak as realloc(X, 0) can return a free-able pointer. */
        free(mem);
        return 0 as *mut libc::c_void
    }
    return Some(resize_cb.expect("non-null function pointer")).expect("non-null function pointer")(mem,
                                                                                                   size);
}
#[c2rust::src_loc = "376:1"]
unsafe extern "C" fn load_module(mut impl_0: *const libc::c_char,
                                 mut reqtypes: verto_ev_type,
                                 mut record: *mut *mut module_record)
 -> libc::c_int {
    let mut success: libc::c_int = 0 as libc::c_int;
    /* Check the cache */
    if !impl_0.is_null() {
        *record = loaded_modules;
        while !(*record).is_null() {
            if !strchr(impl_0, '/' as i32).is_null() &&
                   strcmp(impl_0, (**record).filename) == 0 ||
                   strcmp(impl_0, (*(**record).module).name) == 0 {
                return 1 as libc::c_int
            }
            *record = (**record).next
        }
    } else if !loaded_modules.is_null() {
        *record = loaded_modules;
        while !(*record).is_null() {
            if reqtypes as libc::c_uint ==
                   VERTO_EV_TYPE_NONE as libc::c_int as libc::c_uint ||
                   (*(**record).module).types as libc::c_uint &
                       reqtypes as libc::c_uint == reqtypes as libc::c_uint {
                return 1 as libc::c_int
            }
            *record = (**record).next
        }
    }
    /* BUILTIN_MODULE */
    return success;
}
#[c2rust::src_loc = "482:1"]
unsafe extern "C" fn make_ev(mut ctx: *mut verto_ctx,
                             mut callback: Option<verto_callback>,
                             mut type_0: verto_ev_type,
                             mut flags: verto_ev_flag) -> *mut verto_ev {
    let mut ev: *mut verto_ev = 0 as *mut verto_ev;
    if ctx.is_null() || callback.is_none() { return 0 as *mut verto_ev }
    ev =
        vresize(0 as *mut libc::c_void,
                ::std::mem::size_of::<verto_ev>() as libc::c_ulong) as
            *mut verto_ev;
    if !ev.is_null() {
        memset(ev as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<verto_ev>() as libc::c_ulong);
        (*ev).ctx = ctx;
        (*ev).type_0 = type_0;
        (*ev).callback = callback;
        (*ev).flags = flags
    }
    return ev;
}
#[c2rust::src_loc = "503:1"]
unsafe extern "C" fn push_ev(mut ctx: *mut verto_ctx, mut ev: *mut verto_ev) {
    let mut tmp: *mut verto_ev = 0 as *mut verto_ev;
    if ctx.is_null() || ev.is_null() { return }
    tmp = (*ctx).events;
    (*ctx).events = ev;
    (*(*ctx).events).next = tmp;
}
#[c2rust::src_loc = "516:1"]
unsafe extern "C" fn remove_ev(mut origin: *mut *mut verto_ev,
                               mut item: *mut verto_ev) {
    if origin.is_null() || (*origin).is_null() || item.is_null() { return }
    if *origin == item {
        *origin = (**origin).next
    } else { remove_ev(&mut (**origin).next, item); };
}
#[c2rust::src_loc = "528:1"]
unsafe extern "C" fn signal_ignore(mut ctx: *mut verto_ctx,
                                   mut ev: *mut verto_ev) {
}
#[no_mangle]
#[c2rust::src_loc = "535:1"]
pub unsafe extern "C" fn verto_new(mut impl_0: *const libc::c_char,
                                   mut reqtypes: verto_ev_type)
 -> *mut verto_ctx {
    let mut mr: *mut module_record = 0 as *mut module_record;
    if load_module(impl_0, reqtypes, &mut mr) == 0 {
        return 0 as *mut verto_ctx
    }
    return verto_convert_module((*mr).module, 0 as libc::c_int,
                                0 as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "546:1"]
pub unsafe extern "C" fn verto_default(mut impl_0: *const libc::c_char,
                                       mut reqtypes: verto_ev_type)
 -> *mut verto_ctx {
    let mut mr: *mut module_record = 0 as *mut module_record;
    if load_module(impl_0, reqtypes, &mut mr) == 0 {
        return 0 as *mut verto_ctx
    }
    return verto_convert_module((*mr).module, 1 as libc::c_int,
                                0 as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "557:1"]
pub unsafe extern "C" fn verto_set_default(mut impl_0: *const libc::c_char,
                                           mut reqtypes: verto_ev_type)
 -> libc::c_int {
    let mut mr: *mut module_record = 0 as *mut module_record;
    if !loaded_modules.is_null() || impl_0.is_null() {
        return 0 as libc::c_int
    }
    return load_module(impl_0, reqtypes, &mut mr);
}
#[no_mangle]
#[c2rust::src_loc = "572:1"]
pub unsafe extern "C" fn verto_set_allocator(mut resize:
                                                 Option<unsafe extern "C" fn(_:
                                                                                 *mut libc::c_void,
                                                                             _:
                                                                                 size_t)
                                                            ->
                                                                *mut libc::c_void>,
                                             mut hierarchical: libc::c_int)
 -> libc::c_int {
    if resize_cb.is_some() || resize.is_none() { return 0 as libc::c_int }
    resize_cb = resize;
    resize_cb_hierarchical = hierarchical;
    return 1 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "583:1"]
pub unsafe extern "C" fn verto_free(mut ctx: *mut verto_ctx) {
    if ctx.is_null() { return }
    (*ctx).ref_0 =
        if (*ctx).ref_0 > 0 as libc::c_int as libc::c_ulong {
            (*ctx).ref_0.wrapping_sub(1 as libc::c_int as libc::c_ulong)
        } else { 0 as libc::c_int as libc::c_ulong };
    if (*ctx).ref_0 > 0 as libc::c_int as libc::c_ulong { return }
    /* Cancel all pending events */
    while !(*ctx).events.is_null() { verto_del((*ctx).events); }
    /* Free the private */
    if (*ctx).deflt == 0 || (*(*(*ctx).module).funcs).ctx_default.is_none() {
        (*(*(*ctx).module).funcs).ctx_free.expect("non-null function pointer")((*ctx).ctx);
    }
    vresize(ctx as *mut libc::c_void, 0 as libc::c_int as size_t);
}
#[no_mangle]
#[c2rust::src_loc = "604:1"]
pub unsafe extern "C" fn verto_cleanup() {
    let mut record: *mut module_record = 0 as *mut module_record;
    record = loaded_modules;
    while !record.is_null() {
        module_close((*record).dll);
        free((*record).filename as *mut libc::c_void);
        record = (*record).next
    }
    vresize(loaded_modules as *mut libc::c_void, 0 as libc::c_int as size_t);
    loaded_modules = 0 as *mut module_record;
}
#[no_mangle]
#[c2rust::src_loc = "623:1"]
pub unsafe extern "C" fn verto_run(mut ctx: *mut verto_ctx) {
    if ctx.is_null() { return }
    if (*(*(*ctx).module).funcs).ctx_break.is_some() &&
           (*(*(*ctx).module).funcs).ctx_run.is_some() {
        (*(*(*ctx).module).funcs).ctx_run.expect("non-null function pointer")((*ctx).ctx);
    } else {
        while (*ctx).exit == 0 {
            (*(*(*ctx).module).funcs).ctx_run_once.expect("non-null function pointer")((*ctx).ctx);
        }
        (*ctx).exit = 0 as libc::c_int
    };
}
#[no_mangle]
#[c2rust::src_loc = "638:1"]
pub unsafe extern "C" fn verto_run_once(mut ctx: *mut verto_ctx) {
    if ctx.is_null() { return }
    (*(*(*ctx).module).funcs).ctx_run_once.expect("non-null function pointer")((*ctx).ctx);
}
#[no_mangle]
#[c2rust::src_loc = "646:1"]
pub unsafe extern "C" fn verto_break(mut ctx: *mut verto_ctx) {
    if ctx.is_null() { return }
    if (*(*(*ctx).module).funcs).ctx_break.is_some() &&
           (*(*(*ctx).module).funcs).ctx_run.is_some() {
        (*(*(*ctx).module).funcs).ctx_break.expect("non-null function pointer")((*ctx).ctx);
    } else { (*ctx).exit = 1 as libc::c_int };
}
#[no_mangle]
#[c2rust::src_loc = "658:1"]
pub unsafe extern "C" fn verto_reinitialize(mut ctx: *mut verto_ctx)
 -> libc::c_int {
    let mut tmp: *mut verto_ev = 0 as *mut verto_ev;
    let mut next: *mut verto_ev = 0 as *mut verto_ev;
    let mut error: libc::c_int = 1 as libc::c_int;
    if ctx.is_null() { return 0 as libc::c_int }
    /* Delete all events, but keep around the forkable ev structs */
    tmp = (*ctx).events;
    while !tmp.is_null() {
        next = (*tmp).next;
        if (*tmp).flags as libc::c_uint &
               VERTO_EV_FLAG_REINITIABLE as libc::c_int as libc::c_uint != 0 {
            (*(*(*ctx).module).funcs).ctx_del.expect("non-null function pointer")((*ctx).ctx,
                                                                                  tmp,
                                                                                  (*tmp).ev);
        } else { verto_del(tmp); }
        tmp = next
    }
    /* Reinit the loop */
    if (*(*(*ctx).module).funcs).ctx_reinitialize.is_some() {
        (*(*(*ctx).module).funcs).ctx_reinitialize.expect("non-null function pointer")((*ctx).ctx);
    }
    /* Recreate events that were marked forkable */
    tmp = (*ctx).events;
    while !tmp.is_null() {
        (*tmp).actual =
            ((*tmp).flags as libc::c_uint &
                 !(VERTO_EV_FLAG_PERSIST as libc::c_int |
                       VERTO_EV_FLAG_IO_CLOSE_FD as libc::c_int) as
                     libc::c_uint) as verto_ev_flag;
        (*tmp).ev =
            (*(*(*ctx).module).funcs).ctx_add.expect("non-null function pointer")((*ctx).ctx,
                                                                                  tmp,
                                                                                  &mut (*tmp).actual);
        if (*tmp).ev.is_null() { error = 0 as libc::c_int }
        tmp = (*tmp).next
    }
    return error;
}
#[no_mangle]
#[c2rust::src_loc = "705:1"]
pub unsafe extern "C" fn verto_add_io(mut ctx: *mut verto_ctx,
                                      mut flags: verto_ev_flag,
                                      mut callback: Option<verto_callback>,
                                      mut fd: libc::c_int) -> *mut verto_ev {
    let mut ev: *mut verto_ev = 0 as *mut verto_ev;
    if fd < 0 as libc::c_int ||
           flags as libc::c_uint &
               (VERTO_EV_FLAG_IO_READ as libc::c_int |
                    VERTO_EV_FLAG_IO_WRITE as libc::c_int) as libc::c_uint ==
               0 {
        return 0 as *mut verto_ev
    }
    ev = make_ev(ctx, callback, VERTO_EV_TYPE_IO, flags);
    if !ev.is_null() {
        (*ev).option.io.fd = fd;
        (*ev).actual =
            ((*ev).flags as libc::c_uint &
                 !(VERTO_EV_FLAG_PERSIST as libc::c_int |
                       VERTO_EV_FLAG_IO_CLOSE_FD as libc::c_int) as
                     libc::c_uint) as verto_ev_flag;
        (*ev).ev =
            (*(*(*ctx).module).funcs).ctx_add.expect("non-null function pointer")((*ctx).ctx,
                                                                                  ev,
                                                                                  &mut (*ev).actual);
        if (*ev).ev.is_null() {
            vresize(ev as *mut libc::c_void, 0 as libc::c_int as size_t);
            return 0 as *mut verto_ev
        }
        push_ev(ctx, ev);
    }
    return ev;
}
#[no_mangle]
#[c2rust::src_loc = "718:1"]
pub unsafe extern "C" fn verto_add_timeout(mut ctx: *mut verto_ctx,
                                           mut flags: verto_ev_flag,
                                           mut callback:
                                               Option<verto_callback>,
                                           mut interval: time_t)
 -> *mut verto_ev {
    let mut ev: *mut verto_ev = 0 as *mut verto_ev;
    ev = make_ev(ctx, callback, VERTO_EV_TYPE_TIMEOUT, flags);
    if !ev.is_null() {
        (*ev).option.interval = interval;
        (*ev).actual =
            ((*ev).flags as libc::c_uint &
                 !(VERTO_EV_FLAG_PERSIST as libc::c_int |
                       VERTO_EV_FLAG_IO_CLOSE_FD as libc::c_int) as
                     libc::c_uint) as verto_ev_flag;
        (*ev).ev =
            (*(*(*ctx).module).funcs).ctx_add.expect("non-null function pointer")((*ctx).ctx,
                                                                                  ev,
                                                                                  &mut (*ev).actual);
        if (*ev).ev.is_null() {
            vresize(ev as *mut libc::c_void, 0 as libc::c_int as size_t);
            return 0 as *mut verto_ev
        }
        push_ev(ctx, ev);
    }
    return ev;
}
#[no_mangle]
#[c2rust::src_loc = "727:1"]
pub unsafe extern "C" fn verto_add_idle(mut ctx: *mut verto_ctx,
                                        mut flags: verto_ev_flag,
                                        mut callback: Option<verto_callback>)
 -> *mut verto_ev {
    let mut ev: *mut verto_ev = 0 as *mut verto_ev;
    ev = make_ev(ctx, callback, VERTO_EV_TYPE_IDLE, flags);
    if !ev.is_null() {
        (*ev).actual =
            ((*ev).flags as libc::c_uint &
                 !(VERTO_EV_FLAG_PERSIST as libc::c_int |
                       VERTO_EV_FLAG_IO_CLOSE_FD as libc::c_int) as
                     libc::c_uint) as verto_ev_flag;
        (*ev).ev =
            (*(*(*ctx).module).funcs).ctx_add.expect("non-null function pointer")((*ctx).ctx,
                                                                                  ev,
                                                                                  &mut (*ev).actual);
        if (*ev).ev.is_null() {
            vresize(ev as *mut libc::c_void, 0 as libc::c_int as size_t);
            return 0 as *mut verto_ev
        }
        push_ev(ctx, ev);
    }
    return ev;
}
#[no_mangle]
#[c2rust::src_loc = "736:1"]
pub unsafe extern "C" fn verto_add_signal(mut ctx: *mut verto_ctx,
                                          mut flags: verto_ev_flag,
                                          mut callback:
                                              Option<verto_callback>,
                                          mut signal: libc::c_int)
 -> *mut verto_ev {
    let mut ev: *mut verto_ev = 0 as *mut verto_ev;
    if signal < 0 as libc::c_int { return 0 as *mut verto_ev }
    if signal == 17 as libc::c_int { return 0 as *mut verto_ev }
    if callback ==
           ::std::mem::transmute::<libc::intptr_t,
                                   Option<verto_callback>>(1 as libc::c_int as
                                                               libc::intptr_t)
       {
        callback =
            Some(signal_ignore as
                     unsafe extern "C" fn(_: *mut verto_ctx, _: *mut verto_ev)
                         -> ());
        if flags as libc::c_uint &
               VERTO_EV_FLAG_PERSIST as libc::c_int as libc::c_uint == 0 {
            return 0 as *mut verto_ev
        }
    }
    ev = make_ev(ctx, callback, VERTO_EV_TYPE_SIGNAL, flags);
    if !ev.is_null() {
        (*ev).option.signal = signal;
        (*ev).actual =
            ((*ev).flags as libc::c_uint &
                 !(VERTO_EV_FLAG_PERSIST as libc::c_int |
                       VERTO_EV_FLAG_IO_CLOSE_FD as libc::c_int) as
                     libc::c_uint) as verto_ev_flag;
        (*ev).ev =
            (*(*(*ctx).module).funcs).ctx_add.expect("non-null function pointer")((*ctx).ctx,
                                                                                  ev,
                                                                                  &mut (*ev).actual);
        if (*ev).ev.is_null() {
            vresize(ev as *mut libc::c_void, 0 as libc::c_int as size_t);
            return 0 as *mut verto_ev
        }
        push_ev(ctx, ev);
    }
    return ev;
}
#[no_mangle]
#[c2rust::src_loc = "757:1"]
pub unsafe extern "C" fn verto_add_child(mut ctx: *mut verto_ctx,
                                         mut flags: verto_ev_flag,
                                         mut callback: Option<verto_callback>,
                                         mut proc_0: verto_proc)
 -> *mut verto_ev {
    let mut ev: *mut verto_ev = 0 as *mut verto_ev;
    if flags as libc::c_uint &
           VERTO_EV_FLAG_PERSIST as libc::c_int as libc::c_uint != 0 {
        /* persist makes no sense */
        return 0 as *mut verto_ev
    }
    if proc_0 < 1 as libc::c_int { return 0 as *mut verto_ev }
    ev = make_ev(ctx, callback, VERTO_EV_TYPE_CHILD, flags);
    if !ev.is_null() {
        (*ev).option.child.proc_0 = proc_0;
        (*ev).actual =
            ((*ev).flags as libc::c_uint &
                 !(VERTO_EV_FLAG_PERSIST as libc::c_int |
                       VERTO_EV_FLAG_IO_CLOSE_FD as libc::c_int) as
                     libc::c_uint) as verto_ev_flag;
        (*ev).ev =
            (*(*(*ctx).module).funcs).ctx_add.expect("non-null function pointer")((*ctx).ctx,
                                                                                  ev,
                                                                                  &mut (*ev).actual);
        if (*ev).ev.is_null() {
            vresize(ev as *mut libc::c_void, 0 as libc::c_int as size_t);
            return 0 as *mut verto_ev
        }
        push_ev(ctx, ev);
    }
    return ev;
}
#[no_mangle]
#[c2rust::src_loc = "775:1"]
pub unsafe extern "C" fn verto_set_private(mut ev: *mut verto_ev,
                                           mut priv_0: *mut libc::c_void,
                                           mut free_0:
                                               Option<verto_callback>) {
    if ev.is_null() { return }
    if (*ev).onfree.is_some() && free_0.is_some() {
        (*ev).onfree.expect("non-null function pointer")((*ev).ctx, ev);
    }
    (*ev).priv_0 = priv_0;
    (*ev).onfree = free_0;
}
#[no_mangle]
#[c2rust::src_loc = "786:1"]
pub unsafe extern "C" fn verto_get_private(mut ev: *const verto_ev)
 -> *mut libc::c_void {
    return (*ev).priv_0;
}
#[no_mangle]
#[c2rust::src_loc = "792:1"]
pub unsafe extern "C" fn verto_get_type(mut ev: *const verto_ev)
 -> verto_ev_type {
    return (*ev).type_0;
}
#[no_mangle]
#[c2rust::src_loc = "798:1"]
pub unsafe extern "C" fn verto_get_flags(mut ev: *const verto_ev)
 -> verto_ev_flag {
    return (*ev).flags;
}
#[no_mangle]
#[c2rust::src_loc = "804:1"]
pub unsafe extern "C" fn verto_set_flags(mut ev: *mut verto_ev,
                                         mut flags: verto_ev_flag) {
    if ev.is_null() { return }
    /* No modification is needed, so do nothing. */
    if (*ev).flags as libc::c_uint &
           _VERTO_EV_FLAG_MUTABLE_MASK as libc::c_int as libc::c_uint ==
           flags as libc::c_uint &
               _VERTO_EV_FLAG_MUTABLE_MASK as libc::c_int as libc::c_uint {
        return
    }
    (*ev).flags =
        ::std::mem::transmute::<libc::c_uint,
                                verto_ev_flag>((*ev).flags as libc::c_uint &
                                                   !(_VERTO_EV_FLAG_MUTABLE_MASK
                                                         as libc::c_int) as
                                                       libc::c_uint);
    (*ev).flags =
        ::std::mem::transmute::<libc::c_uint,
                                verto_ev_flag>((*ev).flags as libc::c_uint |
                                                   flags as libc::c_uint &
                                                       _VERTO_EV_FLAG_MUTABLE_MASK
                                                           as libc::c_int as
                                                           libc::c_uint);
    /* If setting flags isn't supported, just rebuild the event */
    if (*(*(*(*ev).ctx).module).funcs).ctx_set_flags.is_none() {
        (*(*(*(*ev).ctx).module).funcs).ctx_del.expect("non-null function pointer")((*(*ev).ctx).ctx,
                                                                                    ev,
                                                                                    (*ev).ev);
        (*ev).actual =
            ((*ev).flags as libc::c_uint &
                 !(VERTO_EV_FLAG_PERSIST as libc::c_int |
                       VERTO_EV_FLAG_IO_CLOSE_FD as libc::c_int) as
                     libc::c_uint) as verto_ev_flag;
        (*ev).ev =
            (*(*(*(*ev).ctx).module).funcs).ctx_add.expect("non-null function pointer")((*(*ev).ctx).ctx,
                                                                                        ev,
                                                                                        &mut (*ev).actual);
        /* implement set_flags(): we cannot fail gracefully. */
        if !(*ev).ev.is_null() {
        } else {
            __assert_fail(b"ev->ev\x00" as *const u8 as *const libc::c_char,
                          b"verto.c\x00" as *const u8 as *const libc::c_char,
                          822 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 48],
                                                    &[libc::c_char; 48]>(b"void verto_set_flags(verto_ev *, verto_ev_flag)\x00")).as_ptr()); /* Here is the main reason why modules should */
        }
        return
    }
    (*ev).actual =
        ::std::mem::transmute::<libc::c_uint,
                                verto_ev_flag>((*ev).actual as libc::c_uint &
                                                   !(_VERTO_EV_FLAG_MUTABLE_MASK
                                                         as libc::c_int) as
                                                       libc::c_uint);
    (*ev).actual =
        ::std::mem::transmute::<libc::c_uint,
                                verto_ev_flag>((*ev).actual as libc::c_uint |
                                                   flags as libc::c_uint &
                                                       _VERTO_EV_FLAG_MUTABLE_MASK
                                                           as libc::c_int as
                                                           libc::c_uint);
    (*(*(*(*ev).ctx).module).funcs).ctx_set_flags.expect("non-null function pointer")((*(*ev).ctx).ctx,
                                                                                      ev,
                                                                                      (*ev).ev);
}
#[no_mangle]
#[c2rust::src_loc = "831:1"]
pub unsafe extern "C" fn verto_get_fd(mut ev: *const verto_ev)
 -> libc::c_int {
    if !ev.is_null() &&
           (*ev).type_0 as libc::c_uint ==
               VERTO_EV_TYPE_IO as libc::c_int as libc::c_uint {
        return (*ev).option.io.fd
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "839:1"]
pub unsafe extern "C" fn verto_get_fd_state(mut ev: *const verto_ev)
 -> verto_ev_flag {
    return (*ev).option.io.state;
}
#[no_mangle]
#[c2rust::src_loc = "845:1"]
pub unsafe extern "C" fn verto_get_interval(mut ev: *const verto_ev)
 -> time_t {
    if !ev.is_null() &&
           (*ev).type_0 as libc::c_uint ==
               VERTO_EV_TYPE_TIMEOUT as libc::c_int as libc::c_uint {
        return (*ev).option.interval
    }
    return 0 as libc::c_int as time_t;
}
#[no_mangle]
#[c2rust::src_loc = "853:1"]
pub unsafe extern "C" fn verto_get_signal(mut ev: *const verto_ev)
 -> libc::c_int {
    if !ev.is_null() &&
           (*ev).type_0 as libc::c_uint ==
               VERTO_EV_TYPE_SIGNAL as libc::c_int as libc::c_uint {
        return (*ev).option.signal
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "861:1"]
pub unsafe extern "C" fn verto_get_proc(mut ev: *const verto_ev)
 -> verto_proc {
    if !ev.is_null() &&
           (*ev).type_0 as libc::c_uint ==
               VERTO_EV_TYPE_CHILD as libc::c_int as libc::c_uint {
        return (*ev).option.child.proc_0
    }
    return 0 as libc::c_int;
}
/*
 * Copyright 2011 Red Hat, Inc.
 *
 * Permission is hereby granted, free of charge, to any person
 * obtaining a copy of this software and associated documentation files
 * (the "Software"), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge,
 * publish, distribute, sublicense, and/or sell copies of the Software,
 * and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
/* For time_t */
/* For pid_t */
/* __cplusplus */
/* *
 * Creates a new event context using an optionally specified implementation
 * and/or optionally specified required features.
 *
 * If you are an application that has already decided on using a particular
 * event loop implementation, you should not call this function, but instead
 * import the verto-NAME.h header and link against the verto-NAME.so, where
 * NAME is the implementation you wish to use.
 *
 * If you are a library, you should generally avoid creating event contexts
 * on your own but allow applications to pass in a verto_ctx you can use.
 *
 * There are two cases where you should use this function.  The first is
 * where you have a need to choose an implementation at run time, usually
 * for testing purposes.  The second and more common is when you simply
 * wish to remain implementation agnostic.  In this later case, you should
 * always call like this: verto_new(NULL, ...).  This lets verto choose the best
 * implementation to use.
 *
 * If impl is not NULL, a new context is returned which is backed by the
 * implementation specified. If the implementation specified is not
 * available or if the required types (reqtypes) are not provided by the
 * named implementation, NULL is returned. The parameter 'impl' can specify:
 *   * The full path to an implementation library
 *   * The name of the implementation library (i.e. - "glib" or "libev")
 *
 * If impl is NULL, verto will attempt to automatically determine the
 * best implementation to use.
 *
 * First, verto will attempt to use an existing, previously loaded
 * implementation. This is handled automatically by internal caching of either
 * the first implementation loaded or the one specified by verto_set_default().
 *
 * Second, verto will attempt to discern if you are already linked to any
 * of the supported implementations (to avoid wasting memory by loading
 * extra unnecessary libraries).  If you are linked to one supported
 * implementation, that implementation will be chosen.  If you are linked
 * to more than one supported implementation one of the ones linked to
 * will be chosen, but the order of the particular choice is undefined.
 *
 * Third, verto will attempt to load the compile-time default, if defined at
 * build time and available at runtime.
 *
 * Last, verto will attempt to load any implementation installed. The specific
 * order of this step is undefined.
 *
 * In all cases above, if the implementation does not support all the specified
 * features (reqtypes), it will be skipped and processing will continue from
 * where it left off. This means that if verto_new() returns non-NULL it is
 * guaranteed to support the features you specified.
 *
 * @see verto_set_default()
 * @param impl The implementation to use, or NULL.
 * @param reqtypes A bitwise or'd list of required event type features.
 * @return A new verto_ctx, or NULL on error.  Call verto_free() when done.
 */
/* *
 * Gets the default event context using an optionally specified implementation.
 *
 * This function is essentially a singleton version of verto_new().  However,
 * since this function must return the same loop as the *_default() call of
 * the underlying implementation (if such a function exists), it is NOT a
 * global singleton, but a per-implementation singleton. For this reason, you
 * must call verto_free() when you are done with this loop. Even after calling
 * verto_free() on the default verto_ctx, you can safely call verto_default()
 * again and receive a new reference to the same (internally default) loop.
 *
 * In all other respects, verto_default() acts exactly like verto_new().
 *
 * @see verto_new()
 * @see verto_free()
 * @param impl The implementation to use, or NULL.
 * @param reqtypes A bitwise or'd list of required event type features.
 * @return The default verto_ctx, or NULL on error.  Call verto_free() when done.
 */
/* *
 * Sets the default implementation to use by its name.
 *
 * This function returns 1 on success and 0 on failure.  It can fail for the
 * following reasons:
 *   1. The default implementation was already set via verto_set_default().
 *   2. The implementation specified could not be found.
 *   3. The implementation specified didn't support the features specified.
 *   4. The impl argument was NULL.
 *   5. verto_new() was already called.
 *   6. verto_default() was already called.
 *   7. verto_new_NAME() was already called.
 *   8. verto_default_NAME() was already called.
 *   9. verto_convert_NAME() was already called.
 *
 * @see verto_new()
 * @see verto_default()
 * @param impl The implementation to use.
 * @param reqtypes A bitwise or'd list of required event type features.
 * @return The default verto_ctx, or NULL on error.  Call verto_free() when done.
 */
/* *
 * Sets the allocator to use for verto_ctx and verto_ev objects.
 *
 * If you plan to set the allocator, you MUST call this function before any
 * other verto_*() calls.
 *
 * @see verto_new()
 * @see verto_default()
 * @see verto_add_io()
 * @see verto_add_timeout()
 * @see verto_add_idle()
 * @see verto_add_signal()
 * @see verto_add_child()
 * @param resize The allocator to use (behaves like realloc();
 *        resize(ptr, 0) must free memory at ptr.)
 * @param hierarchical Zero if the allocator is not hierarchical
 */
/* *
 * Frees a verto_ctx.
 *
 * When called on a default verto_ctx, the reference will be freed but the
 * internal default loop will still be available via another call to
 * verto_default().
 *
 * @see verto_new()
 * @see verto_default()
 * @param ctx The verto_ctx to free.
 */
/* *
 * Frees global state.
 *
 * Remove and free all allocated global state.  Call only when no further
 * contexts exist and all threads have exited.
 *
 * @see verto_new()
 * @see verto_free()
 * @see verto_default()
 */
/* *
 * Run the verto_ctx forever, or at least until verto_break() is called.
 *
 * @see verto_break()
 * @param ctx The verto_ctx to run.
 */
/* *
 * Run the verto_ctx once. May block.
 *
 * @param ctx The verto_ctx to run once.
 */
/* *
 * Exits the currently running verto_ctx.
 *
 * @see verto_run()
 * @param ctx The verto_ctx to exit.
 */
/* *
 * Re-initializes the verto_ctx.
 *
 * This function deletes all events, except those which have set the
 * VERTO_EV_FLAG_REINITIABLE flag. If you fork(), you MUST call this in the
 * child process after the fork!
 *
 * If this function fails it indicates that at least one
 * VERTO_EV_FLAG_REINITIABLE event was not rearmed or that ctx was NULL.
 *
 * @see verto_new()
 * @see verto_default()
 * @param ctx The verto_ctx to re-initialize.
 * @return Non-zero on success, 0 on error.
 */
/* *
 * Adds a callback executed when a file descriptor is ready to be read/written.
 *
 * All verto_ev events are automatically freed when their parent verto_ctx is
 * freed. You do not need to free them manually. If VERTO_EV_FLAG_PERSIST is
 * provided, the event will repeat until verto_del() is called. If
 * VERTO_EV_FLAG_PERSIST is not provided, the event will be freed automatically
 * after its execution. In either case, you may call verto_del() at any time
 * to prevent the event from executing.
 * If VERTO_EV_FLAG_IO_CLOSE_FD is provided the passed in fd is automatically
 * closed when the event is freed with verto_del()
 *
 * NOTE: On Windows, the underlying select() only works with sockets. As such,
 * any attempt to add a non-socket io event on Windows will produce undefined
 * results and may even crash.
 *
 * @see verto_del()
 * @param ctx The verto_ctx which will fire the callback.
 * @param flags The flags to set (at least one VERTO_EV_FLAG_IO* required).
 * @param callback The callback to fire.
 * @param fd The file descriptor to watch for reads.
 * @return The verto_ev registered with the event context or NULL on error.
 */
/* *
 * Adds a callback executed after a period of time.
 *
 * All verto_ev events are automatically freed when their parent verto_ctx is
 * freed. You do not need to free them manually. If VERTO_EV_FLAG_PERSIST is
 * provided, the event will repeat until verto_del() is called. If
 * VERTO_EV_FLAG_PERSIST is not provided, the event will be freed automatically
 * after its execution. In either case, you may call verto_del() at any time
 * to prevent the event from executing.
 *
 * @see verto_del()
 * @param ctx The verto_ctx which will fire the callback.
 * @param flags The flags to set.
 * @param callback The callback to fire.
 * @param interval Time period to wait before firing (in milliseconds).
 * @return The verto_ev registered with the event context.
 */
/* *
 * Adds a callback executed when there is nothing else to do.
 *
 * All verto_ev events are automatically freed when their parent verto_ctx is
 * freed. You do not need to free them manually. If VERTO_EV_FLAG_PERSIST is
 * provided, the event will repeat until verto_del() is called. If
 * VERTO_EV_FLAG_PERSIST is not provided, the event will be freed automatically
 * after its execution. In either case, you may call verto_del() at any time
 * to prevent the event from executing.
 *
 * @see verto_del()
 * @param ctx The verto_ctx which will fire the callback.
 * @param flags The flags to set.
 * @param callback The callback to fire.
 * @return The verto_ev registered with the event context.
 */
/* *
 * Adds a callback executed when a signal is received.
 *
 * All verto_ev events are automatically freed when their parent verto_ctx is
 * freed. You do not need to free them manually. If VERTO_EV_FLAG_PERSIST is
 * provided, the event will repeat until verto_del() is called. If
 * VERTO_EV_FLAG_PERSIST is not provided, the event will be freed automatically
 * after its execution. In either case, you may call verto_del() at any time
 * to prevent the event from executing.
 *
 * NOTE: If you attempt to ignore a signal without the VERTO_EV_FLAG_PERSIST
 * flag, this function fails.
 *
 * NOTE: SIGCHLD is expressly not supported. If you want this notification,
 * please use verto_add_child().
 *
 * WARNNIG: Signal events can only be reliably received in the default verto_ctx
 * in some implementations.  Attempting to receive signal events in non-default
 * loops may result in assert() failures.
 *
 * WARNING: While verto does its best to protect you from crashes, there is
 * essentially no way to do signal events if you mix multiple implementations in
 * a single process. Attempting to do so will result in undefined behavior,
 * and potentially even a crash. You have been warned.
 *
 * @see verto_add_child()
 * @see verto_repeat()
 * @see verto_del()
 * @param ctx The verto_ctx which will fire the callback.
 * @param flags The flags to set.
 * @param callback The callback to fire.
 * @param signal The signal to watch for.
 * @return The verto_ev registered with the event context.
 */
/* *
 * Adds a callback executed when a child process exits.
 *
 * This event will be freed automatically after its execution. Due to the
 * nature of a process' life-cycle, child events cannot persist (processes only
 * exit once). This function returns NULL if you attempt to use
 * VERTO_EV_FLAG_PERSIST. You may, of course, call verto_del() at any time to
 * prevent the callback from firing.
 *
 * @see verto_del()
 * @param ctx The verto_ctx which will fire the callback.
 * @param flags The flags to set.
 * @param callback The callback to fire.
 * @param child The pid (POSIX) or handle (Win32) of the child to watch for.
 * @return The verto_ev registered with the event context.
 */
/* *
 * Sets the private pointer of the verto_ev.
 *
 * The free callback will be called in two cases:
 *   1. When the event is deleted (manually or automatically)
 *   2. When verto_set_private() is called again, unless
 *      free is NULL.
 *
 * @see verto_get_private()
 * @param ev The verto_ev
 * @param priv The private value to store
 * @param free The callback used to free the data or NULL
 */
/* *
 * Gets the private pointer of the verto_ev.
 *
 * @see verto_set_private()
 * @param ev The verto_ev
 * @return The verto_ev private pointer
 */
/* *
 * Gets the type of the verto_ev.
 *
 * @see verto_add_io()
 * @see verto_add_timeout()
 * @see verto_add_idle()
 * @see verto_add_signal()
 * @see verto_add_child()
 * @param ev The verto_ev
 * @return The verto_ev type
 */
/* *
 * Gets the flags associated with the given verto_ev.
 *
 * @see verto_add_io()
 * @see verto_add_timeout()
 * @see verto_add_idle()
 * @see verto_add_signal()
 * @see verto_add_child()
 * @see verto_set_flags()
 * @param ev The verto_ev
 * @return The verto_ev type
 */
/* *
 * Sets the flags associated with the given verto_ev.
 *
 * See _VERTO_EV_FLAG_MUTABLE_MASK for the flags that can be changed
 * with this function. All others will be ignored. If the flags specified
 * are the same as the flags the event already has, this function is a no-op.
 *
 * @see verto_add_io()
 * @see verto_add_timeout()
 * @see verto_add_idle()
 * @see verto_add_signal()
 * @see verto_add_child()
 * @see verto_get_flags()
 * @param ev The verto_ev
 * @param flags The flags for the event
 */
/* *
 * Gets the file descriptor associated with a read/write verto_ev.
 *
 * @see verto_add_io()
 * @param ev The verto_ev to retrieve the file descriptor from.
 * @return The file descriptor, or -1 if not a read/write event.
 */
/* *
 * Gets the file descriptor state from when the event fires.
 *
 * @see verto_add_io()
 * @param ev The verto_ev to retrieve the fd state from.
 * @return The fd state.
 */
/* *
 * Gets the interval associated with a timeout verto_ev.
 *
 * @see verto_add_timeout()
 * @param ev The verto_ev to retrieve the interval from.
 * @return The interval, or 0 if not a timeout event.
 */
/* *
 * Gets the signal associated with a signal verto_ev.
 *
 * @see verto_add_signal()
 * @param ev The verto_ev to retrieve the signal from.
 * @return The signal, or -1 if not a signal event.
 */
/* *
 * Gets the process associated with a child verto_ev.
 *
 * @see verto_add_child()
 * @param ev The verto_ev to retrieve the process from.
 * @return The pid/handle, or 0/NULL if not a child event (POSIX/Win32).
 */
/* *
 * Gets the status of the process which caused this event to fire.
 *
 * @see verto_add_child()
 * @param ev The verto_ev to retrieve the status from.
 * @return The pid/handle status.
 */
#[no_mangle]
#[c2rust::src_loc = "868:1"]
pub unsafe extern "C" fn verto_get_proc_status(mut ev: *const verto_ev)
 -> verto_proc_status {
    return (*ev).option.child.status;
}
/* *
 * Gets the verto_ctx associated with a verto_ev.
 *
 * This is a borrowed reference, don't attempt to free it!
 *
 * @param ev The verto_ev to retrieve the verto_ctx from.
 * @return The verto_ctx.
 */
#[no_mangle]
#[c2rust::src_loc = "874:1"]
pub unsafe extern "C" fn verto_get_ctx(mut ev: *const verto_ev)
 -> *mut verto_ctx {
    return (*ev).ctx;
}
/* *
 * Removes an event from from the event context and frees it.
 *
 * The event and its contents cannot be used after this call.
 *
 * @see verto_add_io()
 * @see verto_add_timeout()
 * @see verto_add_idle()
 * @see verto_add_signal()
 * @see verto_add_child()
 * @param ev The event to delete.
 */
#[no_mangle]
#[c2rust::src_loc = "880:1"]
pub unsafe extern "C" fn verto_del(mut ev: *mut verto_ev) {
    if ev.is_null() { return }
    /* If the event is freed in the callback, we just set a flag so that
     * verto_fire() can actually do the delete when the callback completes.
     *
     * If we don't do this, than verto_fire() will access freed memory. */
    if (*ev).depth > 0 as libc::c_int as libc::c_ulong {
        (*ev).deleted = 1 as libc::c_int;
        return
    }
    if (*ev).onfree.is_some() {
        (*ev).onfree.expect("non-null function pointer")((*ev).ctx, ev);
    }
    (*(*(*(*ev).ctx).module).funcs).ctx_del.expect("non-null function pointer")((*(*ev).ctx).ctx,
                                                                                ev,
                                                                                (*ev).ev);
    remove_ev(&mut (*(*ev).ctx).events, ev);
    if (*ev).type_0 as libc::c_uint ==
           VERTO_EV_TYPE_IO as libc::c_int as libc::c_uint &&
           (*ev).flags as libc::c_uint &
               VERTO_EV_FLAG_IO_CLOSE_FD as libc::c_int as libc::c_uint != 0
           &&
           (*ev).actual as libc::c_uint &
               VERTO_EV_FLAG_IO_CLOSE_FD as libc::c_int as libc::c_uint == 0 {
        close((*ev).option.io.fd);
    }
    vresize(ev as *mut libc::c_void, 0 as libc::c_int as size_t);
}
/* *
 * Returns the event types supported by this implementation.
 *
 * @param ctx The verto_ctx to query.
 * @return The event types supported.
 */
#[no_mangle]
#[c2rust::src_loc = "908:1"]
pub unsafe extern "C" fn verto_get_supported_types(mut ctx: *mut verto_ctx)
 -> verto_ev_type {
    return (*(*ctx).module).types;
}
/* ** THE FOLLOWING ARE FOR IMPLEMENTATION MODULES ONLY ***/
#[no_mangle]
#[c2rust::src_loc = "916:1"]
pub unsafe extern "C" fn verto_convert_module(mut module: *const verto_module,
                                              mut deflt: libc::c_int,
                                              mut mctx: *mut libc::c_void)
 -> *mut verto_ctx {
    let mut current_block: u64; /* TODO: create an error callback */
    let mut ctx: *mut verto_ctx = 0 as *mut verto_ctx;
    let mut mr: *mut module_record = 0 as *mut module_record;
    if module.is_null() { return 0 as *mut verto_ctx }
    if deflt != 0 {
        mr = loaded_modules;
        while !mr.is_null() {
            let mut tmp: *mut verto_ctx = 0 as *mut verto_ctx;
            if (*mr).module == module && !(*mr).defctx.is_null() {
                if !mctx.is_null() {
                    (*(*module).funcs).ctx_free.expect("non-null function pointer")(mctx);
                }
                tmp = (*mr).defctx;
                (*tmp).ref_0 = (*tmp).ref_0.wrapping_add(1);
                return tmp
            }
            mr = (*mr).next
        }
    }
    if mctx.is_null() {
        mctx =
            if deflt != 0 {
                if (*(*module).funcs).ctx_default.is_some() {
                    ::std::mem::transmute::<_,
                                            fn()
                                                ->
                                                    *mut libc::c_void>((*(*module).funcs).ctx_default.expect("non-null function pointer"))()
                } else {
                    ::std::mem::transmute::<_,
                                            fn()
                                                ->
                                                    *mut libc::c_void>((*(*module).funcs).ctx_new.expect("non-null function pointer"))()
                }
            } else {
                ::std::mem::transmute::<_,
                                        fn()
                                            ->
                                                *mut libc::c_void>((*(*module).funcs).ctx_new.expect("non-null function pointer"))()
            };
        if mctx.is_null() {
            current_block = 16582129147921022469;
        } else { current_block = 4808432441040389987; }
    } else { current_block = 4808432441040389987; }
    match current_block {
        4808432441040389987 => {
            ctx =
                vresize(0 as *mut libc::c_void,
                        ::std::mem::size_of::<verto_ctx>() as libc::c_ulong)
                    as *mut verto_ctx;
            if !ctx.is_null() {
                memset(ctx as *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<verto_ctx>() as libc::c_ulong);
                (*ctx).ref_0 = 1 as libc::c_int as size_t;
                (*ctx).ctx = mctx;
                (*ctx).module = module;
                (*ctx).deflt = deflt;
                if deflt != 0 {
                    let mut tmp_0: *mut *mut module_record =
                        0 as *mut *mut module_record;
                    tmp_0 = &mut loaded_modules;
                    mr = loaded_modules;
                    while !mr.is_null() {
                        if (*mr).module == module {
                            if (*mr).defctx.is_null() {
                            } else {
                                __assert_fail(b"mr->defctx == NULL\x00" as
                                                  *const u8 as
                                                  *const libc::c_char,
                                              b"verto.c\x00" as *const u8 as
                                                  *const libc::c_char,
                                              968 as libc::c_int as
                                                  libc::c_uint,
                                              (*::std::mem::transmute::<&[u8; 76],
                                                                        &[libc::c_char; 76]>(b"verto_ctx *verto_convert_module(const verto_module *, int, verto_mod_ctx *)\x00")).as_ptr());
                            }
                            (*mr).defctx = ctx;
                            return ctx
                        }
                        if (*mr).next.is_null() {
                            tmp_0 = &mut (*mr).next;
                            break ;
                        } else { mr = (*mr).next }
                    }
                    *tmp_0 =
                        vresize(0 as *mut libc::c_void,
                                ::std::mem::size_of::<module_record>() as
                                    libc::c_ulong) as *mut module_record;
                    if (*tmp_0).is_null() {
                        vresize(ctx as *mut libc::c_void,
                                0 as libc::c_int as size_t);
                        current_block = 16582129147921022469;
                    } else {
                        memset(*tmp_0 as *mut libc::c_void, 0 as libc::c_int,
                               ::std::mem::size_of::<module_record>() as
                                   libc::c_ulong);
                        (**tmp_0).defctx = ctx;
                        (**tmp_0).module = module;
                        current_block = 6174974146017752131;
                    }
                } else { current_block = 6174974146017752131; }
                match current_block {
                    16582129147921022469 => { }
                    _ => { return ctx }
                }
            }
        }
        _ => { }
    }
    if !mctx.is_null() {
        (*(*module).funcs).ctx_free.expect("non-null function pointer")(mctx);
    }
    return 0 as *mut verto_ctx;
}
#[no_mangle]
#[c2rust::src_loc = "1000:1"]
pub unsafe extern "C" fn verto_fire(mut ev: *mut verto_ev) {
    let mut priv_0: *mut libc::c_void = 0 as *mut libc::c_void;
    (*ev).depth = (*ev).depth.wrapping_add(1);
    (*ev).callback.expect("non-null function pointer")((*ev).ctx, ev);
    (*ev).depth = (*ev).depth.wrapping_sub(1);
    if (*ev).depth == 0 as libc::c_int as libc::c_ulong {
        if (*ev).flags as libc::c_uint &
               VERTO_EV_FLAG_PERSIST as libc::c_int as libc::c_uint == 0 ||
               (*ev).deleted != 0 {
            verto_del(ev);
        } else {
            if (*ev).actual as libc::c_uint &
                   VERTO_EV_FLAG_PERSIST as libc::c_int as libc::c_uint == 0 {
                (*ev).actual =
                    ((*ev).flags as libc::c_uint &
                         !(VERTO_EV_FLAG_PERSIST as libc::c_int |
                               VERTO_EV_FLAG_IO_CLOSE_FD as libc::c_int) as
                             libc::c_uint) as verto_ev_flag;
                priv_0 =
                    (*(*(*(*ev).ctx).module).funcs).ctx_add.expect("non-null function pointer")((*(*ev).ctx).ctx,
                                                                                                ev,
                                                                                                &mut (*ev).actual);
                if !priv_0.is_null() {
                } else {
                    __assert_fail(b"priv\x00" as *const u8 as
                                      *const libc::c_char,
                                  b"verto.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  1016 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 28],
                                                            &[libc::c_char; 28]>(b"void verto_fire(verto_ev *)\x00")).as_ptr());
                }
                (*(*(*(*ev).ctx).module).funcs).ctx_del.expect("non-null function pointer")((*(*ev).ctx).ctx,
                                                                                            ev,
                                                                                            (*ev).ev);
                (*ev).ev = priv_0
            }
            if (*ev).type_0 as libc::c_uint ==
                   VERTO_EV_TYPE_IO as libc::c_int as libc::c_uint {
                (*ev).option.io.state = VERTO_EV_FLAG_NONE
            }
            if (*ev).type_0 as libc::c_uint ==
                   VERTO_EV_TYPE_CHILD as libc::c_int as libc::c_uint {
                (*ev).option.child.status = 0 as libc::c_int
            }
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "1029:1"]
pub unsafe extern "C" fn verto_set_proc_status(mut ev: *mut verto_ev,
                                               mut status:
                                                   verto_proc_status) {
    if !ev.is_null() &&
           (*ev).type_0 as libc::c_uint ==
               VERTO_EV_TYPE_CHILD as libc::c_int as libc::c_uint {
        (*ev).option.child.status = status
    };
}
/* *
 * Converts an existing implementation specific loop to a verto_ctx.
 *
 * This function also sets the internal default implementation so that future
 * calls to verto_new(NULL) or verto_default(NULL) will use this specific
 * implementation if it was not already set.
 *
 * @param name The name of the module (unquoted)
 * @param deflt Whether the ctx is the default context or not
 * @param ctx The context to store
 * @return A new verto_ctx, or NULL on error. Call verto_free() when done.
 */
/* *
 * Converts an existing implementation specific loop to a verto_ctx.
 *
 * If you are a module implementation, you probably want the macro above.  This
 * function is generally used directly only when an application is attempting
 * to expose a home-grown event loop to verto.
 *
 * If deflt is non-zero and a default ctx was already defined for this module
 * and ctx is not NULL, than ctx will be free'd and the previously defined
 * default will be returned.
 *
 * If ctx is non-NULL, than the pre-existing verto_mod_ctx will be converted to
 * to a verto_ctx; if deflt is non-zero than this verto_mod_ctx will also be
 * marked as the default loop for this process. If ctx is NULL, than the
 * appropriate constructor will be called: either module->ctx_new() or
 * module->ctx_default() depending on the boolean value of deflt. If
 * module->ctx_default is NULL and deflt is non-zero, than module->ctx_new()
 * will be called and the resulting verto_mod_ctx will be utilized as the
 * default.
 *
 * This function also sets the internal default implementation so that future
 * calls to verto_new(NULL) or verto_default(NULL) will use this specific
 * implementation if it was not already set.
 *
 * @param name The name of the module (unquoted)
 * @param ctx The context private to store
 * @return A new verto_ctx, or NULL on error. Call verto_free() when done.
 */
/* *
 * Calls the callback of the verto_ev and then frees it via verto_del().
 *
 * The verto_ev is not freed (verto_del() is not called) if it is a signal event.
 *
 * @see verto_add_read()
 * @see verto_add_write()
 * @see verto_add_timeout()
 * @see verto_add_idle()
 * @see verto_add_signal()
 * @see verto_add_child()
 * @see verto_del()
 * @param ev The verto_ev
 */
/* *
 * Sets the status of the pid/handle which caused this event to fire.
 *
 * This function does nothing if the verto_ev is not a child type.
 *
 * @see verto_add_child()
 * @param ev The verto_ev to set the status in.
 * @param status The pid/handle status.
 */
/* *
 * Sets the state of the fd which caused this event to fire.
 *
 * This function does nothing if the verto_ev is not a io type.
 *
 * Only the flags VERTO_EV_FLAG_IO_(READ|WRITE|ERROR) are supported. All other
 * flags are unset.
 *
 * @see verto_add_io()
 * @param ev The verto_ev to set the state in.
 * @param state The fd state.
 */
#[no_mangle]
#[c2rust::src_loc = "1036:1"]
pub unsafe extern "C" fn verto_set_fd_state(mut ev: *mut verto_ev,
                                            mut state: verto_ev_flag) {
    /* Filter out only the io flags */
    state =
        (state as libc::c_uint &
             (VERTO_EV_FLAG_IO_READ as libc::c_int |
                  VERTO_EV_FLAG_IO_WRITE as libc::c_int |
                  VERTO_EV_FLAG_IO_ERROR as libc::c_int) as libc::c_uint) as
            verto_ev_flag;
    /* Don't report read/write if the socket is closed */
    if state as libc::c_uint &
           VERTO_EV_FLAG_IO_ERROR as libc::c_int as libc::c_uint != 0 {
        state = VERTO_EV_FLAG_IO_ERROR
    }
    if !ev.is_null() &&
           (*ev).type_0 as libc::c_uint ==
               VERTO_EV_TYPE_IO as libc::c_int as libc::c_uint {
        (*ev).option.io.state = state
    };
}
