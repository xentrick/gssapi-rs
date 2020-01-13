use ::libc;
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:32"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = libc::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = libc::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = libc::c_uint;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = libc::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = libc::c_long;
    #[c2rust::src_loc = "154:1"]
    pub type __pid_t = libc::c_int;
    #[c2rust::src_loc = "156:1"]
    pub type __clock_t = libc::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = libc::c_long;
    #[c2rust::src_loc = "162:1"]
    pub type __suseconds_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "196:1"]
    pub type __syscall_slong_t = libc::c_long;
    #[c2rust::src_loc = "214:1"]
    pub type __sig_atomic_t = libc::c_int;
}
#[c2rust::header_src = "/usr/include/sys/types.h:32"]
pub mod sys_types_h {
    #[c2rust::src_loc = "97:1"]
    pub type pid_t = __pid_t;
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    use super::types_h::{__pid_t, __ssize_t};
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:32"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/include/bits/types/__sigset_t.h:32"]
pub mod __sigset_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "5:9"]
    pub struct __sigset_t {
        pub __val: [libc::c_ulong; 16],
    }
}
#[c2rust::header_src = "/usr/include/bits/types/sigset_t.h:32"]
pub mod sigset_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type sigset_t = __sigset_t;
    use super::__sigset_t_h::__sigset_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:32"]
pub mod struct_timeval_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8:8"]
    pub struct timeval {
        pub tv_sec: __time_t,
        pub tv_usec: __suseconds_t,
    }
    use super::types_h::{__time_t, __suseconds_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:32"]
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
#[c2rust::header_src = "/usr/include/sys/select.h:32"]
pub mod select_h {
    #[c2rust::src_loc = "49:1"]
    pub type __fd_mask = libc::c_long;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "59:9"]
    pub struct fd_set {
        pub fds_bits: [__fd_mask; 16],
    }
    #[c2rust::src_loc = "77:1"]
    pub type fd_mask = __fd_mask;
    use super::struct_timeval_h::timeval;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "101:1"]
        pub fn select(__nfds: libc::c_int, __readfds: *mut fd_set,
                      __writefds: *mut fd_set, __exceptfds: *mut fd_set,
                      __timeout: *mut timeval) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/__sigval_t.h:67"]
pub mod __sigval_t_h {
    #[c2rust::src_loc = "30:1"]
    pub type __sigval_t = sigval;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:7"]
    pub union sigval {
        pub sival_int: libc::c_int,
        pub sival_ptr: *mut libc::c_void,
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/verto.h:36"]
pub mod verto_h {
    #[c2rust::src_loc = "37:1"]
    pub type verto_proc = pid_t;
    #[c2rust::src_loc = "38:1"]
    pub type verto_proc_status = libc::c_int;
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
    use super::sys_types_h::pid_t;
    use super::time_t_h::time_t;
    extern "C" {
        #[c2rust::src_loc = "48:16"]
        pub type verto_ctx;
        #[c2rust::src_loc = "49:16"]
        pub type verto_ev;
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
        #[no_mangle]
        #[c2rust::src_loc = "440:1"]
        pub fn verto_get_type(ev: *const verto_ev) -> verto_ev_type;
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
        #[no_mangle]
        #[c2rust::src_loc = "455:1"]
        pub fn verto_get_flags(ev: *const verto_ev) -> verto_ev_flag;
        /* *
 * Gets the file descriptor associated with a read/write verto_ev.
 *
 * @see verto_add_io()
 * @param ev The verto_ev to retrieve the file descriptor from.
 * @return The file descriptor, or -1 if not a read/write event.
 */
        #[no_mangle]
        #[c2rust::src_loc = "484:1"]
        pub fn verto_get_fd(ev: *const verto_ev) -> libc::c_int;
        /* *
 * Gets the interval associated with a timeout verto_ev.
 *
 * @see verto_add_timeout()
 * @param ev The verto_ev to retrieve the interval from.
 * @return The interval, or 0 if not a timeout event.
 */
        #[no_mangle]
        #[c2rust::src_loc = "504:1"]
        pub fn verto_get_interval(ev: *const verto_ev) -> time_t;
        /* *
 * Gets the signal associated with a signal verto_ev.
 *
 * @see verto_add_signal()
 * @param ev The verto_ev to retrieve the signal from.
 * @return The signal, or -1 if not a signal event.
 */
        #[no_mangle]
        #[c2rust::src_loc = "514:1"]
        pub fn verto_get_signal(ev: *const verto_ev) -> libc::c_int;
        /* *
 * Gets the process associated with a child verto_ev.
 *
 * @see verto_add_child()
 * @param ev The verto_ev to retrieve the process from.
 * @return The pid/handle, or 0/NULL if not a child event (POSIX/Win32).
 */
        #[no_mangle]
        #[c2rust::src_loc = "524:1"]
        pub fn verto_get_proc(ev: *const verto_ev) -> verto_proc;
    }
    /* VERTO_H_ */
    /* __cplusplus */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/verto/verto-module.h:37"]
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
    #[c2rust::src_loc = "34:1"]
    pub type verto_mod_ctx = ();
    #[c2rust::src_loc = "35:1"]
    pub type verto_mod_ev = ();
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
    use super::verto_h::{verto_ev, verto_ev_flag, verto_ev_type, verto_ctx,
                         verto_proc_status, VERTO_EV_FLAG_NONE};
    extern "C" {
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
        #[no_mangle]
        #[c2rust::src_loc = "141:1"]
        pub fn verto_convert_module(module: *const verto_module,
                                    deflt: libc::c_int,
                                    ctx: *mut libc::c_void) -> *mut verto_ctx;
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
        #[no_mangle]
        #[c2rust::src_loc = "158:1"]
        pub fn verto_fire(ev: *mut verto_ev);
        /* *
 * Sets the status of the pid/handle which caused this event to fire.
 *
 * This function does nothing if the verto_ev is not a child type.
 *
 * @see verto_add_child()
 * @param ev The verto_ev to set the status in.
 * @param status The pid/handle status.
 */
        #[no_mangle]
        #[c2rust::src_loc = "170:1"]
        pub fn verto_set_proc_status(ev: *mut verto_ev,
                                     status: verto_proc_status);
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
        #[c2rust::src_loc = "185:1"]
        pub fn verto_set_fd_state(ev: *mut verto_ev, state: verto_ev_flag);
    }
    /* VERTO_MODULE_H_ */
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:67"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:67"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/types/sig_atomic_t.h:67"]
pub mod sig_atomic_t_h {
    #[c2rust::src_loc = "8:1"]
    pub type sig_atomic_t = __sig_atomic_t;
    use super::types_h::__sig_atomic_t;
}
#[c2rust::header_src = "/usr/include/bits/types/siginfo_t.h:67"]
pub mod siginfo_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:9"]
    pub struct siginfo_t {
        pub si_signo: libc::c_int,
        pub si_errno: libc::c_int,
        pub si_code: libc::c_int,
        pub __pad0: libc::c_int,
        pub _sifields: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "51:5"]
    pub union C2RustUnnamed {
        pub _pad: [libc::c_int; 28],
        pub _kill: C2RustUnnamed_8,
        pub _timer: C2RustUnnamed_7,
        pub _rt: C2RustUnnamed_6,
        pub _sigchld: C2RustUnnamed_5,
        pub _sigfault: C2RustUnnamed_2,
        pub _sigpoll: C2RustUnnamed_1,
        pub _sigsys: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "116:2"]
    pub struct C2RustUnnamed_0 {
        pub _call_addr: *mut libc::c_void,
        pub _syscall: libc::c_int,
        pub _arch: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "108:2"]
    pub struct C2RustUnnamed_1 {
        pub si_band: libc::c_long,
        pub si_fd: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "89:2"]
    pub struct C2RustUnnamed_2 {
        pub si_addr: *mut libc::c_void,
        pub si_addr_lsb: libc::c_short,
        pub _bounds: C2RustUnnamed_3,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "94:6"]
    pub union C2RustUnnamed_3 {
        pub _addr_bnd: C2RustUnnamed_4,
        pub _pkey: __uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "97:3"]
    pub struct C2RustUnnamed_4 {
        pub _lower: *mut libc::c_void,
        pub _upper: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "79:2"]
    pub struct C2RustUnnamed_5 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_status: libc::c_int,
        pub si_utime: __clock_t,
        pub si_stime: __clock_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "71:2"]
    pub struct C2RustUnnamed_6 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_sigval: __sigval_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:2"]
    pub struct C2RustUnnamed_7 {
        pub si_tid: libc::c_int,
        pub si_overrun: libc::c_int,
        pub si_sigval: __sigval_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:2"]
    pub struct C2RustUnnamed_8 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
    }
    use super::types_h::{__uint32_t, __pid_t, __uid_t, __clock_t};
    use super::__sigval_t_h::__sigval_t;
}
#[c2rust::header_src = "/usr/include/signal.h:67"]
pub mod signal_h {
    #[c2rust::src_loc = "72:1"]
    pub type __sighandler_t
        =
        Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
    use super::sigset_t_h::sigset_t;
    use super::sigaction_h::sigaction;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn signal(__sig: libc::c_int, __handler: __sighandler_t)
         -> __sighandler_t;
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "199:1"]
        pub fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "202:1"]
        pub fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "205:1"]
        pub fn sigdelset(__set: *mut sigset_t, __signo: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "229:1"]
        pub fn sigprocmask(__how: libc::c_int, __set: *const sigset_t,
                           __oset: *mut sigset_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "240:1"]
        pub fn sigaction(__sig: libc::c_int, __act: *const sigaction,
                         __oact: *mut sigaction) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/sigaction.h:67"]
pub mod sigaction_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:8"]
    pub struct sigaction {
        pub __sigaction_handler: C2RustUnnamed_9,
        pub sa_mask: __sigset_t,
        pub sa_flags: libc::c_int,
        pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:5"]
    pub union C2RustUnnamed_9 {
        pub sa_handler: __sighandler_t,
        pub sa_sigaction: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *mut siginfo_t,
                                                      _: *mut libc::c_void)
                                     -> ()>,
    }
    use super::__sigset_t_h::__sigset_t;
    use super::signal_h::__sighandler_t;
    use super::siginfo_t_h::siginfo_t;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/verto/ev.h:67"]
pub mod ev_h {
    #[c2rust::src_loc = "153:1"]
    pub type ev_tstamp = libc::c_double;
    #[c2rust::src_loc = "667:1"]
    pub type ev_loop_callback
        =
        Option<unsafe extern "C" fn(_: *mut ev_loop) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "317:16"]
    pub struct ev_io {
        pub active: libc::c_int,
        pub pending: libc::c_int,
        pub priority: libc::c_int,
        pub data: *mut libc::c_void,
        pub cb: Option<unsafe extern "C" fn(_: *mut ev_loop, _: *mut ev_io,
                                            _: libc::c_int) -> ()>,
        pub next: *mut ev_watcher_list,
        pub fd: libc::c_int,
        pub events: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "304:16"]
    pub struct ev_watcher_list {
        pub active: libc::c_int,
        pub pending: libc::c_int,
        pub priority: libc::c_int,
        pub data: *mut libc::c_void,
        pub cb: Option<unsafe extern "C" fn(_: *mut ev_loop,
                                            _: *mut ev_watcher_list,
                                            _: libc::c_int) -> ()>,
        pub next: *mut ev_watcher_list,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "410:16"]
    pub struct ev_check {
        pub active: libc::c_int,
        pub pending: libc::c_int,
        pub priority: libc::c_int,
        pub data: *mut libc::c_void,
        pub cb: Option<unsafe extern "C" fn(_: *mut ev_loop, _: *mut ev_check,
                                            _: libc::c_int) -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "403:16"]
    pub struct ev_prepare {
        pub active: libc::c_int,
        pub pending: libc::c_int,
        pub priority: libc::c_int,
        pub data: *mut libc::c_void,
        pub cb: Option<unsafe extern "C" fn(_: *mut ev_loop,
                                            _: *mut ev_prepare,
                                            _: libc::c_int) -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "394:16"]
    pub struct ev_idle {
        pub active: libc::c_int,
        pub pending: libc::c_int,
        pub priority: libc::c_int,
        pub data: *mut libc::c_void,
        pub cb: Option<unsafe extern "C" fn(_: *mut ev_loop, _: *mut ev_idle,
                                            _: libc::c_int) -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "310:16"]
    pub struct ev_watcher_time {
        pub active: libc::c_int,
        pub pending: libc::c_int,
        pub priority: libc::c_int,
        pub data: *mut libc::c_void,
        pub cb: Option<unsafe extern "C" fn(_: *mut ev_loop,
                                            _: *mut ev_watcher_time,
                                            _: libc::c_int) -> ()>,
        pub at: ev_tstamp,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "298:16"]
    pub struct ev_watcher {
        pub active: libc::c_int,
        pub pending: libc::c_int,
        pub priority: libc::c_int,
        pub data: *mut libc::c_void,
        pub cb: Option<unsafe extern "C" fn(_: *mut ev_loop,
                                            _: *mut ev_watcher,
                                            _: libc::c_int) -> ()>,
    }
    #[c2rust::src_loc = "217:1"]
    pub type C2RustUnnamed_10 = libc::c_int;
    #[c2rust::src_loc = "240:3"]
    pub const EV_ERROR: C2RustUnnamed_10 = -2147483648;
    #[c2rust::src_loc = "239:3"]
    pub const EV_CUSTOM: C2RustUnnamed_10 = 16777216;
    #[c2rust::src_loc = "238:3"]
    pub const EV_ASYNC: C2RustUnnamed_10 = 524288;
    #[c2rust::src_loc = "237:3"]
    pub const EV_CLEANUP: C2RustUnnamed_10 = 262144;
    #[c2rust::src_loc = "236:3"]
    pub const EV_FORK: C2RustUnnamed_10 = 131072;
    #[c2rust::src_loc = "235:3"]
    pub const EV_EMBED: C2RustUnnamed_10 = 65536;
    #[c2rust::src_loc = "234:3"]
    pub const EV_CHECK: C2RustUnnamed_10 = 32768;
    #[c2rust::src_loc = "233:3"]
    pub const EV_PREPARE: C2RustUnnamed_10 = 16384;
    #[c2rust::src_loc = "232:3"]
    pub const EV_IDLE: C2RustUnnamed_10 = 8192;
    #[c2rust::src_loc = "231:3"]
    pub const EV_STAT: C2RustUnnamed_10 = 4096;
    #[c2rust::src_loc = "230:3"]
    pub const EV_CHILD: C2RustUnnamed_10 = 2048;
    #[c2rust::src_loc = "229:3"]
    pub const EV_SIGNAL: C2RustUnnamed_10 = 1024;
    #[c2rust::src_loc = "228:3"]
    pub const EV_PERIODIC: C2RustUnnamed_10 = 512;
    #[c2rust::src_loc = "226:3"]
    pub const EV_TIMEOUT: C2RustUnnamed_10 = 256;
    #[c2rust::src_loc = "224:3"]
    pub const EV_TIMER: C2RustUnnamed_10 = 256;
    #[c2rust::src_loc = "223:3"]
    pub const EV_IO: C2RustUnnamed_10 = 1;
    #[c2rust::src_loc = "222:3"]
    pub const EV__IOFDSET: C2RustUnnamed_10 = 128;
    #[c2rust::src_loc = "221:3"]
    pub const EV_WRITE: C2RustUnnamed_10 = 2;
    #[c2rust::src_loc = "220:3"]
    pub const EV_READ: C2RustUnnamed_10 = 1;
    #[c2rust::src_loc = "219:3"]
    pub const EV_NONE: C2RustUnnamed_10 = 0;
    #[c2rust::src_loc = "218:3"]
    pub const EV_UNDEF: C2RustUnnamed_10 = -1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "327:16"]
    pub struct ev_timer {
        pub active: libc::c_int,
        pub pending: libc::c_int,
        pub priority: libc::c_int,
        pub data: *mut libc::c_void,
        pub cb: Option<unsafe extern "C" fn(_: *mut ev_loop, _: *mut ev_timer,
                                            _: libc::c_int) -> ()>,
        pub at: ev_tstamp,
        pub repeat: ev_tstamp,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "347:16"]
    pub struct ev_signal {
        pub active: libc::c_int,
        pub pending: libc::c_int,
        pub priority: libc::c_int,
        pub data: *mut libc::c_void,
        pub cb: Option<unsafe extern "C" fn(_: *mut ev_loop,
                                            _: *mut ev_signal, _: libc::c_int)
                           -> ()>,
        pub next: *mut ev_watcher_list,
        pub signum: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "357:16"]
    pub struct ev_child {
        pub active: libc::c_int,
        pub pending: libc::c_int,
        pub priority: libc::c_int,
        pub data: *mut libc::c_void,
        pub cb: Option<unsafe extern "C" fn(_: *mut ev_loop, _: *mut ev_child,
                                            _: libc::c_int) -> ()>,
        pub next: *mut ev_watcher_list,
        pub flags: libc::c_int,
        pub pid: libc::c_int,
        pub rpid: libc::c_int,
        pub rstatus: libc::c_int,
    }
    #[c2rust::src_loc = "501:1"]
    pub type C2RustUnnamed_11 = libc::c_uint;
    #[c2rust::src_loc = "513:3"]
    pub const EVFLAG_NOSIGMASK: C2RustUnnamed_11 = 4194304;
    #[c2rust::src_loc = "512:3"]
    pub const EVFLAG_SIGNALFD: C2RustUnnamed_11 = 2097152;
    #[c2rust::src_loc = "510:3"]
    pub const EVFLAG_NOSIGFD: C2RustUnnamed_11 = 0;
    #[c2rust::src_loc = "508:3"]
    pub const EVFLAG_NOINOTIFY: C2RustUnnamed_11 = 1048576;
    #[c2rust::src_loc = "506:3"]
    pub const EVFLAG_FORKCHECK: C2RustUnnamed_11 = 33554432;
    #[c2rust::src_loc = "505:3"]
    pub const EVFLAG_NOENV: C2RustUnnamed_11 = 16777216;
    #[c2rust::src_loc = "503:3"]
    pub const EVFLAG_AUTO: C2RustUnnamed_11 = 0;
    #[c2rust::src_loc = "517:1"]
    pub type C2RustUnnamed_12 = libc::c_uint;
    #[c2rust::src_loc = "525:3"]
    pub const EVBACKEND_MASK: C2RustUnnamed_12 = 65535;
    #[c2rust::src_loc = "524:3"]
    pub const EVBACKEND_ALL: C2RustUnnamed_12 = 63;
    #[c2rust::src_loc = "523:3"]
    pub const EVBACKEND_PORT: C2RustUnnamed_12 = 32;
    #[c2rust::src_loc = "522:3"]
    pub const EVBACKEND_DEVPOLL: C2RustUnnamed_12 = 16;
    #[c2rust::src_loc = "521:3"]
    pub const EVBACKEND_KQUEUE: C2RustUnnamed_12 = 8;
    #[c2rust::src_loc = "520:3"]
    pub const EVBACKEND_EPOLL: C2RustUnnamed_12 = 4;
    #[c2rust::src_loc = "519:3"]
    pub const EVBACKEND_POLL: C2RustUnnamed_12 = 2;
    #[c2rust::src_loc = "518:3"]
    pub const EVBACKEND_SELECT: C2RustUnnamed_12 = 1;
    #[c2rust::src_loc = "626:1"]
    pub type C2RustUnnamed_13 = libc::c_uint;
    #[c2rust::src_loc = "628:3"]
    pub const EVRUN_ONCE: C2RustUnnamed_13 = 2;
    #[c2rust::src_loc = "627:3"]
    pub const EVRUN_NOWAIT: C2RustUnnamed_13 = 1;
    #[c2rust::src_loc = "632:1"]
    pub type C2RustUnnamed_14 = libc::c_uint;
    #[c2rust::src_loc = "635:3"]
    pub const EVBREAK_ALL: C2RustUnnamed_14 = 2;
    #[c2rust::src_loc = "634:3"]
    pub const EVBREAK_ONE: C2RustUnnamed_14 = 1;
    #[c2rust::src_loc = "633:3"]
    pub const EVBREAK_CANCEL: C2RustUnnamed_14 = 0;
    #[inline]
    #[c2rust::src_loc = "571:1"]
    pub unsafe extern "C" fn ev_is_default_loop(mut loop_0: *mut ev_loop)
     -> libc::c_int {
        return (loop_0 == ev_default_loop_uc_()) as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "563:1"]
    pub unsafe extern "C" fn ev_default_loop_uc_() -> *mut ev_loop {
        return k5ev_default_loop_ptr;
    }
    use super::ev_c::{ev_loop, k5ev_default_loop_ptr};
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/verto/ev.c:67"]
pub mod ev_c {
    /*
 * libev event processing core, watcher management
 *
 * Copyright (c) 2007,2008,2009,2010,2011,2012,2013 Marc Alexander Lehmann <libev@schmorp.de>
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without modifica-
 * tion, are permitted provided that the following conditions are met:
 *
 *   1.  Redistributions of source code must retain the above copyright notice,
 *       this list of conditions and the following disclaimer.
 *
 *   2.  Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MER-
 * CHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO
 * EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPE-
 * CIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
 * OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
 * WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTH-
 * ERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * Alternatively, the contents of this file may be used under the terms of
 * the GNU General Public License ("GPL") version 2 or any later version,
 * in which case the provisions of the GPL are applicable instead of
 * the above. If you wish to allow the use of your version of this file
 * only under the terms of the GPL and not to allow others to use your
 * version of this file under the BSD license, indicate your decision
 * by deleting the provisions above and replace them with the notice
 * and other provisions required by the GPL. If you do not delete the
 * provisions above, a recipient may use your version of this file under
 * either the BSD or the GPL.
 */
    /* this big block deduces configuration from config.h */
    /* OS X, in its infinite idiocy, actually HARDCODES
 * a limit of 1024 into their select. Where people have brains,
 * OS X engineers apparently have a vacuum. Or maybe they were
 * ordered to have a vacuum, or they do anything for money.
 * This might help. Or not.
 */
    /* this block tries to deduce configuration from header-defined symbols and defaults */
    /* try to deduce the maximum number of signals on this platform */
    /* debugging */
    /* aix's poll.h seems to cause lots of trouble */
    /* on linux, we can use a (slow) syscall to avoid a dependency on pthread, */
/* which makes programs even slower. might work on other unices, too. */
    /* this block fixes any misconfiguration where we know we run into trouble otherwise */
    /* our minimum requirement is glibc 2.7 which has the stub, but not the header */
    /* our minimum requirement is glibc 2.7 which has the stub, but not the header */
    /* */
    /*
 * This is used to work around floating point rounding problems.
 * This value is good at least till the year 4000.
 */
    /* 1/2**13, good till 4000 */
    /*#define MIN_INTERVAL  0.00000095367431640625 /* 1/2**20, good till 2200 */
    /* minimum timejump that gets detected (if monotonic clock available) */
    /* never wait longer than this time (to detect time jumps) */
    /* the following is ecb.h embedded into libev - use update_ev_c to update from an external copy */
/* ECB.H BEGIN */
/*
 * libecb - http://software.schmorp.de/pkg/libecb
 *
 * Copyright (©) 2009-2015 Marc Alexander Lehmann <libecb@schmorp.de>
 * Copyright (©) 2011 Emanuele Giaquinta
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without modifica-
 * tion, are permitted provided that the following conditions are met:
 *
 *   1.  Redistributions of source code must retain the above copyright notice,
 *       this list of conditions and the following disclaimer.
 *
 *   2.  Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MER-
 * CHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO
 * EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPE-
 * CIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
 * OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
 * WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTH-
 * ERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * Alternatively, the contents of this file may be used under the terms of
 * the GNU General Public License ("GPL") version 2 or any later version,
 * in which case the provisions of the GPL are applicable instead of
 * the above. If you wish to allow the use of your version of this file
 * only under the terms of the GPL and not to allow others to use your
 * version of this file under the BSD license, indicate your decision
 * by deleting the provisions above and replace them with the notice
 * and other provisions required by the GPL. If you do not delete the
 * provisions above, a recipient may use your version of this file under
 * either the BSD or the GPL.
 */
    /* 16 bits major, 16 bits minor */
    /* work around x32 idiocy by defining proper macros */
    /* many compilers define _GNUC_ to some versions but then only implement
 * what their idiot authors think are the "more important" extensions,
 * causing enormous grief in return for some better fake benchmark numbers.
 * or so.
 * we try to detect these and simply assume they are not gcc - if they have
 * an issue with that they should have done it right in the first place.
 */
    /* ****************************************************************************/
    /* ECB_NO_THREADS - ecb is not used by multiple threads, ever */
/* ECB_NO_SMP     - ecb might be used in multiple threads, but only on a single cpu */
    /* http://www-01.ibm.com/support/knowledgecenter/SSGH3R_13.1.0/com.ibm.xlcpp131.aix.doc/compiler_ref/compiler_builtins.html */
    /* ****************************************************************************/
    /* no emulation for ecb_decltype */
    /* http://www-01.ibm.com/support/knowledgecenter/SSGH3R_13.1.0/com.ibm.xlcpp131.aix.doc/language_ref/noreturn.html */
    /* put around conditional expressions if you are very sure that the  */
/* expression is mostly true or mostly false. note that these return */
/* booleans, not the expression.                                     */
    /* for compatibility to the rest of the world */
    /* count trailing zero bits and count # of one bits */
    /* we assume int == 32 bit, long == 32 or 64 bit and long long == 64 bit */
    /* no popcountll */
    /* popcount64 is only available on 64 bit cpus as gcc builtin */
/* so for this version we are lazy */
    /* try to tell the compiler that some condition is definitely true */
    /* the union code still generates code under pressure in gcc, */
  /* but less than using pointers, and always seems to */
  /* successfully return a constant. */
  /* the reason why we have this horrible preprocessor mess */
  /* is to avoid it in all cases, at least on common architectures */
  /* or when using a recent enough gcc version (>= 4.6) */
    /* infinity or NaN */
    /* zero, handled by code below by forcing e to 0 */
    /* subnormal, renormalise */
    /* mask implicit bit */
    /* e and m now are normalised, or zero, (or inf or nan) */
    /* sign bit, the easy part */
    /* the desired exponent */
    /* if it's within range of binary16 normals, use fast path */
    /* mantissa round-to-even */
    /* handle overflow */
    /* handle large numbers and infinity */
    /* handle zero, subnormals and small numbers */
    /* zero */
    /* handle subnormals */
    /* too small, will be zero */
    /* might not be sharp, but is good enough */
    /* make implicit bit explicit */
    /* very tricky - we need to round to the nearest e (+10) bit value */
    /* if this overflows, we will end up with a normalised number */
    /* handle NaNs, preserve leftmost nan bits, but make sure we don't turn them into infinities */
    /* ******************************************************************************/
/* floating point stuff, can be disabled by defining ECB_NO_LIBM */
    /* basically, everything uses "ieee pure-endian" floating point numbers */
/* the only noteworthy exception is ancient armle, which uses order 43218765 */
    /* for memcpy */
    /* for frexp*, ldexp*, INFINITY, NAN */
    /* only the oldest of old doesn't have this one. solaris. */
    /* convert a float to ieee single/binary32 */
    /* converts an ieee single/binary32 to a float */
    /* convert a double to ieee double/binary64 */
    /* converts an ieee double/binary64 to a double */
    /* convert a float to ieee half/binary16 */
    /* convert an ieee half/binary16 to float */
    /* ECB.H END */
    /* required for microsofts broken pseudo-c compiler */
    /* used to suppress some warnings */
    /* ****************************************************************************/
    /* define a suitable floor function (only used by periodics atm) */
    /* a floor() replacement function, should be independent of ev_tstamp type */
    /* the choice of shift factor is not terribly important */
    /* assume FLT_RADIX == 10 */
    /* argument too large for an unsigned long? */
    /* very large number */
    /* special treatment for negative args? */
    /* fits into an unsigned long */
    /* ****************************************************************************/
    /* ****************************************************************************/
    /* some systems, notably openbsd and darwin, fail to properly
   * implement realloc (x, 0) (as required by both ansi c-89 and
   * the single unix specification, so work around them here.
   * recently, also (at least) fedora and debian started breaking it,
   * despite documenting it otherwise.
   */
    /* ****************************************************************************/
    /* set in reify when reification needed */
    /* file descriptor info structure */
    /* the events watched for */
    /* flag set when this ANFD needs reification (EV_ANFD_REIFY, EV__IOFDSET) */
    /* the epoll backend stores the actual kernel mask in here */
    /* stores the pending event set for a given watcher */
    /* the pending event set for the given watcher */
    /* Heap Entry */
    /* a heap element */
    /* access watcher, read-write */
    /* access cached at, read-only */
    /* update at from watcher */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1821:10"]
    pub struct ev_loop {
        pub ev_rt_now: ev_tstamp,
        pub now_floor: ev_tstamp,
        pub mn_now: ev_tstamp,
        pub rtmn_diff: ev_tstamp,
        pub rfeeds: *mut W,
        pub rfeedmax: libc::c_int,
        pub rfeedcnt: libc::c_int,
        pub pendings: [*mut ANPENDING; 5],
        pub pendingmax: [libc::c_int; 5],
        pub pendingcnt: [libc::c_int; 5],
        pub pendingpri: libc::c_int,
        pub pending_w: ev_prepare,
        pub io_blocktime: ev_tstamp,
        pub timeout_blocktime: ev_tstamp,
        pub backend: libc::c_int,
        pub activecnt: libc::c_int,
        pub loop_done: sig_atomic_t,
        pub backend_fd: libc::c_int,
        pub backend_mintime: ev_tstamp,
        pub backend_modify: Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                        _: libc::c_int,
                                                        _: libc::c_int,
                                                        _: libc::c_int)
                                       -> ()>,
        pub backend_poll: Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                      _: ev_tstamp) -> ()>,
        pub anfds: *mut ANFD,
        pub anfdmax: libc::c_int,
        pub evpipe: [libc::c_int; 2],
        pub pipe_w: ev_io,
        pub pipe_write_wanted: sig_atomic_t,
        pub pipe_write_skipped: sig_atomic_t,
        pub curpid: pid_t,
        pub postfork: libc::c_char,
        pub vec_ri: *mut libc::c_void,
        pub vec_ro: *mut libc::c_void,
        pub vec_wi: *mut libc::c_void,
        pub vec_wo: *mut libc::c_void,
        pub vec_max: libc::c_int,
        pub polls: *mut pollfd,
        pub pollmax: libc::c_int,
        pub pollcnt: libc::c_int,
        pub pollidxs: *mut libc::c_int,
        pub pollidxmax: libc::c_int,
        pub fdchanges: *mut libc::c_int,
        pub fdchangemax: libc::c_int,
        pub fdchangecnt: libc::c_int,
        pub timers: *mut ANHE,
        pub timermax: libc::c_int,
        pub timercnt: libc::c_int,
        pub idles: [*mut *mut ev_idle; 5],
        pub idlemax: [libc::c_int; 5],
        pub idlecnt: [libc::c_int; 5],
        pub idleall: libc::c_int,
        pub prepares: *mut *mut ev_prepare,
        pub preparemax: libc::c_int,
        pub preparecnt: libc::c_int,
        pub checks: *mut *mut ev_check,
        pub checkmax: libc::c_int,
        pub checkcnt: libc::c_int,
        pub sig_pending: sig_atomic_t,
        pub sigfd: libc::c_int,
        pub sigfd_w: ev_io,
        pub sigfd_set: sigset_t,
        pub origflags: libc::c_uint,
        pub loop_count: libc::c_uint,
        pub loop_depth: libc::c_uint,
        pub userdata: *mut libc::c_void,
        pub release_cb: Option<unsafe extern "C" fn(_: *mut ev_loop) -> ()>,
        pub acquire_cb: Option<unsafe extern "C" fn(_: *mut ev_loop) -> ()>,
        pub invoke_cb: ev_loop_callback,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1802:11"]
    pub struct ANHE {
        pub at: ev_tstamp,
        pub w: WT,
    }
    #[c2rust::src_loc = "1555:1"]
    pub type WT = *mut ev_watcher_time;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1766:9"]
    pub struct ANFD {
        pub head: WL,
        pub events: libc::c_uchar,
        pub reify: libc::c_uchar,
        pub emask: libc::c_uchar,
        pub unused: libc::c_uchar,
    }
    #[c2rust::src_loc = "1554:1"]
    pub type WL = *mut ev_watcher_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1785:9"]
    pub struct ANPENDING {
        pub w: W,
        pub events: libc::c_int,
    }
    #[c2rust::src_loc = "1553:1"]
    pub type W = *mut ev_watcher;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2373:9"]
    pub struct ANSIG {
        pub pending: sig_atomic_t,
        pub loop_0: *mut ev_loop,
        pub head: WL,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "463:8"]
    pub struct signalfd_siginfo {
        pub ssi_signo: uint32_t,
        pub pad: [libc::c_char; 124],
    }
    /* return true if we are running with elevated privileges and should ignore env variables */
    #[c2rust::src_loc = "2727:1"]
    pub unsafe extern "C" fn k5ev_supported_backends() -> libc::c_uint {
        let mut flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        if 0x4f as libc::c_int & 32 as libc::c_int != 0 {
            flags |= EVBACKEND_EPOLL as libc::c_int as libc::c_uint
        }
        flags |= EVBACKEND_POLL as libc::c_int as libc::c_uint;
        flags |= EVBACKEND_SELECT as libc::c_int as libc::c_uint;
        return flags;
    }
    #[c2rust::src_loc = "2741:1"]
    pub unsafe extern "C" fn k5ev_recommended_backends() -> libc::c_uint {
        let mut flags: libc::c_uint = k5ev_supported_backends();
        /* kqueue is borked on everything but netbsd apparently */
  /* it usually doesn't work correctly on anything but sockets and pipes */
        flags &= !(EVBACKEND_KQUEUE as libc::c_int) as libc::c_uint;
        return flags;
    }
    #[c2rust::src_loc = "1860:1"]
    pub unsafe extern "C" fn k5ev_time() -> ev_tstamp {
        let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
        gettimeofday(&mut tv, 0 as *mut timezone);
        return tv.tv_sec as libc::c_double +
                   tv.tv_usec as libc::c_double * 1e-6f64;
    }
    /* initialise a loop structure, must be zero-initialised */
    /* pid check not overridable via env */
    /* free up a loop structure */
    /* mimic free (0) */
    /* child watcher */
    /*ev_ref (EV_A);*/
      /*ev_io_stop (EV_A_ &pipe_w);*/
    /* have to use the microsoft-never-gets-it-right macro */
    /* pipe_write_wanted must be false now, so modifying fd vars should be safe */
    /* iterate over everything, in case we missed something before */
    /* multiplicity */
    /* child watcher should not keep loop alive */
    /* ****************************************************************************/
    /* pendingpri possibly gets modified in the inner loop */
    /* make idle watchers pending. this handles the "call-idle */
/* only when higher priorities are idle" logic */
    /* make timers pending */
    /*assert (("libev: inactive timer on timer heap detected", ev_is_active (w)));*/
    /* first reschedule or stop timer */
    /* nonrepeating: stop timer */
    /* adjust all timers by a given offset */
    /* fetch new monotonic and realtime times from the kernel */
/* also detect if there was a timejump, and act accordingly */
    /* adjust timers. this is easy, as the offset is the same for all of them */
    /* in case we recurse, ensure ordering stays nice and clean */
    /* penalise the forking check even more */
    /* we might have forked, so reify kernel state if necessary */
    /* update fd-related kernel structures */
    /* calculate blocking time */
    /* remember old timestamp for io_blocktime calculation */
    /* update time to cancel out callback processing overhead */
    /* from now on, we want a pipe-wake-up */
    /* make sure pipe_write_wanted is visible before we check for potential skips */
    /* don't let timeouts decrease the waittime below timeout_blocktime */
    /* at this point, we NEED to wait, so we have to ensure */
            /* to pass a minimum nonzero value to the backend */
    /* extra check because io_blocktime is commonly 0 */
    /* assert for side effect */
    /* assert for side effect */
    /* just an optimisation, no fence needed */
    /* update ev_rt_now, do magic */
    /* queue pending timers and reschedule them */
    /* relative timers called last */
    /* queue idle watchers unless other events are pending */
    /* ****************************************************************************/
/* singly-linked list management, used when the expected list length is short */
    /* internal, faster, version of ev_clear_pending */
    /* ****************************************************************************/
    /* common bug, apparently */
    #[inline(never)]
    #[c2rust::src_loc = "3861:1"]
    pub unsafe extern "C" fn k5ev_io_stop(mut loop_0: *mut ev_loop,
                                          mut w: *mut ev_io) {
        clear_pending(loop_0, w as W);
        if (0 as libc::c_int +
                (*(w as *mut libc::c_void as *mut ev_watcher)).active == 0) as
               libc::c_int as libc::c_long != 0 {
            return
        }
        if (*w).fd >= 0 as libc::c_int && (*w).fd < (*loop_0).anfdmax {
        } else {
            __assert_fail(b"(\"libev: ev_io_stop called with illegal fd (must stay constant after start!)\", w->fd >= 0 && w->fd < anfdmax)\x00"
                              as *const u8 as *const libc::c_char,
                          b"./ev.c\x00" as *const u8 as *const libc::c_char,
                          3868 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 45],
                                                    &[libc::c_char; 45]>(b"void k5ev_io_stop(struct ev_loop *, ev_io *)\x00")).as_ptr());
        }
        wlist_del(&mut (*(*loop_0).anfds.offset((*w).fd as isize)).head,
                  w as WL);
        ev_stop(loop_0, w as W);
        fd_change(loop_0, (*w).fd, 1 as libc::c_int);
    }
    #[c2rust::src_loc = "1901:1"]
    pub unsafe extern "C" fn k5ev_sleep(mut delay: ev_tstamp) {
        if delay > 0.0f64 {
            let mut ts: timespec = timespec{tv_sec: 0, tv_nsec: 0,};
            ts.tv_sec = delay as libc::c_long;
            ts.tv_nsec =
                ((delay - ts.tv_sec as libc::c_double) * 1e9f64) as
                    libc::c_long;
            nanosleep(&mut ts, 0 as *mut timespec);
        };
    }
    /*assert (("libev: internal timer heap corruption", timers [ev_active (w)] == (WT)w));*/
    #[inline(never)]
    #[c2rust::src_loc = "4041:1"]
    pub unsafe extern "C" fn k5ev_signal_start(mut loop_0: *mut ev_loop,
                                               mut w: *mut ev_signal) {
        if (0 as libc::c_int +
                (*(w as *mut libc::c_void as *mut ev_watcher)).active != 0) as
               libc::c_int as libc::c_long != 0 {
            return
        } /* retry without flags */
        if (*w).signum > 0 as libc::c_int &&
               (*w).signum < 64 as libc::c_int + 1 as libc::c_int {
        } else {
            __assert_fail(b"(\"libev: ev_signal_start called with illegal signal number\", w->signum > 0 && w->signum < EV_NSIG)\x00"
                              as *const u8 as *const libc::c_char,
                          b"./ev.c\x00" as *const u8 as *const libc::c_char,
                          4047 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 54],
                                                    &[libc::c_char; 54]>(b"void k5ev_signal_start(struct ev_loop *, ev_signal *)\x00")).as_ptr()); /* doing it twice will not hurt */
        }
        if signals[((*w).signum - 1 as libc::c_int) as usize].loop_0.is_null()
               ||
               signals[((*w).signum - 1 as libc::c_int) as usize].loop_0 ==
                   loop_0 {
        } else {
            __assert_fail(b"(\"libev: a signal must not be attached to two different loops\", !signals [w->signum - 1].loop || signals [w->signum - 1].loop == loop)\x00"
                              as *const u8 as *const libc::c_char,
                          b"./ev.c\x00" as *const u8 as *const libc::c_char,
                          4051 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 54],
                                                    &[libc::c_char; 54]>(b"void k5ev_signal_start(struct ev_loop *, ev_signal *)\x00")).as_ptr());
        }
        signals[((*w).signum - 1 as libc::c_int) as usize].loop_0 = loop_0;
        asm!("" : : : : "volatile");
        if (*loop_0).sigfd == -(2 as libc::c_int) {
            (*loop_0).sigfd =
                signalfd(-(1 as libc::c_int), &mut (*loop_0).sigfd_set,
                         0o4000 as libc::c_int | 0o2000000 as libc::c_int);
            if (*loop_0).sigfd < 0 as libc::c_int &&
                   *__errno_location() == 22 as libc::c_int {
                (*loop_0).sigfd =
                    signalfd(-(1 as libc::c_int), &mut (*loop_0).sigfd_set,
                             0 as libc::c_int)
            }
            if (*loop_0).sigfd >= 0 as libc::c_int {
                fd_intern((*loop_0).sigfd);
                sigemptyset(&mut (*loop_0).sigfd_set);
                let ref mut fresh0 =
                    (*(&mut (*loop_0).sigfd_w as *mut ev_io as
                           *mut libc::c_void as *mut ev_watcher)).pending;
                *fresh0 = 0 as libc::c_int;
                (*(&mut (*loop_0).sigfd_w as *mut ev_io as *mut libc::c_void
                       as *mut ev_watcher)).active = *fresh0;
                (*(&mut (*loop_0).sigfd_w as *mut ev_io as *mut libc::c_void
                       as *mut ev_watcher)).priority = 0 as libc::c_int;
                (*loop_0).sigfd_w.cb =
                    Some(sigfdcb as
                             unsafe extern "C" fn(_: *mut ev_loop,
                                                  _: *mut ev_io,
                                                  _: libc::c_int) -> ());
                memmove(&mut (*(&mut (*loop_0).sigfd_w as *mut ev_io as
                                    *mut ev_watcher)).cb as
                            *mut Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                             _:
                                                                 *mut ev_watcher,
                                                             _: libc::c_int)
                                            -> ()> as *mut libc::c_void,
                        &mut (*loop_0).sigfd_w.cb as
                            *mut Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                             _: *mut ev_io,
                                                             _: libc::c_int)
                                            -> ()> as *const libc::c_void,
                        ::std::mem::size_of::<Option<unsafe extern "C" fn(_:
                                                                              *mut ev_loop,
                                                                          _:
                                                                              *mut ev_io,
                                                                          _:
                                                                              libc::c_int)
                                                         -> ()>>() as
                            libc::c_ulong);
                (*loop_0).sigfd_w.fd = (*loop_0).sigfd;
                (*loop_0).sigfd_w.events =
                    EV_READ as libc::c_int | EV__IOFDSET as libc::c_int;
                (*(&mut (*loop_0).sigfd_w as *mut ev_io as *mut libc::c_void
                       as *mut ev_watcher)).priority =
                    if 0x4f as libc::c_int & 4 as libc::c_int != 0 {
                        2 as libc::c_int
                    } else { 0 as libc::c_int };
                k5ev_io_start(loop_0, &mut (*loop_0).sigfd_w);
                k5ev_unref(loop_0);
                /* signalfd watcher should not keep loop alive */
            }
        }
        if (*loop_0).sigfd >= 0 as libc::c_int {
            /* TODO: check .head */
            sigaddset(&mut (*loop_0).sigfd_set, (*w).signum);
            sigprocmask(0 as libc::c_int, &mut (*loop_0).sigfd_set,
                        0 as *mut sigset_t);
            signalfd((*loop_0).sigfd, &mut (*loop_0).sigfd_set,
                     0 as libc::c_int);
        }
        ev_start(loop_0, w as W, 1 as libc::c_int);
        wlist_add(&mut (*signals.as_mut_ptr().offset(((*w).signum -
                                                          1 as libc::c_int) as
                                                         isize)).head,
                  w as WL);
        if (*(w as WL)).next.is_null() {
            if (*loop_0).sigfd < 0 as libc::c_int {
                /*TODO*/
                let mut sa: sigaction =
                    sigaction{__sigaction_handler:
                                  C2RustUnnamed_9{sa_handler: None,},
                              sa_mask: __sigset_t{__val: [0; 16],},
                              sa_flags: 0,
                              sa_restorer:
                                  None,}; /* if restarting works we save one iteration */
                evpipe_init(loop_0);
                sa.__sigaction_handler.sa_handler =
                    Some(ev_sighandler as
                             unsafe extern "C" fn(_: libc::c_int) -> ());
                sigfillset(&mut sa.sa_mask);
                sa.sa_flags = 0x10000000 as libc::c_int;
                sigaction((*w).signum, &mut sa, 0 as *mut sigaction);
                if (*loop_0).origflags &
                       EVFLAG_NOSIGMASK as libc::c_int as libc::c_uint != 0 {
                    sigemptyset(&mut sa.sa_mask);
                    sigaddset(&mut sa.sa_mask, (*w).signum);
                    sigprocmask(1 as libc::c_int, &mut sa.sa_mask,
                                0 as *mut sigset_t);
                }
            }
        };
    }
    #[c2rust::src_loc = "1731:16"]
    pub static mut alloc:
               Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                           _: libc::c_long)
                          -> *mut libc::c_void> =
        unsafe {
            Some(ev_realloc_emul as
                     unsafe extern "C" fn(_: *mut libc::c_void,
                                          _: libc::c_long)
                         -> *mut libc::c_void)
        };
    #[c2rust::src_loc = "1714:1"]
    pub unsafe extern "C" fn ev_realloc_emul(mut ptr: *mut libc::c_void,
                                             mut size: libc::c_long)
     -> *mut libc::c_void {
        if size != 0 { return realloc(ptr, size as libc::c_ulong) }
        free(ptr);
        return 0 as *mut libc::c_void;
    }
    #[c2rust::src_loc = "1684:15"]
    pub static mut syserr_cb:
               Option<unsafe extern "C" fn(_: *const libc::c_char) -> ()> =
        None;
    #[c2rust::src_loc = "3223:1"]
    pub unsafe extern "C" fn k5ev_default_loop(mut flags: libc::c_uint)
     -> *mut ev_loop {
        if k5ev_default_loop_ptr.is_null() {
            k5ev_default_loop_ptr = &mut default_loop_struct;
            let mut loop_0: *mut ev_loop = k5ev_default_loop_ptr;
            loop_init(loop_0, flags);
            if k5ev_backend(loop_0) != 0 {
                let ref mut fresh1 =
                    (*(&mut childev as *mut ev_signal as *mut libc::c_void as
                           *mut ev_watcher)).pending;
                *fresh1 = 0 as libc::c_int;
                (*(&mut childev as *mut ev_signal as *mut libc::c_void as
                       *mut ev_watcher)).active = *fresh1;
                (*(&mut childev as *mut ev_signal as *mut libc::c_void as
                       *mut ev_watcher)).priority = 0 as libc::c_int;
                childev.cb =
                    Some(childcb as
                             unsafe extern "C" fn(_: *mut ev_loop,
                                                  _: *mut ev_signal,
                                                  _: libc::c_int) -> ());
                memmove(&mut (*(&mut childev as *mut ev_signal as
                                    *mut ev_watcher)).cb as
                            *mut Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                             _:
                                                                 *mut ev_watcher,
                                                             _: libc::c_int)
                                            -> ()> as *mut libc::c_void,
                        &mut childev.cb as
                            *mut Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                             _:
                                                                 *mut ev_signal,
                                                             _: libc::c_int)
                                            -> ()> as *const libc::c_void,
                        ::std::mem::size_of::<Option<unsafe extern "C" fn(_:
                                                                              *mut ev_loop,
                                                                          _:
                                                                              *mut ev_signal,
                                                                          _:
                                                                              libc::c_int)
                                                         -> ()>>() as
                            libc::c_ulong);
                childev.signum = 17 as libc::c_int;
                (*(&mut childev as *mut ev_signal as *mut libc::c_void as
                       *mut ev_watcher)).priority =
                    if 0x4f as libc::c_int & 4 as libc::c_int != 0 {
                        2 as libc::c_int
                    } else { 0 as libc::c_int };
                k5ev_signal_start(loop_0, &mut childev);
                k5ev_unref(loop_0);
            } else { k5ev_default_loop_ptr = 0 as *mut ev_loop }
        }
        return k5ev_default_loop_ptr;
    }
    #[c2rust::src_loc = "1832:31"]
    pub static mut k5ev_default_loop_ptr: *mut ev_loop =
        0 as *const ev_loop as *mut ev_loop;
    #[c2rust::src_loc = "1831:25"]
    pub static mut default_loop_struct: ev_loop =
        ev_loop{ev_rt_now: 0.,
                now_floor: 0.,
                mn_now: 0.,
                rtmn_diff: 0.,
                rfeeds: 0 as *const W as *mut W,
                rfeedmax: 0,
                rfeedcnt: 0,
                pendings: [0 as *const ANPENDING as *mut ANPENDING; 5],
                pendingmax: [0; 5],
                pendingcnt: [0; 5],
                pendingpri: 0,
                pending_w:
                    ev_prepare{active: 0,
                               pending: 0,
                               priority: 0,
                               data:
                                   0 as *const libc::c_void as
                                       *mut libc::c_void,
                               cb: None,},
                io_blocktime: 0.,
                timeout_blocktime: 0.,
                backend: 0,
                activecnt: 0,
                loop_done: 0,
                backend_fd: 0,
                backend_mintime: 0.,
                backend_modify: None,
                backend_poll: None,
                anfds: 0 as *const ANFD as *mut ANFD,
                anfdmax: 0,
                evpipe: [0; 2],
                pipe_w:
                    ev_io{active: 0,
                          pending: 0,
                          priority: 0,
                          data: 0 as *const libc::c_void as *mut libc::c_void,
                          cb: None,
                          next:
                              0 as *const ev_watcher_list as
                                  *mut ev_watcher_list,
                          fd: 0,
                          events: 0,},
                pipe_write_wanted: 0,
                pipe_write_skipped: 0,
                curpid: 0,
                postfork: 0,
                vec_ri: 0 as *const libc::c_void as *mut libc::c_void,
                vec_ro: 0 as *const libc::c_void as *mut libc::c_void,
                vec_wi: 0 as *const libc::c_void as *mut libc::c_void,
                vec_wo: 0 as *const libc::c_void as *mut libc::c_void,
                vec_max: 0,
                polls: 0 as *const pollfd as *mut pollfd,
                pollmax: 0,
                pollcnt: 0,
                pollidxs: 0 as *const libc::c_int as *mut libc::c_int,
                pollidxmax: 0,
                fdchanges: 0 as *const libc::c_int as *mut libc::c_int,
                fdchangemax: 0,
                fdchangecnt: 0,
                timers: 0 as *const ANHE as *mut ANHE,
                timermax: 0,
                timercnt: 0,
                idles: [0 as *const *mut ev_idle as *mut *mut ev_idle; 5],
                idlemax: [0; 5],
                idlecnt: [0; 5],
                idleall: 0,
                prepares: 0 as *const *mut ev_prepare as *mut *mut ev_prepare,
                preparemax: 0,
                preparecnt: 0,
                checks: 0 as *const *mut ev_check as *mut *mut ev_check,
                checkmax: 0,
                checkcnt: 0,
                sig_pending: 0,
                sigfd: 0,
                sigfd_w:
                    ev_io{active: 0,
                          pending: 0,
                          priority: 0,
                          data: 0 as *const libc::c_void as *mut libc::c_void,
                          cb: None,
                          next:
                              0 as *const ev_watcher_list as
                                  *mut ev_watcher_list,
                          fd: 0,
                          events: 0,},
                sigfd_set: __sigset_t{__val: [0; 16],},
                origflags: 0,
                loop_count: 0,
                loop_depth: 0,
                userdata: 0 as *const libc::c_void as *mut libc::c_void,
                release_cb: None,
                acquire_cb: None,
                invoke_cb: None,};
    #[c2rust::src_loc = "3725:1"]
    pub unsafe extern "C" fn k5ev_unref(mut loop_0: *mut ev_loop) {
        (*loop_0).activecnt -= 1;
    }
    #[c2rust::src_loc = "2628:18"]
    pub static mut childev: ev_signal =
        ev_signal{active: 0,
                  pending: 0,
                  priority: 0,
                  data: 0 as *const libc::c_void as *mut libc::c_void,
                  cb: None,
                  next: 0 as *const ev_watcher_list as *mut ev_watcher_list,
                  signum: 0,};
    #[c2rust::src_loc = "2566:1"]
    pub unsafe extern "C" fn ev_sighandler(mut signum: libc::c_int) {
        k5ev_feed_signal(signum);
    }
    #[c2rust::src_loc = "2550:1"]
    pub unsafe extern "C" fn k5ev_feed_signal(mut signum: libc::c_int) {
        let mut loop_0: *mut ev_loop = 0 as *mut ev_loop;
        asm!("" : : : "memory" : "volatile");
        loop_0 = signals[(signum - 1 as libc::c_int) as usize].loop_0;
        if loop_0.is_null() { return }
        ::std::ptr::write_volatile(&mut signals[(signum - 1 as libc::c_int) as
                                                    usize].pending as
                                       *mut sig_atomic_t, 1 as libc::c_int);
        evpipe_write(loop_0, &mut (*loop_0).sig_pending);
    }
    #[inline]
    #[c2rust::src_loc = "2433:1"]
    pub unsafe extern "C" fn evpipe_write(mut loop_0: *mut ev_loop,
                                          mut flag: *mut sig_atomic_t) {
        asm!("mfence" : : : "memory" : "volatile");
        if (*flag != 0) as libc::c_int as libc::c_long != 0 { return }
        ::std::ptr::write_volatile(flag, 1 as libc::c_int);
        asm!("" : : : : "volatile");
        ::std::ptr::write_volatile(&mut (*loop_0).pipe_write_skipped as
                                       *mut sig_atomic_t, 1 as libc::c_int);
        asm!("mfence" : : : "memory" : "volatile");
        if (*loop_0).pipe_write_wanted != 0 {
            let mut old_errno: libc::c_int = 0;
            ::std::ptr::write_volatile(&mut (*loop_0).pipe_write_skipped as
                                           *mut sig_atomic_t,
                                       0 as libc::c_int);
            asm!("" : : : : "volatile");
            old_errno = *__errno_location();
            if (*loop_0).evpipe[0 as libc::c_int as usize] < 0 as libc::c_int
               {
                let mut counter: uint64_t = 1 as libc::c_int as uint64_t;
                write((*loop_0).evpipe[1 as libc::c_int as usize],
                      &mut counter as *mut uint64_t as *const libc::c_void,
                      ::std::mem::size_of::<uint64_t>() as libc::c_ulong);
            } else {
                write((*loop_0).evpipe[1 as libc::c_int as usize],
                      &mut *(*loop_0).evpipe.as_mut_ptr().offset(1 as
                                                                     libc::c_int
                                                                     as isize)
                          as *mut libc::c_int as *const libc::c_void,
                      1 as libc::c_int as size_t);
            }
            *__errno_location() = old_errno
        };
    }
    #[c2rust::src_loc = "2382:14"]
    pub static mut signals: [ANSIG; 64] =
        [ANSIG{pending: 0,
               loop_0: 0 as *const ev_loop as *mut ev_loop,
               head: 0 as *const ev_watcher_list as *mut ev_watcher_list,};
            64];
    #[inline(never)]
    #[c2rust::src_loc = "2388:1"]
    pub unsafe extern "C" fn evpipe_init(mut loop_0: *mut ev_loop) {
        if 0 as libc::c_int +
               (*(&mut (*loop_0).pipe_w as *mut ev_io as *mut libc::c_void as
                      *mut ev_watcher)).active == 0 {
            let mut fds: [libc::c_int; 2] = [0; 2];
            fds[0 as libc::c_int as usize] = -(1 as libc::c_int);
            fds[1 as libc::c_int as usize] =
                eventfd(0 as libc::c_int as libc::c_uint,
                        0o4000 as libc::c_int | 0o2000000 as libc::c_int);
            if fds[1 as libc::c_int as usize] < 0 as libc::c_int &&
                   *__errno_location() == 22 as libc::c_int {
                fds[1 as libc::c_int as usize] =
                    eventfd(0 as libc::c_int as libc::c_uint,
                            0 as libc::c_int)
            }
            if fds[1 as libc::c_int as usize] < 0 as libc::c_int {
                while pipe(fds.as_mut_ptr()) != 0 {
                    ev_syserr(b"(libev) error creating signal/async pipe\x00"
                                  as *const u8 as *const libc::c_char);
                }
                fd_intern(fds[0 as libc::c_int as usize]);
            }
            (*loop_0).evpipe[0 as libc::c_int as usize] =
                fds[0 as libc::c_int as usize];
            if (*loop_0).evpipe[1 as libc::c_int as usize] < 0 as libc::c_int
               {
                (*loop_0).evpipe[1 as libc::c_int as usize] =
                    fds[1 as libc::c_int as usize]
            } else {
                dup2(fds[1 as libc::c_int as usize],
                     (*loop_0).evpipe[1 as libc::c_int as usize]);
                close(fds[1 as libc::c_int as usize]);
            }
            fd_intern((*loop_0).evpipe[1 as libc::c_int as usize]);
            (*loop_0).pipe_w.fd =
                if (*loop_0).evpipe[0 as libc::c_int as usize] <
                       0 as libc::c_int {
                    (*loop_0).evpipe[1 as libc::c_int as usize]
                } else { (*loop_0).evpipe[0 as libc::c_int as usize] };
            (*loop_0).pipe_w.events =
                EV_READ as libc::c_int | EV__IOFDSET as libc::c_int;
            k5ev_io_start(loop_0, &mut (*loop_0).pipe_w);
            k5ev_unref(loop_0);
        };
    }
    #[inline(never)]
    #[c2rust::src_loc = "3835:1"]
    pub unsafe extern "C" fn k5ev_io_start(mut loop_0: *mut ev_loop,
                                           mut w: *mut ev_io) {
        let mut fd: libc::c_int = (*w).fd;
        if (0 as libc::c_int +
                (*(w as *mut libc::c_void as *mut ev_watcher)).active != 0) as
               libc::c_int as libc::c_long != 0 {
            return
        }
        if fd >= 0 as libc::c_int {
        } else {
            __assert_fail(b"(\"libev: ev_io_start called with negative fd\", fd >= 0)\x00"
                              as *const u8 as *const libc::c_char,
                          b"./ev.c\x00" as *const u8 as *const libc::c_char,
                          3843 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 46],
                                                    &[libc::c_char; 46]>(b"void k5ev_io_start(struct ev_loop *, ev_io *)\x00")).as_ptr());
        }
        if (*w).events &
               !(EV__IOFDSET as libc::c_int | EV_READ as libc::c_int |
                     EV_WRITE as libc::c_int) == 0 {
        } else {
            __assert_fail(b"(\"libev: ev_io_start called with illegal event mask\", !(w->events & ~(EV__IOFDSET | EV_READ | EV_WRITE)))\x00"
                              as *const u8 as *const libc::c_char,
                          b"./ev.c\x00" as *const u8 as *const libc::c_char,
                          3844 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 46],
                                                    &[libc::c_char; 46]>(b"void k5ev_io_start(struct ev_loop *, ev_io *)\x00")).as_ptr());
        }
        ev_start(loop_0, w as W, 1 as libc::c_int);
        if (fd + 1 as libc::c_int > (*loop_0).anfdmax) as libc::c_int as
               libc::c_long != 0 {
            let mut ocur_: libc::c_int = (*loop_0).anfdmax;
            (*loop_0).anfds =
                array_realloc(::std::mem::size_of::<ANFD>() as libc::c_ulong
                                  as libc::c_int,
                              (*loop_0).anfds as *mut libc::c_void,
                              &mut (*loop_0).anfdmax, fd + 1 as libc::c_int)
                    as *mut ANFD;
            memset((*loop_0).anfds.offset(ocur_ as isize) as
                       *mut libc::c_void, 0 as libc::c_int,
                   (::std::mem::size_of::<ANFD>() as
                        libc::c_ulong).wrapping_mul(((*loop_0).anfdmax -
                                                         ocur_) as
                                                        libc::c_ulong));
        }
        wlist_add(&mut (*(*loop_0).anfds.offset(fd as isize)).head, w as WL);
        if (*(w as WL)).next != w as WL {
        } else {
            __assert_fail(b"(\"libev: ev_io_start called with corrupted watcher\", ((WL)w)->next != (WL)w)\x00"
                              as *const u8 as *const libc::c_char,
                          b"./ev.c\x00" as *const u8 as *const libc::c_char,
                          3853 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 46],
                                                    &[libc::c_char; 46]>(b"void k5ev_io_start(struct ev_loop *, ev_io *)\x00")).as_ptr());
        }
        fd_change(loop_0, fd,
                  (*w).events & EV__IOFDSET as libc::c_int |
                      1 as libc::c_int);
        (*w).events &= !(EV__IOFDSET as libc::c_int);
    }
    #[inline]
    #[c2rust::src_loc = "2132:1"]
    pub unsafe extern "C" fn fd_change(mut loop_0: *mut ev_loop,
                                       mut fd: libc::c_int,
                                       mut flags: libc::c_int) {
        let mut reify: libc::c_uchar =
            (*(*loop_0).anfds.offset(fd as isize)).reify;
        let ref mut fresh2 = (*(*loop_0).anfds.offset(fd as isize)).reify;
        *fresh2 = (*fresh2 as libc::c_int | flags) as libc::c_uchar;
        if (reify == 0) as libc::c_int as libc::c_long != 0 {
            (*loop_0).fdchangecnt += 1;
            if ((*loop_0).fdchangecnt > (*loop_0).fdchangemax) as libc::c_int
                   as libc::c_long != 0 {
                let mut ocur_: libc::c_int = (*loop_0).fdchangemax;
                (*loop_0).fdchanges =
                    array_realloc(::std::mem::size_of::<libc::c_int>() as
                                      libc::c_ulong as libc::c_int,
                                  (*loop_0).fdchanges as *mut libc::c_void,
                                  &mut (*loop_0).fdchangemax,
                                  (*loop_0).fdchangecnt) as *mut libc::c_int
            }
            *(*loop_0).fdchanges.offset(((*loop_0).fdchangecnt -
                                             1 as libc::c_int) as isize) = fd
        };
    }
    #[inline(never)]
    #[c2rust::src_loc = "1952:1"]
    pub unsafe extern "C" fn array_realloc(mut elem: libc::c_int,
                                           mut base: *mut libc::c_void,
                                           mut cur: *mut libc::c_int,
                                           mut cnt: libc::c_int)
     -> *mut libc::c_void {
        *cur = array_nextsize(elem, *cur, cnt);
        return ev_realloc(base, (elem * *cur) as libc::c_long);
    }
    #[inline]
    #[c2rust::src_loc = "1739:1"]
    pub unsafe extern "C" fn ev_realloc(mut ptr: *mut libc::c_void,
                                        mut size: libc::c_long)
     -> *mut libc::c_void {
        ptr = alloc.expect("non-null function pointer")(ptr, size);
        if ptr.is_null() && size != 0 {
            fprintf(stderr,
                    b"(libev) cannot allocate %ld bytes, aborting.\x00" as
                        *const u8 as *const libc::c_char, size);
            abort();
        }
        return ptr;
    }
    #[inline]
    #[c2rust::src_loc = "1931:1"]
    pub unsafe extern "C" fn array_nextsize(mut elem: libc::c_int,
                                            mut cur: libc::c_int,
                                            mut cnt: libc::c_int)
     -> libc::c_int {
        let mut ncur: libc::c_int = cur + 1 as libc::c_int;
        loop  { ncur <<= 1 as libc::c_int; if !(cnt > ncur) { break ; } }
        if (elem * ncur) as libc::c_ulong >
               (4096 as libc::c_int as
                    libc::c_ulong).wrapping_sub((::std::mem::size_of::<*mut libc::c_void>()
                                                     as
                                                     libc::c_ulong).wrapping_mul(4
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong))
           {
            ncur *= elem;
            ncur =
                (((ncur + elem + (4096 as libc::c_int - 1 as libc::c_int)) as
                      libc::c_ulong).wrapping_add((::std::mem::size_of::<*mut libc::c_void>()
                                                       as
                                                       libc::c_ulong).wrapping_mul(4
                                                                                       as
                                                                                       libc::c_int
                                                                                       as
                                                                                       libc::c_ulong))
                     &
                     !(4096 as libc::c_int - 1 as libc::c_int) as
                         libc::c_ulong) as libc::c_int;
            ncur =
                (ncur as
                     libc::c_ulong).wrapping_sub((::std::mem::size_of::<*mut libc::c_void>()
                                                      as
                                                      libc::c_ulong).wrapping_mul(4
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong))
                    as libc::c_int;
            ncur /= elem
        }
        return ncur;
    }
    #[inline]
    #[c2rust::src_loc = "3759:1"]
    pub unsafe extern "C" fn wlist_add(mut head: *mut WL, mut elem: WL) {
        (*elem).next = *head;
        *head = elem;
    }
    #[inline]
    #[c2rust::src_loc = "3818:1"]
    pub unsafe extern "C" fn ev_start(mut loop_0: *mut ev_loop, mut w: W,
                                      mut active: libc::c_int) {
        pri_adjust(loop_0, w);
        (*w).active = active;
        k5ev_ref(loop_0);
    }
    #[c2rust::src_loc = "3719:1"]
    pub unsafe extern "C" fn k5ev_ref(mut loop_0: *mut ev_loop) {
        (*loop_0).activecnt += 1;
    }
    #[inline]
    #[c2rust::src_loc = "3809:1"]
    pub unsafe extern "C" fn pri_adjust(mut loop_0: *mut ev_loop, mut w: W) {
        let mut pri: libc::c_int =
            (*(w as *mut libc::c_void as *mut ev_watcher)).priority;
        pri =
            if pri <
                   (if 0x4f as libc::c_int & 4 as libc::c_int != 0 {
                        -(2 as libc::c_int)
                    } else { 0 as libc::c_int }) {
                if 0x4f as libc::c_int & 4 as libc::c_int != 0 {
                    -(2 as libc::c_int)
                } else { 0 as libc::c_int }
            } else { pri };
        pri =
            if pri >
                   (if 0x4f as libc::c_int & 4 as libc::c_int != 0 {
                        2 as libc::c_int
                    } else { 0 as libc::c_int }) {
                if 0x4f as libc::c_int & 4 as libc::c_int != 0 {
                    2 as libc::c_int
                } else { 0 as libc::c_int }
            } else { pri };
        (*(w as *mut libc::c_void as *mut ev_watcher)).priority = pri;
    }
    #[inline]
    #[c2rust::src_loc = "2213:1"]
    pub unsafe extern "C" fn fd_intern(mut fd: libc::c_int) {
        fcntl(fd, 2 as libc::c_int, 1 as libc::c_int);
        fcntl(fd, 4 as libc::c_int, 0o4000 as libc::c_int);
    }
    #[inline(never)]
    #[c2rust::src_loc = "1692:1"]
    pub unsafe extern "C" fn ev_syserr(mut msg: *const libc::c_char) {
        if msg.is_null() {
            msg =
                b"(libev) system error\x00" as *const u8 as
                    *const libc::c_char
        }
        if syserr_cb.is_some() {
            syserr_cb.expect("non-null function pointer")(msg);
        } else { perror(msg); abort(); };
    }
    #[c2rust::src_loc = "2602:1"]
    pub unsafe extern "C" fn sigfdcb(mut loop_0: *mut ev_loop,
                                     mut iow: *mut ev_io,
                                     mut revents: libc::c_int) {
        let mut si: [signalfd_siginfo; 2] =
            [signalfd_siginfo{ssi_signo: 0, pad: [0; 124],}; 2];
        let mut sip: *mut signalfd_siginfo = 0 as *mut signalfd_siginfo;
        loop  {
            let mut res: ssize_t =
                read((*loop_0).sigfd, si.as_mut_ptr() as *mut libc::c_void,
                     ::std::mem::size_of::<[signalfd_siginfo; 2]>() as
                         libc::c_ulong);
            sip = si.as_mut_ptr();
            while (sip as *mut libc::c_char) <
                      (si.as_mut_ptr() as
                           *mut libc::c_char).offset(res as isize) {
                k5ev_feed_signal_event(loop_0,
                                       (*sip).ssi_signo as libc::c_int);
                sip = sip.offset(1)
            }
            if res <
                   ::std::mem::size_of::<[signalfd_siginfo; 2]>() as
                       libc::c_ulong as ssize_t {
                break ;
            }
        };
    }
    #[inline(never)]
    #[c2rust::src_loc = "2576:1"]
    pub unsafe extern "C" fn k5ev_feed_signal_event(mut loop_0: *mut ev_loop,
                                                    mut signum: libc::c_int) {
        let mut w: WL = 0 as *mut ev_watcher_list;
        if (signum <= 0 as libc::c_int ||
                signum >= 64 as libc::c_int + 1 as libc::c_int) as libc::c_int
               as libc::c_long != 0 {
            return
        }
        signum -= 1;
        if (signals[signum as usize].loop_0 != loop_0) as libc::c_int as
               libc::c_long != 0 {
            return
        }
        ::std::ptr::write_volatile(&mut signals[signum as usize].pending as
                                       *mut sig_atomic_t, 0 as libc::c_int);
        asm!("" : : : : "volatile");
        w = signals[signum as usize].head;
        while !w.is_null() {
            k5ev_feed_event(loop_0, w as W as *mut libc::c_void,
                            EV_SIGNAL as libc::c_int);
            w = (*w).next
        };
    }
    #[inline(never)]
    #[c2rust::src_loc = "1992:1"]
    pub unsafe extern "C" fn k5ev_feed_event(mut loop_0: *mut ev_loop,
                                             mut w: *mut libc::c_void,
                                             mut revents: libc::c_int) {
        let mut w_: W = w as W;
        let mut pri: libc::c_int =
            (*w_).priority -
                (if 0x4f as libc::c_int & 4 as libc::c_int != 0 {
                     -(2 as libc::c_int)
                 } else { 0 as libc::c_int });
        if ((*w_).pending != 0) as libc::c_int as libc::c_long != 0 {
            (*(*loop_0).pendings[pri as
                                     usize].offset(((*w_).pending -
                                                        1 as libc::c_int) as
                                                       isize)).events |=
                revents
        } else {
            (*loop_0).pendingcnt[pri as usize] += 1;
            (*w_).pending = (*loop_0).pendingcnt[pri as usize];
            if ((*w_).pending > (*loop_0).pendingmax[pri as usize]) as
                   libc::c_int as libc::c_long != 0 {
                let mut ocur_: libc::c_int =
                    (*loop_0).pendingmax[pri as usize];
                (*loop_0).pendings[pri as usize] =
                    array_realloc(::std::mem::size_of::<ANPENDING>() as
                                      libc::c_ulong as libc::c_int,
                                  (*loop_0).pendings[pri as usize] as
                                      *mut libc::c_void,
                                  &mut *(*loop_0).pendingmax.as_mut_ptr().offset(pri
                                                                                     as
                                                                                     isize),
                                  (*w_).pending) as *mut ANPENDING
            }
            let ref mut fresh3 =
                (*(*loop_0).pendings[pri as
                                         usize].offset(((*w_).pending -
                                                            1 as libc::c_int)
                                                           as isize)).w;
            *fresh3 = w_;
            (*(*loop_0).pendings[pri as
                                     usize].offset(((*w_).pending -
                                                        1 as libc::c_int) as
                                                       isize)).events =
                revents
        }
        (*loop_0).pendingpri =
            (if 0x4f as libc::c_int & 4 as libc::c_int != 0 {
                 2 as libc::c_int
             } else { 0 as libc::c_int }) -
                (if 0x4f as libc::c_int & 4 as libc::c_int != 0 {
                     -(2 as libc::c_int)
                 } else { 0 as libc::c_int }) + 1 as libc::c_int -
                1 as libc::c_int;
    }
    #[c2rust::src_loc = "2659:1"]
    pub unsafe extern "C" fn childcb(mut loop_0: *mut ev_loop,
                                     mut sw: *mut ev_signal,
                                     mut revents: libc::c_int) {
        let mut pid: libc::c_int = 0;
        let mut status: libc::c_int = 0;
        pid =
            waitpid(-(1 as libc::c_int), &mut status,
                    1 as libc::c_int | 2 as libc::c_int | 8 as libc::c_int);
        if 0 as libc::c_int >= pid {
            if 8 as libc::c_int == 0 ||
                   *__errno_location() != 22 as libc::c_int ||
                   {
                       pid =
                           waitpid(-(1 as libc::c_int), &mut status,
                                   1 as libc::c_int | 2 as libc::c_int);
                       (0 as libc::c_int) >= pid
                   } {
                return
            }
        }
        k5ev_feed_event(loop_0, sw as W as *mut libc::c_void,
                        EV_SIGNAL as libc::c_int);
        child_reap(loop_0, pid, pid, status);
        if (if 0x4f as libc::c_int & 2 as libc::c_int != 0 {
                16 as libc::c_int
            } else { 1 as libc::c_int }) > 1 as libc::c_int {
            child_reap(loop_0, 0 as libc::c_int, pid, status);
        };
    }
    #[inline]
    #[c2rust::src_loc = "2635:1"]
    pub unsafe extern "C" fn child_reap(mut loop_0: *mut ev_loop,
                                        mut chain: libc::c_int,
                                        mut pid: libc::c_int,
                                        mut status: libc::c_int) {
        let mut w: *mut ev_child = 0 as *mut ev_child;
        let mut traced: libc::c_int =
            (status & 0xff as libc::c_int == 0x7f as libc::c_int ||
                 status == 0xffff as libc::c_int) as libc::c_int;
        w =
            childs[(chain &
                        (if 0x4f as libc::c_int & 2 as libc::c_int != 0 {
                             16 as libc::c_int
                         } else { 1 as libc::c_int }) - 1 as libc::c_int) as
                       usize] as *mut ev_child;
        while !w.is_null() {
            if ((*w).pid == pid || (*w).pid == 0) &&
                   (traced == 0 || (*w).flags & 1 as libc::c_int != 0) {
                (*(w as *mut libc::c_void as *mut ev_watcher)).priority =
                    if 0x4f as libc::c_int & 4 as libc::c_int != 0 {
                        2 as libc::c_int
                    } else { 0 as libc::c_int };
                (*w).rpid = pid;
                (*w).rstatus = status;
                k5ev_feed_event(loop_0, w as W as *mut libc::c_void,
                                EV_CHILD as libc::c_int);
            }
            w = (*(w as WL)).next as *mut ev_child
        };
    }
    #[c2rust::src_loc = "2626:11"]
    pub static mut childs: [WL; 16] =
        [0 as *const ev_watcher_list as *mut ev_watcher_list; 16];
    #[c2rust::src_loc = "2775:1"]
    pub unsafe extern "C" fn k5ev_backend(mut loop_0: *mut ev_loop)
     -> libc::c_uint {
        return (*loop_0).backend as libc::c_uint;
    }
    #[inline(never)]
    #[c2rust::src_loc = "2833:1"]
    pub unsafe extern "C" fn loop_init(mut loop_0: *mut ev_loop,
                                       mut flags: libc::c_uint) {
        if (*loop_0).backend == 0 {
            (*loop_0).origflags = flags;
            if flags & EVFLAG_FORKCHECK as libc::c_int as libc::c_uint != 0 {
                (*loop_0).curpid = getpid()
            }
            if flags & EVFLAG_NOENV as libc::c_int as libc::c_uint == 0 &&
                   enable_secure() == 0 &&
                   !getenv(b"LIBEV_FLAGS\x00" as *const u8 as
                               *const libc::c_char).is_null() {
                flags =
                    atoi(getenv(b"LIBEV_FLAGS\x00" as *const u8 as
                                    *const libc::c_char)) as libc::c_uint
            }
            (*loop_0).ev_rt_now = k5ev_time();
            (*loop_0).mn_now = get_clock();
            (*loop_0).now_floor = (*loop_0).mn_now;
            (*loop_0).rtmn_diff = (*loop_0).ev_rt_now - (*loop_0).mn_now;
            (*loop_0).invoke_cb =
                Some(k5ev_invoke_pending as
                         unsafe extern "C" fn(_: *mut ev_loop) -> ());
            (*loop_0).io_blocktime = 0.0f64;
            (*loop_0).timeout_blocktime = 0.0f64;
            (*loop_0).backend = 0 as libc::c_int;
            (*loop_0).backend_fd = -(1 as libc::c_int);
            ::std::ptr::write_volatile(&mut (*loop_0).sig_pending as
                                           *mut sig_atomic_t,
                                       0 as libc::c_int);
            ::std::ptr::write_volatile(&mut (*loop_0).pipe_write_skipped as
                                           *mut sig_atomic_t,
                                       0 as libc::c_int);
            ::std::ptr::write_volatile(&mut (*loop_0).pipe_write_wanted as
                                           *mut sig_atomic_t,
                                       0 as libc::c_int);
            (*loop_0).evpipe[0 as libc::c_int as usize] = -(1 as libc::c_int);
            (*loop_0).evpipe[1 as libc::c_int as usize] = -(1 as libc::c_int);
            (*loop_0).sigfd =
                if flags & EVFLAG_SIGNALFD as libc::c_int as libc::c_uint != 0
                   {
                    -(2 as libc::c_int)
                } else { -(1 as libc::c_int) };
            if flags & EVBACKEND_MASK as libc::c_int as libc::c_uint == 0 {
                flags |= k5ev_recommended_backends()
            }
            if (*loop_0).backend == 0 &&
                   flags & EVBACKEND_POLL as libc::c_int as libc::c_uint != 0
               {
                (*loop_0).backend = poll_init(loop_0, flags as libc::c_int)
            }
            if (*loop_0).backend == 0 &&
                   flags & EVBACKEND_SELECT as libc::c_int as libc::c_uint !=
                       0 {
                (*loop_0).backend = select_init(loop_0, flags as libc::c_int)
            }
            let ref mut fresh4 =
                (*(&mut (*loop_0).pending_w as *mut ev_prepare as
                       *mut libc::c_void as *mut ev_watcher)).pending;
            *fresh4 = 0 as libc::c_int;
            (*(&mut (*loop_0).pending_w as *mut ev_prepare as
                   *mut libc::c_void as *mut ev_watcher)).active = *fresh4;
            (*(&mut (*loop_0).pending_w as *mut ev_prepare as
                   *mut libc::c_void as *mut ev_watcher)).priority =
                0 as libc::c_int;
            (*loop_0).pending_w.cb =
                Some(pendingcb as
                         unsafe extern "C" fn(_: *mut ev_loop,
                                              _: *mut ev_prepare,
                                              _: libc::c_int) -> ());
            memmove(&mut (*(&mut (*loop_0).pending_w as *mut ev_prepare as
                                *mut ev_watcher)).cb as
                        *mut Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                         _: *mut ev_watcher,
                                                         _: libc::c_int)
                                        -> ()> as *mut libc::c_void,
                    &mut (*loop_0).pending_w.cb as
                        *mut Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                         _: *mut ev_prepare,
                                                         _: libc::c_int)
                                        -> ()> as *const libc::c_void,
                    ::std::mem::size_of::<Option<unsafe extern "C" fn(_:
                                                                          *mut ev_loop,
                                                                      _:
                                                                          *mut ev_prepare,
                                                                      _:
                                                                          libc::c_int)
                                                     -> ()>>() as
                        libc::c_ulong);
            let ref mut fresh5 =
                (*(&mut (*loop_0).pipe_w as *mut ev_io as *mut libc::c_void as
                       *mut ev_watcher)).pending;
            *fresh5 = 0 as libc::c_int;
            (*(&mut (*loop_0).pipe_w as *mut ev_io as *mut libc::c_void as
                   *mut ev_watcher)).active = *fresh5;
            (*(&mut (*loop_0).pipe_w as *mut ev_io as *mut libc::c_void as
                   *mut ev_watcher)).priority = 0 as libc::c_int;
            (*loop_0).pipe_w.cb =
                Some(pipecb as
                         unsafe extern "C" fn(_: *mut ev_loop, _: *mut ev_io,
                                              _: libc::c_int) -> ());
            memmove(&mut (*(&mut (*loop_0).pipe_w as *mut ev_io as
                                *mut ev_watcher)).cb as
                        *mut Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                         _: *mut ev_watcher,
                                                         _: libc::c_int)
                                        -> ()> as *mut libc::c_void,
                    &mut (*loop_0).pipe_w.cb as
                        *mut Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                         _: *mut ev_io,
                                                         _: libc::c_int)
                                        -> ()> as *const libc::c_void,
                    ::std::mem::size_of::<Option<unsafe extern "C" fn(_:
                                                                          *mut ev_loop,
                                                                      _:
                                                                          *mut ev_io,
                                                                      _:
                                                                          libc::c_int)
                                                     -> ()>>() as
                        libc::c_ulong);
            (*(&mut (*loop_0).pipe_w as *mut ev_io as *mut libc::c_void as
                   *mut ev_watcher)).priority =
                if 0x4f as libc::c_int & 4 as libc::c_int != 0 {
                    2 as libc::c_int
                } else { 0 as libc::c_int }
        };
    }
    #[c2rust::src_loc = "2483:1"]
    pub unsafe extern "C" fn pipecb(mut loop_0: *mut ev_loop,
                                    mut iow: *mut ev_io,
                                    mut revents: libc::c_int) {
        let mut i: libc::c_int = 0;
        if revents & EV_READ as libc::c_int != 0 {
            if (*loop_0).evpipe[0 as libc::c_int as usize] < 0 as libc::c_int
               {
                let mut counter: uint64_t = 0;
                read((*loop_0).evpipe[1 as libc::c_int as usize],
                     &mut counter as *mut uint64_t as *mut libc::c_void,
                     ::std::mem::size_of::<uint64_t>() as libc::c_ulong);
            } else {
                let mut dummy: [libc::c_char; 4] = [0; 4];
                read((*loop_0).evpipe[0 as libc::c_int as usize],
                     &mut dummy as *mut [libc::c_char; 4] as
                         *mut libc::c_void,
                     ::std::mem::size_of::<[libc::c_char; 4]>() as
                         libc::c_ulong);
            }
        }
        ::std::ptr::write_volatile(&mut (*loop_0).pipe_write_skipped as
                                       *mut sig_atomic_t, 0 as libc::c_int);
        asm!("mfence" : : : "memory" : "volatile");
        if (*loop_0).sig_pending != 0 {
            ::std::ptr::write_volatile(&mut (*loop_0).sig_pending as
                                           *mut sig_atomic_t,
                                       0 as libc::c_int);
            asm!("mfence" : : : "memory" : "volatile");
            i = 64 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int;
            loop  {
                let fresh6 = i;
                i = i - 1;
                if !(fresh6 != 0) { break ; }
                if (signals[i as usize].pending != 0) as libc::c_int as
                       libc::c_long != 0 {
                    k5ev_feed_signal_event(loop_0, i + 1 as libc::c_int);
                }
            }
        };
    }
    #[inline]
    #[c2rust::src_loc = "3826:1"]
    pub unsafe extern "C" fn ev_stop(mut loop_0: *mut ev_loop, mut w: W) {
        k5ev_unref(loop_0);
        (*w).active = 0 as libc::c_int;
    }
    #[inline(never)]
    #[c2rust::src_loc = "1987:1"]
    pub unsafe extern "C" fn pendingcb(mut loop_0: *mut ev_loop,
                                       mut w: *mut ev_prepare,
                                       mut revents: libc::c_int) {
    }
    #[inline]
    #[c2rust::src_loc = "2054:1"]
    pub unsafe extern "C" fn fd_event(mut loop_0: *mut ev_loop,
                                      mut fd: libc::c_int,
                                      mut revents: libc::c_int) {
        let mut anfd: *mut ANFD = (*loop_0).anfds.offset(fd as isize);
        if ((*anfd).reify == 0) as libc::c_int as libc::c_long != 0 {
            fd_event_nocheck(loop_0, fd, revents);
        };
    }
    #[inline]
    #[c2rust::src_loc = "2037:1"]
    pub unsafe extern "C" fn fd_event_nocheck(mut loop_0: *mut ev_loop,
                                              mut fd: libc::c_int,
                                              mut revents: libc::c_int) {
        let mut anfd: *mut ANFD = (*loop_0).anfds.offset(fd as isize);
        let mut w: *mut ev_io = 0 as *mut ev_io;
        w = (*anfd).head as *mut ev_io;
        while !w.is_null() {
            let mut ev: libc::c_int = (*w).events & revents;
            if ev != 0 {
                k5ev_feed_event(loop_0, w as W as *mut libc::c_void, ev);
            }
            w = (*(w as WL)).next as *mut ev_io
        };
    }
    #[inline(never)]
    #[c2rust::src_loc = "2183:1"]
    pub unsafe extern "C" fn fd_enomem(mut loop_0: *mut ev_loop) {
        let mut fd: libc::c_int = 0;
        fd = (*loop_0).anfdmax;
        loop  {
            let fresh9 = fd;
            fd = fd - 1;
            if !(fresh9 != 0) { break ; }
            if !((*(*loop_0).anfds.offset(fd as isize)).events != 0) {
                continue ;
            }
            fd_kill(loop_0, fd);
            break ;
        };
    }
    #[inline]
    #[c2rust::src_loc = "2147:1"]
    pub unsafe extern "C" fn fd_kill(mut loop_0: *mut ev_loop,
                                     mut fd: libc::c_int) {
        let mut w: *mut ev_io = 0 as *mut ev_io;
        loop  {
            w = (*(*loop_0).anfds.offset(fd as isize)).head as *mut ev_io;
            if w.is_null() { break ; }
            k5ev_io_stop(loop_0, w);
            k5ev_feed_event(loop_0, w as W as *mut libc::c_void,
                            EV_ERROR as libc::c_int | EV_READ as libc::c_int |
                                EV_WRITE as libc::c_int);
        };
    }
    /* number of loop iterations */
    /* #ev_loop enters - #ev_loop leaves */
    /* abort if loop data corrupted */
    /* sleep at least this time, default 0 */
    /* sleep at least this time, default 0 */
    /* advanced stuff for threading etc. support, see docs */
    /* C++ doesn't allow the use of the ev_loop_callback typedef here, so we need to spell it out */
    /* number of pending events, if any */
    /* invoke all pending watchers */
    /*
 * stop/start the timer handling.
 */
    /* these may evaluate ev multiple times, and the other arguments at most once */
/* either use ev_init + ev_TYPE_set, or the ev_TYPE_init macro, below, to first initialise a watcher */
    /* nop, yes, this is a serious in-joke */
    /* nop, yes, this is a serious in-joke */
    /* nop, yes, this is a serious in-joke */
    /* nop, yes, this is a serious in-joke */
    /* nop, yes, this is a serious in-joke */
    /* nop, yes, this is a serious in-joke */
    /* ro, true when watcher is waiting for callback invocation */
    /* ro, true when the watcher has been started */
    /* rw */
    /* stopping (enabling, adding) a watcher does nothing if it is already running */
/* stopping (disabling, deleting) a watcher does nothing unless it's already running */
    /* feeds an event into a watcher as if the event actually occurred */
/* accepts any ev_watcher type */
    /* stops if active and no repeat, restarts if active and repeating, starts if inactive and repeating */
    /* return remaining time */
    /* only supported in the default loop */
    /* only supported in the default loop */
    /* unattach from signal */
    #[c2rust::src_loc = "4581:1"]
    pub unsafe extern "C" fn k5ev_idle_stop(mut loop_0: *mut ev_loop,
                                            mut w: *mut ev_idle) {
        clear_pending(loop_0, w as W);
        if (0 as libc::c_int +
                (*(w as *mut libc::c_void as *mut ev_watcher)).active == 0) as
               libc::c_int as libc::c_long != 0 {
            return
        }
        let mut active: libc::c_int = (*(w as W)).active;
        (*loop_0).idlecnt[((*(w as W)).priority -
                               (if 0x4f as libc::c_int & 4 as libc::c_int != 0
                                   {
                                    -(2 as libc::c_int)
                                } else { 0 as libc::c_int })) as usize] -= 1;
        let ref mut fresh10 =
            *(*loop_0).idles[((*(w as W)).priority -
                                  (if 0x4f as libc::c_int & 4 as libc::c_int
                                          != 0 {
                                       -(2 as libc::c_int)
                                   } else { 0 as libc::c_int })) as
                                 usize].offset((active - 1 as libc::c_int) as
                                                   isize);
        *fresh10 =
            *(*loop_0).idles[((*(w as W)).priority -
                                  (if 0x4f as libc::c_int & 4 as libc::c_int
                                          != 0 {
                                       -(2 as libc::c_int)
                                   } else { 0 as libc::c_int })) as
                                 usize].offset((*loop_0).idlecnt[((*(w as
                                                                         W)).priority
                                                                      -
                                                                      (if 0x4f
                                                                              as
                                                                              libc::c_int
                                                                              &
                                                                              4
                                                                                  as
                                                                                  libc::c_int
                                                                              !=
                                                                              0
                                                                          {
                                                                           -(2
                                                                                 as
                                                                                 libc::c_int)
                                                                       } else {
                                                                           0
                                                                               as
                                                                               libc::c_int
                                                                       })) as
                                                                     usize] as
                                                   isize);
        (*(*(*loop_0).idles[((*(w as W)).priority -
                                 (if 0x4f as libc::c_int & 4 as libc::c_int !=
                                         0 {
                                      -(2 as libc::c_int)
                                  } else { 0 as libc::c_int })) as
                                usize].offset((active - 1 as libc::c_int) as
                                                  isize) as W)).active =
            active;
        ev_stop(loop_0, w as W);
        (*loop_0).idleall -= 1;
    }
    #[c2rust::src_loc = "4558:1"]
    pub unsafe extern "C" fn k5ev_idle_start(mut loop_0: *mut ev_loop,
                                             mut w: *mut ev_idle) {
        if (0 as libc::c_int +
                (*(w as *mut libc::c_void as *mut ev_watcher)).active != 0) as
               libc::c_int as libc::c_long != 0 {
            return
        }
        pri_adjust(loop_0, w as W);
        (*loop_0).idlecnt[((*(w as W)).priority -
                               (if 0x4f as libc::c_int & 4 as libc::c_int != 0
                                   {
                                    -(2 as libc::c_int)
                                } else { 0 as libc::c_int })) as usize] += 1;
        let mut active: libc::c_int =
            (*loop_0).idlecnt[((*(w as W)).priority -
                                   (if 0x4f as libc::c_int & 4 as libc::c_int
                                           != 0 {
                                        -(2 as libc::c_int)
                                    } else { 0 as libc::c_int })) as usize];
        (*loop_0).idleall += 1;
        ev_start(loop_0, w as W, active);
        if (active >
                (*loop_0).idlemax[((*(w as W)).priority -
                                       (if 0x4f as libc::c_int &
                                               4 as libc::c_int != 0 {
                                            -(2 as libc::c_int)
                                        } else { 0 as libc::c_int })) as
                                      usize]) as libc::c_int as libc::c_long
               != 0 {
            let mut ocur_: libc::c_int =
                (*loop_0).idlemax[((*(w as W)).priority -
                                       (if 0x4f as libc::c_int &
                                               4 as libc::c_int != 0 {
                                            -(2 as libc::c_int)
                                        } else { 0 as libc::c_int })) as
                                      usize];
            (*loop_0).idles[((*(w as W)).priority -
                                 (if 0x4f as libc::c_int & 4 as libc::c_int !=
                                         0 {
                                      -(2 as libc::c_int)
                                  } else { 0 as libc::c_int })) as usize] =
                array_realloc(::std::mem::size_of::<*mut ev_idle>() as
                                  libc::c_ulong as libc::c_int,
                              (*loop_0).idles[((*(w as W)).priority -
                                                   (if 0x4f as libc::c_int &
                                                           4 as libc::c_int !=
                                                           0 {
                                                        -(2 as libc::c_int)
                                                    } else {
                                                        0 as libc::c_int
                                                    })) as usize] as
                                  *mut libc::c_void,
                              &mut *(*loop_0).idlemax.as_mut_ptr().offset(((*(w
                                                                                  as
                                                                                  W)).priority
                                                                               -
                                                                               (if 0x4f
                                                                                       as
                                                                                       libc::c_int
                                                                                       &
                                                                                       4
                                                                                           as
                                                                                           libc::c_int
                                                                                       !=
                                                                                       0
                                                                                   {
                                                                                    -(2
                                                                                          as
                                                                                          libc::c_int)
                                                                                } else {
                                                                                    0
                                                                                        as
                                                                                        libc::c_int
                                                                                }))
                                                                              as
                                                                              isize),
                              active) as *mut *mut ev_idle
        }
        let ref mut fresh11 =
            *(*loop_0).idles[((*(w as W)).priority -
                                  (if 0x4f as libc::c_int & 4 as libc::c_int
                                          != 0 {
                                       -(2 as libc::c_int)
                                   } else { 0 as libc::c_int })) as
                                 usize].offset((active - 1 as libc::c_int) as
                                                   isize);
        *fresh11 = w;
    }
    #[c2rust::src_loc = "4181:1"]
    pub unsafe extern "C" fn k5ev_child_stop(mut loop_0: *mut ev_loop,
                                             mut w: *mut ev_child) {
        clear_pending(loop_0, w as W);
        if (0 as libc::c_int +
                (*(w as *mut libc::c_void as *mut ev_watcher)).active == 0) as
               libc::c_int as libc::c_long != 0 {
            return
        }
        wlist_del(&mut *childs.as_mut_ptr().offset(((*w).pid &
                                                        (if 0x4f as
                                                                libc::c_int &
                                                                2 as
                                                                    libc::c_int
                                                                != 0 {
                                                             16 as libc::c_int
                                                         } else {
                                                             1 as libc::c_int
                                                         }) -
                                                            1 as libc::c_int)
                                                       as isize), w as WL);
        ev_stop(loop_0, w as W);
    }
    #[c2rust::src_loc = "4164:1"]
    pub unsafe extern "C" fn k5ev_child_start(mut loop_0: *mut ev_loop,
                                              mut w: *mut ev_child) {
        if loop_0 == k5ev_default_loop_ptr {
        } else {
            __assert_fail(b"(\"libev: child watchers are only supported in the default loop\", loop == ev_default_loop_ptr)\x00"
                              as *const u8 as *const libc::c_char,
                          b"./ev.c\x00" as *const u8 as *const libc::c_char,
                          4168 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 52],
                                                    &[libc::c_char; 52]>(b"void k5ev_child_start(struct ev_loop *, ev_child *)\x00")).as_ptr());
        }
        if (0 as libc::c_int +
                (*(w as *mut libc::c_void as *mut ev_watcher)).active != 0) as
               libc::c_int as libc::c_long != 0 {
            return
        }
        ev_start(loop_0, w as W, 1 as libc::c_int);
        wlist_add(&mut *childs.as_mut_ptr().offset(((*w).pid &
                                                        (if 0x4f as
                                                                libc::c_int &
                                                                2 as
                                                                    libc::c_int
                                                                != 0 {
                                                             16 as libc::c_int
                                                         } else {
                                                             1 as libc::c_int
                                                         }) -
                                                            1 as libc::c_int)
                                                       as isize), w as WL);
    }
    #[inline(never)]
    #[c2rust::src_loc = "3880:1"]
    pub unsafe extern "C" fn k5ev_timer_start(mut loop_0: *mut ev_loop,
                                              mut w: *mut ev_timer) {
        if (0 as libc::c_int +
                (*(w as *mut libc::c_void as *mut ev_watcher)).active != 0) as
               libc::c_int as libc::c_long != 0 {
            return
        }
        let ref mut fresh12 = (*(w as WT)).at;
        *fresh12 += (*loop_0).mn_now;
        if (*w).repeat >= 0.0f64 {
        } else {
            __assert_fail(b"(\"libev: ev_timer_start called with negative timer repeat value\", w->repeat >= 0.)\x00"
                              as *const u8 as *const libc::c_char,
                          b"./ev.c\x00" as *const u8 as *const libc::c_char,
                          3888 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 52],
                                                    &[libc::c_char; 52]>(b"void k5ev_timer_start(struct ev_loop *, ev_timer *)\x00")).as_ptr());
        }
        (*loop_0).timercnt += 1;
        ev_start(loop_0, w as W,
                 (*loop_0).timercnt + (4 as libc::c_int - 1 as libc::c_int) -
                     1 as libc::c_int);
        if ((*(w as W)).active + 1 as libc::c_int > (*loop_0).timermax) as
               libc::c_int as libc::c_long != 0 {
            let mut ocur_: libc::c_int = (*loop_0).timermax;
            (*loop_0).timers =
                array_realloc(::std::mem::size_of::<ANHE>() as libc::c_ulong
                                  as libc::c_int,
                              (*loop_0).timers as *mut libc::c_void,
                              &mut (*loop_0).timermax,
                              (*(w as W)).active + 1 as libc::c_int) as
                    *mut ANHE
        }
        let ref mut fresh13 =
            (*(*loop_0).timers.offset((*(w as W)).active as isize)).w;
        *fresh13 = w as WT;
        (*(*loop_0).timers.offset((*(w as W)).active as isize)).at =
            (*(*(*loop_0).timers.offset((*(w as W)).active as isize)).w).at;
        upheap((*loop_0).timers, (*(w as W)).active);
    }
    #[c2rust::src_loc = "3713:1"]
    pub unsafe extern "C" fn k5ev_break(mut loop_0: *mut ev_loop,
                                        mut how: libc::c_int) {
        ::std::ptr::write_volatile(&mut (*loop_0).loop_done as
                                       *mut sig_atomic_t, how);
    }
    #[inline]
    #[c2rust::src_loc = "3044:1"]
    pub unsafe extern "C" fn loop_fork(mut loop_0: *mut ev_loop) {
        if 0 as libc::c_int +
               (*(&mut (*loop_0).pipe_w as *mut ev_io as *mut libc::c_void as
                      *mut ev_watcher)).active != 0 &&
               (*loop_0).postfork as libc::c_int != 2 as libc::c_int {
            k5ev_ref(loop_0);
            k5ev_io_stop(loop_0, &mut (*loop_0).pipe_w);
            if (*loop_0).evpipe[0 as libc::c_int as usize] >= 0 as libc::c_int
               {
                close((*loop_0).evpipe[0 as libc::c_int as usize]);
            }
            evpipe_init(loop_0);
            k5ev_feed_event(loop_0,
                            &mut (*loop_0).pipe_w as *mut ev_io as
                                *mut libc::c_void, EV_CUSTOM as libc::c_int);
        }
        (*loop_0).postfork = 0 as libc::c_int as libc::c_char;
    }
    #[inline]
    #[c2rust::src_loc = "2072:1"]
    pub unsafe extern "C" fn fd_reify(mut loop_0: *mut ev_loop) {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*loop_0).fdchangecnt {
            let mut fd: libc::c_int = *(*loop_0).fdchanges.offset(i as isize);
            let mut anfd: *mut ANFD = (*loop_0).anfds.offset(fd as isize);
            let mut w: *mut ev_io = 0 as *mut ev_io;
            let mut o_events: libc::c_uchar = (*anfd).events;
            let mut o_reify: libc::c_uchar = (*anfd).reify;
            (*anfd).reify = 0 as libc::c_int as libc::c_uchar;
            (*anfd).events = 0 as libc::c_int as libc::c_uchar;
            w = (*anfd).head as *mut ev_io;
            while !w.is_null() {
                (*anfd).events =
                    ((*anfd).events as libc::c_int |
                         (*w).events as libc::c_uchar as libc::c_int) as
                        libc::c_uchar;
                w = (*(w as WL)).next as *mut ev_io
            }
            if o_events as libc::c_int != (*anfd).events as libc::c_int {
                o_reify = EV__IOFDSET as libc::c_int as libc::c_uchar
            }
            if o_reify as libc::c_int & EV__IOFDSET as libc::c_int != 0 {
                (*loop_0).backend_modify.expect("non-null function pointer")(loop_0,
                                                                             fd,
                                                                             o_events
                                                                                 as
                                                                                 libc::c_int,
                                                                             (*anfd).events
                                                                                 as
                                                                                 libc::c_int);
            }
            i += 1
        }
        (*loop_0).fdchangecnt = 0 as libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "2327:1"]
    pub unsafe extern "C" fn upheap(mut heap: *mut ANHE, mut k: libc::c_int) {
        let mut he: ANHE = *heap.offset(k as isize);
        loop  {
            let mut p: libc::c_int =
                (k - (4 as libc::c_int - 1 as libc::c_int) - 1 as libc::c_int)
                    / 4 as libc::c_int +
                    (4 as libc::c_int - 1 as libc::c_int);
            if p == k || (*heap.offset(p as isize)).at <= he.at { break ; }
            *heap.offset(k as isize) = *heap.offset(p as isize);
            (*((*heap.offset(k as isize)).w as W)).active = k;
            k = p
        }
        *heap.offset(k as isize) = he;
        (*(he.w as W)).active = k;
    }
    #[inline]
    #[c2rust::src_loc = "2247:1"]
    pub unsafe extern "C" fn downheap(mut heap: *mut ANHE, mut N: libc::c_int,
                                      mut k: libc::c_int) {
        let mut he: ANHE = *heap.offset(k as isize);
        let mut E: *mut ANHE =
            heap.offset(N as
                            isize).offset((4 as libc::c_int -
                                               1 as libc::c_int) as isize);
        loop  {
            let mut minat: ev_tstamp = 0.;
            let mut minpos: *mut ANHE = 0 as *mut ANHE;
            let mut pos: *mut ANHE =
                heap.offset((4 as libc::c_int *
                                 (k - (4 as libc::c_int - 1 as libc::c_int)))
                                as
                                isize).offset((4 as libc::c_int -
                                                   1 as libc::c_int) as
                                                  isize).offset(1 as
                                                                    libc::c_int
                                                                    as isize);
            if (pos.offset(4 as libc::c_int as
                               isize).offset(-(1 as libc::c_int as isize)) <
                    E) as libc::c_int as libc::c_long != 0 {
                minpos = pos.offset(0 as libc::c_int as isize);
                minat = (*minpos).at;
                if (*pos.offset(1 as libc::c_int as isize)).at < minat {
                    minpos = pos.offset(1 as libc::c_int as isize);
                    minat = (*minpos).at
                }
                if (*pos.offset(2 as libc::c_int as isize)).at < minat {
                    minpos = pos.offset(2 as libc::c_int as isize);
                    minat = (*minpos).at
                }
                if (*pos.offset(3 as libc::c_int as isize)).at < minat {
                    minpos = pos.offset(3 as libc::c_int as isize);
                    minat = (*minpos).at
                }
            } else {
                if !(pos < E) { break ; }
                minpos = pos.offset(0 as libc::c_int as isize);
                minat = (*minpos).at;
                if pos.offset(1 as libc::c_int as isize) < E &&
                       (*pos.offset(1 as libc::c_int as isize)).at < minat {
                    minpos = pos.offset(1 as libc::c_int as isize);
                    minat = (*minpos).at
                }
                if pos.offset(2 as libc::c_int as isize) < E &&
                       (*pos.offset(2 as libc::c_int as isize)).at < minat {
                    minpos = pos.offset(2 as libc::c_int as isize);
                    minat = (*minpos).at
                }
                if pos.offset(3 as libc::c_int as isize) < E &&
                       (*pos.offset(3 as libc::c_int as isize)).at < minat {
                    minpos = pos.offset(3 as libc::c_int as isize);
                    minat = (*minpos).at
                }
            }
            if he.at <= minat { break ; }
            *heap.offset(k as isize) = *minpos;
            (*((*minpos).w as W)).active = k;
            k =
                minpos.wrapping_offset_from(heap) as libc::c_long as
                    libc::c_int
        }
        *heap.offset(k as isize) = he;
        (*(he.w as W)).active = k;
    }
    #[inline]
    #[c2rust::src_loc = "2349:1"]
    pub unsafe extern "C" fn adjustheap(mut heap: *mut ANHE,
                                        mut N: libc::c_int,
                                        mut k: libc::c_int) {
        if k > 4 as libc::c_int - 1 as libc::c_int &&
               (*heap.offset(k as isize)).at <=
                   (*heap.offset(((k - (4 as libc::c_int - 1 as libc::c_int) -
                                       1 as libc::c_int) / 4 as libc::c_int +
                                      (4 as libc::c_int - 1 as libc::c_int))
                                     as isize)).at {
            upheap(heap, k);
        } else { downheap(heap, N, k); };
    }
    #[inline(never)]
    #[c2rust::src_loc = "3904:1"]
    pub unsafe extern "C" fn k5ev_timer_stop(mut loop_0: *mut ev_loop,
                                             mut w: *mut ev_timer) {
        clear_pending(loop_0, w as W);
        if (0 as libc::c_int +
                (*(w as *mut libc::c_void as *mut ev_watcher)).active == 0) as
               libc::c_int as libc::c_long != 0 {
            return
        }
        let mut active: libc::c_int = (*(w as W)).active;
        if (*(*loop_0).timers.offset(active as isize)).w == w as WT {
        } else {
            __assert_fail(b"(\"libev: internal timer heap corruption\", ANHE_w (timers [active]) == (WT)w)\x00"
                              as *const u8 as *const libc::c_char,
                          b"./ev.c\x00" as *const u8 as *const libc::c_char,
                          3916 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 51],
                                                    &[libc::c_char; 51]>(b"void k5ev_timer_stop(struct ev_loop *, ev_timer *)\x00")).as_ptr());
        }
        (*loop_0).timercnt -= 1;
        if (active <
                (*loop_0).timercnt + (4 as libc::c_int - 1 as libc::c_int)) as
               libc::c_int as libc::c_long != 0 {
            *(*loop_0).timers.offset(active as isize) =
                *(*loop_0).timers.offset(((*loop_0).timercnt +
                                              (4 as libc::c_int -
                                                   1 as libc::c_int)) as
                                             isize);
            adjustheap((*loop_0).timers, (*loop_0).timercnt, active);
        }
        let ref mut fresh14 = (*(w as WT)).at;
        *fresh14 -= (*loop_0).mn_now;
        ev_stop(loop_0, w as W);
    }
    #[inline]
    #[c2rust::src_loc = "2011:1"]
    pub unsafe extern "C" fn feed_reverse(mut loop_0: *mut ev_loop,
                                          mut w: W) {
        if ((*loop_0).rfeedcnt + 1 as libc::c_int > (*loop_0).rfeedmax) as
               libc::c_int as libc::c_long != 0 {
            let mut ocur_: libc::c_int = (*loop_0).rfeedmax;
            (*loop_0).rfeeds =
                array_realloc(::std::mem::size_of::<W>() as libc::c_ulong as
                                  libc::c_int,
                              (*loop_0).rfeeds as *mut libc::c_void,
                              &mut (*loop_0).rfeedmax,
                              (*loop_0).rfeedcnt + 1 as libc::c_int) as *mut W
        }
        let fresh15 = (*loop_0).rfeedcnt;
        (*loop_0).rfeedcnt = (*loop_0).rfeedcnt + 1;
        let ref mut fresh16 = *(*loop_0).rfeeds.offset(fresh15 as isize);
        *fresh16 = w;
    }
    #[inline]
    #[c2rust::src_loc = "2018:1"]
    pub unsafe extern "C" fn feed_reverse_done(mut loop_0: *mut ev_loop,
                                               mut revents: libc::c_int) {
        loop  {
            (*loop_0).rfeedcnt -= 1;
            k5ev_feed_event(loop_0,
                            *(*loop_0).rfeeds.offset((*loop_0).rfeedcnt as
                                                         isize) as
                                *mut libc::c_void, revents);
            if !((*loop_0).rfeedcnt != 0) { break ; }
        };
    }
    #[inline]
    #[c2rust::src_loc = "3327:1"]
    pub unsafe extern "C" fn timers_reify(mut loop_0: *mut ev_loop) {
        if (*loop_0).timercnt != 0 &&
               (*(*loop_0).timers.offset((4 as libc::c_int - 1 as libc::c_int)
                                             as isize)).at < (*loop_0).mn_now
           {
            loop  {
                let mut w: *mut ev_timer =
                    (*(*loop_0).timers.offset((4 as libc::c_int -
                                                   1 as libc::c_int) as
                                                  isize)).w as *mut ev_timer;
                if (*w).repeat != 0. {
                    let ref mut fresh17 = (*(w as WT)).at;
                    *fresh17 += (*w).repeat;
                    if (*(w as WT)).at < (*loop_0).mn_now {
                        (*(w as WT)).at = (*loop_0).mn_now
                    }
                    if (*w).repeat > 0.0f64 {
                    } else {
                        __assert_fail(b"(\"libev: negative ev_timer repeat value found while processing timers\", w->repeat > 0.)\x00"
                                          as *const u8 as *const libc::c_char,
                                      b"./ev.c\x00" as *const u8 as
                                          *const libc::c_char,
                                      3347 as libc::c_int as libc::c_uint,
                                      (*::std::mem::transmute::<&[u8; 36],
                                                                &[libc::c_char; 36]>(b"void timers_reify(struct ev_loop *)\x00")).as_ptr());
                    }
                    (*(*loop_0).timers.offset((4 as libc::c_int -
                                                   1 as libc::c_int) as
                                                  isize)).at =
                        (*(*(*loop_0).timers.offset((4 as libc::c_int -
                                                         1 as libc::c_int) as
                                                        isize)).w).at;
                    downheap((*loop_0).timers, (*loop_0).timercnt,
                             4 as libc::c_int - 1 as libc::c_int);
                } else { k5ev_timer_stop(loop_0, w); }
                feed_reverse(loop_0, w as W);
                if !((*loop_0).timercnt != 0 &&
                         (*(*loop_0).timers.offset((4 as libc::c_int -
                                                        1 as libc::c_int) as
                                                       isize)).at <
                             (*loop_0).mn_now) {
                    break ;
                }
            }
            feed_reverse_done(loop_0, EV_TIMER as libc::c_int);
        };
    }
    #[inline]
    #[c2rust::src_loc = "2026:1"]
    pub unsafe extern "C" fn queue_events(mut loop_0: *mut ev_loop,
                                          mut events: *mut W,
                                          mut eventcnt: libc::c_int,
                                          mut type_0: libc::c_int) {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < eventcnt {
            k5ev_feed_event(loop_0,
                            *events.offset(i as isize) as *mut libc::c_void,
                            type_0);
            i += 1
        };
    }
    #[inline]
    #[c2rust::src_loc = "3304:1"]
    pub unsafe extern "C" fn idle_reify(mut loop_0: *mut ev_loop) {
        if ((*loop_0).idleall != 0) as libc::c_int as libc::c_long != 0 {
            let mut pri: libc::c_int = 0;
            pri =
                (if 0x4f as libc::c_int & 4 as libc::c_int != 0 {
                     2 as libc::c_int
                 } else { 0 as libc::c_int }) -
                    (if 0x4f as libc::c_int & 4 as libc::c_int != 0 {
                         -(2 as libc::c_int)
                     } else { 0 as libc::c_int }) + 1 as libc::c_int;
            loop  {
                let fresh18 = pri;
                pri = pri - 1;
                if !(fresh18 != 0) { break ; }
                if (*loop_0).pendingcnt[pri as usize] != 0 { break ; }
                if !((*loop_0).idlecnt[pri as usize] != 0) { continue ; }
                queue_events(loop_0, (*loop_0).idles[pri as usize] as *mut W,
                             (*loop_0).idlecnt[pri as usize],
                             EV_IDLE as libc::c_int);
                break ;
            }
        };
    }
    #[c2rust::src_loc = "3541:1"]
    pub unsafe extern "C" fn k5ev_run(mut loop_0: *mut ev_loop,
                                      mut flags: libc::c_int) -> libc::c_int {
        (*loop_0).loop_depth = (*loop_0).loop_depth.wrapping_add(1);
        if (*loop_0).loop_done != 0x80 as libc::c_int {
        } else {
            __assert_fail(b"(\"libev: ev_loop recursion during release detected\", loop_done != EVBREAK_RECURSE)\x00"
                              as *const u8 as *const libc::c_char,
                          b"./ev.c\x00" as *const u8 as *const libc::c_char,
                          3548 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 36],
                                                    &[libc::c_char; 36]>(b"int k5ev_run(struct ev_loop *, int)\x00")).as_ptr());
        }
        ::std::ptr::write_volatile(&mut (*loop_0).loop_done as
                                       *mut sig_atomic_t,
                                   EVBREAK_CANCEL as libc::c_int);
        (*loop_0).invoke_cb.expect("non-null function pointer")(loop_0);
        loop  {
            if ((*loop_0).curpid != 0) as libc::c_int as libc::c_long != 0 {
                if (getpid() != (*loop_0).curpid) as libc::c_int as
                       libc::c_long != 0 {
                    (*loop_0).curpid = getpid();
                    (*loop_0).postfork = 1 as libc::c_int as libc::c_char
                }
            }
            if ((*loop_0).loop_done != 0) as libc::c_int as libc::c_long != 0
               {
                break ;
            }
            if ((*loop_0).postfork != 0) as libc::c_int as libc::c_long != 0 {
                loop_fork(loop_0);
            }
            fd_reify(loop_0);
            let mut waittime: ev_tstamp = 0.0f64;
            let mut sleeptime: ev_tstamp = 0.0f64;
            let mut prev_mn_now: ev_tstamp = (*loop_0).mn_now;
            time_update(loop_0, 1e100f64);
            ::std::ptr::write_volatile(&mut (*loop_0).pipe_write_wanted as
                                           *mut sig_atomic_t,
                                       1 as libc::c_int);
            asm!("mfence" : : : "memory" : "volatile");
            if !(flags & EVRUN_NOWAIT as libc::c_int != 0 ||
                     (*loop_0).idleall != 0 || (*loop_0).activecnt == 0 ||
                     (*loop_0).pipe_write_skipped != 0) as libc::c_int as
                   libc::c_long != 0 {
                waittime = 59.743f64;
                if (*loop_0).timercnt != 0 {
                    let mut to: ev_tstamp =
                        (*(*loop_0).timers.offset((4 as libc::c_int -
                                                       1 as libc::c_int) as
                                                      isize)).at -
                            (*loop_0).mn_now;
                    if waittime > to { waittime = to }
                }
                if (waittime < (*loop_0).timeout_blocktime) as libc::c_int as
                       libc::c_long != 0 {
                    waittime = (*loop_0).timeout_blocktime
                }
                if (waittime < (*loop_0).backend_mintime) as libc::c_int as
                       libc::c_long != 0 {
                    waittime = (*loop_0).backend_mintime
                }
                if ((*loop_0).io_blocktime != 0.) as libc::c_int as
                       libc::c_long != 0 {
                    sleeptime =
                        (*loop_0).io_blocktime -
                            ((*loop_0).mn_now - prev_mn_now);
                    if sleeptime > waittime - (*loop_0).backend_mintime {
                        sleeptime = waittime - (*loop_0).backend_mintime
                    }
                    if (sleeptime > 0.0f64) as libc::c_int as libc::c_long !=
                           0 {
                        k5ev_sleep(sleeptime);
                        waittime -= sleeptime
                    }
                }
            }
            (*loop_0).loop_count = (*loop_0).loop_count.wrapping_add(1);
            ::std::ptr::write_volatile(&mut (*loop_0).loop_done as
                                           *mut sig_atomic_t,
                                       0x80 as libc::c_int);
            if 1 as libc::c_int != 0 {
            } else {
                __assert_fail(b"(loop_done = EVBREAK_RECURSE, 1)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"./ev.c\x00" as *const u8 as
                                  *const libc::c_char,
                              3660 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 36],
                                                        &[libc::c_char; 36]>(b"int k5ev_run(struct ev_loop *, int)\x00")).as_ptr());
            }
            (*loop_0).backend_poll.expect("non-null function pointer")(loop_0,
                                                                       waittime);
            ::std::ptr::write_volatile(&mut (*loop_0).loop_done as
                                           *mut sig_atomic_t,
                                       EVBREAK_CANCEL as libc::c_int);
            if 1 as libc::c_int != 0 {
            } else {
                __assert_fail(b"(loop_done = EVBREAK_CANCEL, 1)\x00" as
                                  *const u8 as *const libc::c_char,
                              b"./ev.c\x00" as *const u8 as
                                  *const libc::c_char,
                              3662 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 36],
                                                        &[libc::c_char; 36]>(b"int k5ev_run(struct ev_loop *, int)\x00")).as_ptr());
            }
            ::std::ptr::write_volatile(&mut (*loop_0).pipe_write_wanted as
                                           *mut sig_atomic_t,
                                       0 as libc::c_int);
            asm!("" : : : "memory" : "volatile");
            if (*loop_0).pipe_write_skipped != 0 {
                if 0 as libc::c_int +
                       (*(&mut (*loop_0).pipe_w as *mut ev_io as
                              *mut libc::c_void as *mut ev_watcher)).active !=
                       0 {
                } else {
                    __assert_fail(b"(\"libev: pipe_w not active, but pipe not written\", ev_is_active (&pipe_w))\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"./ev.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  3669 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 36],
                                                            &[libc::c_char; 36]>(b"int k5ev_run(struct ev_loop *, int)\x00")).as_ptr());
                }
                k5ev_feed_event(loop_0,
                                &mut (*loop_0).pipe_w as *mut ev_io as
                                    *mut libc::c_void,
                                EV_CUSTOM as libc::c_int);
            }
            time_update(loop_0, waittime + sleeptime);
            timers_reify(loop_0);
            idle_reify(loop_0);
            (*loop_0).invoke_cb.expect("non-null function pointer")(loop_0);
            if !(((*loop_0).activecnt != 0 && (*loop_0).loop_done == 0 &&
                      flags &
                          (EVRUN_ONCE as libc::c_int |
                               EVRUN_NOWAIT as libc::c_int) == 0) as
                     libc::c_int as libc::c_long != 0) {
                break ;
            }
        }
        if (*loop_0).loop_done == EVBREAK_ONE as libc::c_int {
            ::std::ptr::write_volatile(&mut (*loop_0).loop_done as
                                           *mut sig_atomic_t,
                                       EVBREAK_CANCEL as libc::c_int)
        }
        (*loop_0).loop_depth = (*loop_0).loop_depth.wrapping_sub(1);
        return (*loop_0).activecnt;
    }
    #[inline(never)]
    #[c2rust::src_loc = "3457:1"]
    pub unsafe extern "C" fn timers_reschedule(mut loop_0: *mut ev_loop,
                                               mut adjust: ev_tstamp) {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*loop_0).timercnt {
            let mut he: *mut ANHE =
                (*loop_0).timers.offset(i as
                                            isize).offset((4 as libc::c_int -
                                                               1 as
                                                                   libc::c_int)
                                                              as isize);
            (*(*he).w).at += adjust;
            (*he).at = (*(*he).w).at;
            i += 1
        };
    }
    #[inline]
    #[c2rust::src_loc = "3472:1"]
    pub unsafe extern "C" fn time_update(mut loop_0: *mut ev_loop,
                                         mut max_block: ev_tstamp) {
        (*loop_0).ev_rt_now = k5ev_time();
        if ((*loop_0).mn_now > (*loop_0).ev_rt_now ||
                (*loop_0).ev_rt_now > (*loop_0).mn_now + max_block + 1.0f64)
               as libc::c_int as libc::c_long != 0 {
            timers_reschedule(loop_0, (*loop_0).ev_rt_now - (*loop_0).mn_now);
        }
        (*loop_0).mn_now = (*loop_0).ev_rt_now;
    }
    #[c2rust::src_loc = "3255:1"]
    pub unsafe extern "C" fn k5ev_loop_fork(mut loop_0: *mut ev_loop) {
        (*loop_0).postfork = 1 as libc::c_int as libc::c_char;
    }
    #[inline(never)]
    #[c2rust::src_loc = "4123:1"]
    pub unsafe extern "C" fn k5ev_signal_stop(mut loop_0: *mut ev_loop,
                                              mut w: *mut ev_signal) {
        clear_pending(loop_0, w as W);
        if (0 as libc::c_int +
                (*(w as *mut libc::c_void as *mut ev_watcher)).active == 0) as
               libc::c_int as libc::c_long != 0 {
            return
        }
        wlist_del(&mut (*signals.as_mut_ptr().offset(((*w).signum -
                                                          1 as libc::c_int) as
                                                         isize)).head,
                  w as WL);
        ev_stop(loop_0, w as W);
        if signals[((*w).signum - 1 as libc::c_int) as usize].head.is_null() {
            signals[((*w).signum - 1 as libc::c_int) as usize].loop_0 =
                0 as *mut ev_loop;
            if (*loop_0).sigfd >= 0 as libc::c_int {
                let mut ss: sigset_t = __sigset_t{__val: [0; 16],};
                sigemptyset(&mut ss);
                sigaddset(&mut ss, (*w).signum);
                sigdelset(&mut (*loop_0).sigfd_set, (*w).signum);
                signalfd((*loop_0).sigfd, &mut (*loop_0).sigfd_set,
                         0 as libc::c_int);
                sigprocmask(1 as libc::c_int, &mut ss, 0 as *mut sigset_t);
            } else { signal((*w).signum, None); }
        };
    }
    #[c2rust::src_loc = "2930:1"]
    pub unsafe extern "C" fn k5ev_loop_destroy(mut loop_0: *mut ev_loop) {
        let mut i: libc::c_int = 0;
        if loop_0.is_null() { return }
        if ev_is_default_loop(loop_0) != 0 &&
               0 as libc::c_int +
                   (*(&mut childev as *mut ev_signal as *mut libc::c_void as
                          *mut ev_watcher)).active != 0 {
            k5ev_ref(loop_0);
            k5ev_signal_stop(loop_0, &mut childev);
        }
        if 0 as libc::c_int +
               (*(&mut (*loop_0).pipe_w as *mut ev_io as *mut libc::c_void as
                      *mut ev_watcher)).active != 0 {
            if (*loop_0).evpipe[0 as libc::c_int as usize] >= 0 as libc::c_int
               {
                close((*loop_0).evpipe[0 as libc::c_int as usize]);
            }
            if (*loop_0).evpipe[1 as libc::c_int as usize] >= 0 as libc::c_int
               {
                close((*loop_0).evpipe[1 as libc::c_int as usize]);
            }
        }
        if 0 as libc::c_int +
               (*(&mut (*loop_0).sigfd_w as *mut ev_io as *mut libc::c_void as
                      *mut ev_watcher)).active != 0 {
            close((*loop_0).sigfd);
        }
        if (*loop_0).backend_fd >= 0 as libc::c_int {
            close((*loop_0).backend_fd);
        }
        if (*loop_0).backend == EVBACKEND_POLL as libc::c_int {
            poll_destroy(loop_0);
        }
        if (*loop_0).backend == EVBACKEND_SELECT as libc::c_int {
            select_destroy(loop_0);
        }
        i =
            (if 0x4f as libc::c_int & 4 as libc::c_int != 0 {
                 2 as libc::c_int
             } else { 0 as libc::c_int }) -
                (if 0x4f as libc::c_int & 4 as libc::c_int != 0 {
                     -(2 as libc::c_int)
                 } else { 0 as libc::c_int }) + 1 as libc::c_int;
        loop  {
            let fresh19 = i;
            i = i - 1;
            if !(fresh19 != 0) { break ; }
            ev_realloc((*loop_0).pendings[i as usize] as *mut libc::c_void,
                       0 as libc::c_int as libc::c_long);
            (*loop_0).pendingmax[i as usize] = 0 as libc::c_int;
            (*loop_0).pendingcnt[i as usize] =
                (*loop_0).pendingmax[i as usize];
            (*loop_0).pendings[i as usize] = 0 as *mut ANPENDING;
            ev_realloc((*loop_0).idles[i as usize] as *mut libc::c_void,
                       0 as libc::c_int as libc::c_long);
            (*loop_0).idlemax[i as usize] = 0 as libc::c_int;
            (*loop_0).idlecnt[i as usize] = (*loop_0).idlemax[i as usize];
            (*loop_0).idles[i as usize] = 0 as *mut *mut ev_idle
        }
        ev_realloc((*loop_0).anfds as *mut libc::c_void,
                   0 as libc::c_int as libc::c_long);
        (*loop_0).anfds = 0 as *mut ANFD;
        (*loop_0).anfdmax = 0 as libc::c_int;
        ev_realloc((*loop_0).rfeeds as *mut libc::c_void,
                   0 as libc::c_int as libc::c_long);
        (*loop_0).rfeedmax = 0 as libc::c_int;
        (*loop_0).rfeedcnt = (*loop_0).rfeedmax;
        (*loop_0).rfeeds = 0 as *mut W;
        ev_realloc((*loop_0).fdchanges as *mut libc::c_void,
                   0 as libc::c_int as libc::c_long);
        (*loop_0).fdchangemax = 0 as libc::c_int;
        (*loop_0).fdchangecnt = (*loop_0).fdchangemax;
        (*loop_0).fdchanges = 0 as *mut libc::c_int;
        ev_realloc((*loop_0).timers as *mut libc::c_void,
                   0 as libc::c_int as libc::c_long);
        (*loop_0).timermax = 0 as libc::c_int;
        (*loop_0).timercnt = (*loop_0).timermax;
        (*loop_0).timers = 0 as *mut ANHE;
        ev_realloc((*loop_0).prepares as *mut libc::c_void,
                   0 as libc::c_int as libc::c_long);
        (*loop_0).preparemax = 0 as libc::c_int;
        (*loop_0).preparecnt = (*loop_0).preparemax;
        (*loop_0).prepares = 0 as *mut *mut ev_prepare;
        ev_realloc((*loop_0).checks as *mut libc::c_void,
                   0 as libc::c_int as libc::c_long);
        (*loop_0).checkmax = 0 as libc::c_int;
        (*loop_0).checkcnt = (*loop_0).checkmax;
        (*loop_0).checks = 0 as *mut *mut ev_check;
        (*loop_0).backend = 0 as libc::c_int;
        if ev_is_default_loop(loop_0) != 0 {
            k5ev_default_loop_ptr = 0 as *mut ev_loop
        } else {
            ev_realloc(loop_0 as *mut libc::c_void,
                       0 as libc::c_int as libc::c_long);
        };
    }
    #[c2rust::src_loc = "3082:1"]
    pub unsafe extern "C" fn k5ev_loop_new(mut flags: libc::c_uint)
     -> *mut ev_loop {
        let mut loop_0: *mut ev_loop =
            ev_realloc(0 as *mut libc::c_void,
                       ::std::mem::size_of::<ev_loop>() as libc::c_ulong as
                           libc::c_long) as *mut ev_loop;
        memset(loop_0 as *mut libc::c_void, 0 as libc::c_int,
               ::std::mem::size_of::<ev_loop>() as libc::c_ulong);
        loop_init(loop_0, flags);
        if k5ev_backend(loop_0) != 0 { return loop_0 }
        ev_realloc(loop_0 as *mut libc::c_void,
                   0 as libc::c_int as libc::c_long);
        return 0 as *mut ev_loop;
    }
    #[inline]
    #[c2rust::src_loc = "2716:1"]
    pub unsafe extern "C" fn enable_secure() -> libc::c_int {
        return (getuid() != geteuid() || getgid() != getegid()) as
                   libc::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "1878:1"]
    pub unsafe extern "C" fn get_clock() -> ev_tstamp { return k5ev_time(); }
    #[inline(never)]
    #[c2rust::src_loc = "3281:1"]
    pub unsafe extern "C" fn k5ev_invoke_pending(mut loop_0: *mut ev_loop) {
        (*loop_0).pendingpri =
            (if 0x4f as libc::c_int & 4 as libc::c_int != 0 {
                 2 as libc::c_int
             } else { 0 as libc::c_int }) -
                (if 0x4f as libc::c_int & 4 as libc::c_int != 0 {
                     -(2 as libc::c_int)
                 } else { 0 as libc::c_int }) + 1 as libc::c_int;
        while (*loop_0).pendingpri != 0 {
            (*loop_0).pendingpri -= 1;
            while (*loop_0).pendingcnt[(*loop_0).pendingpri as usize] != 0 {
                (*loop_0).pendingcnt[(*loop_0).pendingpri as usize] -= 1;
                let mut p: *mut ANPENDING =
                    (*loop_0).pendings[(*loop_0).pendingpri as
                                           usize].offset((*loop_0).pendingcnt[(*loop_0).pendingpri
                                                                                  as
                                                                                  usize]
                                                             as isize);
                (*(*p).w).pending = 0 as libc::c_int;
                (*(*p).w).cb.expect("non-null function pointer")(loop_0,
                                                                 (*p).w,
                                                                 (*p).events);
            }
        };
    }
    #[inline]
    #[c2rust::src_loc = "2160:1"]
    pub unsafe extern "C" fn fd_valid(mut fd: libc::c_int) -> libc::c_int {
        return (fcntl(fd, 1 as libc::c_int) != -(1 as libc::c_int)) as
                   libc::c_int;
    }
    #[inline(never)]
    #[c2rust::src_loc = "2171:1"]
    pub unsafe extern "C" fn fd_ebadf(mut loop_0: *mut ev_loop) {
        let mut fd: libc::c_int = 0;
        fd = 0 as libc::c_int;
        while fd < (*loop_0).anfdmax {
            if (*(*loop_0).anfds.offset(fd as isize)).events != 0 {
                if fd_valid(fd) == 0 &&
                       *__errno_location() == 9 as libc::c_int {
                    fd_kill(loop_0, fd);
                }
            }
            fd += 1
        };
    }
    #[inline]
    #[c2rust::src_loc = "3782:1"]
    pub unsafe extern "C" fn clear_pending(mut loop_0: *mut ev_loop,
                                           mut w: W) {
        if (*w).pending != 0 {
            let ref mut fresh28 =
                (*(*loop_0).pendings[((*w).priority -
                                          (if 0x4f as libc::c_int &
                                                  4 as libc::c_int != 0 {
                                               -(2 as libc::c_int)
                                           } else { 0 as libc::c_int })) as
                                         usize].offset(((*w).pending -
                                                            1 as libc::c_int)
                                                           as isize)).w;
            *fresh28 = &mut (*loop_0).pending_w as *mut ev_prepare as W;
            (*w).pending = 0 as libc::c_int
        };
    }
    #[inline]
    #[c2rust::src_loc = "3766:1"]
    pub unsafe extern "C" fn wlist_del(mut head: *mut WL, mut elem: WL) {
        while !(*head).is_null() {
            if (*head == elem) as libc::c_int as libc::c_long != 0 {
                *head = (*elem).next;
                break ;
            } else { head = &mut (**head).next }
        };
    }
    use super::ev_h::{ev_tstamp, ev_prepare, ev_io, ev_idle, ev_check,
                      ev_loop_callback, ev_watcher_time, ev_watcher_list,
                      ev_watcher, EVBACKEND_PORT, EVBACKEND_KQUEUE,
                      EVBACKEND_EPOLL, EVBACKEND_POLL, EVBACKEND_SELECT,
                      ev_signal, EV_READ, EV__IOFDSET, EVFLAG_NOSIGMASK,
                      EV_WRITE, EV_SIGNAL, ev_child, EV_CHILD,
                      EVFLAG_FORKCHECK, EVFLAG_NOENV, EVFLAG_SIGNALFD,
                      EVBACKEND_MASK, EV_ERROR, ev_timer, EV_CUSTOM, EV_TIMER,
                      EV_IDLE, EVBREAK_CANCEL, EVRUN_NOWAIT, EVRUN_ONCE,
                      EVBREAK_ONE, ev_is_default_loop};
    use super::sig_atomic_t_h::sig_atomic_t;
    use super::sys_types_h::{pid_t, ssize_t};
    use super::poll_h::pollfd;
    use super::sigset_t_h::sigset_t;
    use super::stdint_uintn_h::{uint32_t, uint64_t};
    use super::struct_timeval_h::timeval;
    use super::types_h::{__time_t, __suseconds_t, __syscall_slong_t};
    use super::time_h::{gettimeofday, timezone};
    use super::assert_h::__assert_fail;
    use super::struct_timespec_h::timespec;
    use super::include_time_h::nanosleep;
    use super::errno_h::__errno_location;
    use super::signal_h::{sigemptyset, sigaddset, sigprocmask, __sighandler_t,
                          sigfillset, sigaction, sigdelset, signal};
    use super::string_h::{memmove, memset};
    use super::sigaction_h::{sigaction, C2RustUnnamed_9};
    use super::__sigset_t_h::__sigset_t;
    use super::stdlib_h::{realloc, free, abort, getenv, atoi};
    use super::unistd_h::{write, pipe, dup2, close, read, getpid, getuid,
                          geteuid, getgid, getegid};
    use super::stddef_h::size_t;
    use super::stdio_h::{fprintf, stderr, perror};
    use super::fcntl_h::fcntl;
    use super::wait_h::waitpid;
    use super::ev_poll_c::{poll_init, poll_destroy};
    use super::ev_select_c::{select_init, select_destroy};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "445:20"]
        pub fn eventfd(initval: libc::c_uint, flags: libc::c_int)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "461:21"]
        pub fn signalfd(fd: libc::c_int, mask: *const sigset_t,
                        flags: libc::c_int) -> libc::c_int;
    }
    /* ****************************************************************************/
}
#[c2rust::header_src = "/usr/include/sys/poll.h:67"]
pub mod poll_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:8"]
    pub struct pollfd {
        pub fd: libc::c_int,
        pub events: libc::c_short,
        pub revents: libc::c_short,
    }
    #[c2rust::src_loc = "33:1"]
    pub type nfds_t = libc::c_ulong;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn poll(__fds: *mut pollfd, __nfds: nfds_t,
                    __timeout: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/time.h:67"]
pub mod time_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:8"]
    pub struct timezone {
        pub tz_minuteswest: libc::c_int,
        pub tz_dsttime: libc::c_int,
    }
    #[c2rust::src_loc = "58:1"]
    pub type __timezone_ptr_t = *mut timezone;
    use super::struct_timeval_h::timeval;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "68:1"]
        pub fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:67"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint64_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/stdlib.h:32"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "104:1"]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "539:14"]
        pub fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
        #[no_mangle]
        #[c2rust::src_loc = "634:1"]
        pub fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:33"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "46:14"]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "60:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:34"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/unistd.h:36"]
pub mod unistd_h {
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    use super::types_h::{__pid_t, __uid_t, __gid_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "360:1"]
        pub fn read(__fd: libc::c_int, __buf: *mut libc::c_void,
                    __nbytes: size_t) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn write(__fd: libc::c_int, __buf: *const libc::c_void,
                     __n: size_t) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "417:1"]
        pub fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "534:1"]
        pub fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "628:1"]
        pub fn getpid() -> __pid_t;
        #[no_mangle]
        #[c2rust::src_loc = "675:1"]
        pub fn getuid() -> __uid_t;
        #[no_mangle]
        #[c2rust::src_loc = "678:1"]
        pub fn geteuid() -> __uid_t;
        #[no_mangle]
        #[c2rust::src_loc = "681:1"]
        pub fn getgid() -> __gid_t;
        #[no_mangle]
        #[c2rust::src_loc = "684:1"]
        pub fn getegid() -> __gid_t;
    }
}
#[c2rust::header_src = "/usr/include/time.h:36"]
pub mod include_time_h {
    use super::struct_timespec_h::timespec;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "211:1"]
        pub fn nanosleep(__requested_time: *const timespec,
                         __remaining: *mut timespec) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/assert.h:67"]
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
#[c2rust::header_src = "/usr/include/sys/wait.h:67"]
pub mod wait_h {
    use super::types_h::__pid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "111:1"]
        pub fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int,
                       __options: libc::c_int) -> __pid_t;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:67"]
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
        #[c2rust::src_loc = "775:1"]
        pub fn perror(__s: *const libc::c_char);
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:67"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "175:1"]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/verto/ev_select.c:67"]
pub mod ev_select_c {
    /*
 * libev select fd activity backend
 *
 * Copyright (c) 2007,2008,2009,2010,2011 Marc Alexander Lehmann <libev@schmorp.de>
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without modifica-
 * tion, are permitted provided that the following conditions are met:
 *
 *   1.  Redistributions of source code must retain the above copyright notice,
 *       this list of conditions and the following disclaimer.
 *
 *   2.  Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MER-
 * CHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO
 * EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPE-
 * CIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
 * OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
 * WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTH-
 * ERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * Alternatively, the contents of this file may be used under the terms of
 * the GNU General Public License ("GPL") version 2 or any later version,
 * in which case the provisions of the GPL are applicable instead of
 * the above. If you wish to allow the use of your version of this file
 * only under the terms of the GPL and not to allow others to use your
 * version of this file under the BSD license, indicate your decision
 * by deleting the provisions above and replace them with the notice
 * and other provisions required by the GPL. If you do not delete the
 * provisions above, a recipient may use your version of this file under
 * either the BSD or the GPL.
 */
    /* for unix systems */
    /* for REAL unix systems */
    /* could free/malloc */
    /* could free/malloc */
    #[inline]
    #[c2rust::src_loc = "274:1"]
    pub unsafe extern "C" fn select_init(mut loop_0: *mut ev_loop,
                                         mut flags: libc::c_int)
     -> libc::c_int {
        (*loop_0).backend_mintime = 1e-6f64;
        (*loop_0).backend_modify =
            Some(select_modify as
                     unsafe extern "C" fn(_: *mut ev_loop, _: libc::c_int,
                                          _: libc::c_int, _: libc::c_int)
                         -> ());
        (*loop_0).backend_poll =
            Some(select_poll as
                     unsafe extern "C" fn(_: *mut ev_loop, _: ev_tstamp)
                         -> ());
        (*loop_0).vec_max = 0 as libc::c_int;
        (*loop_0).vec_ri = 0 as *mut libc::c_void;
        (*loop_0).vec_ro = 0 as *mut libc::c_void;
        (*loop_0).vec_wi = 0 as *mut libc::c_void;
        (*loop_0).vec_wo = 0 as *mut libc::c_void;
        return EVBACKEND_SELECT as libc::c_int;
    }
    #[c2rust::src_loc = "139:1"]
    pub unsafe extern "C" fn select_poll(mut loop_0: *mut ev_loop,
                                         mut timeout: ev_tstamp) {
        let mut tv: timeval = timeval{tv_sec: 0, tv_usec: 0,};
        let mut res: libc::c_int = 0;
        let mut fd_setsize: libc::c_int = 0;
        if (*loop_0).release_cb.is_some() as libc::c_int as libc::c_long != 0
           {
            (*loop_0).release_cb.expect("non-null function pointer")(loop_0);
        }
        tv.tv_sec = timeout as libc::c_long;
        tv.tv_usec =
            ((timeout - tv.tv_sec as libc::c_double) * 1e6f64) as
                libc::c_long;
        fd_setsize =
            (*loop_0).vec_max *
                (8 as libc::c_int *
                     ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as
                         libc::c_int / 8 as libc::c_int);
        memcpy((*loop_0).vec_ro, (*loop_0).vec_ri,
               fd_setsize as libc::c_ulong);
        memcpy((*loop_0).vec_wo, (*loop_0).vec_wi,
               fd_setsize as libc::c_ulong);
        res =
            select((*loop_0).vec_max *
                       (8 as libc::c_int *
                            ::std::mem::size_of::<__fd_mask>() as
                                libc::c_ulong as libc::c_int),
                   (*loop_0).vec_ro as *mut fd_set,
                   (*loop_0).vec_wo as *mut fd_set, 0 as *mut fd_set,
                   &mut tv);
        if (*loop_0).acquire_cb.is_some() as libc::c_int as libc::c_long != 0
           {
            (*loop_0).acquire_cb.expect("non-null function pointer")(loop_0);
        }
        if (res < 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
            if *__errno_location() == 9 as libc::c_int {
                fd_ebadf(loop_0);
            } else if *__errno_location() == 12 as libc::c_int &&
                          syserr_cb.is_none() {
                fd_enomem(loop_0);
            } else if *__errno_location() != 4 as libc::c_int {
                ev_syserr(b"(libev) select\x00" as *const u8 as
                              *const libc::c_char);
            }
            return
        }
        let mut word: libc::c_int = 0;
        let mut bit: libc::c_int = 0;
        word = (*loop_0).vec_max;
        loop  {
            let fresh7 = word;
            word = word - 1;
            if !(fresh7 != 0) { break ; }
            let mut word_r: fd_mask =
                *((*loop_0).vec_ro as *mut fd_mask).offset(word as isize);
            let mut word_w: fd_mask =
                *((*loop_0).vec_wo as *mut fd_mask).offset(word as isize);
            if word_r != 0 || word_w != 0 {
                bit =
                    8 as libc::c_int *
                        ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as
                            libc::c_int;
                loop  {
                    let fresh8 = bit;
                    bit = bit - 1;
                    if !(fresh8 != 0) { break ; }
                    let mut mask: fd_mask =
                        ((1 as libc::c_ulong) << bit) as fd_mask;
                    let mut events: libc::c_int = 0 as libc::c_int;
                    events |=
                        if word_r & mask != 0 {
                            EV_READ as libc::c_int
                        } else { 0 as libc::c_int };
                    events |=
                        if word_w & mask != 0 {
                            EV_WRITE as libc::c_int
                        } else { 0 as libc::c_int };
                    if (events != 0) as libc::c_int as libc::c_long != 0 {
                        fd_event(loop_0,
                                 word *
                                     (8 as libc::c_int *
                                          ::std::mem::size_of::<__fd_mask>()
                                              as libc::c_ulong as libc::c_int)
                                     + bit, events);
                    }
                }
            }
        };
    }
    #[inline]
    #[c2rust::src_loc = "303:1"]
    pub unsafe extern "C" fn select_destroy(mut loop_0: *mut ev_loop) {
        ev_realloc((*loop_0).vec_ri, 0 as libc::c_int as libc::c_long);
        ev_realloc((*loop_0).vec_ro, 0 as libc::c_int as libc::c_long);
        ev_realloc((*loop_0).vec_wi, 0 as libc::c_int as libc::c_long);
        ev_realloc((*loop_0).vec_wo, 0 as libc::c_int as libc::c_long);
    }
    #[c2rust::src_loc = "70:1"]
    pub unsafe extern "C" fn select_modify(mut loop_0: *mut ev_loop,
                                           mut fd: libc::c_int,
                                           mut oev: libc::c_int,
                                           mut nev: libc::c_int) {
        if oev == nev { return }
        let mut word: libc::c_int =
            fd /
                (8 as libc::c_int *
                     ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as
                         libc::c_int);
        let mut mask: fd_mask =
            ((1 as libc::c_ulong) <<
                 fd %
                     (8 as libc::c_int *
                          ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                              as libc::c_int)) as fd_mask;
        if ((*loop_0).vec_max <= word) as libc::c_int as libc::c_long != 0 {
            let mut new_max: libc::c_int = word + 1 as libc::c_int;
            (*loop_0).vec_ri =
                ev_realloc((*loop_0).vec_ri,
                           (new_max *
                                (8 as libc::c_int *
                                     ::std::mem::size_of::<__fd_mask>() as
                                         libc::c_ulong as libc::c_int /
                                     8 as libc::c_int)) as libc::c_long);
            (*loop_0).vec_ro =
                ev_realloc((*loop_0).vec_ro,
                           (new_max *
                                (8 as libc::c_int *
                                     ::std::mem::size_of::<__fd_mask>() as
                                         libc::c_ulong as libc::c_int /
                                     8 as libc::c_int)) as libc::c_long);
            (*loop_0).vec_wi =
                ev_realloc((*loop_0).vec_wi,
                           (new_max *
                                (8 as libc::c_int *
                                     ::std::mem::size_of::<__fd_mask>() as
                                         libc::c_ulong as libc::c_int /
                                     8 as libc::c_int)) as libc::c_long);
            (*loop_0).vec_wo =
                ev_realloc((*loop_0).vec_wo,
                           (new_max *
                                (8 as libc::c_int *
                                     ::std::mem::size_of::<__fd_mask>() as
                                         libc::c_ulong as libc::c_int /
                                     8 as libc::c_int)) as libc::c_long);
            while (*loop_0).vec_max < new_max {
                let ref mut fresh23 =
                    *((*loop_0).vec_wi as
                          *mut fd_mask).offset((*loop_0).vec_max as isize);
                *fresh23 = 0 as libc::c_int as fd_mask;
                *((*loop_0).vec_ri as
                      *mut fd_mask).offset((*loop_0).vec_max as isize) =
                    *fresh23;
                (*loop_0).vec_max += 1
            }
        }
        let ref mut fresh24 =
            *((*loop_0).vec_ri as *mut fd_mask).offset(word as isize);
        *fresh24 |= mask;
        if nev & EV_READ as libc::c_int == 0 {
            let ref mut fresh25 =
                *((*loop_0).vec_ri as *mut fd_mask).offset(word as isize);
            *fresh25 &= !mask
        }
        let ref mut fresh26 =
            *((*loop_0).vec_wi as *mut fd_mask).offset(word as isize);
        *fresh26 |= mask;
        if nev & EV_WRITE as libc::c_int == 0 {
            let ref mut fresh27 =
                *((*loop_0).vec_wi as *mut fd_mask).offset(word as isize);
            *fresh27 &= !mask
        };
    }
    use super::ev_c::{ev_loop, fd_ebadf, syserr_cb, fd_enomem, ev_syserr,
                      fd_event, ev_realloc};
    use super::ev_h::{ev_tstamp, EVBACKEND_SELECT, EV_READ, EV_WRITE};
    use super::struct_timeval_h::timeval;
    use super::types_h::{__time_t, __suseconds_t};
    use super::select_h::{__fd_mask, select, fd_set, fd_mask};
    use super::string_h::memcpy;
    use super::errno_h::__errno_location;
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/util/verto/ev_poll.c:67"]
pub mod ev_poll_c {
    /*
 * libev poll fd activity backend
 *
 * Copyright (c) 2007,2008,2009,2010,2011 Marc Alexander Lehmann <libev@schmorp.de>
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without modifica-
 * tion, are permitted provided that the following conditions are met:
 *
 *   1.  Redistributions of source code must retain the above copyright notice,
 *       this list of conditions and the following disclaimer.
 *
 *   2.  Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MER-
 * CHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO
 * EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPE-
 * CIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
 * OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
 * WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTH-
 * ERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * Alternatively, the contents of this file may be used under the terms of
 * the GNU General Public License ("GPL") version 2 or any later version,
 * in which case the provisions of the GPL are applicable instead of
 * the above. If you wish to allow the use of your version of this file
 * only under the terms of the GPL and not to allow others to use your
 * version of this file under the BSD license, indicate your decision
 * by deleting the provisions above and replace them with the notice
 * and other provisions required by the GPL. If you do not delete the
 * provisions above, a recipient may use your version of this file under
 * either the BSD or the GPL.
 */
    /* consider using memset (.., -1, ...), which is practically guaranteed
   * to work on all systems implementing poll */
    /* need to allocate a new pollfd */
    /* remove pollfd */
    /* this expect is debatable */
    #[inline]
    #[c2rust::src_loc = "142:1"]
    pub unsafe extern "C" fn poll_destroy(mut loop_0: *mut ev_loop) {
        ev_realloc((*loop_0).pollidxs as *mut libc::c_void,
                   0 as libc::c_int as libc::c_long);
        ev_realloc((*loop_0).polls as *mut libc::c_void,
                   0 as libc::c_int as libc::c_long);
    }
    #[inline]
    #[c2rust::src_loc = "42:1"]
    pub unsafe extern "C" fn pollidx_init(mut base: *mut libc::c_int,
                                          mut count: libc::c_int) {
        loop  {
            let fresh20 = count;
            count = count - 1;
            if !(fresh20 != 0) { break ; }
            let fresh21 = base;
            base = base.offset(1);
            *fresh21 = -(1 as libc::c_int)
        };
    }
    #[c2rust::src_loc = "51:1"]
    pub unsafe extern "C" fn poll_modify(mut loop_0: *mut ev_loop,
                                         mut fd: libc::c_int,
                                         mut oev: libc::c_int,
                                         mut nev: libc::c_int) {
        let mut idx: libc::c_int = 0;
        if oev == nev { return }
        if (fd + 1 as libc::c_int > (*loop_0).pollidxmax) as libc::c_int as
               libc::c_long != 0 {
            let mut ocur_: libc::c_int = (*loop_0).pollidxmax;
            (*loop_0).pollidxs =
                array_realloc(::std::mem::size_of::<libc::c_int>() as
                                  libc::c_ulong as libc::c_int,
                              (*loop_0).pollidxs as *mut libc::c_void,
                              &mut (*loop_0).pollidxmax,
                              fd + 1 as libc::c_int) as *mut libc::c_int;
            pollidx_init((*loop_0).pollidxs.offset(ocur_ as isize),
                         (*loop_0).pollidxmax - ocur_);
        }
        idx = *(*loop_0).pollidxs.offset(fd as isize);
        if idx < 0 as libc::c_int {
            let fresh22 = (*loop_0).pollcnt;
            (*loop_0).pollcnt = (*loop_0).pollcnt + 1;
            idx = fresh22;
            *(*loop_0).pollidxs.offset(fd as isize) = idx;
            if ((*loop_0).pollcnt > (*loop_0).pollmax) as libc::c_int as
                   libc::c_long != 0 {
                let mut ocur__0: libc::c_int = (*loop_0).pollmax;
                (*loop_0).polls =
                    array_realloc(::std::mem::size_of::<pollfd>() as
                                      libc::c_ulong as libc::c_int,
                                  (*loop_0).polls as *mut libc::c_void,
                                  &mut (*loop_0).pollmax, (*loop_0).pollcnt)
                        as *mut pollfd
            }
            (*(*loop_0).polls.offset(idx as isize)).fd = fd
        }
        if (*(*loop_0).polls.offset(idx as isize)).fd == fd {
        } else {
            __assert_fail(b"polls [idx].fd == fd\x00" as *const u8 as
                              *const libc::c_char,
                          b"./ev_poll.c\x00" as *const u8 as
                              *const libc::c_char,
                          70 as libc::c_int as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 50],
                                                    &[libc::c_char; 50]>(b"void poll_modify(struct ev_loop *, int, int, int)\x00")).as_ptr());
        }
        if nev != 0 {
            (*(*loop_0).polls.offset(idx as isize)).events =
                ((if nev & EV_READ as libc::c_int != 0 {
                      0x1 as libc::c_int
                  } else { 0 as libc::c_int }) |
                     (if nev & EV_WRITE as libc::c_int != 0 {
                          0x4 as libc::c_int
                      } else { 0 as libc::c_int })) as libc::c_short
        } else {
            *(*loop_0).pollidxs.offset(fd as isize) = -(1 as libc::c_int);
            (*loop_0).pollcnt -= 1;
            if (idx < (*loop_0).pollcnt) as libc::c_int as libc::c_long != 0 {
                *(*loop_0).polls.offset(idx as isize) =
                    *(*loop_0).polls.offset((*loop_0).pollcnt as isize);
                *(*loop_0).pollidxs.offset((*(*loop_0).polls.offset(idx as
                                                                        isize)).fd
                                               as isize) = idx
            }
        };
    }
    #[c2rust::src_loc = "88:1"]
    pub unsafe extern "C" fn poll_poll(mut loop_0: *mut ev_loop,
                                       mut timeout: ev_tstamp) {
        let mut p: *mut pollfd = 0 as *mut pollfd;
        let mut res: libc::c_int = 0;
        if (*loop_0).release_cb.is_some() as libc::c_int as libc::c_long != 0
           {
            (*loop_0).release_cb.expect("non-null function pointer")(loop_0);
        }
        res =
            poll((*loop_0).polls, (*loop_0).pollcnt as nfds_t,
                 (timeout * 1e3f64) as libc::c_int);
        if (*loop_0).acquire_cb.is_some() as libc::c_int as libc::c_long != 0
           {
            (*loop_0).acquire_cb.expect("non-null function pointer")(loop_0);
        }
        if (res < 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
            if *__errno_location() == 9 as libc::c_int {
                fd_ebadf(loop_0);
            } else if *__errno_location() == 12 as libc::c_int &&
                          syserr_cb.is_none() {
                fd_enomem(loop_0);
            } else if *__errno_location() != 4 as libc::c_int {
                ev_syserr(b"(libev) poll\x00" as *const u8 as
                              *const libc::c_char);
            }
        } else {
            p = (*loop_0).polls;
            while res != 0 {
                if p < (*loop_0).polls.offset((*loop_0).pollcnt as isize) {
                } else {
                    __assert_fail(b"(\"libev: poll() returned illegal result, broken BSD kernel?\", p < polls + pollcnt)\x00"
                                      as *const u8 as *const libc::c_char,
                                  b"./ev_poll.c\x00" as *const u8 as
                                      *const libc::c_char,
                                  110 as libc::c_int as libc::c_uint,
                                  (*::std::mem::transmute::<&[u8; 44],
                                                            &[libc::c_char; 44]>(b"void poll_poll(struct ev_loop *, ev_tstamp)\x00")).as_ptr());
                }
                if ((*p).revents != 0) as libc::c_int as libc::c_long != 0 {
                    res -= 1;
                    if ((*p).revents as libc::c_int & 0x20 as libc::c_int !=
                            0) as libc::c_int as libc::c_long != 0 {
                        fd_kill(loop_0, (*p).fd);
                    } else {
                        fd_event(loop_0, (*p).fd,
                                 (if (*p).revents as libc::c_int &
                                         (0x4 as libc::c_int |
                                              0x8 as libc::c_int |
                                              0x10 as libc::c_int) != 0 {
                                      EV_WRITE as libc::c_int
                                  } else { 0 as libc::c_int }) |
                                     (if (*p).revents as libc::c_int &
                                             (0x1 as libc::c_int |
                                                  0x8 as libc::c_int |
                                                  0x10 as libc::c_int) != 0 {
                                          EV_READ as libc::c_int
                                      } else { 0 as libc::c_int }));
                    }
                }
                p = p.offset(1)
            }
        };
    }
    #[inline]
    #[c2rust::src_loc = "129:1"]
    pub unsafe extern "C" fn poll_init(mut loop_0: *mut ev_loop,
                                       mut flags: libc::c_int)
     -> libc::c_int {
        (*loop_0).backend_mintime = 1e-3f64;
        (*loop_0).backend_modify =
            Some(poll_modify as
                     unsafe extern "C" fn(_: *mut ev_loop, _: libc::c_int,
                                          _: libc::c_int, _: libc::c_int)
                         -> ());
        (*loop_0).backend_poll =
            Some(poll_poll as
                     unsafe extern "C" fn(_: *mut ev_loop, _: ev_tstamp)
                         -> ());
        (*loop_0).pollidxs = 0 as *mut libc::c_int;
        (*loop_0).pollidxmax = 0 as libc::c_int;
        (*loop_0).polls = 0 as *mut pollfd;
        (*loop_0).pollmax = 0 as libc::c_int;
        (*loop_0).pollcnt = 0 as libc::c_int;
        return EVBACKEND_POLL as libc::c_int;
    }
    use super::ev_c::{ev_loop, ev_realloc, array_realloc, fd_ebadf, syserr_cb,
                      fd_enomem, ev_syserr, fd_kill, fd_event};
    use super::poll_h::{pollfd, poll, nfds_t};
    use super::assert_h::__assert_fail;
    use super::ev_h::{EV_READ, EV_WRITE, ev_tstamp, EVBACKEND_POLL};
    use super::errno_h::__errno_location;
}
pub use self::stddef_h::size_t;
pub use self::types_h::{__uint32_t, __uint64_t, __uid_t, __gid_t, __off_t,
                        __off64_t, __pid_t, __clock_t, __time_t,
                        __suseconds_t, __ssize_t, __syscall_slong_t,
                        __sig_atomic_t};
pub use self::sys_types_h::{pid_t, ssize_t};
pub use self::time_t_h::time_t;
pub use self::__sigset_t_h::__sigset_t;
pub use self::sigset_t_h::sigset_t;
pub use self::struct_timeval_h::timeval;
pub use self::struct_timespec_h::timespec;
pub use self::select_h::{__fd_mask, fd_set, fd_mask, select};
pub use self::__sigval_t_h::{__sigval_t, sigval};
pub use self::verto_h::{verto_proc, verto_proc_status, verto_ev_type,
                        VERTO_EV_TYPE_CHILD, VERTO_EV_TYPE_SIGNAL,
                        VERTO_EV_TYPE_IDLE, VERTO_EV_TYPE_TIMEOUT,
                        VERTO_EV_TYPE_IO, VERTO_EV_TYPE_NONE, verto_ev_flag,
                        _VERTO_EV_FLAG_MAX, _VERTO_EV_FLAG_MUTABLE_MASK,
                        VERTO_EV_FLAG_REINITIABLE, VERTO_EV_FLAG_IO_CLOSE_FD,
                        VERTO_EV_FLAG_IO_ERROR, VERTO_EV_FLAG_IO_WRITE,
                        VERTO_EV_FLAG_IO_READ, VERTO_EV_FLAG_PRIORITY_HIGH,
                        VERTO_EV_FLAG_PRIORITY_MEDIUM,
                        VERTO_EV_FLAG_PRIORITY_LOW, VERTO_EV_FLAG_PERSIST,
                        VERTO_EV_FLAG_NONE, verto_ctx, verto_ev,
                        verto_get_type, verto_get_flags, verto_get_fd,
                        verto_get_interval, verto_get_signal, verto_get_proc};
pub use self::verto_module_h::{verto_mod_ctx, verto_mod_ev, verto_ctx_funcs,
                               verto_module, verto_convert_module, verto_fire,
                               verto_set_proc_status, verto_set_fd_state};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::sig_atomic_t_h::sig_atomic_t;
pub use self::siginfo_t_h::{siginfo_t, C2RustUnnamed, C2RustUnnamed_0,
                            C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3,
                            C2RustUnnamed_4, C2RustUnnamed_5, C2RustUnnamed_6,
                            C2RustUnnamed_7, C2RustUnnamed_8};
pub use self::signal_h::{__sighandler_t, signal, sigemptyset, sigfillset,
                         sigaddset, sigdelset, sigprocmask, sigaction};
pub use self::sigaction_h::{sigaction, C2RustUnnamed_9};
pub use self::ev_h::{ev_tstamp, ev_loop_callback, ev_io, ev_watcher_list,
                     ev_check, ev_prepare, ev_idle, ev_watcher_time,
                     ev_watcher, C2RustUnnamed_10, EV_ERROR, EV_CUSTOM,
                     EV_ASYNC, EV_CLEANUP, EV_FORK, EV_EMBED, EV_CHECK,
                     EV_PREPARE, EV_IDLE, EV_STAT, EV_CHILD, EV_SIGNAL,
                     EV_PERIODIC, EV_TIMEOUT, EV_TIMER, EV_IO, EV__IOFDSET,
                     EV_WRITE, EV_READ, EV_NONE, EV_UNDEF, ev_timer,
                     ev_signal, ev_child, C2RustUnnamed_11, EVFLAG_NOSIGMASK,
                     EVFLAG_SIGNALFD, EVFLAG_NOSIGFD, EVFLAG_NOINOTIFY,
                     EVFLAG_FORKCHECK, EVFLAG_NOENV, EVFLAG_AUTO,
                     C2RustUnnamed_12, EVBACKEND_MASK, EVBACKEND_ALL,
                     EVBACKEND_PORT, EVBACKEND_DEVPOLL, EVBACKEND_KQUEUE,
                     EVBACKEND_EPOLL, EVBACKEND_POLL, EVBACKEND_SELECT,
                     C2RustUnnamed_13, EVRUN_ONCE, EVRUN_NOWAIT,
                     C2RustUnnamed_14, EVBREAK_ALL, EVBREAK_ONE,
                     EVBREAK_CANCEL, ev_is_default_loop, ev_default_loop_uc_};
pub use self::ev_c::{ev_loop, ANHE, WT, ANFD, WL, ANPENDING, W, ANSIG,
                     signalfd_siginfo, k5ev_supported_backends,
                     k5ev_recommended_backends, k5ev_time, k5ev_io_stop,
                     k5ev_sleep, k5ev_signal_start, alloc, ev_realloc_emul,
                     syserr_cb, k5ev_default_loop, k5ev_default_loop_ptr,
                     default_loop_struct, k5ev_unref, childev, ev_sighandler,
                     k5ev_feed_signal, evpipe_write, signals, evpipe_init,
                     k5ev_io_start, fd_change, array_realloc, ev_realloc,
                     array_nextsize, wlist_add, ev_start, k5ev_ref,
                     pri_adjust, fd_intern, ev_syserr, sigfdcb,
                     k5ev_feed_signal_event, k5ev_feed_event, childcb,
                     child_reap, childs, k5ev_backend, loop_init, pipecb,
                     ev_stop, pendingcb, fd_event, fd_event_nocheck,
                     fd_enomem, fd_kill, k5ev_idle_stop, k5ev_idle_start,
                     k5ev_child_stop, k5ev_child_start, k5ev_timer_start,
                     k5ev_break, loop_fork, fd_reify, upheap, downheap,
                     adjustheap, k5ev_timer_stop, feed_reverse,
                     feed_reverse_done, timers_reify, queue_events,
                     idle_reify, k5ev_run, timers_reschedule, time_update,
                     k5ev_loop_fork, k5ev_signal_stop, k5ev_loop_destroy,
                     k5ev_loop_new, enable_secure, get_clock,
                     k5ev_invoke_pending, fd_valid, fd_ebadf, clear_pending,
                     wlist_del, eventfd, signalfd};
pub use self::poll_h::{pollfd, nfds_t, poll};
pub use self::time_h::{timezone, __timezone_ptr_t, gettimeofday};
pub use self::stdint_uintn_h::{uint64_t, uint32_t};
use self::stdlib_h::{atoi, malloc, realloc, free, abort, getenv};
use self::string_h::{memcpy, memmove, memset};
use self::errno_h::__errno_location;
use self::unistd_h::{close, read, write, pipe, dup2, getpid, getuid, geteuid,
                     getgid, getegid};
use self::include_time_h::nanosleep;
use self::assert_h::__assert_fail;
use self::wait_h::waitpid;
use self::stdio_h::{stderr, fprintf, perror};
use self::fcntl_h::fcntl;
pub use self::ev_select_c::{select_init, select_poll, select_destroy,
                            select_modify};
pub use self::ev_poll_c::{poll_destroy, pollidx_init, poll_modify, poll_poll,
                          poll_init};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "165:5"]
pub union C2RustUnnamed_15 {
    pub watcher: *mut ev_watcher,
    pub io: *mut ev_io,
    pub timer: *mut ev_timer,
    pub idle: *mut ev_idle,
    pub signal: *mut ev_signal,
    pub child: *mut ev_child,
}
#[c2rust::src_loc = "69:1"]
unsafe extern "C" fn k5ev_ctx_new() -> *mut libc::c_void {
    return k5ev_loop_new(EVFLAG_AUTO as libc::c_int as libc::c_uint) as
               *mut libc::c_void;
}
#[c2rust::src_loc = "75:1"]
unsafe extern "C" fn k5ev_ctx_default() -> *mut libc::c_void {
    return k5ev_default_loop(EVFLAG_AUTO as libc::c_int as libc::c_uint) as
               *mut libc::c_void;
}
#[c2rust::src_loc = "81:1"]
unsafe extern "C" fn k5ev_ctx_free(mut ctx: *mut libc::c_void) {
    if ctx !=
           k5ev_default_loop(0 as libc::c_int as libc::c_uint) as
               *mut libc::c_void {
        k5ev_loop_destroy(ctx as *mut ev_loop);
    };
}
#[c2rust::src_loc = "88:1"]
unsafe extern "C" fn k5ev_ctx_run(mut ctx: *mut libc::c_void) {
    k5ev_run(ctx as *mut ev_loop, 0 as libc::c_int);
}
#[c2rust::src_loc = "94:1"]
unsafe extern "C" fn k5ev_ctx_run_once(mut ctx: *mut libc::c_void) {
    k5ev_run(ctx as *mut ev_loop, EVRUN_ONCE as libc::c_int);
}
#[c2rust::src_loc = "100:1"]
unsafe extern "C" fn k5ev_ctx_break(mut ctx: *mut libc::c_void) {
    k5ev_break(ctx as *mut ev_loop, EVBREAK_ONE as libc::c_int);
}
#[c2rust::src_loc = "106:1"]
unsafe extern "C" fn k5ev_ctx_reinitialize(mut ctx: *mut libc::c_void) {
    k5ev_loop_fork(ctx as *mut ev_loop);
}
#[c2rust::src_loc = "112:1"]
unsafe extern "C" fn libev_callback(mut loop_0: *mut ev_loop,
                                    mut w: *mut ev_watcher,
                                    mut revents: libc::c_int) {
    let mut state: verto_ev_flag = VERTO_EV_FLAG_NONE;
    /* Match the check in ev.h, which doesn't mark this unused */
    if verto_get_type((*w).data as *const verto_ev) as libc::c_uint ==
           VERTO_EV_TYPE_CHILD as libc::c_int as libc::c_uint {
        verto_set_proc_status((*w).data as *mut verto_ev,
                              (*(w as *mut ev_child)).rstatus);
    }
    if revents & EV_READ as libc::c_int != 0 {
        state =
            ::std::mem::transmute::<libc::c_uint,
                                    verto_ev_flag>(state as libc::c_uint |
                                                       VERTO_EV_FLAG_IO_READ
                                                           as libc::c_int as
                                                           libc::c_uint)
    }
    if revents & EV_WRITE as libc::c_int != 0 {
        state =
            ::std::mem::transmute::<libc::c_uint,
                                    verto_ev_flag>(state as libc::c_uint |
                                                       VERTO_EV_FLAG_IO_WRITE
                                                           as libc::c_int as
                                                           libc::c_uint)
    }
    if revents & EV_ERROR as libc::c_int != 0 {
        state =
            ::std::mem::transmute::<libc::c_uint,
                                    verto_ev_flag>(state as libc::c_uint |
                                                       VERTO_EV_FLAG_IO_ERROR
                                                           as libc::c_int as
                                                           libc::c_uint)
    }
    verto_set_fd_state((*w).data as *mut verto_ev, state);
    verto_fire((*w).data as *mut verto_ev);
}
#[c2rust::src_loc = "136:1"]
unsafe extern "C" fn k5ev_ctx_set_flags(mut ctx: *mut libc::c_void,
                                        mut ev: *const verto_ev,
                                        mut evpriv: *mut libc::c_void) {
    if verto_get_type(ev) as libc::c_uint ==
           VERTO_EV_TYPE_IO as libc::c_int as libc::c_uint {
        let mut events: libc::c_int = EV_NONE as libc::c_int;
        if verto_get_flags(ev) as libc::c_uint &
               VERTO_EV_FLAG_IO_READ as libc::c_int as libc::c_uint != 0 {
            events |= EV_READ as libc::c_int
        }
        if verto_get_flags(ev) as libc::c_uint &
               VERTO_EV_FLAG_IO_WRITE as libc::c_int as libc::c_uint != 0 {
            events |= EV_WRITE as libc::c_int
        }
        k5ev_io_stop(ctx as *mut ev_loop, evpriv as *mut ev_io);
        (*(evpriv as *mut ev_io)).fd = verto_get_fd(ev);
        (*(evpriv as *mut ev_io)).events =
            events | EV__IOFDSET as libc::c_int;
        k5ev_io_start(ctx as *mut ev_loop, evpriv as *mut ev_io);
    };
}
#[c2rust::src_loc = "162:1"]
unsafe extern "C" fn k5ev_ctx_add(mut ctx: *mut libc::c_void,
                                  mut ev: *const verto_ev,
                                  mut flags: *mut verto_ev_flag)
 -> *mut libc::c_void {
    let mut w: C2RustUnnamed_15 =
        C2RustUnnamed_15{watcher: 0 as *mut ev_watcher,};
    let mut interval: ev_tstamp = 0.;
    w.watcher = 0 as *mut ev_watcher;
    *flags =
        ::std::mem::transmute::<libc::c_uint,
                                verto_ev_flag>(*flags as libc::c_uint |
                                                   VERTO_EV_FLAG_PERSIST as
                                                       libc::c_int as
                                                       libc::c_uint);
    match verto_get_type(ev) as libc::c_uint {
        1 => {
            w.io =
                malloc(::std::mem::size_of::<ev_io>() as libc::c_ulong) as
                    *mut ev_io;
            if !w.io.is_null() {
                let ref mut fresh29 =
                    (*(w.io as *mut libc::c_void as *mut ev_watcher)).pending;
                *fresh29 = 0 as libc::c_int;
                (*(w.io as *mut libc::c_void as *mut ev_watcher)).active =
                    *fresh29;
                (*(w.io as *mut libc::c_void as *mut ev_watcher)).priority =
                    0 as libc::c_int;
                (*w.io).cb =
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *mut ev_loop,
                                                                        _:
                                                                            *mut ev_watcher,
                                                                        _:
                                                                            libc::c_int)
                                                       -> ()>,
                                            Option<unsafe extern "C" fn(_:
                                                                            *mut ev_loop,
                                                                        _:
                                                                            *mut ev_io,
                                                                        _:
                                                                            libc::c_int)
                                                       ->
                                                           ()>>(Some(libev_callback
                                                                         as
                                                                         unsafe extern "C" fn(_:
                                                                                                  *mut ev_loop,
                                                                                              _:
                                                                                                  *mut ev_watcher,
                                                                                              _:
                                                                                                  libc::c_int)
                                                                             ->
                                                                                 ()));
                memmove(&mut (*(w.io as *mut ev_watcher)).cb as
                            *mut Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                             _:
                                                                 *mut ev_watcher,
                                                             _: libc::c_int)
                                            -> ()> as *mut libc::c_void,
                        &mut (*w.io).cb as
                            *mut Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                             _: *mut ev_io,
                                                             _: libc::c_int)
                                            -> ()> as *const libc::c_void,
                        ::std::mem::size_of::<Option<unsafe extern "C" fn(_:
                                                                              *mut ev_loop,
                                                                          _:
                                                                              *mut ev_io,
                                                                          _:
                                                                              libc::c_int)
                                                         -> ()>>() as
                            libc::c_ulong);
                (*w.io).fd = verto_get_fd(ev);
                (*w.io).events =
                    EV_NONE as libc::c_int | EV__IOFDSET as libc::c_int;
                k5ev_io_start(ctx as *mut ev_loop, w.io);
            }
            /* Not supported */
        }
        2 => {
            interval =
                verto_get_interval(ev) as ev_tstamp /
                    1000.0f64; /* Child events don't persist */
            w.timer =
                malloc(::std::mem::size_of::<ev_timer>() as libc::c_ulong) as
                    *mut ev_timer;
            if !w.timer.is_null() {
                let ref mut fresh30 =
                    (*(w.timer as *mut libc::c_void as
                           *mut ev_watcher)).pending;
                *fresh30 = 0 as libc::c_int;
                (*(w.timer as *mut libc::c_void as *mut ev_watcher)).active =
                    *fresh30;
                (*(w.timer as *mut libc::c_void as *mut ev_watcher)).priority
                    = 0 as libc::c_int;
                (*w.timer).cb =
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *mut ev_loop,
                                                                        _:
                                                                            *mut ev_watcher,
                                                                        _:
                                                                            libc::c_int)
                                                       -> ()>,
                                            Option<unsafe extern "C" fn(_:
                                                                            *mut ev_loop,
                                                                        _:
                                                                            *mut ev_timer,
                                                                        _:
                                                                            libc::c_int)
                                                       ->
                                                           ()>>(Some(libev_callback
                                                                         as
                                                                         unsafe extern "C" fn(_:
                                                                                                  *mut ev_loop,
                                                                                              _:
                                                                                                  *mut ev_watcher,
                                                                                              _:
                                                                                                  libc::c_int)
                                                                             ->
                                                                                 ()));
                memmove(&mut (*(w.timer as *mut ev_watcher)).cb as
                            *mut Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                             _:
                                                                 *mut ev_watcher,
                                                             _: libc::c_int)
                                            -> ()> as *mut libc::c_void,
                        &mut (*w.timer).cb as
                            *mut Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                             _: *mut ev_timer,
                                                             _: libc::c_int)
                                            -> ()> as *const libc::c_void,
                        ::std::mem::size_of::<Option<unsafe extern "C" fn(_:
                                                                              *mut ev_loop,
                                                                          _:
                                                                              *mut ev_timer,
                                                                          _:
                                                                              libc::c_int)
                                                         -> ()>>() as
                            libc::c_ulong);
                (*(w.timer as *mut ev_watcher_time)).at = interval;
                (*w.timer).repeat = interval;
                k5ev_timer_start(ctx as *mut ev_loop, w.timer);
            }
        }
        4 => {
            w.idle =
                malloc(::std::mem::size_of::<ev_idle>() as libc::c_ulong) as
                    *mut ev_idle;
            if !w.idle.is_null() {
                let ref mut fresh31 =
                    (*(w.idle as *mut libc::c_void as
                           *mut ev_watcher)).pending;
                *fresh31 = 0 as libc::c_int;
                (*(w.idle as *mut libc::c_void as *mut ev_watcher)).active =
                    *fresh31;
                (*(w.idle as *mut libc::c_void as *mut ev_watcher)).priority =
                    0 as libc::c_int;
                (*w.idle).cb =
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *mut ev_loop,
                                                                        _:
                                                                            *mut ev_watcher,
                                                                        _:
                                                                            libc::c_int)
                                                       -> ()>,
                                            Option<unsafe extern "C" fn(_:
                                                                            *mut ev_loop,
                                                                        _:
                                                                            *mut ev_idle,
                                                                        _:
                                                                            libc::c_int)
                                                       ->
                                                           ()>>(Some(libev_callback
                                                                         as
                                                                         unsafe extern "C" fn(_:
                                                                                                  *mut ev_loop,
                                                                                              _:
                                                                                                  *mut ev_watcher,
                                                                                              _:
                                                                                                  libc::c_int)
                                                                             ->
                                                                                 ()));
                memmove(&mut (*(w.idle as *mut ev_watcher)).cb as
                            *mut Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                             _:
                                                                 *mut ev_watcher,
                                                             _: libc::c_int)
                                            -> ()> as *mut libc::c_void,
                        &mut (*w.idle).cb as
                            *mut Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                             _: *mut ev_idle,
                                                             _: libc::c_int)
                                            -> ()> as *const libc::c_void,
                        ::std::mem::size_of::<Option<unsafe extern "C" fn(_:
                                                                              *mut ev_loop,
                                                                          _:
                                                                              *mut ev_idle,
                                                                          _:
                                                                              libc::c_int)
                                                         -> ()>>() as
                            libc::c_ulong);
                k5ev_idle_start(ctx as *mut ev_loop, w.idle);
            }
        }
        8 => {
            w.signal =
                malloc(::std::mem::size_of::<ev_signal>() as libc::c_ulong) as
                    *mut ev_signal;
            if !w.signal.is_null() {
                let ref mut fresh32 =
                    (*(w.signal as *mut libc::c_void as
                           *mut ev_watcher)).pending;
                *fresh32 = 0 as libc::c_int;
                (*(w.signal as *mut libc::c_void as *mut ev_watcher)).active =
                    *fresh32;
                (*(w.signal as *mut libc::c_void as *mut ev_watcher)).priority
                    = 0 as libc::c_int;
                (*w.signal).cb =
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *mut ev_loop,
                                                                        _:
                                                                            *mut ev_watcher,
                                                                        _:
                                                                            libc::c_int)
                                                       -> ()>,
                                            Option<unsafe extern "C" fn(_:
                                                                            *mut ev_loop,
                                                                        _:
                                                                            *mut ev_signal,
                                                                        _:
                                                                            libc::c_int)
                                                       ->
                                                           ()>>(Some(libev_callback
                                                                         as
                                                                         unsafe extern "C" fn(_:
                                                                                                  *mut ev_loop,
                                                                                              _:
                                                                                                  *mut ev_watcher,
                                                                                              _:
                                                                                                  libc::c_int)
                                                                             ->
                                                                                 ()));
                memmove(&mut (*(w.signal as *mut ev_watcher)).cb as
                            *mut Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                             _:
                                                                 *mut ev_watcher,
                                                             _: libc::c_int)
                                            -> ()> as *mut libc::c_void,
                        &mut (*w.signal).cb as
                            *mut Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                             _:
                                                                 *mut ev_signal,
                                                             _: libc::c_int)
                                            -> ()> as *const libc::c_void,
                        ::std::mem::size_of::<Option<unsafe extern "C" fn(_:
                                                                              *mut ev_loop,
                                                                          _:
                                                                              *mut ev_signal,
                                                                          _:
                                                                              libc::c_int)
                                                         -> ()>>() as
                            libc::c_ulong);
                (*w.signal).signum = verto_get_signal(ev);
                k5ev_signal_start(ctx as *mut ev_loop, w.signal);
            }
        }
        16 => {
            *flags =
                ::std::mem::transmute::<libc::c_uint,
                                        verto_ev_flag>(*flags as libc::c_uint
                                                           &
                                                           !(VERTO_EV_FLAG_PERSIST
                                                                 as
                                                                 libc::c_int)
                                                               as
                                                               libc::c_uint);
            w.child =
                malloc(::std::mem::size_of::<ev_child>() as libc::c_ulong) as
                    *mut ev_child;
            if !w.child.is_null() {
                let ref mut fresh33 =
                    (*(w.child as *mut libc::c_void as
                           *mut ev_watcher)).pending;
                *fresh33 = 0 as libc::c_int;
                (*(w.child as *mut libc::c_void as *mut ev_watcher)).active =
                    *fresh33;
                (*(w.child as *mut libc::c_void as *mut ev_watcher)).priority
                    = 0 as libc::c_int;
                (*w.child).cb =
                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                            *mut ev_loop,
                                                                        _:
                                                                            *mut ev_watcher,
                                                                        _:
                                                                            libc::c_int)
                                                       -> ()>,
                                            Option<unsafe extern "C" fn(_:
                                                                            *mut ev_loop,
                                                                        _:
                                                                            *mut ev_child,
                                                                        _:
                                                                            libc::c_int)
                                                       ->
                                                           ()>>(Some(libev_callback
                                                                         as
                                                                         unsafe extern "C" fn(_:
                                                                                                  *mut ev_loop,
                                                                                              _:
                                                                                                  *mut ev_watcher,
                                                                                              _:
                                                                                                  libc::c_int)
                                                                             ->
                                                                                 ()));
                memmove(&mut (*(w.child as *mut ev_watcher)).cb as
                            *mut Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                             _:
                                                                 *mut ev_watcher,
                                                             _: libc::c_int)
                                            -> ()> as *mut libc::c_void,
                        &mut (*w.child).cb as
                            *mut Option<unsafe extern "C" fn(_: *mut ev_loop,
                                                             _: *mut ev_child,
                                                             _: libc::c_int)
                                            -> ()> as *const libc::c_void,
                        ::std::mem::size_of::<Option<unsafe extern "C" fn(_:
                                                                              *mut ev_loop,
                                                                          _:
                                                                              *mut ev_child,
                                                                          _:
                                                                              libc::c_int)
                                                         -> ()>>() as
                            libc::c_ulong);
                (*w.child).pid = verto_get_proc(ev);
                (*w.child).flags = (0 as libc::c_int != 0) as libc::c_int;
                k5ev_child_start(ctx as *mut ev_loop, w.child);
            }
        }
        _ => { }
    }
    if !w.watcher.is_null() {
        (*w.watcher).data = ev as *mut libc::c_void;
        k5ev_ctx_set_flags(ctx, ev, w.watcher as *mut libc::c_void);
    }
    return w.watcher as *mut libc::c_void;
}
#[c2rust::src_loc = "201:1"]
unsafe extern "C" fn k5ev_ctx_del(mut ctx: *mut libc::c_void,
                                  mut ev: *const verto_ev,
                                  mut evpriv: *mut libc::c_void) {
    match verto_get_type(ev) as libc::c_uint {
        1 => { k5ev_io_stop(ctx as *mut ev_loop, evpriv as *mut ev_io); }
        2 => {
            k5ev_timer_stop(ctx as *mut ev_loop, evpriv as *mut ev_timer);
        }
        4 => { k5ev_idle_stop(ctx as *mut ev_loop, evpriv as *mut ev_idle); }
        8 => {
            k5ev_signal_stop(ctx as *mut ev_loop, evpriv as *mut ev_signal);
        }
        16 => {
            k5ev_child_stop(ctx as *mut ev_loop, evpriv as *mut ev_child);
        }
        _ => { }
    }
    free(evpriv);
}
#[no_mangle]
#[c2rust::src_loc = "230:1"]
pub unsafe extern "C" fn verto_new_k5ev() -> *mut verto_ctx {
    return verto_convert_module(&mut verto_module_table_k5ev,
                                0 as libc::c_int, 0 as *mut libc::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "230:1"]
pub static mut verto_module_table_k5ev: verto_module =
    unsafe {
        {
            let mut init =
                verto_module{vers: 3 as libc::c_int as libc::c_uint,
                             name:
                                 b"k5ev\x00" as *const u8 as
                                     *const libc::c_char,
                             symb:
                                 b"NULL\x00" as *const u8 as
                                     *const libc::c_char,
                             types:
                                 (VERTO_EV_TYPE_IO as libc::c_int |
                                      VERTO_EV_TYPE_TIMEOUT as libc::c_int |
                                      VERTO_EV_TYPE_IDLE as libc::c_int |
                                      VERTO_EV_TYPE_SIGNAL as libc::c_int |
                                      VERTO_EV_TYPE_CHILD as libc::c_int) as
                                     verto_ev_type,
                             funcs:
                                 &k5ev_funcs as *const verto_ctx_funcs as
                                     *mut verto_ctx_funcs,};
            init
        }
    };
#[c2rust::src_loc = "230:1"]
static mut k5ev_funcs: verto_ctx_funcs =
    unsafe {
        {
            let mut init =
                verto_ctx_funcs{ctx_new:
                                    ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                       ->
                                                                           *mut libc::c_void>,
                                                            Option<unsafe extern "C" fn()
                                                                       ->
                                                                           *mut libc::c_void>>(Some(k5ev_ctx_new
                                                                                                        as
                                                                                                        unsafe extern "C" fn()
                                                                                                            ->
                                                                                                                *mut libc::c_void)),
                                ctx_default:
                                    ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                                       ->
                                                                           *mut libc::c_void>,
                                                            Option<unsafe extern "C" fn()
                                                                       ->
                                                                           *mut libc::c_void>>(Some(k5ev_ctx_default
                                                                                                        as
                                                                                                        unsafe extern "C" fn()
                                                                                                            ->
                                                                                                                *mut libc::c_void)),
                                ctx_free:
                                    Some(k5ev_ctx_free as
                                             unsafe extern "C" fn(_:
                                                                      *mut libc::c_void)
                                                 -> ()),
                                ctx_run:
                                    Some(k5ev_ctx_run as
                                             unsafe extern "C" fn(_:
                                                                      *mut libc::c_void)
                                                 -> ()),
                                ctx_run_once:
                                    Some(k5ev_ctx_run_once as
                                             unsafe extern "C" fn(_:
                                                                      *mut libc::c_void)
                                                 -> ()),
                                ctx_break:
                                    Some(k5ev_ctx_break as
                                             unsafe extern "C" fn(_:
                                                                      *mut libc::c_void)
                                                 -> ()),
                                ctx_reinitialize:
                                    Some(k5ev_ctx_reinitialize as
                                             unsafe extern "C" fn(_:
                                                                      *mut libc::c_void)
                                                 -> ()),
                                ctx_set_flags:
                                    Some(k5ev_ctx_set_flags as
                                             unsafe extern "C" fn(_:
                                                                      *mut libc::c_void,
                                                                  _:
                                                                      *const verto_ev,
                                                                  _:
                                                                      *mut libc::c_void)
                                                 -> ()),
                                ctx_add:
                                    Some(k5ev_ctx_add as
                                             unsafe extern "C" fn(_:
                                                                      *mut libc::c_void,
                                                                  _:
                                                                      *const verto_ev,
                                                                  _:
                                                                      *mut verto_ev_flag)
                                                 -> *mut libc::c_void),
                                ctx_del:
                                    Some(k5ev_ctx_del as
                                             unsafe extern "C" fn(_:
                                                                      *mut libc::c_void,
                                                                  _:
                                                                      *const verto_ev,
                                                                  _:
                                                                      *mut libc::c_void)
                                                 -> ()),};
            init
        }
    };
#[no_mangle]
#[c2rust::src_loc = "230:1"]
pub unsafe extern "C" fn verto_default_k5ev() -> *mut verto_ctx {
    return verto_convert_module(&mut verto_module_table_k5ev,
                                1 as libc::c_int, 0 as *mut libc::c_void);
}
