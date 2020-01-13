use ::libc;
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
}
#[c2rust::header_src = "/usr/include/bits/types.h:54"]
pub mod types_h {
    #[c2rust::src_loc = "33:1"]
    pub type __u_int = libc::c_uint;
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
    #[c2rust::src_loc = "174:1"]
    pub type __blksize_t = libc::c_long;
    #[c2rust::src_loc = "179:1"]
    pub type __blkcnt_t = libc::c_long;
    #[c2rust::src_loc = "193:1"]
    pub type __ssize_t = libc::c_long;
    #[c2rust::src_loc = "196:1"]
    pub type __syscall_slong_t = libc::c_long;
    #[c2rust::src_loc = "203:1"]
    pub type __caddr_t = *mut libc::c_char;
    #[c2rust::src_loc = "209:1"]
    pub type __socklen_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/sys/types.h:54"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "69:1"]
    pub type mode_t = __mode_t;
    #[c2rust::src_loc = "97:1"]
    pub type pid_t = __pid_t;
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __mode_t, __pid_t, __ssize_t, __caddr_t};
}
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:54"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:54"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:54"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/__sigset_t.h:54"]
pub mod __sigset_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "5:9"]
    pub struct __sigset_t {
        pub __val: [libc::c_ulong; 16],
    }
}
#[c2rust::header_src = "/usr/include/bits/types/sigset_t.h:54"]
pub mod sigset_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type sigset_t = __sigset_t;
    use super::__sigset_t_h::__sigset_t;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:54"]
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
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:54"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:54"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:54"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:54"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stat.h:54"]
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
#[c2rust::header_src = "/usr/include/unistd.h:54"]
pub mod unistd_h {
    #[c2rust::src_loc = "274:1"]
    pub type socklen_t = __socklen_t;
    use super::types_h::{__socklen_t, __pid_t};
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn write(__fd: libc::c_int, __buf: *const libc::c_void,
                     __n: size_t) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "432:1"]
        pub fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
        #[no_mangle]
        #[c2rust::src_loc = "444:1"]
        pub fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
        #[no_mangle]
        #[c2rust::src_loc = "531:1"]
        pub fn dup(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "534:1"]
        pub fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "563:1"]
        pub fn execv(__path: *const libc::c_char,
                     __argv: *const *mut libc::c_char) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "603:13"]
        pub fn _exit(_: libc::c_int) -> !;
        #[no_mangle]
        #[c2rust::src_loc = "628:1"]
        pub fn getpid() -> __pid_t;
        #[no_mangle]
        #[c2rust::src_loc = "631:1"]
        pub fn getppid() -> __pid_t;
        #[no_mangle]
        #[c2rust::src_loc = "756:1"]
        pub fn fork() -> __pid_t;
        #[no_mangle]
        #[c2rust::src_loc = "935:1"]
        pub fn daemon(__nochdir: libc::c_int, __noclose: libc::c_int)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/__sigval_t.h:54"]
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
#[c2rust::header_src = "/usr/include/bits/getopt_ext.h:54"]
pub mod getopt_ext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct option {
        pub name: *const libc::c_char,
        pub has_arg: libc::c_int,
        pub flag: *mut libc::c_int,
        pub val: libc::c_int,
    }
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "66:1"]
        pub fn getopt_long(___argc: libc::c_int,
                           ___argv: *const *mut libc::c_char,
                           __shortopts: *const libc::c_char,
                           __longopts: *const option,
                           __longind: *mut libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:54"]
pub mod krb5_h {
    #[c2rust::src_loc = "136:1"]
    pub type krb5_octet = uint8_t;
    #[c2rust::src_loc = "139:1"]
    pub type krb5_int32 = int32_t;
    #[c2rust::src_loc = "140:1"]
    pub type krb5_ui_4 = uint32_t;
    #[c2rust::src_loc = "174:1"]
    pub type krb5_boolean = libc::c_uint;
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    #[c2rust::src_loc = "178:1"]
    pub type krb5_addrtype = krb5_int32;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "181:1"]
    pub type krb5_authdatatype = krb5_int32;
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
    #[c2rust::src_loc = "225:1"]
    pub type krb5_pointer = *mut libc::c_void;
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
    #[c2rust::src_loc = "261:1"]
    pub type krb5_const_principal = *const krb5_principal_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "323:16"]
    pub struct _krb5_address {
        pub magic: krb5_magic,
        pub addrtype: krb5_addrtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "323:1"]
    pub type krb5_address = _krb5_address;
    #[c2rust::src_loc = "8510:1"]
    pub type krb5_post_recv_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: krb5_error_code, _: *const krb5_data,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "351:1"]
    pub type krb5_context = *mut _krb5_context;
    #[c2rust::src_loc = "8475:1"]
    pub type krb5_pre_send_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    #[c2rust::src_loc = "8391:1"]
    pub type krb5_trace_callback
        =
        Option<unsafe extern "C" fn(_: krb5_context,
                                    _: *const krb5_trace_info,
                                    _: *mut libc::c_void) -> ()>;
    #[c2rust::src_loc = "8387:1"]
    pub type krb5_trace_info = _krb5_trace_info;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "8387:16"]
    pub struct _krb5_trace_info {
        pub message: *const libc::c_char,
    }
    #[c2rust::src_loc = "7865:1"]
    pub type krb5_prompt_type = krb5_int32;
    #[c2rust::src_loc = "354:1"]
    pub type krb5_auth_context = *mut _krb5_auth_context;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "398:16"]
    pub struct _krb5_enc_data {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub kvno: krb5_kvno,
        pub ciphertext: krb5_data,
    }
    #[c2rust::src_loc = "398:1"]
    pub type krb5_enc_data = _krb5_enc_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1936:16"]
    pub struct _krb5_ticket_times {
        pub authtime: krb5_timestamp,
        pub starttime: krb5_timestamp,
        pub endtime: krb5_timestamp,
        pub renew_till: krb5_timestamp,
    }
    #[c2rust::src_loc = "1936:1"]
    pub type krb5_ticket_times = _krb5_ticket_times;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1946:16"]
    pub struct _krb5_authdata {
        pub magic: krb5_magic,
        pub ad_type: krb5_authdatatype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    #[c2rust::src_loc = "1946:1"]
    pub type krb5_authdata = _krb5_authdata;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1954:16"]
    pub struct _krb5_transited {
        pub magic: krb5_magic,
        pub tr_type: krb5_octet,
        pub tr_contents: krb5_data,
    }
    #[c2rust::src_loc = "1954:1"]
    pub type krb5_transited = _krb5_transited;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1961:16"]
    pub struct _krb5_enc_tkt_part {
        pub magic: krb5_magic,
        pub flags: krb5_flags,
        pub session: *mut krb5_keyblock,
        pub client: krb5_principal,
        pub transited: krb5_transited,
        pub times: krb5_ticket_times,
        pub caddrs: *mut *mut krb5_address,
        pub authorization_data: *mut *mut krb5_authdata,
    }
    #[c2rust::src_loc = "1961:1"]
    pub type krb5_enc_tkt_part = _krb5_enc_tkt_part;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1979:16"]
    pub struct _krb5_ticket {
        pub magic: krb5_magic,
        pub server: krb5_principal,
        pub enc_part: krb5_enc_data,
        pub enc_part2: *mut krb5_enc_tkt_part,
    }
    #[c2rust::src_loc = "1979:1"]
    pub type krb5_ticket = _krb5_ticket;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2107:16"]
    pub struct _krb5_error {
        pub magic: krb5_magic,
        pub ctime: krb5_timestamp,
        pub cusec: krb5_int32,
        pub susec: krb5_int32,
        pub stime: krb5_timestamp,
        pub error: krb5_ui_4,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub text: krb5_data,
        pub e_data: krb5_data,
    }
    #[c2rust::src_loc = "2107:1"]
    pub type krb5_error = _krb5_error;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2246:16"]
    pub struct krb5_replay_data {
        pub timestamp: krb5_timestamp,
        pub usec: krb5_int32,
        pub seq: krb5_ui_4,
    }
    #[c2rust::src_loc = "2724:1"]
    pub type krb5_kt_cursor = krb5_pointer;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2727:16"]
    pub struct krb5_keytab_entry_st {
        pub magic: krb5_magic,
        pub principal: krb5_principal,
        pub timestamp: krb5_timestamp,
        pub vno: krb5_kvno,
        pub key: krb5_keyblock,
    }
    #[c2rust::src_loc = "2727:1"]
    pub type krb5_keytab_entry = krb5_keytab_entry_st;
    #[c2rust::src_loc = "2736:1"]
    pub type krb5_keytab = *mut _krb5_kt;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::stdint_intn_h::int32_t;
    use super::k5_int_h::{_krb5_context, _krb5_kt};
    use super::stddef_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        #[c2rust::src_loc = "353:8"]
        pub type _krb5_auth_context;
        #[no_mangle]
        #[c2rust::src_loc = "2972:1"]
        pub fn krb5_free_context(context: krb5_context);
        #[no_mangle]
        #[c2rust::src_loc = "3292:1"]
        pub fn krb5_mk_error(context: krb5_context,
                             dec_err: *const krb5_error,
                             enc_err: *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3309:1"]
        pub fn krb5_rd_error(context: krb5_context,
                             enc_errbuf: *const krb5_data,
                             dec_error: *mut *mut krb5_error)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3349:1"]
        pub fn krb5_rd_safe(context: krb5_context,
                            auth_context_0: krb5_auth_context,
                            inbuf: *const krb5_data,
                            userdata_out: *mut krb5_data,
                            rdata_out: *mut krb5_replay_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3390:1"]
        pub fn krb5_rd_priv(context: krb5_context,
                            auth_context_0: krb5_auth_context,
                            inbuf: *const krb5_data,
                            userdata_out: *mut krb5_data,
                            rdata_out: *mut krb5_replay_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3489:1"]
        pub fn krb5_unparse_name(context: krb5_context,
                                 principal: krb5_const_principal,
                                 name: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "3813:1"]
        pub fn krb5_copy_principal(context: krb5_context,
                                   inprinc: krb5_const_principal,
                                   outprinc: *mut krb5_principal)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4173:1"]
        pub fn krb5_kt_resolve(context: krb5_context,
                               name: *const libc::c_char,
                               ktid: *mut krb5_keytab) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
        #[no_mangle]
        #[c2rust::src_loc = "4644:1"]
        pub fn krb5_free_ticket(context: krb5_context, val: *mut krb5_ticket);
        #[no_mangle]
        #[c2rust::src_loc = "4655:1"]
        pub fn krb5_free_error(context: krb5_context, val: *mut krb5_error);
        #[no_mangle]
        #[c2rust::src_loc = "4758:1"]
        pub fn krb5_free_data_contents(context: krb5_context,
                                       val: *mut krb5_data);
        #[no_mangle]
        #[c2rust::src_loc = "4819:1"]
        pub fn krb5_us_timeofday(context: krb5_context,
                                 seconds: *mut krb5_timestamp,
                                 microseconds: *mut krb5_int32)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4887:1"]
        pub fn krb5_get_default_realm(context: krb5_context,
                                      lrealm: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4903:1"]
        pub fn krb5_set_default_realm(context: krb5_context,
                                      lrealm: *const libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "4912:1"]
        pub fn krb5_free_default_realm(context: krb5_context,
                                       lrealm: *mut libc::c_char);
        #[no_mangle]
        #[c2rust::src_loc = "5293:1"]
        pub fn krb5_mk_safe(context: krb5_context,
                            auth_context_0: krb5_auth_context,
                            userdata: *const krb5_data,
                            der_out: *mut krb5_data,
                            rdata_out: *mut krb5_replay_data)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5421:1"]
        pub fn krb5_recvauth(context: krb5_context,
                             auth_context_0: *mut krb5_auth_context,
                             fd: krb5_pointer,
                             appl_version: *mut libc::c_char,
                             server_0: krb5_principal, flags: krb5_int32,
                             keytab: krb5_keytab,
                             ticket: *mut *mut krb5_ticket)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5613:1"]
        pub fn krb5_auth_con_init(context: krb5_context,
                                  auth_context_0: *mut krb5_auth_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5644:1"]
        pub fn krb5_auth_con_setflags(context: krb5_context,
                                      auth_context_0: krb5_auth_context,
                                      flags: krb5_int32) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5718:1"]
        pub fn krb5_auth_con_setaddrs(context: krb5_context,
                                      auth_context_0: krb5_auth_context,
                                      local_addr: *mut krb5_address,
                                      remote_addr: *mut krb5_address)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "5987:1"]
        pub fn krb5_auth_con_initivector(context: krb5_context,
                                         auth_context_0: krb5_auth_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6266:1"]
        pub fn krb5_string_to_enctype(string: *mut libc::c_char,
                                      enctypep: *mut krb5_enctype)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "6341:1"]
        pub fn krb5_enctype_to_name(enctype: krb5_enctype,
                                    shortest: krb5_boolean,
                                    buffer: *mut libc::c_char, buflen: size_t)
         -> krb5_error_code;
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:54"]
pub mod k5_int_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1208:8"]
    pub struct _krb5_context {
        pub magic: krb5_magic,
        pub in_tkt_etypes: *mut krb5_enctype,
        pub tgs_etypes: *mut krb5_enctype,
        pub os_context: _krb5_os_context,
        pub default_realm: *mut libc::c_char,
        pub profile: profile_t,
        pub dal_handle: *mut kdb5_dal_handle,
        pub clockskew: krb5_deltat,
        pub kdc_default_options: krb5_flags,
        pub library_options: krb5_flags,
        pub profile_secure: krb5_boolean,
        pub fcc_default_format: libc::c_int,
        pub prompt_types: *mut krb5_prompt_type,
        pub udp_pref_limit: libc::c_int,
        pub use_conf_ktypes: krb5_boolean,
        pub libkrb5_plugins: plugin_dir_handle,
        pub preauth_context: krb5_preauth_context,
        pub ccselect_handles: *mut *mut ccselect_module_handle,
        pub localauth_handles: *mut *mut localauth_module_handle,
        pub hostrealm_handles: *mut *mut hostrealm_module_handle,
        pub tls: *mut k5_tls_vtable_st,
        pub err: errinfo,
        pub err_fmt: *mut libc::c_char,
        pub kdblog_context: *mut _kdb_log_context,
        pub allow_weak_crypto: krb5_boolean,
        pub ignore_acceptor_hostname: krb5_boolean,
        pub enforce_ok_as_delegate: krb5_boolean,
        pub dns_canonicalize_hostname: dns_canonhost,
        pub trace_callback: krb5_trace_callback,
        pub trace_callback_data: *mut libc::c_void,
        pub kdc_send_hook: krb5_pre_send_fn,
        pub kdc_send_hook_data: *mut libc::c_void,
        pub kdc_recv_hook: krb5_post_recv_fn,
        pub kdc_recv_hook_data: *mut libc::c_void,
        pub plugins: [plugin_interface; 13],
        pub plugin_base_dir: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1137:8"]
    pub struct plugin_interface {
        pub modules: *mut *mut plugin_mapping,
        pub configured: krb5_boolean,
    }
    #[c2rust::src_loc = "1194:1"]
    pub type dns_canonhost = libc::c_uint;
    #[c2rust::src_loc = "1197:5"]
    pub const CANONHOST_FALLBACK: dns_canonhost = 2;
    #[c2rust::src_loc = "1196:5"]
    pub const CANONHOST_TRUE: dns_canonhost = 1;
    #[c2rust::src_loc = "1195:5"]
    pub const CANONHOST_FALSE: dns_canonhost = 0;
    #[c2rust::src_loc = "1203:1"]
    pub type krb5_preauth_context = *mut krb5_preauth_context_st;
    #[c2rust::src_loc = "1201:1"]
    pub type kdb5_dal_handle = _kdb5_dal_handle;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "702:16"]
    pub struct _krb5_os_context {
        pub magic: krb5_magic,
        pub time_offset: krb5_int32,
        pub usec_offset: krb5_int32,
        pub os_flags: krb5_int32,
        pub default_ccname: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2076:8"]
    pub struct _krb5_kt {
        pub magic: krb5_magic,
        pub ops: *const _krb5_kt_ops,
        pub data: krb5_pointer,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2040:16"]
    pub struct _krb5_kt_ops {
        pub magic: krb5_magic,
        pub prefix: *mut libc::c_char,
        pub resolve: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *const libc::c_char,
                                                 _: *mut krb5_keytab)
                                -> krb5_error_code>,
        pub get_name: Option<unsafe extern "C" fn(_: krb5_context,
                                                  _: krb5_keytab,
                                                  _: *mut libc::c_char,
                                                  _: libc::c_uint)
                                 -> krb5_error_code>,
        pub close: Option<unsafe extern "C" fn(_: krb5_context,
                                               _: krb5_keytab)
                              -> krb5_error_code>,
        pub get: Option<unsafe extern "C" fn(_: krb5_context, _: krb5_keytab,
                                             _: krb5_const_principal,
                                             _: krb5_kvno, _: krb5_enctype,
                                             _: *mut krb5_keytab_entry)
                            -> krb5_error_code>,
        pub start_seq_get: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: krb5_keytab,
                                                       _: *mut krb5_kt_cursor)
                                      -> krb5_error_code>,
        pub get_next: Option<unsafe extern "C" fn(_: krb5_context,
                                                  _: krb5_keytab,
                                                  _: *mut krb5_keytab_entry,
                                                  _: *mut krb5_kt_cursor)
                                 -> krb5_error_code>,
        pub end_get: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: krb5_keytab,
                                                 _: *mut krb5_kt_cursor)
                                -> krb5_error_code>,
        pub add: Option<unsafe extern "C" fn(_: krb5_context, _: krb5_keytab,
                                             _: *mut krb5_keytab_entry)
                            -> krb5_error_code>,
        pub remove: Option<unsafe extern "C" fn(_: krb5_context,
                                                _: krb5_keytab,
                                                _: *mut krb5_keytab_entry)
                               -> krb5_error_code>,
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_pointer, krb5_error_code, krb5_context,
                        krb5_keytab, krb5_const_principal, krb5_kvno,
                        krb5_keytab_entry, krb5_kt_cursor, krb5_data};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::kdb_log_h::_kdb_log_context;
    extern "C" {
        #[c2rust::src_loc = "1134:8"]
        pub type plugin_mapping;
        #[c2rust::src_loc = "1207:8"]
        pub type k5_tls_vtable_st;
        #[c2rust::src_loc = "1206:8"]
        pub type hostrealm_module_handle;
        #[c2rust::src_loc = "1205:8"]
        pub type localauth_module_handle;
        #[c2rust::src_loc = "1204:8"]
        pub type ccselect_module_handle;
        #[c2rust::src_loc = "1203:16"]
        pub type krb5_preauth_context_st;
        #[c2rust::src_loc = "1200:8"]
        pub type _kdb5_dal_handle;
        #[no_mangle]
        #[c2rust::src_loc = "614:1"]
        pub fn krb5_lock_file(_: krb5_context, _: libc::c_int, _: libc::c_int)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "619:1"]
        pub fn krb5int_init_context_kdc(_: *mut krb5_context)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2214:1"]
        pub fn krb5_read_message(_: krb5_context, _: krb5_pointer,
                                 _: *mut krb5_data) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "2215:1"]
        pub fn krb5_write_message(_: krb5_context, _: krb5_pointer,
                                  _: *mut krb5_data) -> krb5_error_code;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kdb_log.h:78"]
pub mod kdb_log_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "102:16"]
    pub struct _kdb_log_context {
        pub iproprole: iprop_role,
        pub ulog: *mut kdb_hlog_t,
        pub ulogentries: uint32_t,
        pub ulogfd: libc::c_int,
    }
    #[c2rust::src_loc = "81:1"]
    pub type kdb_hlog_t = kdb_hlog;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:16"]
    pub struct kdb_hlog {
        pub kdb_hmagic: uint32_t,
        pub db_version_num: uint16_t,
        pub kdb_num: uint32_t,
        pub kdb_first_time: kdbe_time_t,
        pub kdb_last_time: kdbe_time_t,
        pub kdb_first_sno: kdb_sno_t,
        pub kdb_last_sno: kdb_sno_t,
        pub kdb_state: uint16_t,
        pub kdb_block: uint16_t,
    }
    #[c2rust::src_loc = "102:1"]
    pub type kdb_log_context = _kdb_log_context;
    use super::iprop_hdr_h::{iprop_role, IPROP_NULL};
    use super::stdint_uintn_h::{uint32_t, uint16_t};
    use super::iprop_h::{kdbe_time_t, kdb_sno_t, kdb_incr_result_t,
                         kdb_last_t};
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_error_code};
    extern "C" {
        /* Log header magic # */
        /* Kerberos database version no. */
        /* # of updates in log */
        /* Timestamp of first update */
        /* Timestamp of last update */
        /* First serial # in the update log */
        /* Last serial # in the update log */
        /* State of update log */
        /* Block size of each element */
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
        /* #pragma ident        "@(#)kdb_log.h  1.3     04/02/23 SMI" */
        /*
 * DB macros
 */
        /*
 * Current DB version #
 */
        /*
 * DB log states
 */
        /*
 * DB log constants
 */
        /*
 * Default ulog file attributes
 */
        /* in seconds */
        /*
 * Max size of update entry + update header
 * We make this large since resizing can be costly.
 */
        /* Default size of principal record */
        /* 256 MB log file */
        /*
 * Prototype declarations
 */
        #[no_mangle]
        #[c2rust::src_loc = "61:1"]
        pub fn ulog_map(context: krb5_context, logname: *const libc::c_char,
                        entries: uint32_t) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "67:1"]
        pub fn ulog_replay(context: krb5_context,
                           incr_ret: *mut kdb_incr_result_t,
                           db_args_0: *mut *mut libc::c_char)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "74:1"]
        pub fn ulog_set_role(ctx: krb5_context, role: iprop_role)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "77:1"]
        pub fn ulog_get_last(context: krb5_context, last_out: *mut kdb_last_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn ulog_fini(context: krb5_context);
    }
    /* !_KDB_LOG_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop.h:76"]
pub mod iprop_h {
    #[c2rust::src_loc = "22:1"]
    pub type kdb_sno_t = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:8"]
    pub struct kdbe_time_t {
        pub seconds: uint32_t,
        pub useconds: uint32_t,
    }
    /*
 * Please do not edit this file.
 * It was generated using rpcgen.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "17:9"]
    pub struct utf8str_t {
        pub utf8str_t_len: u_int,
        pub utf8str_t_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:8"]
    pub struct kdbe_key_t {
        pub k_ver: int32_t,
        pub k_kvno: int32_t,
        pub k_enctype: C2RustUnnamed_24,
        pub k_contents: C2RustUnnamed_23,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:2"]
    pub struct C2RustUnnamed_23 {
        pub k_contents_len: u_int,
        pub k_contents_val: *mut utf8str_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:2"]
    pub struct C2RustUnnamed_24 {
        pub k_enctype_len: u_int,
        pub k_enctype_val: *mut int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "44:8"]
    pub struct kdbe_data_t {
        pub k_magic: int32_t,
        pub k_data: utf8str_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct kdbe_princ_t {
        pub k_realm: utf8str_t,
        pub k_components: C2RustUnnamed_25,
        pub k_nametype: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:2"]
    pub struct C2RustUnnamed_25 {
        pub k_components_len: u_int,
        pub k_components_val: *mut kdbe_data_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:8"]
    pub struct kdbe_tl_t {
        pub tl_type: int16_t,
        pub tl_data: C2RustUnnamed_26,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:2"]
    pub struct C2RustUnnamed_26 {
        pub tl_data_len: u_int,
        pub tl_data_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "69:9"]
    pub struct kdbe_pw_hist_t {
        pub kdbe_pw_hist_t_len: u_int,
        pub kdbe_pw_hist_t_val: *mut kdbe_key_t,
    }
    #[c2rust::src_loc = "74:1"]
    pub type kdbe_attr_type_t = libc::c_uint;
    #[c2rust::src_loc = "94:2"]
    pub const AT_PW_HIST: kdbe_attr_type_t = 19;
    #[c2rust::src_loc = "93:2"]
    pub const AT_PW_HIST_KVNO: kdbe_attr_type_t = 18;
    #[c2rust::src_loc = "92:2"]
    pub const AT_PW_POLICY_SWITCH: kdbe_attr_type_t = 17;
    #[c2rust::src_loc = "91:2"]
    pub const AT_PW_POLICY: kdbe_attr_type_t = 16;
    #[c2rust::src_loc = "90:2"]
    pub const AT_PW_LAST_CHANGE: kdbe_attr_type_t = 15;
    #[c2rust::src_loc = "89:2"]
    pub const AT_MOD_WHERE: kdbe_attr_type_t = 14;
    #[c2rust::src_loc = "88:2"]
    pub const AT_MOD_TIME: kdbe_attr_type_t = 13;
    #[c2rust::src_loc = "87:2"]
    pub const AT_MOD_PRINC: kdbe_attr_type_t = 12;
    #[c2rust::src_loc = "86:2"]
    pub const AT_LEN: kdbe_attr_type_t = 11;
    #[c2rust::src_loc = "85:2"]
    pub const AT_TL_DATA: kdbe_attr_type_t = 10;
    #[c2rust::src_loc = "84:2"]
    pub const AT_KEYDATA: kdbe_attr_type_t = 9;
    #[c2rust::src_loc = "83:2"]
    pub const AT_PRINC: kdbe_attr_type_t = 8;
    #[c2rust::src_loc = "82:2"]
    pub const AT_FAIL_AUTH_COUNT: kdbe_attr_type_t = 7;
    #[c2rust::src_loc = "81:2"]
    pub const AT_LAST_FAILED: kdbe_attr_type_t = 6;
    #[c2rust::src_loc = "80:2"]
    pub const AT_LAST_SUCCESS: kdbe_attr_type_t = 5;
    #[c2rust::src_loc = "79:2"]
    pub const AT_PW_EXP: kdbe_attr_type_t = 4;
    #[c2rust::src_loc = "78:2"]
    pub const AT_EXP: kdbe_attr_type_t = 3;
    #[c2rust::src_loc = "77:2"]
    pub const AT_MAX_RENEW_LIFE: kdbe_attr_type_t = 2;
    #[c2rust::src_loc = "76:2"]
    pub const AT_MAX_LIFE: kdbe_attr_type_t = 1;
    #[c2rust::src_loc = "75:2"]
    pub const AT_ATTRFLAGS: kdbe_attr_type_t = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:8"]
    pub struct kdbe_val_t {
        pub av_type: kdbe_attr_type_t,
        pub kdbe_val_t_u: C2RustUnnamed_27,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "100:2"]
    pub union C2RustUnnamed_27 {
        pub av_attrflags: uint32_t,
        pub av_max_life: uint32_t,
        pub av_max_renew_life: uint32_t,
        pub av_exp: uint32_t,
        pub av_pw_exp: uint32_t,
        pub av_last_success: uint32_t,
        pub av_last_failed: uint32_t,
        pub av_fail_auth_count: uint32_t,
        pub av_princ: kdbe_princ_t,
        pub av_keydata: C2RustUnnamed_31,
        pub av_tldata: C2RustUnnamed_30,
        pub av_len: int16_t,
        pub av_pw_last_change: uint32_t,
        pub av_mod_princ: kdbe_princ_t,
        pub av_mod_time: uint32_t,
        pub av_mod_where: utf8str_t,
        pub av_pw_policy: utf8str_t,
        pub av_pw_policy_switch: libc::c_int,
        pub av_pw_hist_kvno: uint32_t,
        pub av_pw_hist: C2RustUnnamed_29,
        pub av_extension: C2RustUnnamed_28,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "130:3"]
    pub struct C2RustUnnamed_28 {
        pub av_extension_len: u_int,
        pub av_extension_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "126:3"]
    pub struct C2RustUnnamed_29 {
        pub av_pw_hist_len: u_int,
        pub av_pw_hist_val: *mut kdbe_pw_hist_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "114:3"]
    pub struct C2RustUnnamed_30 {
        pub av_tldata_len: u_int,
        pub av_tldata_val: *mut kdbe_tl_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "110:3"]
    pub struct C2RustUnnamed_31 {
        pub av_keydata_len: u_int,
        pub av_keydata_val: *mut kdbe_key_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "138:9"]
    pub struct kdbe_t {
        pub kdbe_t_len: u_int,
        pub kdbe_t_val: *mut kdbe_val_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "143:8"]
    pub struct kdb_incr_update_t {
        pub kdb_princ_name: utf8str_t,
        pub kdb_entry_sno: kdb_sno_t,
        pub kdb_time: kdbe_time_t,
        pub kdb_update: kdbe_t,
        pub kdb_deleted: libc::c_int,
        pub kdb_commit: libc::c_int,
        pub kdb_kdcs_seen_by: C2RustUnnamed_33,
        pub kdb_futures: C2RustUnnamed_32,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "154:2"]
    pub struct C2RustUnnamed_32 {
        pub kdb_futures_len: u_int,
        pub kdb_futures_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "150:2"]
    pub struct C2RustUnnamed_33 {
        pub kdb_kdcs_seen_by_len: u_int,
        pub kdb_kdcs_seen_by_val: *mut utf8str_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "161:9"]
    pub struct kdb_ulog_t {
        pub kdb_ulog_t_len: u_int,
        pub kdb_ulog_t_val: *mut kdb_incr_update_t,
    }
    #[c2rust::src_loc = "166:1"]
    pub type update_status_t = libc::c_uint;
    #[c2rust::src_loc = "172:2"]
    pub const UPDATE_PERM_DENIED: update_status_t = 5;
    #[c2rust::src_loc = "171:2"]
    pub const UPDATE_NIL: update_status_t = 4;
    #[c2rust::src_loc = "170:2"]
    pub const UPDATE_BUSY: update_status_t = 3;
    #[c2rust::src_loc = "169:2"]
    pub const UPDATE_FULL_RESYNC_NEEDED: update_status_t = 2;
    #[c2rust::src_loc = "168:2"]
    pub const UPDATE_ERROR: update_status_t = 1;
    #[c2rust::src_loc = "167:2"]
    pub const UPDATE_OK: update_status_t = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "176:8"]
    pub struct kdb_last_t {
        pub last_sno: kdb_sno_t,
        pub last_time: kdbe_time_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "182:8"]
    pub struct kdb_incr_result_t {
        pub lastentry: kdb_last_t,
        pub updates: kdb_ulog_t,
        pub ret: update_status_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "189:8"]
    pub struct kdb_fullresync_result_t {
        pub lastentry: kdb_last_t,
        pub ret: update_status_t,
    }
    use super::stdint_uintn_h::uint32_t;
    use super::sys_types_h::u_int;
    use super::stdint_intn_h::{int32_t, int16_t};
    use super::clnt_h::CLIENT;
    use super::xdr_h::XDR;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "203:1"]
        pub fn iprop_get_updates_1(_: *mut kdb_last_t, _: *mut CLIENT)
         -> *mut kdb_incr_result_t;
        #[no_mangle]
        #[c2rust::src_loc = "248:1"]
        pub fn xdr_kdb_fullresync_result_t(_: *mut XDR,
                                           _: *mut kdb_fullresync_result_t)
         -> libc::c_int;
    }
    /* !_IPROP_H_RPCGEN */
    /* K&R C */
    /* K&R C */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop_hdr.h:75"]
pub mod iprop_hdr_h {
    /* Backoff for a maximum for 5 mts */
    #[c2rust::src_loc = "32:1"]
    pub type iprop_role = libc::c_uint;
    #[c2rust::src_loc = "35:5"]
    pub const IPROP_REPLICA: iprop_role = 2;
    #[c2rust::src_loc = "34:5"]
    pub const IPROP_MASTER: iprop_role = 1;
    #[c2rust::src_loc = "33:5"]
    pub const IPROP_NULL: iprop_role = 0;
    /* !_IPROP_HDR_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:54"]
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
    /* K5_ERR_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:54"]
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
    extern "C" {
        #[c2rust::src_loc = "80:8"]
        pub type plugin_file_handle;
    }
    /* K5_PLUGIN_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:54"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/com_err.h:54"]
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
    #[c2rust::src_loc = "27:1"]
    pub type et_old_error_hook_func
        =
        Option<unsafe extern "C" fn(_: *const libc::c_char, _: errcode_t,
                                    _: *const libc::c_char,
                                    _: *mut __va_list_tag) -> ()>;
    use super::internal::__va_list_tag;
    extern "C" {
        /* Public interfaces */
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn com_err(_: *const libc::c_char, _: errcode_t,
                       _: *const libc::c_char, _: ...);
        #[no_mangle]
        #[c2rust::src_loc = "54:1"]
        pub fn error_message(_: errcode_t) -> *const libc::c_char;
        /*@modifies internalState@*/
        /*
 * The display routine should be application specific.  A global hook,
 * may cause inappropriate display procedures to be called between
 * applications under non-Unix environments.
 */
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn set_com_err_hook(_: et_old_error_hook_func)
         -> et_old_error_hook_func;
    }
    /* ! defined(__COM_ERR_H) */
}
#[c2rust::header_src = "/usr/include/bits/socket_type.h:54"]
pub mod socket_type_h {
    #[c2rust::src_loc = "24:1"]
    pub type __socket_type = libc::c_uint;
    #[c2rust::src_loc = "52:3"]
    pub const SOCK_NONBLOCK: __socket_type = 2048;
    #[c2rust::src_loc = "49:3"]
    pub const SOCK_CLOEXEC: __socket_type = 524288;
    #[c2rust::src_loc = "41:3"]
    pub const SOCK_PACKET: __socket_type = 10;
    #[c2rust::src_loc = "39:3"]
    pub const SOCK_DCCP: __socket_type = 6;
    #[c2rust::src_loc = "36:3"]
    pub const SOCK_SEQPACKET: __socket_type = 5;
    #[c2rust::src_loc = "34:3"]
    pub const SOCK_RDM: __socket_type = 4;
    #[c2rust::src_loc = "32:3"]
    pub const SOCK_RAW: __socket_type = 3;
    #[c2rust::src_loc = "29:3"]
    pub const SOCK_DGRAM: __socket_type = 2;
    #[c2rust::src_loc = "26:3"]
    pub const SOCK_STREAM: __socket_type = 1;
}
#[c2rust::header_src = "/usr/include/bits/sockaddr.h:54"]
pub mod sockaddr_h {
    #[c2rust::src_loc = "28:1"]
    pub type sa_family_t = libc::c_ushort;
}
#[c2rust::header_src = "/usr/include/bits/socket.h:54"]
pub mod socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "178:8"]
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "191:8"]
    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        pub __ss_padding: [libc::c_char; 118],
        pub __ss_align: libc::c_ulong,
    }
    use super::sockaddr_h::sa_family_t;
}
#[c2rust::header_src = "/usr/include/sys/socket.h:54"]
pub mod sys_socket_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "79:9"]
    pub union __SOCKADDR_ARG {
        pub __sockaddr__: *mut sockaddr,
        pub __sockaddr_at__: *mut sockaddr_at,
        pub __sockaddr_ax25__: *mut sockaddr_ax25,
        pub __sockaddr_dl__: *mut sockaddr_dl,
        pub __sockaddr_eon__: *mut sockaddr_eon,
        pub __sockaddr_in__: *mut sockaddr_in,
        pub __sockaddr_in6__: *mut sockaddr_in6,
        pub __sockaddr_inarp__: *mut sockaddr_inarp,
        pub __sockaddr_ipx__: *mut sockaddr_ipx,
        pub __sockaddr_iso__: *mut sockaddr_iso,
        pub __sockaddr_ns__: *mut sockaddr_ns,
        pub __sockaddr_un__: *mut sockaddr_un,
        pub __sockaddr_x25__: *mut sockaddr_x25,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "83:9"]
    pub union __CONST_SOCKADDR_ARG {
        pub __sockaddr__: *const sockaddr,
        pub __sockaddr_at__: *const sockaddr_at,
        pub __sockaddr_ax25__: *const sockaddr_ax25,
        pub __sockaddr_dl__: *const sockaddr_dl,
        pub __sockaddr_eon__: *const sockaddr_eon,
        pub __sockaddr_in__: *const sockaddr_in,
        pub __sockaddr_in6__: *const sockaddr_in6,
        pub __sockaddr_inarp__: *const sockaddr_inarp,
        pub __sockaddr_ipx__: *const sockaddr_ipx,
        pub __sockaddr_iso__: *const sockaddr_iso,
        pub __sockaddr_ns__: *const sockaddr_ns,
        pub __sockaddr_un__: *const sockaddr_un,
        pub __sockaddr_x25__: *const sockaddr_x25,
    }
    use super::socket_h::sockaddr;
    use super::in_h::{sockaddr_in, sockaddr_in6};
    use super::unistd_h::socklen_t;
    extern "C" {
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_x25;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_un;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ns;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_iso;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ipx;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_inarp;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_eon;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_dl;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_ax25;
        #[c2rust::src_loc = "79:17"]
        pub type sockaddr_at;
        #[no_mangle]
        #[c2rust::src_loc = "102:1"]
        pub fn socket(__domain: libc::c_int, __type: libc::c_int,
                      __protocol: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "112:1"]
        pub fn bind(__fd: libc::c_int, __addr: __CONST_SOCKADDR_ARG,
                    __len: socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "116:1"]
        pub fn getsockname(__fd: libc::c_int, __addr: __SOCKADDR_ARG,
                           __len: *mut socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "130:1"]
        pub fn getpeername(__fd: libc::c_int, __addr: __SOCKADDR_ARG,
                           __len: *mut socklen_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "215:1"]
        pub fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                          __optname: libc::c_int,
                          __optval: *const libc::c_void, __optlen: socklen_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "222:1"]
        pub fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "232:1"]
        pub fn accept(__fd: libc::c_int, __addr: __SOCKADDR_ARG,
                      __addr_len: *mut socklen_t) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/netinet/in.h:54"]
pub mod in_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "253:8"]
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: uint32_t,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "212:8"]
    pub struct in6_addr {
        pub __in6_u: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "214:5"]
    pub union C2RustUnnamed {
        pub __u6_addr8: [uint8_t; 16],
        pub __u6_addr16: [uint16_t; 8],
        pub __u6_addr32: [uint32_t; 4],
    }
    #[c2rust::src_loc = "119:1"]
    pub type in_port_t = uint16_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "238:8"]
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [libc::c_uchar; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:8"]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    #[c2rust::src_loc = "30:1"]
    pub type in_addr_t = uint32_t;
    #[c2rust::src_loc = "40:1"]
    pub type C2RustUnnamed_0 = libc::c_uint;
    #[c2rust::src_loc = "92:5"]
    pub const IPPROTO_MAX: C2RustUnnamed_0 = 256;
    #[c2rust::src_loc = "90:5"]
    pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
    #[c2rust::src_loc = "88:5"]
    pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
    #[c2rust::src_loc = "86:5"]
    pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
    #[c2rust::src_loc = "84:5"]
    pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
    #[c2rust::src_loc = "82:5"]
    pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
    #[c2rust::src_loc = "80:5"]
    pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
    #[c2rust::src_loc = "78:5"]
    pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
    #[c2rust::src_loc = "76:5"]
    pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
    #[c2rust::src_loc = "74:5"]
    pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
    #[c2rust::src_loc = "72:5"]
    pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
    #[c2rust::src_loc = "70:5"]
    pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
    #[c2rust::src_loc = "68:5"]
    pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
    #[c2rust::src_loc = "66:5"]
    pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
    #[c2rust::src_loc = "64:5"]
    pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
    #[c2rust::src_loc = "62:5"]
    pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
    #[c2rust::src_loc = "60:5"]
    pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
    #[c2rust::src_loc = "58:5"]
    pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
    #[c2rust::src_loc = "56:5"]
    pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
    #[c2rust::src_loc = "54:5"]
    pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
    #[c2rust::src_loc = "52:5"]
    pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
    #[c2rust::src_loc = "50:5"]
    pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
    #[c2rust::src_loc = "48:5"]
    pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
    #[c2rust::src_loc = "46:5"]
    pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
    #[c2rust::src_loc = "44:5"]
    pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
    #[c2rust::src_loc = "42:5"]
    pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
    use super::sockaddr_h::sa_family_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t, uint16_t};
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "375:1"]
        pub fn ntohl(__netlong: uint32_t) -> uint32_t;
        #[no_mangle]
        #[c2rust::src_loc = "378:1"]
        pub fn htonl(__hostlong: uint32_t) -> uint32_t;
    }
}
#[c2rust::header_src = "/usr/include/netdb.h:54"]
pub mod netdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "565:8"]
    pub struct addrinfo {
        pub ai_flags: libc::c_int,
        pub ai_family: libc::c_int,
        pub ai_socktype: libc::c_int,
        pub ai_protocol: libc::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_addr: *mut sockaddr,
        pub ai_canonname: *mut libc::c_char,
        pub ai_next: *mut addrinfo,
    }
    use super::unistd_h::socklen_t;
    use super::socket_h::sockaddr;
}
#[c2rust::header_src = "/usr/include/bits/types/siginfo_t.h:54"]
pub mod siginfo_t_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:9"]
    pub struct siginfo_t {
        pub si_signo: libc::c_int,
        pub si_errno: libc::c_int,
        pub si_code: libc::c_int,
        pub __pad0: libc::c_int,
        pub _sifields: C2RustUnnamed_1,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "51:5"]
    pub union C2RustUnnamed_1 {
        pub _pad: [libc::c_int; 28],
        pub _kill: C2RustUnnamed_10,
        pub _timer: C2RustUnnamed_9,
        pub _rt: C2RustUnnamed_8,
        pub _sigchld: C2RustUnnamed_7,
        pub _sigfault: C2RustUnnamed_4,
        pub _sigpoll: C2RustUnnamed_3,
        pub _sigsys: C2RustUnnamed_2,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "116:2"]
    pub struct C2RustUnnamed_2 {
        pub _call_addr: *mut libc::c_void,
        pub _syscall: libc::c_int,
        pub _arch: libc::c_uint,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "108:2"]
    pub struct C2RustUnnamed_3 {
        pub si_band: libc::c_long,
        pub si_fd: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "89:2"]
    pub struct C2RustUnnamed_4 {
        pub si_addr: *mut libc::c_void,
        pub si_addr_lsb: libc::c_short,
        pub _bounds: C2RustUnnamed_5,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "94:6"]
    pub union C2RustUnnamed_5 {
        pub _addr_bnd: C2RustUnnamed_6,
        pub _pkey: __uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "97:3"]
    pub struct C2RustUnnamed_6 {
        pub _lower: *mut libc::c_void,
        pub _upper: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "79:2"]
    pub struct C2RustUnnamed_7 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_status: libc::c_int,
        pub si_utime: __clock_t,
        pub si_stime: __clock_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "71:2"]
    pub struct C2RustUnnamed_8 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
        pub si_sigval: __sigval_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "63:2"]
    pub struct C2RustUnnamed_9 {
        pub si_tid: libc::c_int,
        pub si_overrun: libc::c_int,
        pub si_sigval: __sigval_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "56:2"]
    pub struct C2RustUnnamed_10 {
        pub si_pid: __pid_t,
        pub si_uid: __uid_t,
    }
    use super::types_h::{__uint32_t, __pid_t, __uid_t, __clock_t};
    use super::__sigval_t_h::__sigval_t;
}
#[c2rust::header_src = "/usr/include/signal.h:54"]
pub mod signal_h {
    #[c2rust::src_loc = "72:1"]
    pub type __sighandler_t
        =
        Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
    use super::types_h::__pid_t;
    use super::sigset_t_h::sigset_t;
    use super::sigaction_h::sigaction;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "112:1"]
        pub fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "196:1"]
        pub fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "240:1"]
        pub fn sigaction(__sig: libc::c_int, __act: *const sigaction,
                         __oact: *mut sigaction) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/sigaction.h:54"]
pub mod sigaction_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:8"]
    pub struct sigaction {
        pub __sigaction_handler: C2RustUnnamed_11,
        pub sa_mask: __sigset_t,
        pub sa_flags: libc::c_int,
        pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "31:5"]
    pub union C2RustUnnamed_11 {
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
#[c2rust::header_src = "/usr/include/sys/time.h:54"]
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
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kadm5/admin.h:77"]
pub mod admin_h {
    #[c2rust::src_loc = "71:1"]
    pub type kadm5_ret_t = libc::c_long;
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
    use super::krb5_h::{krb5_enctype, krb5_deltat, krb5_timestamp, krb5_flags,
                        krb5_int32, krb5_kvno, krb5_context, krb5_error_code,
                        krb5_ui_4};
    use super::kdb_h::krb5_key_salt_tuple;
    use super::stdint_uintn_h::uint32_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "294:1"]
        pub fn kadm5_get_config_params(context: krb5_context,
                                       use_kdc_config: libc::c_int,
                                       params_in: *mut kadm5_config_params,
                                       params_out: *mut kadm5_config_params)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "335:1"]
        pub fn kadm5_init_with_skey(context: krb5_context,
                                    client_name: *mut libc::c_char,
                                    keytab: *mut libc::c_char,
                                    service_name: *mut libc::c_char,
                                    params_0: *mut kadm5_config_params,
                                    struct_version: krb5_ui_4,
                                    api_version: krb5_ui_4,
                                    db_args_0: *mut *mut libc::c_char,
                                    server_handle: *mut *mut libc::c_void)
         -> kadm5_ret_t;
        #[no_mangle]
        #[c2rust::src_loc = "356:1"]
        pub fn kadm5_destroy(server_handle: *mut libc::c_void) -> kadm5_ret_t;
    }
    /* __KADM5_ADMIN_H__ */
}
#[c2rust::header_src = "/usr/include/ctype.h:59"]
pub mod ctype_h {
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed_12 = libc::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed_12 = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed_12 = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed_12 = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed_12 = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed_12 = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed_12 = 16384;
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed_12 = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed_12 = 4096;
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed_12 = 2048;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed_12 = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed_12 = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed_12 = 256;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:76"]
pub mod gssrpc_types_h {
    #[c2rust::src_loc = "88:1"]
    pub type rpcprog_t = uint32_t;
    #[c2rust::src_loc = "89:1"]
    pub type rpcvers_t = uint32_t;
    #[c2rust::src_loc = "91:1"]
    pub type rpcproc_t = uint32_t;
    /* @(#)types.h	2.3 88/08/15 4.0 RPCSRC */
/*
 * Copyright (c) 2010, Oracle America, Inc.
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 *     * Neither the name of the “Oracle America, Inc.” nor the names of
 *       its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
 * TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/*      @(#)types.h 1.18 87/07/24 SMI      */
    /*
 * Rpc additions to <sys/types.h>
 */
    /*
 * Try to get MAXHOSTNAMELEN from somewhere.
 */
    /* #include <netdb.h> */
    /* Get htonl(), ntohl(), etc. */
    /* Define if we need to fake up some BSD type aliases. */
    /* Allow application to override. */
    /* #undef GSSRPC__BSD_TYPEALIASES */
    #[c2rust::src_loc = "93:1"]
    pub type rpc_inline_t = int32_t;
    use super::stdint_uintn_h::uint32_t;
    use super::stdint_intn_h::int32_t;
    /* !defined(GSSRPC_TYPES_H) */
    /*
 * The below should probably be internal-only, but seem to be
 * traditionally exported in RPC implementations.
 */
    /* XXX namespace */
    /* This is for rpc/netdb.h */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:76"]
pub mod xdr_h {
    /* @(#)xdr.h	2.2 88/07/29 4.0 RPCSRC */
/*
 * Copyright (c) 2010, Oracle America, Inc.
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 *     * Neither the name of the "Oracle America, Inc." nor the names of
 *       its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
 * TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/*      @(#)xdr.h 1.19 87/04/22 SMI      */
    /*
 * xdr.h, External Data Representation Serialization Routines.
 */
    /* for FILE */
    /*
 * XDR provides a conventional way for converting between C data
 * types and an external bit-string representation.  Library supplied
 * routines provide for the conversion on built-in C data types.  These
 * routines and utility routines defined here are used to help implement
 * a type encode/decode routine for each user-defined type.
 *
 * Each data type provides a single procedure which takes two arguments:
 *
 *	bool_t
 *	xdrproc(xdrs, argresp)
 *		XDR *xdrs;
 *		<type> *argresp;
 *
 * xdrs is an instance of a XDR handle, to which or from which the data
 * type is to be converted.  argresp is a pointer to the structure to be
 * converted.  The XDR handle contains an operation field which indicates
 * which of the operations (ENCODE, DECODE * or FREE) is to be performed.
 *
 * XDR_DECODE may allocate space if the pointer argresp is null.  This
 * data can be freed with the XDR_FREE operation.
 *
 * We write only one procedure per data type to make it easy
 * to keep the encode and decode procedures for a data type consistent.
 * In many cases the same code performs all operations on a user defined type,
 * because all the hard work is done in the component type routines.
 * decode as a series of calls on the nested data types.
 */
    /*
 * Xdr operations.  XDR_ENCODE causes the type to be encoded into the
 * stream.  XDR_DECODE causes the type to be extracted from the stream.
 * XDR_FREE can be used to release the space allocated by an XDR_DECODE
 * request.
 */
    #[c2rust::src_loc = "81:1"]
    pub type xdr_op = libc::c_uint;
    #[c2rust::src_loc = "84:2"]
    pub const XDR_FREE: xdr_op = 2;
    #[c2rust::src_loc = "83:2"]
    pub const XDR_DECODE: xdr_op = 1;
    #[c2rust::src_loc = "82:2"]
    pub const XDR_ENCODE: xdr_op = 0;
    /*
 * This is the number of bytes per unit of external data.
 */
    /*
 * A xdrproc_t exists for each data type which is to be encoded or decoded.
 *
 * The second argument to the xdrproc_t is a pointer to an opaque pointer.
 * The opaque pointer generally points to a structure of the data type
 * to be decoded.  If this pointer is 0, then the type routines should
 * allocate dynamic storage of the appropriate size and return it.
 * bool_t	(*xdrproc_t)(XDR *, caddr_t *);
 *
 * XXX can't actually prototype it, because some take three args!!!
 */
    #[c2rust::src_loc = "105:1"]
    pub type xdrproc_t = Option<unsafe extern "C" fn() -> libc::c_int>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "113:16"]
    pub struct XDR {
        pub x_op: xdr_op,
        pub x_ops: *mut xdr_ops,
        pub x_public: caddr_t,
        pub x_private: *mut libc::c_void,
        pub x_base: caddr_t,
        pub x_handy: libc::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "115:9"]
    pub struct xdr_ops {
        pub x_getlong: Option<unsafe extern "C" fn(_: *mut XDR,
                                                   _: *mut libc::c_long)
                                  -> libc::c_int>,
        pub x_putlong: Option<unsafe extern "C" fn(_: *mut XDR,
                                                   _: *mut libc::c_long)
                                  -> libc::c_int>,
        pub x_getbytes: Option<unsafe extern "C" fn(_: *mut XDR, _: caddr_t,
                                                    _: u_int) -> libc::c_int>,
        pub x_putbytes: Option<unsafe extern "C" fn(_: *mut XDR, _: caddr_t,
                                                    _: u_int) -> libc::c_int>,
        pub x_getpostn: Option<unsafe extern "C" fn(_: *mut XDR) -> u_int>,
        pub x_setpostn: Option<unsafe extern "C" fn(_: *mut XDR, _: u_int)
                                   -> libc::c_int>,
        pub x_inline: Option<unsafe extern "C" fn(_: *mut XDR, _: libc::c_int)
                                 -> *mut rpc_inline_t>,
        pub x_destroy: Option<unsafe extern "C" fn(_: *mut XDR) -> ()>,
    }
    use super::sys_types_h::{caddr_t, u_int};
    use super::gssrpc_types_h::rpc_inline_t;
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        /*
 * In-line routines for fast encode/decode of primitve data types.
 * Caveat emptor: these use single memory cycles to get the
 * data from the underlying buffer, and will fail to operate
 * properly if the data is not aligned.  The standard way to use these
 * is to say:
 *	if ((buf = XDR_INLINE(xdrs, count)) == NULL)
 *		return (FALSE);
 *	<<< macro calls >>>
 * where ``count'' is the number of bytes of data occupied
 * by the primitive data types.
 *
 * N.B. and frozen for all time: each data type here uses 4 bytes
 * of external representation.
 */
        /*
 * These are the "generic" xdr routines.
 */
        #[no_mangle]
        #[c2rust::src_loc = "251:1"]
        pub fn gssrpc_xdr_void(_: *mut XDR, _: *mut libc::c_void)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "298:1"]
        pub fn gssrpc_xdr_u_int32(_: *mut XDR, _: *mut uint32_t)
         -> libc::c_int;
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/auth.h:76"]
pub mod auth_h {
    #[c2rust::src_loc = "55:1"]
    pub type auth_stat = libc::c_uint;
    #[c2rust::src_loc = "74:2"]
    pub const RPCSEC_GSS_CTXPROBLEM: auth_stat = 14;
    #[c2rust::src_loc = "73:2"]
    pub const RPCSEC_GSS_CREDPROBLEM: auth_stat = 13;
    #[c2rust::src_loc = "69:2"]
    pub const AUTH_FAILED: auth_stat = 7;
    #[c2rust::src_loc = "68:2"]
    pub const AUTH_INVALIDRESP: auth_stat = 6;
    #[c2rust::src_loc = "64:2"]
    pub const AUTH_TOOWEAK: auth_stat = 5;
    #[c2rust::src_loc = "63:2"]
    pub const AUTH_REJECTEDVERF: auth_stat = 4;
    #[c2rust::src_loc = "62:2"]
    pub const AUTH_BADVERF: auth_stat = 3;
    #[c2rust::src_loc = "61:2"]
    pub const AUTH_REJECTEDCRED: auth_stat = 2;
    #[c2rust::src_loc = "60:2"]
    pub const AUTH_BADCRED: auth_stat = 1;
    #[c2rust::src_loc = "56:2"]
    pub const AUTH_OK: auth_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "77:7"]
    pub union des_block {
        pub c: [libc::c_char; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "86:8"]
    pub struct opaque_auth {
        pub oa_flavor: libc::c_int,
        pub oa_base: caddr_t,
        pub oa_length: u_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "98:16"]
    pub struct AUTH {
        pub ah_cred: opaque_auth,
        pub ah_verf: opaque_auth,
        pub ah_key: des_block,
        pub ah_ops: *mut auth_ops,
        pub ah_private: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "102:9"]
    pub struct auth_ops {
        pub ah_nextverf: Option<unsafe extern "C" fn(_: *mut AUTH) -> ()>,
        pub ah_marshal: Option<unsafe extern "C" fn(_: *mut AUTH, _: *mut XDR)
                                   -> libc::c_int>,
        pub ah_validate: Option<unsafe extern "C" fn(_: *mut AUTH,
                                                     _: *mut opaque_auth)
                                    -> libc::c_int>,
        pub ah_refresh: Option<unsafe extern "C" fn(_: *mut AUTH,
                                                    _: *mut rpc_msg)
                                   -> libc::c_int>,
        pub ah_destroy: Option<unsafe extern "C" fn(_: *mut AUTH) -> ()>,
        pub ah_wrap: Option<unsafe extern "C" fn(_: *mut AUTH, _: *mut XDR,
                                                 _: xdrproc_t, _: caddr_t)
                                -> libc::c_int>,
        pub ah_unwrap: Option<unsafe extern "C" fn(_: *mut AUTH, _: *mut XDR,
                                                   _: xdrproc_t, _: caddr_t)
                                  -> libc::c_int>,
    }
    use super::sys_types_h::{caddr_t, u_int};
    use super::xdr_h::{XDR, xdrproc_t};
    use super::rpc_msg_h::rpc_msg;
    /* !defined(GSSRPC_AUTH_H) */
    /* RPCSEC_GSS */
    /* GSS-API style */
    /* des style (encrypted timestamps) */
    /* short hand unix style */
    /* unix style (uid, gids) */
    /* backward compatibility */
    /* no authentication */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/rpc_msg.h:76"]
pub mod rpc_msg_h {
    /* @(#)rpc_msg.h	2.1 88/07/29 4.0 RPCSRC */
/*
 * Copyright (c) 2010, Oracle America, Inc.
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 *     * Neither the name of the "Oracle America, Inc." nor the names of
 *       its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
 * TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/*      @(#)rpc_msg.h 1.7 86/07/16 SMI      */
    /*
 * rpc_msg.h
 * rpc message definition
 */
    /*
 * Bottom up definition of an rpc message.
 * NOTE: call and reply use the same overall stuct but
 * different parts of unions within it.
 */
    /*
 * Reply part of an rpc exchange
 */
    /*
 * Reply to an rpc request that was accepted by the server.
 * Note: there could be an error even though the request was
 * accepted.
 */
    /* and many other null cases */
    /*
 * Reply to an rpc request that was rejected by the server.
 */
    /* why authentication did not work */
    /*
 * Body of a reply to an rpc request.
 */
    /*
 * Body of an rpc request call.
 */
    /* must be equal to two */
    /* protocol specific - provided by client */
    /*
 * The rpc message
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "150:8"]
    pub struct rpc_msg {
        pub rm_xid: uint32_t,
        pub rm_direction: msg_type,
        pub ru: C2RustUnnamed_13,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:2"]
    pub union C2RustUnnamed_13 {
        pub RM_cmb: call_body,
        pub RM_rmb: reply_body,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "125:8"]
    pub struct reply_body {
        pub rp_stat: reply_stat,
        pub ru: C2RustUnnamed_14,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "127:2"]
    pub union C2RustUnnamed_14 {
        pub RP_ar: accepted_reply,
        pub RP_dr: rejected_reply,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "109:8"]
    pub struct rejected_reply {
        pub rj_stat: reject_stat,
        pub ru: C2RustUnnamed_15,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "111:2"]
    pub union C2RustUnnamed_15 {
        pub RJ_versions: C2RustUnnamed_16,
        pub RJ_why: auth_stat,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:3"]
    pub struct C2RustUnnamed_16 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
    #[c2rust::src_loc = "74:1"]
    pub type reject_stat = libc::c_uint;
    #[c2rust::src_loc = "76:2"]
    pub const AUTH_ERROR: reject_stat = 1;
    #[c2rust::src_loc = "75:2"]
    pub const RPC_MISMATCH: reject_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "88:8"]
    pub struct accepted_reply {
        pub ar_verf: opaque_auth,
        pub ar_stat: accept_stat,
        pub ru: C2RustUnnamed_17,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "91:2"]
    pub union C2RustUnnamed_17 {
        pub AR_versions: C2RustUnnamed_19,
        pub AR_results: C2RustUnnamed_18,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "96:3"]
    pub struct C2RustUnnamed_18 {
        pub where_0: caddr_t,
        pub proc_0: xdrproc_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:3"]
    pub struct C2RustUnnamed_19 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
    #[c2rust::src_loc = "65:1"]
    pub type accept_stat = libc::c_uint;
    #[c2rust::src_loc = "71:2"]
    pub const SYSTEM_ERR: accept_stat = 5;
    #[c2rust::src_loc = "70:2"]
    pub const GARBAGE_ARGS: accept_stat = 4;
    #[c2rust::src_loc = "69:2"]
    pub const PROC_UNAVAIL: accept_stat = 3;
    #[c2rust::src_loc = "68:2"]
    pub const PROG_MISMATCH: accept_stat = 2;
    #[c2rust::src_loc = "67:2"]
    pub const PROG_UNAVAIL: accept_stat = 1;
    #[c2rust::src_loc = "66:2"]
    pub const SUCCESS: accept_stat = 0;
    #[c2rust::src_loc = "60:1"]
    pub type reply_stat = libc::c_uint;
    #[c2rust::src_loc = "62:2"]
    pub const MSG_DENIED: reply_stat = 1;
    #[c2rust::src_loc = "61:2"]
    pub const MSG_ACCEPTED: reply_stat = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "138:8"]
    pub struct call_body {
        pub cb_rpcvers: rpcvers_t,
        pub cb_prog: rpcprog_t,
        pub cb_vers: rpcvers_t,
        pub cb_proc: rpcproc_t,
        pub cb_cred: opaque_auth,
        pub cb_verf: opaque_auth,
    }
    #[c2rust::src_loc = "55:1"]
    pub type msg_type = libc::c_uint;
    #[c2rust::src_loc = "57:2"]
    pub const REPLY: msg_type = 1;
    #[c2rust::src_loc = "56:2"]
    pub const CALL: msg_type = 0;
    use super::stdint_uintn_h::uint32_t;
    use super::auth_h::{auth_stat, opaque_auth};
    use super::gssrpc_types_h::{rpcvers_t, rpcprog_t, rpcproc_t};
    use super::sys_types_h::caddr_t;
    use super::xdr_h::xdrproc_t;
    /* !defined(GSSRPC_RPC_MSG_H) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/clnt.h:76"]
pub mod clnt_h {
    /* @(#)clnt.h	2.1 88/07/29 4.0 RPCSRC; from 1.31 88/02/08 SMI*/
/*
 * Copyright (c) 2010, Oracle America, Inc.
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in
 *       the documentation and/or other materials provided with the
 *       distribution.
 *
 *     * Neither the name of the "Oracle America, Inc." nor the names of
 *       its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
 * IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
 * PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED
 * TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
    /*
 * clnt.h - Client side remote procedure call interface.
 */
    /*
 * Rpc calls return an enum clnt_stat.  This should be looked at more,
 * since each implementation is required to live with this (implementation
 * independent) list of errors.
 */
    #[c2rust::src_loc = "48:1"]
    pub type clnt_stat = libc::c_uint;
    /* remote program is not registered */
    /*
	 * unspecified error
	 */
    #[c2rust::src_loc = "83:2"]
    pub const RPC_FAILED: clnt_stat = 16;
    /* the pmapper failed in its call */
    #[c2rust::src_loc = "79:2"]
    pub const RPC_PROGNOTREGISTERED: clnt_stat = 15;
    /* unknown protocol */
    /*
	 * _ create errors
	 */
    #[c2rust::src_loc = "78:2"]
    pub const RPC_PMAPFAILURE: clnt_stat = 14;
    /* unknown host name */
    #[c2rust::src_loc = "73:2"]
    pub const RPC_UNKNOWNPROTO: clnt_stat = 17;
    /* generic "other problem" */
    /*
	 * callrpc & clnt_create errors
	 */
    #[c2rust::src_loc = "72:2"]
    pub const RPC_UNKNOWNHOST: clnt_stat = 13;
    /* decode arguments error */
    #[c2rust::src_loc = "67:2"]
    pub const RPC_SYSTEMERROR: clnt_stat = 12;
    /* procedure unavailable */
    #[c2rust::src_loc = "66:2"]
    pub const RPC_CANTDECODEARGS: clnt_stat = 11;
    /* program version mismatched */
    #[c2rust::src_loc = "65:2"]
    pub const RPC_PROCUNAVAIL: clnt_stat = 10;
    /* program not available */
    #[c2rust::src_loc = "64:2"]
    pub const RPC_PROGVERSMISMATCH: clnt_stat = 9;
    /* authentication error */
    #[c2rust::src_loc = "63:2"]
    pub const RPC_PROGUNAVAIL: clnt_stat = 8;
    /* rpc versions not compatible */
    #[c2rust::src_loc = "62:2"]
    pub const RPC_AUTHERROR: clnt_stat = 7;
    /* call timed out */
    /*
	 * remote errors
	 */
    #[c2rust::src_loc = "61:2"]
    pub const RPC_VERSMISMATCH: clnt_stat = 6;
    /* failure in receiving result */
    #[c2rust::src_loc = "57:2"]
    pub const RPC_TIMEDOUT: clnt_stat = 5;
    /* failure in sending call */
    #[c2rust::src_loc = "56:2"]
    pub const RPC_CANTRECV: clnt_stat = 4;
    /* can't decode results */
    #[c2rust::src_loc = "55:2"]
    pub const RPC_CANTSEND: clnt_stat = 3;
    /* can't encode arguments */
    #[c2rust::src_loc = "54:2"]
    pub const RPC_CANTDECODERES: clnt_stat = 2;
    /* call succeeded */
    /*
	 * local errors
	 */
    #[c2rust::src_loc = "53:2"]
    pub const RPC_CANTENCODEARGS: clnt_stat = 1;
    #[c2rust::src_loc = "49:2"]
    pub const RPC_SUCCESS: clnt_stat = 0;
    /*
 * Error info.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "90:8"]
    pub struct rpc_err {
        pub re_status: clnt_stat,
        pub ru: C2RustUnnamed_20,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:2"]
    pub union C2RustUnnamed_20 {
        pub RE_errno: libc::c_int,
        pub RE_why: auth_stat,
        pub RE_vers: C2RustUnnamed_22,
        pub RE_lb: C2RustUnnamed_21,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "99:3"]
    pub struct C2RustUnnamed_21 {
        pub s1: int32_t,
        pub s2: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "95:3"]
    pub struct C2RustUnnamed_22 {
        pub low: rpcvers_t,
        pub high: rpcvers_t,
    }
    /*
 * Client rpc handle.
 * Created by individual implementations, see e.g. rpc_udp.c.
 * Client is responsible for initializing auth, see e.g. auth_none.c.
 */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "116:16"]
    pub struct CLIENT {
        pub cl_auth: *mut AUTH,
        pub cl_ops: *mut clnt_ops,
        pub cl_private: *mut libc::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "118:9"]
    pub struct clnt_ops {
        pub cl_call: Option<unsafe extern "C" fn(_: *mut CLIENT, _: rpcproc_t,
                                                 _: xdrproc_t,
                                                 _: *mut libc::c_void,
                                                 _: xdrproc_t,
                                                 _: *mut libc::c_void,
                                                 _: timeval) -> clnt_stat>,
        pub cl_abort: Option<unsafe extern "C" fn(_: *mut CLIENT) -> ()>,
        pub cl_geterr: Option<unsafe extern "C" fn(_: *mut CLIENT,
                                                   _: *mut rpc_err) -> ()>,
        pub cl_freeres: Option<unsafe extern "C" fn(_: *mut CLIENT,
                                                    _: xdrproc_t,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int>,
        pub cl_destroy: Option<unsafe extern "C" fn(_: *mut CLIENT) -> ()>,
        pub cl_control: Option<unsafe extern "C" fn(_: *mut CLIENT,
                                                    _: libc::c_int,
                                                    _: *mut libc::c_void)
                                   -> libc::c_int>,
    }
    use super::auth_h::{auth_stat, AUTH};
    use super::stdint_intn_h::int32_t;
    use super::gssrpc_types_h::{rpcvers_t, rpcproc_t};
    use super::xdr_h::xdrproc_t;
    use super::struct_timeval_h::timeval;
    extern "C" {
        /* stderr */
        /*
 * Print an English error message, given the client error code
 */
        #[no_mangle]
        #[c2rust::src_loc = "321:1"]
        pub fn gssrpc_clnt_perror(_: *mut CLIENT, _: *mut libc::c_char);
    }
    /* !defined(GSSRPC_CLNT_H) */
    /* a more reasonable packet size */
    /* rpc imposed limit on udp msg size */
    /* string */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:77"]
pub mod kdb_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    use super::krb5_h::{krb5_enctype, krb5_int32, krb5_context,
                        krb5_error_code};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "363:1"]
        pub fn krb5_db_fini(kcontext: krb5_context) -> krb5_error_code;
    }
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src = "/usr/include/assert.h:54"]
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
#[c2rust::header_src = "/usr/include/string.h:54"]
pub mod string_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "42:14"]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
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
        #[no_mangle]
        #[c2rust::src_loc = "396:14"]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:54"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "550:14"]
        pub fn realloc(_: *mut libc::c_void, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "595:1"]
        pub fn atexit(__func: Option<unsafe extern "C" fn() -> ()>)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "617:13"]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:54"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    use super::internal::__va_list_tag;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "138:14"]
        pub static mut stdout: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "139:14"]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "148:1"]
        pub fn rename(__old: *const libc::c_char, __new: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "213:1"]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "218:1"]
        pub fn fflush(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "246:14"]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        #[c2rust::src_loc = "326:12"]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "332:12"]
        pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "354:12"]
        pub fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                        _: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "358:12"]
        pub fn vsnprintf(_: *mut libc::c_char, _: libc::c_ulong,
                         _: *const libc::c_char, _: ::std::ffi::VaList)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "372:1"]
        pub fn asprintf(__ptr: *mut *mut libc::c_char,
                        __fmt: *const libc::c_char, _: ...) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "564:1"]
        pub fn fgets(__s: *mut libc::c_char, __n: libc::c_int,
                     __stream: *mut FILE) -> *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "759:1"]
        pub fn feof(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "775:1"]
        pub fn perror(__s: *const libc::c_char);
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:54"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "195:1"]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/errno.h:54"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/getopt_core.h:54"]
pub mod getopt_core_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "36:14"]
        pub static mut optarg: *mut libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "50:12"]
        pub static mut optind: libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/time.h:54"]
pub mod include_time_h {
    use super::time_t_h::time_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "75:1"]
        pub fn time(__timer: *mut time_t) -> time_t;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:54"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:54"]
pub mod sys_stat_h {
    use super::stat_h::stat;
    use super::types_h::__mode_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "210:1"]
        pub fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "308:1"]
        pub fn umask(__mask: __mode_t) -> __mode_t;
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/fake-addrinfo.h:56"]
pub mod fake_addrinfo_h {
    use super::netdb_h::addrinfo;
    use super::socket_h::sockaddr;
    use super::unistd_h::socklen_t;
    use super::stddef_h::size_t;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 2001,2002,2003,2004 by the Massachusetts Institute of Technology,
 * Cambridge, MA, USA.  All Rights Reserved.
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
        /* Approach overview:

   If a system version is available but buggy, save handles to it (via
   inline functions in a support library), redefine the names to refer
   to library functions, and in those functions, call the system
   versions and fix up the returned data.  Use the native data
   structures and flag values.

   If no system version exists, use gethostby* and fake it.  Define
   the data structures and flag values locally.


   On macOS, getaddrinfo results aren't cached (though
   gethostbyname results are), so we need to build a cache here.  Now
   things are getting really messy.  Because the cache is in use, we
   use getservbyname, and throw away thread safety.  (Not that the
   cache is thread safe, but when we get locking support, that'll be
   dealt with.)  This code needs tearing down and rebuilding, soon.


   Note that recent Windows developers' code has an interesting hack:
   When you include the right header files, with the right set of
   macros indicating system versions, you'll get an inline function
   that looks for getaddrinfo (or whatever) in the system library, and
   calls it if it's there.  If it's not there, it fakes it with
   gethostby* calls.

   We're taking a simpler approach: A system provides these routines or
   it does not.

   Someday, we may want to take into account different versions (say,
   different revs of GNU libc) where some are broken in one way, and
   some work or are broken in another way.  Cross that bridge when we
   come to it.  */
        /* To do, maybe:

   + For AIX 4.3.3, using the RFC 2133 definition: Implement
   AI_NUMERICHOST.  It's not defined in the header file.

   For certain (old?) versions of GNU libc, AI_NUMERICHOST is
   defined but not implemented.

   + Use gethostbyname2, inet_aton and other IPv6 or thread-safe
   functions if available.  But, see
   https://bugs.debian.org/cgi-bin/bugreport.cgi?bug=135182 for one
   gethostbyname2 problem on Linux.  And besides, if a platform is
   supporting IPv6 at all, they really should be doing getaddrinfo
   by now.

   + inet_ntop, inet_pton

   + Conditionally export/import the function definitions, so a
   library can have a single copy instead of multiple.

   + Upgrade host requirements to include working implementations of
   these functions, and throw all this away.  Pleeease?  :-)  */
        /* ! HAVE_GETADDRINFO */
        /* Fudge things on older gai implementations.  */
/* AIX 4.3.3 is based on RFC 2133; no AI_NUMERICHOST.  */
        /* Partial RFC 2553 implementations may not have AI_ADDRCONFIG and
   friends, which RFC 3493 says are now part of the getaddrinfo
   interface, and we'll want to use.  */
        /* Call out to stuff defined in libkrb5support.  */
        #[no_mangle]
        #[c2rust::src_loc = "214:1"]
        pub fn krb5int_getaddrinfo(node: *const libc::c_char,
                                   service: *const libc::c_char,
                                   hints: *const addrinfo,
                                   aip: *mut *mut addrinfo) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "218:1"]
        pub fn krb5int_gai_strerror(err: libc::c_int) -> *const libc::c_char;
        #[no_mangle]
        #[c2rust::src_loc = "219:1"]
        pub fn krb5int_getnameinfo(sa: *const sockaddr, salen: socklen_t,
                                   hbuf: *mut libc::c_char, hbuflen: size_t,
                                   sbuf: *mut libc::c_char, sbuflen: size_t,
                                   flags: libc::c_int) -> libc::c_int;
    }
    /* FAI_DEFINED */
}
#[c2rust::header_src = "/usr/include/locale.h:58"]
pub mod locale_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "122:1"]
        pub fn setlocale(__category: libc::c_int,
                         __locale: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/sys/wait.h:67"]
pub mod wait_h {
    use super::types_h::__pid_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "88:1"]
        pub fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
        #[no_mangle]
        #[c2rust::src_loc = "111:1"]
        pub fn waitpid(__pid: __pid_t, __stat_loc: *mut libc::c_int,
                       __options: libc::c_int) -> __pid_t;
    }
}
#[c2rust::header_src = "/usr/include/sys/syslog.h:72"]
pub mod syslog_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "181:1"]
        pub fn openlog(__ident: *const libc::c_char, __option: libc::c_int,
                       __facility: libc::c_int);
        #[no_mangle]
        #[c2rust::src_loc = "190:1"]
        pub fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
    }
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/kprop/kprop.h:74"]
pub mod kprop_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_address, krb5_principal,
                        krb5_error_code};
    use super::socket_h::sockaddr;
    extern "C" {
        /* pathnames are in osconf.h, included via k5-int.h */
        #[no_mangle]
        #[c2rust::src_loc = "38:1"]
        pub fn sockaddr2krbaddr(context: krb5_context, family: libc::c_int,
                                sa: *mut sockaddr,
                                dest: *mut *mut krb5_address) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "41:1"]
        pub fn sn2princ_realm(context: krb5_context,
                              hostname: *const libc::c_char,
                              sname: *const libc::c_char,
                              realm_0: *const libc::c_char,
                              princ_out: *mut krb5_principal)
         -> krb5_error_code;
    }
}
pub use self::internal::__va_list_tag;
pub use self::types_h::{__u_int, __uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __dev_t, __uid_t, __gid_t, __ino_t,
                        __mode_t, __nlink_t, __off_t, __off64_t, __pid_t,
                        __clock_t, __time_t, __suseconds_t, __blksize_t,
                        __blkcnt_t, __ssize_t, __syscall_slong_t, __caddr_t,
                        __socklen_t};
pub use self::sys_types_h::{u_int, mode_t, pid_t, ssize_t, caddr_t};
pub use self::time_t_h::time_t;
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::__sigset_t_h::__sigset_t;
pub use self::sigset_t_h::sigset_t;
pub use self::struct_timeval_h::timeval;
pub use self::struct_timespec_h::timespec;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data,
                              _IO_codecvt, _IO_marker};
pub use self::FILE_h::FILE;
pub use self::stat_h::stat;
pub use self::unistd_h::{socklen_t, close, write, alarm, sleep, dup, dup2,
                         execv, _exit, getpid, getppid, fork, daemon};
pub use self::__sigval_t_h::{__sigval_t, sigval};
pub use self::getopt_ext_h::{option, getopt_long};
pub use self::krb5_h::{krb5_octet, krb5_int32, krb5_ui_4, krb5_boolean,
                       krb5_kvno, krb5_addrtype, krb5_enctype,
                       krb5_authdatatype, krb5_flags, krb5_timestamp,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_pointer, krb5_principal_data,
                       krb5_principal, krb5_const_principal, _krb5_address,
                       krb5_address, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, krb5_auth_context,
                       _krb5_keyblock, krb5_keyblock, _krb5_enc_data,
                       krb5_enc_data, _krb5_ticket_times, krb5_ticket_times,
                       _krb5_authdata, krb5_authdata, _krb5_transited,
                       krb5_transited, _krb5_enc_tkt_part, krb5_enc_tkt_part,
                       _krb5_ticket, krb5_ticket, _krb5_error, krb5_error,
                       krb5_replay_data, krb5_kt_cursor, krb5_keytab_entry_st,
                       krb5_keytab_entry, krb5_keytab, _profile_t,
                       _krb5_auth_context, krb5_free_context, krb5_mk_error,
                       krb5_rd_error, krb5_rd_safe, krb5_rd_priv,
                       krb5_unparse_name, krb5_copy_principal,
                       krb5_kt_resolve, krb5_free_principal, krb5_free_ticket,
                       krb5_free_error, krb5_free_data_contents,
                       krb5_us_timeofday, krb5_get_default_realm,
                       krb5_set_default_realm, krb5_free_default_realm,
                       krb5_mk_safe, krb5_recvauth, krb5_auth_con_init,
                       krb5_auth_con_setflags, krb5_auth_con_setaddrs,
                       krb5_auth_con_initivector, krb5_string_to_enctype,
                       krb5_enctype_to_name, krb5_get_error_message,
                       krb5_free_error_message};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, _krb5_kt, _krb5_kt_ops,
                         plugin_mapping, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         _kdb5_dal_handle, krb5_lock_file,
                         krb5int_init_context_kdc, krb5_read_message,
                         krb5_write_message};
pub use self::kdb_log_h::{_kdb_log_context, kdb_hlog_t, kdb_hlog,
                          kdb_log_context, ulog_map, ulog_replay,
                          ulog_set_role, ulog_get_last, ulog_fini};
pub use self::iprop_h::{kdb_sno_t, kdbe_time_t, utf8str_t, kdbe_key_t,
                        C2RustUnnamed_23, C2RustUnnamed_24, kdbe_data_t,
                        kdbe_princ_t, C2RustUnnamed_25, kdbe_tl_t,
                        C2RustUnnamed_26, kdbe_pw_hist_t, kdbe_attr_type_t,
                        AT_PW_HIST, AT_PW_HIST_KVNO, AT_PW_POLICY_SWITCH,
                        AT_PW_POLICY, AT_PW_LAST_CHANGE, AT_MOD_WHERE,
                        AT_MOD_TIME, AT_MOD_PRINC, AT_LEN, AT_TL_DATA,
                        AT_KEYDATA, AT_PRINC, AT_FAIL_AUTH_COUNT,
                        AT_LAST_FAILED, AT_LAST_SUCCESS, AT_PW_EXP, AT_EXP,
                        AT_MAX_RENEW_LIFE, AT_MAX_LIFE, AT_ATTRFLAGS,
                        kdbe_val_t, C2RustUnnamed_27, C2RustUnnamed_28,
                        C2RustUnnamed_29, C2RustUnnamed_30, C2RustUnnamed_31,
                        kdbe_t, kdb_incr_update_t, C2RustUnnamed_32,
                        C2RustUnnamed_33, kdb_ulog_t, update_status_t,
                        UPDATE_PERM_DENIED, UPDATE_NIL, UPDATE_BUSY,
                        UPDATE_FULL_RESYNC_NEEDED, UPDATE_ERROR, UPDATE_OK,
                        kdb_last_t, kdb_incr_result_t,
                        kdb_fullresync_result_t, iprop_get_updates_1,
                        xdr_kdb_fullresync_result_t};
pub use self::iprop_hdr_h::{iprop_role, IPROP_REPLICA, IPROP_MASTER,
                            IPROP_NULL};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::profile_h::profile_t;
pub use self::com_err_h::{errcode_t, et_old_error_hook_func, com_err,
                          error_message, set_com_err_hook};
pub use self::socket_type_h::{__socket_type, SOCK_NONBLOCK, SOCK_CLOEXEC,
                              SOCK_PACKET, SOCK_DCCP, SOCK_SEQPACKET,
                              SOCK_RDM, SOCK_RAW, SOCK_DGRAM, SOCK_STREAM};
pub use self::sockaddr_h::sa_family_t;
pub use self::socket_h::{sockaddr, sockaddr_storage};
pub use self::sys_socket_h::{__SOCKADDR_ARG, __CONST_SOCKADDR_ARG,
                             sockaddr_x25, sockaddr_un, sockaddr_ns,
                             sockaddr_iso, sockaddr_ipx, sockaddr_inarp,
                             sockaddr_eon, sockaddr_dl, sockaddr_ax25,
                             sockaddr_at, socket, bind, getsockname,
                             getpeername, setsockopt, listen, accept};
pub use self::in_h::{sockaddr_in6, in6_addr, C2RustUnnamed, in_port_t,
                     sockaddr_in, in_addr, in_addr_t, C2RustUnnamed_0,
                     IPPROTO_MAX, IPPROTO_RAW, IPPROTO_MPLS, IPPROTO_UDPLITE,
                     IPPROTO_SCTP, IPPROTO_COMP, IPPROTO_PIM, IPPROTO_ENCAP,
                     IPPROTO_BEETPH, IPPROTO_MTP, IPPROTO_AH, IPPROTO_ESP,
                     IPPROTO_GRE, IPPROTO_RSVP, IPPROTO_IPV6, IPPROTO_DCCP,
                     IPPROTO_TP, IPPROTO_IDP, IPPROTO_UDP, IPPROTO_PUP,
                     IPPROTO_EGP, IPPROTO_TCP, IPPROTO_IPIP, IPPROTO_IGMP,
                     IPPROTO_ICMP, IPPROTO_IP, ntohl, htonl};
pub use self::netdb_h::addrinfo;
pub use self::siginfo_t_h::{siginfo_t, C2RustUnnamed_1, C2RustUnnamed_2,
                            C2RustUnnamed_3, C2RustUnnamed_4, C2RustUnnamed_5,
                            C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
                            C2RustUnnamed_9, C2RustUnnamed_10};
pub use self::signal_h::{__sighandler_t, kill, sigemptyset, sigaction};
pub use self::sigaction_h::{sigaction, C2RustUnnamed_11};
pub use self::time_h::{timezone, __timezone_ptr_t, gettimeofday};
pub use self::admin_h::{kadm5_ret_t, _kadm5_config_params,
                        kadm5_config_params, kadm5_get_config_params,
                        kadm5_init_with_skey, kadm5_destroy};
pub use self::ctype_h::{C2RustUnnamed_12, _ISalnum, _ISpunct, _IScntrl,
                        _ISblank, _ISgraph, _ISprint, _ISspace, _ISxdigit,
                        _ISdigit, _ISalpha, _ISlower, _ISupper,
                        __ctype_b_loc};
pub use self::gssrpc_types_h::{rpcprog_t, rpcvers_t, rpcproc_t, rpc_inline_t};
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdr_void, gssrpc_xdr_u_int32};
pub use self::auth_h::{auth_stat, RPCSEC_GSS_CTXPROBLEM,
                       RPCSEC_GSS_CREDPROBLEM, AUTH_FAILED, AUTH_INVALIDRESP,
                       AUTH_TOOWEAK, AUTH_REJECTEDVERF, AUTH_BADVERF,
                       AUTH_REJECTEDCRED, AUTH_BADCRED, AUTH_OK, des_block,
                       opaque_auth, AUTH, auth_ops};
pub use self::rpc_msg_h::{rpc_msg, C2RustUnnamed_13, reply_body,
                          C2RustUnnamed_14, rejected_reply, C2RustUnnamed_15,
                          C2RustUnnamed_16, reject_stat, AUTH_ERROR,
                          RPC_MISMATCH, accepted_reply, C2RustUnnamed_17,
                          C2RustUnnamed_18, C2RustUnnamed_19, accept_stat,
                          SYSTEM_ERR, GARBAGE_ARGS, PROC_UNAVAIL,
                          PROG_MISMATCH, PROG_UNAVAIL, SUCCESS, reply_stat,
                          MSG_DENIED, MSG_ACCEPTED, call_body, msg_type,
                          REPLY, CALL};
pub use self::clnt_h::{clnt_stat, RPC_FAILED, RPC_PROGNOTREGISTERED,
                       RPC_PMAPFAILURE, RPC_UNKNOWNPROTO, RPC_UNKNOWNHOST,
                       RPC_SYSTEMERROR, RPC_CANTDECODEARGS, RPC_PROCUNAVAIL,
                       RPC_PROGVERSMISMATCH, RPC_PROGUNAVAIL, RPC_AUTHERROR,
                       RPC_VERSMISMATCH, RPC_TIMEDOUT, RPC_CANTRECV,
                       RPC_CANTSEND, RPC_CANTDECODERES, RPC_CANTENCODEARGS,
                       RPC_SUCCESS, rpc_err, C2RustUnnamed_20,
                       C2RustUnnamed_21, C2RustUnnamed_22, CLIENT, clnt_ops,
                       gssrpc_clnt_perror};
pub use self::kdb_h::{__krb5_key_salt_tuple, krb5_key_salt_tuple,
                      krb5_db_fini};
use self::assert_h::__assert_fail;
use self::string_h::{memcpy, memset, strncmp, strdup, strlen, strerror};
use self::stdlib_h::{realloc, free, atexit, exit};
use self::stdio_h::{stdout, stderr, rename, fclose, fflush, fopen, fprintf,
                    printf, snprintf, vsnprintf, asprintf, fgets, feof,
                    perror};
use self::fcntl_h::open;
use self::errno_h::__errno_location;
use self::getopt_core_h::{optarg, optind};
use self::include_time_h::time;
use self::libintl_h::dgettext;
use self::sys_stat_h::{fstat, umask};
use self::fake_addrinfo_h::{krb5int_getaddrinfo, krb5int_gai_strerror,
                            krb5int_getnameinfo};
use self::locale_h::setlocale;
use self::wait_h::{wait, waitpid};
use self::syslog_h::{openlog, syslog};
use self::kprop_h::{sockaddr2krbaddr, sn2princ_realm};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "101:16"]
pub struct _kadm5_iprop_handle_t {
    pub magic_number: krb5_ui_4,
    pub struct_version: krb5_ui_4,
    pub api_version: krb5_ui_4,
    pub cache_name: *mut libc::c_char,
    pub destroy_cache: libc::c_int,
    pub clnt: *mut CLIENT,
    pub context: krb5_context,
    pub params: kadm5_config_params,
    pub lhandle: *mut _kadm5_iprop_handle_t,
}
/*
 * This struct simulates the use of _kadm5_server_handle_t
 *
 * This is a COPY of kadm5_server_handle_t from
 * lib/kadm5/clnt/client_internal.h!
 */
#[c2rust::src_loc = "101:1"]
pub type kadm5_iprop_handle_t = *mut _kadm5_iprop_handle_t;
#[c2rust::src_loc = "1047:12"]
pub const PID_FILE: C2RustUnnamed_34 = 256;
#[c2rust::src_loc = "1047:5"]
pub type C2RustUnnamed_34 = libc::c_uint;
#[c2rust::src_loc = "194:1"]
pub type sig_handler_fn = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
#[no_mangle]
#[c2rust::src_loc = "93:5"]
pub static mut runonce: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "113:14"]
static mut kprop_version: *mut libc::c_char =
    b"kprop5_01\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "115:28"]
static mut params: kadm5_config_params =
    kadm5_config_params{mask: 0,
                        realm: 0 as *const libc::c_char as *mut libc::c_char,
                        kadmind_port: 0,
                        kpasswd_port: 0,
                        admin_server:
                            0 as *const libc::c_char as *mut libc::c_char,
                        dbname: 0 as *const libc::c_char as *mut libc::c_char,
                        acl_file:
                            0 as *const libc::c_char as *mut libc::c_char,
                        dict_file:
                            0 as *const libc::c_char as *mut libc::c_char,
                        mkey_from_kbd: 0,
                        stash_file:
                            0 as *const libc::c_char as *mut libc::c_char,
                        mkey_name:
                            0 as *const libc::c_char as *mut libc::c_char,
                        enctype: 0,
                        max_life: 0,
                        max_rlife: 0,
                        expiration: 0,
                        flags: 0,
                        keysalts:
                            0 as *const krb5_key_salt_tuple as
                                *mut krb5_key_salt_tuple,
                        num_keysalts: 0,
                        kvno: 0,
                        iprop_enabled: 0,
                        iprop_ulogsize: 0,
                        iprop_poll_time: 0,
                        iprop_logfile:
                            0 as *const libc::c_char as *mut libc::c_char,
                        iprop_port: 0,
                        iprop_resync_timeout: 0,
                        kadmind_listen:
                            0 as *const libc::c_char as *mut libc::c_char,
                        kpasswd_listen:
                            0 as *const libc::c_char as *mut libc::c_char,
                        iprop_listen:
                            0 as *const libc::c_char as *mut libc::c_char,};
#[c2rust::src_loc = "117:14"]
static mut progname: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "118:12"]
static mut debug: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "119:12"]
static mut nodaemon: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "120:14"]
static mut keytab_path: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "121:12"]
static mut standalone: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "122:20"]
static mut pid_file: *const libc::c_char = 0 as *const libc::c_char;
#[c2rust::src_loc = "124:14"]
static mut fullprop_child: pid_t = -(1 as libc::c_int);
#[c2rust::src_loc = "126:23"]
static mut server: krb5_principal =
    0 as *const krb5_principal_data as *mut krb5_principal_data;
/* This is our server principal name */
#[c2rust::src_loc = "127:23"]
static mut client: krb5_principal =
    0 as *const krb5_principal_data as *mut krb5_principal_data;
/* This is who we're talking to */
#[c2rust::src_loc = "128:21"]
static mut kpropd_context: krb5_context =
    0 as *const _krb5_context as *mut _krb5_context;
#[c2rust::src_loc = "129:26"]
static mut auth_context: krb5_auth_context =
    0 as *const _krb5_auth_context as *mut _krb5_auth_context;
#[c2rust::src_loc = "130:14"]
static mut realm: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/* Our realm */
#[c2rust::src_loc = "131:14"]
static mut def_realm: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
/* Ref pointer for default realm */
#[c2rust::src_loc = "132:14"]
static mut file: *mut libc::c_char =
    b"/usr/local/var/krb5kdc/from_master\x00" as *const u8 as
        *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "133:14"]
static mut temp_file_name: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "134:14"]
static mut kdb5_util: *mut libc::c_char =
    b"/usr/local/sbin/kdb5_util\x00" as *const u8 as *const libc::c_char as
        *mut libc::c_char;
#[c2rust::src_loc = "135:14"]
static mut kerb_database: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "136:14"]
static mut acl_file_name: *mut libc::c_char =
    b"/usr/local/var/krb5kdc/kpropd.acl\x00" as *const u8 as
        *const libc::c_char as *mut libc::c_char;
#[c2rust::src_loc = "138:22"]
static mut sender_addr: *mut krb5_address =
    0 as *const krb5_address as *mut krb5_address;
#[c2rust::src_loc = "139:22"]
static mut receiver_addr: *mut krb5_address =
    0 as *const krb5_address as *mut krb5_address;
#[c2rust::src_loc = "140:20"]
static mut port: *const libc::c_char =
    b"krb5_prop\x00" as *const u8 as *const libc::c_char;
#[c2rust::src_loc = "142:15"]
static mut db_args: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
#[c2rust::src_loc = "143:12"]
static mut db_args_size: libc::c_int = 0 as libc::c_int;
#[c2rust::src_loc = "167:1"]
unsafe extern "C" fn usage() {
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\nUsage: %s [-r realm] [-s keytab] [-dS] [-f replica_file]\n\x00"
                         as *const u8 as *const libc::c_char), progname);
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t[-F kerberos_db_file ] [-p kdb5_util_pathname]\n\x00"
                         as *const u8 as *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t[-x db_args]* [-P port] [-a acl_file]\n\x00" as
                         *const u8 as *const libc::c_char));
    fprintf(stderr,
            dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                     b"\t[-A admin_server] [--pid-file=pid_file]\n\x00" as
                         *const u8 as *const libc::c_char));
    exit(1 as libc::c_int);
}
#[c2rust::src_loc = "179:1"]
unsafe extern "C" fn write_pid_file(mut path: *const libc::c_char)
 -> krb5_error_code {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut pid: libc::c_ulong = 0;
    fp = fopen(path, b"w\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() { return *__errno_location() }
    pid = getpid() as libc::c_ulong;
    if fprintf(fp, b"%ld\n\x00" as *const u8 as *const libc::c_char, pid) <
           0 as libc::c_int || fclose(fp) == -(1 as libc::c_int) {
        return *__errno_location()
    }
    return 0 as libc::c_int;
}
#[c2rust::src_loc = "196:1"]
unsafe extern "C" fn signal_wrapper(mut sig: libc::c_int,
                                    mut handler: sig_handler_fn) {
    let mut s_action: sigaction =
        sigaction{__sigaction_handler: C2RustUnnamed_11{sa_handler: None,},
                  sa_mask: __sigset_t{__val: [0; 16],},
                  sa_flags: 0,
                  sa_restorer: None,};
    memset(&mut s_action as *mut sigaction as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<sigaction>() as libc::c_ulong);
    sigemptyset(&mut s_action.sa_mask);
    s_action.__sigaction_handler.sa_handler = handler;
    sigaction(sig, &mut s_action, 0 as *mut sigaction);
}
#[c2rust::src_loc = "211:1"]
unsafe extern "C" fn alarm_handler(mut sig: libc::c_int) {
    static mut timeout_msg: *mut libc::c_char =
        b"Full propagation timed out\n\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char;
    write(2 as libc::c_int, timeout_msg as *const libc::c_void,
          strlen(timeout_msg));
    exit(1 as libc::c_int);
}
#[c2rust::src_loc = "220:1"]
unsafe extern "C" fn usr1_handler(mut sig: libc::c_int) {
    /* Nothing to do, just let the signal interrupt sleep(). */
}
#[c2rust::src_loc = "226:1"]
unsafe extern "C" fn kill_do_standalone(mut sig: libc::c_int) {
    if fullprop_child > 0 as libc::c_int {
        if debug != 0 {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Killing fullprop child (%d)\n\x00" as *const u8
                                 as *const libc::c_char), fullprop_child);
        }
        kill(fullprop_child, sig);
    }
    /* Make sure our exit status code reflects our having been signaled */
    signal_wrapper(sig, None);
    kill(getpid(), sig);
}
#[c2rust::src_loc = "241:1"]
unsafe extern "C" fn atexit_kill_do_standalone() {
    if fullprop_child > 0 as libc::c_int {
        kill(fullprop_child, 1 as libc::c_int);
    };
}
#[c2rust::src_loc = "248:1"]
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut retval: krb5_error_code = 0;
    let mut log_ctx: *mut kdb_log_context = 0 as *mut kdb_log_context;
    let mut devnull: libc::c_int = 0;
    let mut sock: libc::c_int = 0;
    let mut st: stat =
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
    setlocale(6 as libc::c_int, b"\x00" as *const u8 as *const libc::c_char);
    parse_args(argc, argv);
    if fstat(0 as libc::c_int, &mut st) == -(1 as libc::c_int) {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while checking if stdin is a socket\x00" as
                             *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    /*
     * Detect whether we're running from inetd; if not then we're in
     * standalone mode.
     */
    standalone =
        !(st.st_mode & 0o170000 as libc::c_int as libc::c_uint ==
              0o140000 as libc::c_int as libc::c_uint) as libc::c_int;
    log_ctx = (*kpropd_context).kdblog_context;
    signal_wrapper(13 as libc::c_int,
                   ::std::mem::transmute::<libc::intptr_t,
                                           __sighandler_t>(1 as libc::c_int as
                                                               libc::intptr_t));
    if standalone != 0 {
        /* "ready" is a sentinel for the test framework. */
        if debug == 0 && nodaemon == 0 {
            daemon(0 as libc::c_int, 0 as libc::c_int);
        } else {
            printf(dgettext(b"mit-krb5\x00" as *const u8 as
                                *const libc::c_char,
                            b"ready\n\x00" as *const u8 as
                                *const libc::c_char));
            fflush(stdout);
        }
        if !pid_file.is_null() {
            retval = write_pid_file(pid_file);
            if retval != 0 {
                syslog(3 as libc::c_int,
                       dgettext(b"mit-krb5\x00" as *const u8 as
                                    *const libc::c_char,
                                b"Could not write pid file %s: %s\x00" as
                                    *const u8 as *const libc::c_char),
                       pid_file, strerror(*__errno_location()));
                exit(1 as libc::c_int);
            }
        }
    } else {
        /*
         * We're an inetd nowait service.  Let's not risk anything
         * read/write from/to the inetd socket unintentionally.
         */
        devnull =
            open(b"/dev/null\x00" as *const u8 as *const libc::c_char,
                 0o2 as libc::c_int);
        if devnull == -(1 as libc::c_int) {
            syslog(3 as libc::c_int,
                   dgettext(b"mit-krb5\x00" as *const u8 as
                                *const libc::c_char,
                            b"Could not open /dev/null: %s\x00" as *const u8
                                as *const libc::c_char),
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        sock = dup(0 as libc::c_int);
        if sock == -(1 as libc::c_int) {
            syslog(3 as libc::c_int,
                   dgettext(b"mit-krb5\x00" as *const u8 as
                                *const libc::c_char,
                            b"Could not dup the inetd socket: %s\x00" as
                                *const u8 as *const libc::c_char),
                   strerror(*__errno_location()));
            exit(1 as libc::c_int);
        }
        dup2(devnull, 0 as libc::c_int);
        dup2(devnull, 1 as libc::c_int);
        dup2(devnull, 2 as libc::c_int);
        close(devnull);
        doit(sock);
        exit(0 as libc::c_int);
    }
    if log_ctx.is_null() ||
           (*log_ctx).iproprole as libc::c_uint !=
               IPROP_REPLICA as libc::c_int as libc::c_uint {
        do_standalone();
        /* do_standalone() should never return */
        __assert_fail(b"0\x00" as *const u8 as *const libc::c_char,
                      b"kpropd.c\x00" as *const u8 as *const libc::c_char,
                      319 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 23],
                                                &[libc::c_char; 23]>(b"int main(int, char **)\x00")).as_ptr());
    }
    /*
     * This is the iprop case.  We'll fork a child to run do_standalone().  The
     * parent will run do_iprop().  We try to kill the child if we get killed.
     * Catch SIGUSR1, which can be used to interrupt the sleep timer and force
     * an iprop request.
     */
    signal_wrapper(1 as libc::c_int,
                   Some(kill_do_standalone as
                            unsafe extern "C" fn(_: libc::c_int) -> ()));
    signal_wrapper(2 as libc::c_int,
                   Some(kill_do_standalone as
                            unsafe extern "C" fn(_: libc::c_int) -> ()));
    signal_wrapper(3 as libc::c_int,
                   Some(kill_do_standalone as
                            unsafe extern "C" fn(_: libc::c_int) -> ()));
    signal_wrapper(15 as libc::c_int,
                   Some(kill_do_standalone as
                            unsafe extern "C" fn(_: libc::c_int) -> ()));
    signal_wrapper(11 as libc::c_int,
                   Some(kill_do_standalone as
                            unsafe extern "C" fn(_: libc::c_int) -> ()));
    signal_wrapper(10 as libc::c_int,
                   Some(usr1_handler as
                            unsafe extern "C" fn(_: libc::c_int) -> ()));
    atexit(Some(atexit_kill_do_standalone as unsafe extern "C" fn() -> ()));
    fullprop_child = fork();
    match fullprop_child {
        -1 => {
            com_err(progname, *__errno_location() as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"do_iprop failed.\n\x00" as *const u8 as
                                 *const libc::c_char));
        }
        0 => { do_standalone(); }
        _ => {
            retval = do_iprop();
            /* do_iprop() can return due to failures and runonce. */
            kill(fullprop_child, 1 as libc::c_int);
            wait(0 as *mut libc::c_int);
            if retval != 0 {
                com_err(progname, retval as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"do_iprop failed.\n\x00" as *const u8 as
                                     *const libc::c_char));
            } else { exit(0 as libc::c_int); }
        }
    }
    exit(1 as libc::c_int);
}
/* Use getaddrinfo to determine a wildcard listener address, preferring
 * IPv6 if available. */
#[c2rust::src_loc = "361:1"]
unsafe extern "C" fn get_wildcard_addr(mut res: *mut *mut addrinfo)
 -> libc::c_int {
    let mut hints: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    let mut error: libc::c_int = 0;
    memset(&mut hints as *mut addrinfo as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    hints.ai_flags = 0x1 as libc::c_int | 0x20 as libc::c_int;
    hints.ai_family = 10 as libc::c_int;
    error =
        krb5int_getaddrinfo(0 as *const libc::c_char, port, &mut hints, res);
    if error == 0 as libc::c_int { return 0 as libc::c_int }
    hints.ai_family = 2 as libc::c_int;
    return krb5int_getaddrinfo(0 as *const libc::c_char, port, &mut hints,
                               res);
}
#[c2rust::src_loc = "378:1"]
unsafe extern "C" fn do_standalone() {
    let mut frominet: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut res: *mut addrinfo = 0 as *mut addrinfo;
    let mut fromlen: socklen_t = 0;
    let mut finet: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut error: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut child_pid: pid_t = 0;
    let mut wait_pid: pid_t = 0;
    error = get_wildcard_addr(&mut res);
    if error != 0 as libc::c_int {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"getaddrinfo: %s\n\x00" as *const u8 as
                             *const libc::c_char),
                krb5int_gai_strerror(error));
        exit(1 as libc::c_int);
    }
    finet = socket((*res).ai_family, (*res).ai_socktype, (*res).ai_protocol);
    if finet < 0 as libc::c_int {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while obtaining socket\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    val = 1 as libc::c_int;
    if setsockopt(finet, 1 as libc::c_int, 2 as libc::c_int,
                  &mut val as *mut libc::c_int as *const libc::c_void,
                  ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                      socklen_t) < 0 as libc::c_int {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while setting SO_REUSEADDR option\x00" as *const u8
                             as *const libc::c_char));
    }
    /* Make sure dual-stack support is enabled on IPv6 listener sockets if
     * possible. */
    val = 0 as libc::c_int;
    if (*res).ai_family == 10 as libc::c_int &&
           setsockopt(finet, IPPROTO_IPV6 as libc::c_int, 26 as libc::c_int,
                      &mut val as *mut libc::c_int as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                          socklen_t) < 0 as libc::c_int {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while unsetting IPV6_V6ONLY option\x00" as
                             *const u8 as *const libc::c_char));
    }
    ret =
        bind(finet, __CONST_SOCKADDR_ARG{__sockaddr__: (*res).ai_addr,},
             (*res).ai_addrlen);
    if ret < 0 as libc::c_int {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while binding listener socket\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    if listen(finet, 5 as libc::c_int) < 0 as libc::c_int {
        com_err(progname, *__errno_location() as errcode_t,
                b"in listen call\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    loop  {
        memset(&mut frominet as *mut sockaddr_in as *mut libc::c_void,
               0 as libc::c_int,
               ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong);
        fromlen =
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                socklen_t;
        if debug != 0 {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"waiting for a kprop connection\n\x00" as
                                 *const u8 as *const libc::c_char));
        }
        s =
            accept(finet,
                   __SOCKADDR_ARG{__sockaddr__:
                                      &mut frominet as *mut sockaddr_in as
                                          *mut sockaddr,}, &mut fromlen);
        if s < 0 as libc::c_int {
            let mut e: libc::c_int = *__errno_location();
            if e != 4 as libc::c_int {
                com_err(progname, e as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while accepting connection\x00" as
                                     *const u8 as *const libc::c_char));
            }
        }
        child_pid = fork();
        match child_pid {
            -1 => {
                com_err(progname, *__errno_location() as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while forking\x00" as *const u8 as
                                     *const libc::c_char));
                exit(1 as libc::c_int);
            }
            0 => { close(finet); doit(s); close(s); _exit(0 as libc::c_int); }
            _ => {
                loop  {
                    wait_pid =
                        waitpid(child_pid, &mut status, 0 as libc::c_int);
                    if !(wait_pid == -(1 as libc::c_int) &&
                             *__errno_location() == 4 as libc::c_int) {
                        break ;
                    }
                }
                if wait_pid == -(1 as libc::c_int) {
                    /* Something bad happened; panic. */
                    if debug != 0 {
                        fprintf(stderr,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"waitpid() failed to wait for doit() (%d %s)\n\x00"
                                             as *const u8 as
                                             *const libc::c_char),
                                *__errno_location(),
                                strerror(*__errno_location()));
                    }
                    com_err(progname, *__errno_location() as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while waiting to receive database\x00"
                                         as *const u8 as
                                         *const libc::c_char));
                    exit(1 as libc::c_int);
                }
                if debug != 0 {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"Database load process for full propagation completed.\n\x00"
                                         as *const u8 as
                                         *const libc::c_char));
                }
                close(s);
                /* If we are the fullprop child in iprop mode, notify the parent
             * process that it should poll for incremental updates. */
                if fullprop_child == 0 as libc::c_int {
                    kill(getppid(), 10 as libc::c_int);
                } else if runonce != 0 { exit(0 as libc::c_int); }
            }
        }
    };
}
#[c2rust::src_loc = "478:1"]
unsafe extern "C" fn doit(mut fd: libc::c_int) {
    let mut from: sockaddr_storage =
        sockaddr_storage{ss_family: 0,
                         __ss_padding: [0; 118],
                         __ss_align: 0,};
    let mut on: libc::c_int = 1 as libc::c_int;
    let mut fromlen: socklen_t = 0;
    let mut retval: krb5_error_code = 0;
    let mut confmsg: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut lock_fd: libc::c_int = 0;
    let mut omask: mode_t = 0;
    let mut etype: krb5_enctype = 0;
    let mut database_fd: libc::c_int = 0;
    let mut host: [libc::c_char; 47] = [0; 47];
    signal_wrapper(14 as libc::c_int,
                   Some(alarm_handler as
                            unsafe extern "C" fn(_: libc::c_int) -> ()));
    alarm(params.iprop_resync_timeout as libc::c_uint);
    fromlen =
        ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
            socklen_t;
    if getpeername(fd,
                   __SOCKADDR_ARG{__sockaddr__:
                                      &mut from as *mut sockaddr_storage as
                                          *mut sockaddr,}, &mut fromlen) <
           0 as libc::c_int {
        if *__errno_location() == 88 as libc::c_int && fd == 0 as libc::c_int
               && standalone == 0 {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"%s: Standard input does not appear to be a network socket.\n\t(Not run from inetd, and missing the -S option?)\n\x00"
                                 as *const u8 as *const libc::c_char),
                    progname);
            exit(1 as libc::c_int);
        }
        fprintf(stderr, b"%s: \x00" as *const u8 as *const libc::c_char,
                progname);
        perror(b"getpeername\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    if setsockopt(fd, 1 as libc::c_int, 9 as libc::c_int,
                  &mut on as *mut libc::c_int as *const libc::c_void,
                  ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                      socklen_t) < 0 as libc::c_int {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while attempting setsockopt (SO_KEEPALIVE)\x00" as
                             *const u8 as *const libc::c_char));
    }
    if krb5int_getnameinfo(&mut from as *mut sockaddr_storage as
                               *const sockaddr, fromlen, host.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 47]>() as
                               libc::c_ulong, 0 as *mut libc::c_char,
                           0 as libc::c_int as size_t, 0 as libc::c_int) ==
           0 as libc::c_int {
        syslog(6 as libc::c_int,
               dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Connection from %s\x00" as *const u8 as
                            *const libc::c_char), host.as_mut_ptr());
        if debug != 0 {
            fprintf(stderr,
                    b"Connection from %s\n\x00" as *const u8 as
                        *const libc::c_char, host.as_mut_ptr());
        }
    }
    /*
     * Now do the authentication
     */
    kerberos_authenticate(kpropd_context, fd, &mut client, &mut etype,
                          &mut from);
    if authorized_principal(kpropd_context, client, etype) == 0 {
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        retval =
            krb5_unparse_name(kpropd_context, client as krb5_const_principal,
                              &mut name);
        if retval != 0 {
            com_err(progname, retval as errcode_t,
                    b"While unparsing client name\x00" as *const u8 as
                        *const libc::c_char);
            exit(1 as libc::c_int);
        }
        if debug != 0 {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Rejected connection from unauthorized principal %s\n\x00"
                                 as *const u8 as *const libc::c_char), name);
        }
        syslog(4 as libc::c_int,
               dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Rejected connection from unauthorized principal %s\x00"
                            as *const u8 as *const libc::c_char), name);
        free(name as *mut libc::c_void);
        exit(1 as libc::c_int);
    }
    omask = umask(0o77 as libc::c_int as __mode_t);
    lock_fd =
        open(temp_file_name, 0o2 as libc::c_int | 0o100 as libc::c_int,
             0o600 as libc::c_int);
    umask(omask);
    retval =
        krb5_lock_file(kpropd_context, lock_fd,
                       0x2 as libc::c_int | 0x4 as libc::c_int);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while trying to lock \'%s\'\x00" as *const u8 as
                             *const libc::c_char), temp_file_name);
        exit(1 as libc::c_int);
    }
    database_fd =
        open(temp_file_name,
             0o1 as libc::c_int | 0o100 as libc::c_int |
                 0o1000 as libc::c_int, 0o600 as libc::c_int);
    if database_fd < 0 as libc::c_int {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while opening database file, \'%s\'\x00" as
                             *const u8 as *const libc::c_char),
                temp_file_name);
        exit(1 as libc::c_int);
    }
    recv_database(kpropd_context, fd, database_fd, &mut confmsg);
    if rename(temp_file_name, file) != 0 {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while renaming %s to %s\x00" as *const u8 as
                             *const libc::c_char), temp_file_name, file);
        exit(1 as libc::c_int);
    }
    retval = krb5_lock_file(kpropd_context, lock_fd, 0x1 as libc::c_int);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while downgrading lock on \'%s\'\x00" as *const u8
                             as *const libc::c_char), temp_file_name);
        exit(1 as libc::c_int);
    }
    load_database(kpropd_context, kdb5_util, file);
    retval = krb5_lock_file(kpropd_context, lock_fd, 0x8 as libc::c_int);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while unlocking \'%s\'\x00" as *const u8 as
                             *const libc::c_char), temp_file_name);
        exit(1 as libc::c_int);
    }
    close(lock_fd);
    /*
     * Send the acknowledgement message generated in
     * recv_database, then close the socket.
     */
    retval =
        krb5_write_message(kpropd_context,
                           &mut fd as *mut libc::c_int as krb5_pointer,
                           &mut confmsg);
    if retval != 0 {
        krb5_free_data_contents(kpropd_context, &mut confmsg);
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while sending # of received bytes\x00" as *const u8
                             as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    krb5_free_data_contents(kpropd_context, &mut confmsg);
    if close(fd) < 0 as libc::c_int {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while trying to close database file\x00" as
                             *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    exit(0 as libc::c_int);
}
/* Default timeout can be changed using clnt_control() */
#[c2rust::src_loc = "603:23"]
static mut full_resync_timeout: timeval =
    {
        let mut init =
            timeval{tv_sec: 25 as libc::c_int as __time_t,
                    tv_usec:
                        0 as libc::c_int as
                            __suseconds_t,}; /* max version we support */
        init
    };
#[c2rust::src_loc = "605:1"]
unsafe extern "C" fn full_resync(mut clnt: *mut CLIENT)
 -> *mut kdb_fullresync_result_t {
    static mut clnt_res: kdb_fullresync_result_t =
        kdb_fullresync_result_t{lastentry:
                                    kdb_last_t{last_sno: 0,
                                               last_time:
                                                   kdbe_time_t{seconds: 0,
                                                               useconds:
                                                                   0,},},
                                ret: UPDATE_OK,};
    let mut vers: uint32_t = 1 as libc::c_int as uint32_t;
    let mut status: clnt_stat = RPC_SUCCESS;
    memset(&mut clnt_res as *mut kdb_fullresync_result_t as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kdb_fullresync_result_t>() as libc::c_ulong);
    status =
        Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                3
                                                                                                                    as
                                                                                                                    libc::c_int
                                                                                                                    as
                                                                                                                    rpcproc_t,
                                                                                                                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                                        *mut XDR,
                                                                                                                                                                    _:
                                                                                                                                                                        *mut uint32_t)
                                                                                                                                                   ->
                                                                                                                                                       libc::c_int>,
                                                                                                                                        xdrproc_t>(Some(gssrpc_xdr_u_int32
                                                                                                                                                            as
                                                                                                                                                            unsafe extern "C" fn(_:
                                                                                                                                                                                     *mut XDR,
                                                                                                                                                                                 _:
                                                                                                                                                                                     *mut uint32_t)
                                                                                                                                                                ->
                                                                                                                                                                    libc::c_int)),
                                                                                                                &mut vers
                                                                                                                    as
                                                                                                                    *mut uint32_t
                                                                                                                    as
                                                                                                                    *mut libc::c_void,
                                                                                                                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                                        *mut XDR,
                                                                                                                                                                    _:
                                                                                                                                                                        *mut kdb_fullresync_result_t)
                                                                                                                                                   ->
                                                                                                                                                       libc::c_int>,
                                                                                                                                        xdrproc_t>(Some(xdr_kdb_fullresync_result_t
                                                                                                                                                            as
                                                                                                                                                            unsafe extern "C" fn(_:
                                                                                                                                                                                     *mut XDR,
                                                                                                                                                                                 _:
                                                                                                                                                                                     *mut kdb_fullresync_result_t)
                                                                                                                                                                ->
                                                                                                                                                                    libc::c_int)),
                                                                                                                &mut clnt_res
                                                                                                                    as
                                                                                                                    *mut kdb_fullresync_result_t
                                                                                                                    as
                                                                                                                    *mut libc::c_void,
                                                                                                                full_resync_timeout);
    if status as libc::c_uint ==
           RPC_PROCUNAVAIL as libc::c_int as libc::c_uint {
        status =
            Some((*(*clnt).cl_ops).cl_call.expect("non-null function pointer")).expect("non-null function pointer")(clnt,
                                                                                                                    2
                                                                                                                        as
                                                                                                                        libc::c_int
                                                                                                                        as
                                                                                                                        rpcproc_t,
                                                                                                                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                                            *mut XDR,
                                                                                                                                                                        _:
                                                                                                                                                                            *mut libc::c_void)
                                                                                                                                                       ->
                                                                                                                                                           libc::c_int>,
                                                                                                                                            xdrproc_t>(Some(gssrpc_xdr_void
                                                                                                                                                                as
                                                                                                                                                                unsafe extern "C" fn(_:
                                                                                                                                                                                         *mut XDR,
                                                                                                                                                                                     _:
                                                                                                                                                                                         *mut libc::c_void)
                                                                                                                                                                    ->
                                                                                                                                                                        libc::c_int)),
                                                                                                                    &mut vers
                                                                                                                        as
                                                                                                                        *mut uint32_t
                                                                                                                        as
                                                                                                                        *mut libc::c_void,
                                                                                                                    ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                                                                                                            *mut XDR,
                                                                                                                                                                        _:
                                                                                                                                                                            *mut kdb_fullresync_result_t)
                                                                                                                                                       ->
                                                                                                                                                           libc::c_int>,
                                                                                                                                            xdrproc_t>(Some(xdr_kdb_fullresync_result_t
                                                                                                                                                                as
                                                                                                                                                                unsafe extern "C" fn(_:
                                                                                                                                                                                         *mut XDR,
                                                                                                                                                                                     _:
                                                                                                                                                                                         *mut kdb_fullresync_result_t)
                                                                                                                                                                    ->
                                                                                                                                                                        libc::c_int)),
                                                                                                                    &mut clnt_res
                                                                                                                        as
                                                                                                                        *mut kdb_fullresync_result_t
                                                                                                                        as
                                                                                                                        *mut libc::c_void,
                                                                                                                    full_resync_timeout)
    }
    return if status as libc::c_uint ==
                  RPC_SUCCESS as libc::c_int as libc::c_uint {
               &mut clnt_res
           } else { 0 as *mut kdb_fullresync_result_t };
}
/*
 * Beg for incrementals from the KDC.
 *
 * Returns 0 on success IFF runonce is true.
 * Returns non-zero on failure due to errors.
 */
#[c2rust::src_loc = "632:1"]
unsafe extern "C" fn do_iprop() -> krb5_error_code {
    let mut current_block: u64;
    let mut retval: kadm5_ret_t = 0;
    let mut iprop_svc_principal: krb5_principal =
        0 as *mut krb5_principal_data;
    let mut server_handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut iprop_svc_princstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut master_svc_princstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pollin: libc::c_uint = 0;
    let mut backoff_time: libc::c_uint = 0;
    let mut backoff_cnt: libc::c_int = 0 as libc::c_int;
    let mut reinit_cnt: libc::c_int = 0 as libc::c_int;
    let mut iprop_start: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut iprop_end: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    let mut usec: libc::c_ulong = 0;
    let mut frrequested: time_t = 0 as libc::c_int as time_t;
    let mut now: time_t = 0;
    let mut incr_ret: *mut kdb_incr_result_t = 0 as *mut kdb_incr_result_t;
    let mut mylast: kdb_last_t =
        kdb_last_t{last_sno: 0,
                   last_time: kdbe_time_t{seconds: 0, useconds: 0,},};
    let mut full_ret: *mut kdb_fullresync_result_t =
        0 as *mut kdb_fullresync_result_t;
    let mut handle: kadm5_iprop_handle_t = 0 as *mut _kadm5_iprop_handle_t;
    if debug != 0 {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Incremental propagation enabled\n\x00" as *const u8
                             as *const libc::c_char));
    }
    pollin = params.iprop_poll_time as libc::c_uint;
    if pollin == 0 as libc::c_int as libc::c_uint {
        pollin = 10 as libc::c_int as libc::c_uint
    }
    if master_svc_princstr.is_null() {
        retval =
            kadm5_get_kiprop_host_srv_name(kpropd_context, realm,
                                           &mut master_svc_princstr);
        if retval != 0 {
            com_err(progname, retval,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"%s: unable to get kiprop host based service name for realm %s\n\x00"
                                 as *const u8 as *const libc::c_char),
                    progname, realm);
            return retval as krb5_error_code
        }
    }
    retval =
        sn2princ_realm(kpropd_context, 0 as *const libc::c_char,
                       b"kiprop\x00" as *const u8 as *const libc::c_char,
                       realm, &mut iprop_svc_principal) as kadm5_ret_t;
    if retval != 0 {
        com_err(progname, retval,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while trying to construct host service principal\x00"
                             as *const u8 as *const libc::c_char));
        return retval as krb5_error_code
    }
    retval =
        krb5_unparse_name(kpropd_context,
                          iprop_svc_principal as krb5_const_principal,
                          &mut iprop_svc_princstr) as kadm5_ret_t;
    if retval != 0 {
        com_err(progname, retval,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while canonicalizing principal name\x00" as
                             *const u8 as *const libc::c_char));
        krb5_free_principal(kpropd_context, iprop_svc_principal);
        return retval as krb5_error_code
    }
    krb5_free_principal(kpropd_context, iprop_svc_principal);
    'c_21631:
        loop  {
            /*
     * Authentication, initialize rpcsec_gss handle etc.
     */
            if debug != 0 {
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"Initializing kadm5 as client %s\n\x00" as
                                     *const u8 as *const libc::c_char),
                        iprop_svc_princstr);
            }
            retval =
                kadm5_init_with_skey(kpropd_context, iprop_svc_princstr,
                                     keytab_path, master_svc_princstr,
                                     &mut params,
                                     (0x12345600 as libc::c_int |
                                          0x1 as libc::c_int) as krb5_ui_4,
                                     (0x12345700 as libc::c_int |
                                          0x4 as libc::c_int) as krb5_ui_4,
                                     db_args, &mut server_handle);
            if retval != 0 {
                if debug != 0 {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"kadm5 initialization failed!\n\x00" as
                                         *const u8 as *const libc::c_char));
                }
                if retval == 43787528 as libc::c_long {
                    reinit_cnt += 1;
                    if !server_handle.is_null() {
                        kadm5_destroy(server_handle);
                    }
                    server_handle = 0 as *mut libc::c_void;
                    handle = 0 as kadm5_iprop_handle_t;
                    com_err(progname, retval,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while attempting to connect to master KDC ... retrying\x00"
                                         as *const u8 as
                                         *const libc::c_char));
                    backoff_time = backoff_from_master(&mut reinit_cnt);
                    if debug != 0 {
                        fprintf(stderr,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"Sleeping %d seconds to re-initialize kadm5 (RPC ERROR)\n\x00"
                                             as *const u8 as
                                             *const libc::c_char),
                                backoff_time);
                    }
                    sleep(backoff_time);
                } else {
                    if retval == 43787562 as libc::c_long ||
                           retval == 43787563 as libc::c_long {
                        com_err(progname, retval,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"while initializing %s interface\x00"
                                             as *const u8 as
                                             *const libc::c_char), progname);
                        usage();
                    }
                    reinit_cnt += 1;
                    com_err(progname, retval,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"while initializing %s interface, retrying\x00"
                                         as *const u8 as *const libc::c_char),
                            progname);
                    backoff_time = backoff_from_master(&mut reinit_cnt);
                    if debug != 0 {
                        fprintf(stderr,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"Sleeping %d seconds to re-initialize kadm5 (krb5kdc not running?)\n\x00"
                                             as *const u8 as
                                             *const libc::c_char),
                                backoff_time);
                    }
                    sleep(backoff_time);
                }
            } else {
                if debug != 0 {
                    fprintf(stderr,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"kadm5 initialization succeeded\n\x00"
                                         as *const u8 as
                                         *const libc::c_char));
                }
                /*
     * Reset re-initialization count to zero now.
     */
                backoff_time = 0 as libc::c_int as libc::c_uint;
                reinit_cnt = backoff_time as libc::c_int;
                /*
     * Reset the handle to the correct type for the RPC call
     */
                handle = server_handle as kadm5_iprop_handle_t;
                loop  {
                    incr_ret = 0 as *mut kdb_incr_result_t;
                    full_ret = 0 as *mut kdb_fullresync_result_t;
                    /*
         * Get the most recent ulog entry sno + ts, which
         * we package in the request to the master KDC
         */
                    retval =
                        ulog_get_last(kpropd_context, &mut mylast) as
                            kadm5_ret_t;
                    if retval != 0 {
                        com_err(progname, retval,
                                dgettext(b"mit-krb5\x00" as *const u8 as
                                             *const libc::c_char,
                                         b"reading update log header\x00" as
                                             *const u8 as
                                             *const libc::c_char));
                        current_block = 2327375404519713487;
                        break 'c_21631 ;
                    } else {
                        /*
         * Loop continuously on an iprop_get_updates_1(),
         * so that we can keep probing the master for updates
         * or (if needed) do a full resync of the krb5 db.
         */
                        if debug != 0 {
                            fprintf(stderr,
                                    dgettext(b"mit-krb5\x00" as *const u8 as
                                                 *const libc::c_char,
                                             b"Calling iprop_get_updates_1 (sno=%u sec=%u usec=%u)\n\x00"
                                                 as *const u8 as
                                                 *const libc::c_char),
                                    mylast.last_sno, mylast.last_time.seconds,
                                    mylast.last_time.useconds);
                        }
                        gettimeofday(&mut iprop_start, 0 as *mut timezone);
                        incr_ret =
                            iprop_get_updates_1(&mut mylast, (*handle).clnt);
                        if incr_ret.is_null() {
                            gssrpc_clnt_perror((*handle).clnt,
                                               dgettext(b"mit-krb5\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        b"iprop_get_updates call failed\x00"
                                                            as *const u8 as
                                                            *const libc::c_char));
                            if !server_handle.is_null() {
                                kadm5_destroy(server_handle);
                            }
                            server_handle = 0 as *mut libc::c_void;
                            handle =
                                0 as *mut libc::c_void as
                                    kadm5_iprop_handle_t;
                            if debug != 0 {
                                fprintf(stderr,
                                        dgettext(b"mit-krb5\x00" as *const u8
                                                     as *const libc::c_char,
                                                 b"Reinitializing iprop because get updates failed\n\x00"
                                                     as *const u8 as
                                                     *const libc::c_char));
                            }
                            break ;
                        } else {
                            match (*incr_ret).ret as libc::c_uint {
                                2 => {
                                    /*
             * If we're already asked for a full resync and we still
             * need one and the last one hasn't timed out then just keep
             * asking for updates as eventually the resync will finish
             * (or, if it times out we'll just try again).  Note that
             * doit() also applies a timeout to the full resync, thus
             * it's OK for us to do the same here.
             */
                                    now = time(0 as *mut time_t);
                                    if frrequested != 0 &&
                                           now - frrequested <
                                               params.iprop_resync_timeout as
                                                   libc::c_long {
                                        if debug != 0 {
                                            fprintf(stderr,
                                                    dgettext(b"mit-krb5\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             b"Still waiting for full resync\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char));
                                        }
                                    } else {
                                        frrequested = now;
                                        if debug != 0 {
                                            fprintf(stderr,
                                                    dgettext(b"mit-krb5\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             b"Full resync needed\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char));
                                        }
                                        syslog(6 as libc::c_int,
                                               dgettext(b"mit-krb5\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        b"kpropd: Full resync needed.\x00"
                                                            as *const u8 as
                                                            *const libc::c_char));
                                        full_ret =
                                            full_resync((*handle).clnt);
                                        if full_ret.is_null() {
                                            gssrpc_clnt_perror((*handle).clnt,
                                                               dgettext(b"mit-krb5\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char,
                                                                        b"iprop_full_resync call failed\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char));
                                            kadm5_destroy(server_handle);
                                            server_handle =
                                                0 as *mut libc::c_void;
                                            handle =
                                                0 as kadm5_iprop_handle_t;
                                            break ;
                                        } else {
                                            match (*full_ret).ret as
                                                      libc::c_uint {
                                                0 => {
                                                    if debug != 0 {
                                                        fprintf(stderr,
                                                                dgettext(b"mit-krb5\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         b"Full resync request granted\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char));
                                                    }
                                                    syslog(6 as libc::c_int,
                                                           dgettext(b"mit-krb5\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    b"Full resync request granted.\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char));
                                                    backoff_cnt =
                                                        0 as libc::c_int
                                                }
                                                3 => {
                                                    /*
                 * Exponential backoff
                 */
                                                    if debug != 0 {
                                                        fprintf(stderr,
                                                                dgettext(b"mit-krb5\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         b"Exponential backoff\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char));
                                                    }
                                                    backoff_cnt += 1
                                                }
                                                5 => {
                                                    if debug != 0 {
                                                        fprintf(stderr,
                                                                dgettext(b"mit-krb5\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         b"Full resync permission denied\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char));
                                                    }
                                                    syslog(3 as libc::c_int,
                                                           dgettext(b"mit-krb5\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    b"Full resync, permission denied.\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char));
                                                    current_block =
                                                        13147447227483120622;
                                                    break 'c_21631 ;
                                                }
                                                1 => {
                                                    if debug != 0 {
                                                        fprintf(stderr,
                                                                dgettext(b"mit-krb5\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         b"Full resync error from master\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char));
                                                    }
                                                    syslog(3 as libc::c_int,
                                                           dgettext(b"mit-krb5\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    b" Full resync, error returned from master KDC.\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char));
                                                    current_block =
                                                        13147447227483120622;
                                                    break 'c_21631 ;
                                                }
                                                _ => {
                                                    backoff_cnt =
                                                        0 as libc::c_int;
                                                    if debug != 0 {
                                                        fprintf(stderr,
                                                                dgettext(b"mit-krb5\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char,
                                                                         b"Full resync invalid result from master\n\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char));
                                                    }
                                                    syslog(3 as libc::c_int,
                                                           dgettext(b"mit-krb5\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char,
                                                                    b"Full resync, invalid return from master KDC.\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char));
                                                }
                                            }
                                        }
                                    }
                                }
                                0 => {
                                    backoff_cnt = 0 as libc::c_int;
                                    frrequested = 0 as libc::c_int as time_t;
                                    /*
             * ulog_replay() will convert the ulog updates to db
             * entries using the kdb conv api and will commit
             * the entries to the replica kdc database
             */
                                    if debug != 0 {
                                        fprintf(stderr,
                                                dgettext(b"mit-krb5\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         b"Got incremental updates (sno=%u sec=%u usec=%u)\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char),
                                                (*incr_ret).lastentry.last_sno,
                                                (*incr_ret).lastentry.last_time.seconds,
                                                (*incr_ret).lastentry.last_time.useconds);
                                    }
                                    retval =
                                        ulog_replay(kpropd_context, incr_ret,
                                                    db_args) as kadm5_ret_t;
                                    if retval != 0 {
                                        let mut msg: *const libc::c_char =
                                            krb5_get_error_message(kpropd_context,
                                                                   retval as
                                                                       krb5_error_code);
                                        if debug != 0 {
                                            fprintf(stderr,
                                                    dgettext(b"mit-krb5\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             b"ulog_replay failed (%s), updates not registered\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char),
                                                    msg);
                                        }
                                        syslog(3 as libc::c_int,
                                               dgettext(b"mit-krb5\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        b"ulog_replay failed (%s), updates not registered.\x00"
                                                            as *const u8 as
                                                            *const libc::c_char),
                                               msg);
                                        krb5_free_error_message(kpropd_context,
                                                                msg);
                                    } else {
                                        gettimeofday(&mut iprop_end,
                                                     0 as *mut timezone);
                                        usec =
                                            ((iprop_end.tv_sec -
                                                  iprop_start.tv_sec) *
                                                 1000000 as libc::c_int as
                                                     libc::c_long +
                                                 iprop_end.tv_usec -
                                                 iprop_start.tv_usec) as
                                                libc::c_ulong;
                                        syslog(6 as libc::c_int,
                                               dgettext(b"mit-krb5\x00" as
                                                            *const u8 as
                                                            *const libc::c_char,
                                                        b"Incremental updates: %d updates / %lu us\x00"
                                                            as *const u8 as
                                                            *const libc::c_char),
                                               (*incr_ret).updates.kdb_ulog_t_len,
                                               usec);
                                        if debug != 0 {
                                            fprintf(stderr,
                                                    dgettext(b"mit-krb5\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char,
                                                             b"Incremental updates: %d updates / %lu us\n\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char),
                                                    (*incr_ret).updates.kdb_ulog_t_len,
                                                    usec);
                                        }
                                    }
                                }
                                5 => {
                                    if debug != 0 {
                                        fprintf(stderr,
                                                dgettext(b"mit-krb5\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         b"get_updates permission denied\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char));
                                    }
                                    syslog(3 as libc::c_int,
                                           dgettext(b"mit-krb5\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    b"get_updates, permission denied.\x00"
                                                        as *const u8 as
                                                        *const libc::c_char));
                                    current_block = 13147447227483120622;
                                    break 'c_21631 ;
                                }
                                1 => {
                                    if debug != 0 {
                                        fprintf(stderr,
                                                dgettext(b"mit-krb5\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         b"get_updates error from master\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char));
                                    }
                                    syslog(3 as libc::c_int,
                                           dgettext(b"mit-krb5\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    b"get_updates, error returned from master KDC.\x00"
                                                        as *const u8 as
                                                        *const libc::c_char));
                                    current_block = 13147447227483120622;
                                    break 'c_21631 ;
                                }
                                3 => {
                                    /*
             * Exponential backoff
             */
                                    if debug != 0 {
                                        fprintf(stderr,
                                                dgettext(b"mit-krb5\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         b"get_updates master busy; backoff\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char));
                                    }
                                    backoff_cnt += 1
                                }
                                4 => {
                                    /*
             * Master-replica are in sync
             */
                                    if debug != 0 {
                                        fprintf(stderr,
                                                dgettext(b"mit-krb5\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         b"KDC is synchronized with master.\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char));
                                    }
                                    backoff_cnt = 0 as libc::c_int;
                                    frrequested = 0 as libc::c_int as time_t
                                }
                                _ => {
                                    backoff_cnt = 0 as libc::c_int;
                                    if debug != 0 {
                                        fprintf(stderr,
                                                dgettext(b"mit-krb5\x00" as
                                                             *const u8 as
                                                             *const libc::c_char,
                                                         b"get_updates invalid result from master\n\x00"
                                                             as *const u8 as
                                                             *const libc::c_char));
                                    }
                                    syslog(3 as libc::c_int,
                                           dgettext(b"mit-krb5\x00" as
                                                        *const u8 as
                                                        *const libc::c_char,
                                                    b"get_updates, invalid return from master KDC.\x00"
                                                        as *const u8 as
                                                        *const libc::c_char));
                                }
                            }
                            if runonce == 1 as libc::c_int &&
                                   (*incr_ret).ret as libc::c_uint !=
                                       UPDATE_FULL_RESYNC_NEEDED as
                                           libc::c_int as libc::c_uint {
                                current_block = 2327375404519713487;
                                break 'c_21631 ;
                            }
                            /*
         * Sleep for the specified poll interval (Default is 2 mts),
         * or do a binary exponential backoff if we get an
         * UPDATE_BUSY signal
         */
                            if backoff_cnt > 0 as libc::c_int {
                                backoff_time =
                                    backoff_from_master(&mut backoff_cnt);
                                if debug != 0 {
                                    fprintf(stderr,
                                            dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"Busy signal received from master, backoff for %d secs\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char),
                                            backoff_time);
                                }
                                sleep(backoff_time);
                            } else {
                                if debug != 0 {
                                    fprintf(stderr,
                                            dgettext(b"mit-krb5\x00" as
                                                         *const u8 as
                                                         *const libc::c_char,
                                                     b"Waiting for %d seconds before checking for updates again\n\x00"
                                                         as *const u8 as
                                                         *const libc::c_char),
                                            pollin);
                                }
                                sleep(pollin);
                            }
                        }
                    }
                }
            }
        }
    match current_block {
        13147447227483120622 => {
            if debug != 0 {
                fprintf(stderr,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"ERROR returned by master, bailing\n\x00" as
                                     *const u8 as *const libc::c_char));
            }
            syslog(3 as libc::c_int,
                   dgettext(b"mit-krb5\x00" as *const u8 as
                                *const libc::c_char,
                            b"ERROR returned by master KDC, bailing.\n\x00" as
                                *const u8 as *const libc::c_char));
        }
        _ => { }
    }
    free(iprop_svc_princstr as *mut libc::c_void);
    free(master_svc_princstr as *mut libc::c_void);
    krb5_free_default_realm(kpropd_context, def_realm);
    kadm5_destroy(server_handle);
    krb5_db_fini(kpropd_context);
    ulog_fini(kpropd_context);
    krb5_free_context(kpropd_context);
    return if runonce == 1 as libc::c_int {
               0 as libc::c_int
           } else { 1 as libc::c_int };
}
/* Do exponential backoff, since master KDC is BUSY or down. */
#[c2rust::src_loc = "1006:1"]
unsafe extern "C" fn backoff_from_master(mut cnt: *mut libc::c_int)
 -> libc::c_uint {
    let mut btime: libc::c_uint = 0;
    btime = ((2 as libc::c_int) << *cnt) as libc::c_uint;
    if btime > 300 as libc::c_int as libc::c_uint {
        btime = 300 as libc::c_int as libc::c_uint;
        *cnt -= 1
    }
    return btime;
}
#[c2rust::src_loc = "1028:1"]
unsafe extern "C" fn kpropd_com_err_proc(mut whoami: *const libc::c_char,
                                         mut code: libc::c_long,
                                         mut fmt: *const libc::c_char,
                                         mut args: ::std::ffi::VaList) {
    let mut error_buf: [libc::c_char; 8096] = [0; 8096];
    error_buf[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    if !fmt.is_null() {
        vsnprintf(error_buf.as_mut_ptr(),
                  ::std::mem::size_of::<[libc::c_char; 8096]>() as
                      libc::c_ulong, fmt, args.as_va_list());
    }
    syslog(3 as libc::c_int,
           b"%s%s%s%s%s\x00" as *const u8 as *const libc::c_char,
           if !whoami.is_null() {
               whoami
           } else { b"\x00" as *const u8 as *const libc::c_char },
           if !whoami.is_null() {
               b": \x00" as *const u8 as *const libc::c_char
           } else { b"\x00" as *const u8 as *const libc::c_char },
           if code != 0 {
               error_message(code)
           } else { b"\x00" as *const u8 as *const libc::c_char },
           if code != 0 {
               b" \x00" as *const u8 as *const libc::c_char
           } else { b"\x00" as *const u8 as *const libc::c_char },
           error_buf.as_mut_ptr());
}
#[c2rust::src_loc = "1041:1"]
unsafe extern "C" fn parse_args(mut argc: libc::c_int,
                                mut argv: *mut *mut libc::c_char) {
    let mut newargs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut retval: krb5_error_code = 0;
    let mut long_options: [option; 1] =
        [{
             let mut init =
                 option{name:
                            b"pid-file\x00" as *const u8 as
                                *const libc::c_char,
                        has_arg: 1 as libc::c_int,
                        flag: 0 as *mut libc::c_int,
                        val: PID_FILE as libc::c_int,};
             init
         }];
    memset(&mut params as *mut kadm5_config_params as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<kadm5_config_params>() as libc::c_ulong);
    /* Since we may modify the KDB with ulog_replay(), we must read the KDC
     * profile. */
    retval = krb5int_init_context_kdc(&mut kpropd_context);
    if retval != 0 {
        com_err(*argv.offset(0 as libc::c_int as isize), retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while initializing krb5\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    progname = *argv.offset(0 as libc::c_int as isize);
    loop  {
        c =
            getopt_long(argc, argv,
                        b"A:f:F:p:P:r:s:DdSa:tx:\x00" as *const u8 as
                            *const libc::c_char, long_options.as_mut_ptr(),
                        0 as *mut libc::c_int);
        if !(c != -(1 as libc::c_int)) { break ; }
        match c {
            65 => {
                params.mask |= 0x10000 as libc::c_int as libc::c_long;
                params.admin_server = optarg
            }
            102 => { file = optarg }
            70 => { kerb_database = optarg }
            112 => { kdb5_util = optarg }
            80 => { port = optarg }
            114 => { realm = optarg }
            115 => { keytab_path = optarg }
            68 => { nodaemon += 1 }
            100 => { debug += 1 }
            83 => { }
            97 => { acl_file_name = optarg }
            116 => {
                /* Undocumented option - for testing only.  Run the kpropd
             * server exactly once. */
                runonce = 1 as libc::c_int
            }
            120 => {
                newargs =
                    realloc(db_args as *mut libc::c_void,
                            ((db_args_size + 2 as libc::c_int) as
                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                                 as
                                                                 libc::c_ulong))
                        as *mut *mut libc::c_char;
                if newargs.is_null() {
                    com_err(*argv.offset(0 as libc::c_int as isize),
                            *__errno_location() as errcode_t,
                            dgettext(b"mit-krb5\x00" as *const u8 as
                                         *const libc::c_char,
                                     b"copying db args\x00" as *const u8 as
                                         *const libc::c_char));
                    exit(1 as libc::c_int);
                }
                db_args = newargs;
                let ref mut fresh0 = *db_args.offset(db_args_size as isize);
                *fresh0 = optarg;
                let ref mut fresh1 =
                    *db_args.offset((db_args_size + 1 as libc::c_int) as
                                        isize);
                *fresh1 = 0 as *mut libc::c_char;
                db_args_size += 1
            }
            256 => { pid_file = optarg }
            _ => { usage(); }
        }
    }
    if optind != argc { usage(); }
    openlog(b"kpropd\x00" as *const u8 as *const libc::c_char,
            0x1 as libc::c_int | 0x4 as libc::c_int,
            (3 as libc::c_int) << 3 as libc::c_int);
    if debug == 0 {
        set_com_err_hook(Some(kpropd_com_err_proc as
                                  unsafe extern "C" fn(_: *const libc::c_char,
                                                       _: libc::c_long,
                                                       _: *const libc::c_char,
                                                       _: ::std::ffi::VaList)
                                      -> ()));
    }
    if realm.is_null() {
        retval = krb5_get_default_realm(kpropd_context, &mut def_realm);
        if retval != 0 {
            com_err(progname, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Unable to get default realm\x00" as *const u8
                                 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
        realm = def_realm
    } else {
        retval = krb5_set_default_realm(kpropd_context, realm);
        if retval != 0 {
            com_err(progname, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Unable to set default realm\x00" as *const u8
                                 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
    }
    /* Construct service name from local hostname. */
    retval =
        sn2princ_realm(kpropd_context, 0 as *const libc::c_char,
                       b"host\x00" as *const u8 as *const libc::c_char, realm,
                       &mut server);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while trying to construct my service name\x00" as
                             *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    /* Construct the name of the temporary file. */
    if asprintf(&mut temp_file_name as *mut *mut libc::c_char,
                b"%s.temp\x00" as *const u8 as *const libc::c_char, file) <
           0 as libc::c_int {
        com_err(progname, 12 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while allocating filename for temp file\x00" as
                             *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    params.realm = realm;
    params.mask |= 0x1 as libc::c_int as libc::c_long;
    retval =
        kadm5_get_config_params(kpropd_context, 1 as libc::c_int, &mut params,
                                &mut params);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while initializing\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    if params.iprop_enabled == 1 as libc::c_int {
        ulog_set_role(kpropd_context, IPROP_REPLICA);
        if ulog_map(kpropd_context, params.iprop_logfile,
                    params.iprop_ulogsize) != 0 {
            com_err(progname, *__errno_location() as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Unable to map log!\n\x00" as *const u8 as
                                 *const libc::c_char));
            exit(1 as libc::c_int);
        }
    };
}
/*
 * Figure out who's calling on the other end of the connection....
 */
#[c2rust::src_loc = "1182:1"]
unsafe extern "C" fn kerberos_authenticate(mut context: krb5_context,
                                           mut fd: libc::c_int,
                                           mut clientp: *mut krb5_principal,
                                           mut etype: *mut krb5_enctype,
                                           mut my_sin:
                                               *mut sockaddr_storage) {
    let mut retval: krb5_error_code = 0;
    let mut ticket: *mut krb5_ticket = 0 as *mut krb5_ticket;
    let mut r_sin: sockaddr_storage =
        sockaddr_storage{ss_family: 0,
                         __ss_padding: [0; 118],
                         __ss_align: 0,};
    let mut sin_length: socklen_t = 0;
    let mut keytab: krb5_keytab = 0 as krb5_keytab;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut etypebuf: [libc::c_char; 100] = [0; 100];
    /* Set recv_addr and send_addr. */
    sockaddr2krbaddr(context, (*my_sin).ss_family as libc::c_int,
                     my_sin as *mut sockaddr, &mut sender_addr);
    sin_length =
        ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
            socklen_t;
    if getsockname(fd,
                   __SOCKADDR_ARG{__sockaddr__:
                                      &mut r_sin as *mut sockaddr_storage as
                                          *mut sockaddr,}, &mut sin_length) !=
           0 {
        com_err(progname, *__errno_location() as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while getting local socket address\x00" as
                             *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    sockaddr2krbaddr(context, r_sin.ss_family as libc::c_int,
                     &mut r_sin as *mut sockaddr_storage as *mut sockaddr,
                     &mut receiver_addr);
    if debug != 0 {
        retval =
            krb5_unparse_name(context, server as krb5_const_principal,
                              &mut name);
        if retval != 0 {
            com_err(progname, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while unparsing client name\x00" as *const u8
                                 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
        fprintf(stderr,
                b"krb5_recvauth(%d, %s, %s, ...)\n\x00" as *const u8 as
                    *const libc::c_char, fd, kprop_version, name);
        free(name as *mut libc::c_void);
    }
    retval = krb5_auth_con_init(context, &mut auth_context);
    if retval != 0 {
        syslog(3 as libc::c_int,
               dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Error in krb5_auth_con_ini: %s\x00" as *const u8 as
                            *const libc::c_char),
               error_message(retval as errcode_t));
        exit(1 as libc::c_int);
    }
    retval =
        krb5_auth_con_setflags(context, auth_context, 0x4 as libc::c_int);
    if retval != 0 {
        syslog(3 as libc::c_int,
               dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Error in krb5_auth_con_setflags: %s\x00" as
                            *const u8 as *const libc::c_char),
               error_message(retval as errcode_t));
        exit(1 as libc::c_int);
    }
    retval =
        krb5_auth_con_setaddrs(context, auth_context, receiver_addr,
                               sender_addr);
    if retval != 0 {
        syslog(3 as libc::c_int,
               dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Error in krb5_auth_con_setaddrs: %s\x00" as
                            *const u8 as *const libc::c_char),
               error_message(retval as errcode_t));
        exit(1 as libc::c_int);
    }
    if !keytab_path.is_null() {
        retval = krb5_kt_resolve(context, keytab_path, &mut keytab);
        if retval != 0 {
            syslog(3 as libc::c_int,
                   dgettext(b"mit-krb5\x00" as *const u8 as
                                *const libc::c_char,
                            b"Error in krb5_kt_resolve: %s\x00" as *const u8
                                as *const libc::c_char),
                   error_message(retval as errcode_t));
            exit(1 as libc::c_int);
        }
    }
    retval =
        krb5_recvauth(context, &mut auth_context,
                      &mut fd as *mut libc::c_int as krb5_pointer,
                      kprop_version, server, 0 as libc::c_int, keytab,
                      &mut ticket);
    if retval != 0 {
        syslog(3 as libc::c_int,
               dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Error in krb5_recvauth: %s\x00" as *const u8 as
                            *const libc::c_char),
               error_message(retval as errcode_t));
        exit(1 as libc::c_int);
    }
    retval =
        krb5_copy_principal(context,
                            (*(*ticket).enc_part2).client as
                                krb5_const_principal, clientp);
    if retval != 0 {
        syslog(3 as libc::c_int,
               dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"Error in krb5_copy_prinicpal: %s\x00" as *const u8
                            as *const libc::c_char),
               error_message(retval as errcode_t));
        exit(1 as libc::c_int);
    }
    *etype = (*ticket).enc_part.enctype;
    if debug != 0 {
        retval =
            krb5_unparse_name(context, *clientp as krb5_const_principal,
                              &mut name);
        if retval != 0 {
            com_err(progname, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while unparsing client name\x00" as *const u8
                                 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
        retval =
            krb5_enctype_to_name(*etype, 0 as libc::c_int as krb5_boolean,
                                 etypebuf.as_mut_ptr(),
                                 ::std::mem::size_of::<[libc::c_char; 100]>()
                                     as libc::c_ulong);
        if retval != 0 {
            com_err(progname, retval as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while unparsing ticket etype\x00" as *const u8
                                 as *const libc::c_char));
            exit(1 as libc::c_int);
        }
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"authenticated client: %s (etype == %s)\n\x00" as
                             *const u8 as *const libc::c_char), name,
                etypebuf.as_mut_ptr());
        free(name as *mut libc::c_void);
    }
    krb5_free_ticket(context, ticket);
}
#[c2rust::src_loc = "1288:1"]
unsafe extern "C" fn authorized_principal(mut context: krb5_context,
                                          mut p: krb5_principal,
                                          mut auth_etype: krb5_enctype)
 -> krb5_boolean {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut retval: krb5_error_code = 0;
    let mut acl_file: *mut FILE = 0 as *mut FILE;
    let mut end: libc::c_int = 0;
    let mut acl_etype: krb5_enctype = 0;
    retval = krb5_unparse_name(context, p as krb5_const_principal, &mut name);
    if retval != 0 { return 0 as libc::c_int as krb5_boolean }
    acl_file =
        fopen(acl_file_name, b"r\x00" as *const u8 as *const libc::c_char);
    if acl_file.is_null() { return 0 as libc::c_int as krb5_boolean }
    while feof(acl_file) == 0 {
        if fgets(buf.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 1024]>() as
                     libc::c_ulong as libc::c_int, acl_file).is_null() {
            break ;
        }
        end =
            strlen(buf.as_mut_ptr()).wrapping_sub(1 as libc::c_int as
                                                      libc::c_ulong) as
                libc::c_int;
        if buf[end as usize] as libc::c_int == '\n' as i32 {
            buf[end as usize] = '\u{0}' as i32 as libc::c_char
        }
        if !(strncmp(name, buf.as_mut_ptr(), strlen(name)) == 0) {
            continue ;
        }
        ptr = buf.as_mut_ptr().offset(strlen(name) as isize);
        /* If the next character is not whitespace or null, then the match
             * is only partial.  Continue on to new lines. */
        if *ptr as libc::c_int != '\u{0}' as i32 &&
               *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as
                   libc::c_int &
                   _ISspace as libc::c_int as libc::c_ushort as libc::c_int ==
                   0 {
            continue ;
        }
        /* Otherwise, skip trailing whitespace. */
        while *ptr as libc::c_int != '\u{0}' as i32 &&
                  *(*__ctype_b_loc()).offset(*ptr as libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                      != 0 {
            ptr = ptr.offset(1)
        }
        /*
             * Now, look for an etype string.  If there isn't one, return true.
             * If there is an invalid string, continue.  If there is a valid
             * string, return true only if it matches the etype passed in,
             * otherwise continue.
             */
        if *ptr as libc::c_int != '\u{0}' as i32 &&
               {
                   retval = krb5_string_to_enctype(ptr, &mut acl_etype);
                   (retval != 0) || acl_etype != auth_etype
               } {
            continue ;
        }
        free(name as *mut libc::c_void);
        fclose(acl_file);
        return 1 as libc::c_int as krb5_boolean
    }
    free(name as *mut libc::c_void);
    fclose(acl_file);
    return 0 as libc::c_int as krb5_boolean;
}
#[c2rust::src_loc = "1344:1"]
unsafe extern "C" fn recv_database(mut context: krb5_context,
                                   mut fd: libc::c_int,
                                   mut database_fd: libc::c_int,
                                   mut confmsg: *mut krb5_data) {
    let mut database_size: krb5_ui_4 = 0;
    let mut received_size: krb5_ui_4 = 0;
    let mut n: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut inbuf: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut outbuf: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut retval: krb5_error_code = 0;
    /* Receive and decode size from client. */
    retval =
        krb5_read_message(context,
                          &mut fd as *mut libc::c_int as krb5_pointer,
                          &mut inbuf);
    if retval != 0 {
        send_error(context, fd, retval,
                   b"while reading database size\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char);
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while reading size of database from client\x00" as
                             *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    if !(&mut inbuf as *mut krb5_data).is_null() && inbuf.length != 0 &&
           *inbuf.data.offset(0 as libc::c_int as isize) as libc::c_int &
               !(0x20 as libc::c_int) ==
               30 as libc::c_int | 0x40 as libc::c_int {
        recv_error(context, &mut inbuf);
    }
    retval =
        krb5_rd_safe(context, auth_context, &mut inbuf, &mut outbuf,
                     0 as *mut krb5_replay_data);
    if retval != 0 {
        send_error(context, fd, retval,
                   b"while decoding database size\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char);
        krb5_free_data_contents(context, &mut inbuf);
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while decoding database size from client\x00" as
                             *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    memcpy(&mut database_size as *mut krb5_ui_4 as *mut libc::c_void,
           outbuf.data as *const libc::c_void,
           ::std::mem::size_of::<krb5_ui_4>() as libc::c_ulong);
    krb5_free_data_contents(context, &mut inbuf);
    krb5_free_data_contents(context, &mut outbuf);
    database_size = ntohl(database_size);
    /* Initialize the initial vector. */
    retval = krb5_auth_con_initivector(context, auth_context);
    if retval != 0 {
        send_error(context, fd, retval,
                   b"failed while initializing i_vector\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char);
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while initializing i_vector\x00" as *const u8 as
                             *const libc::c_char));
        exit(1 as libc::c_int);
    }
    if debug != 0 {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Full propagation transfer started.\n\x00" as
                             *const u8 as *const libc::c_char));
    }
    /* Now start receiving the database from the net. */
    received_size = 0 as libc::c_int as krb5_ui_4;
    while received_size < database_size {
        retval =
            krb5_read_message(context,
                              &mut fd as *mut libc::c_int as krb5_pointer,
                              &mut inbuf);
        if retval != 0 {
            snprintf(buf.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong,
                     b"while reading database block starting at offset %d\x00"
                         as *const u8 as *const libc::c_char, received_size);
            com_err(progname, retval as errcode_t,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    buf.as_mut_ptr());
            send_error(context, fd, retval, buf.as_mut_ptr());
            exit(1 as libc::c_int);
        }
        if !(&mut inbuf as *mut krb5_data).is_null() && inbuf.length != 0 &&
               *inbuf.data.offset(0 as libc::c_int as isize) as libc::c_int &
                   !(0x20 as libc::c_int) ==
                   30 as libc::c_int | 0x40 as libc::c_int {
            recv_error(context, &mut inbuf);
        }
        retval =
            krb5_rd_priv(context, auth_context, &mut inbuf, &mut outbuf,
                         0 as *mut krb5_replay_data);
        if retval != 0 {
            snprintf(buf.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong,
                     b"while decoding database block starting at offset %d\x00"
                         as *const u8 as *const libc::c_char, received_size);
            com_err(progname, retval as errcode_t,
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    buf.as_mut_ptr());
            send_error(context, fd, retval, buf.as_mut_ptr());
            krb5_free_data_contents(context, &mut inbuf);
            exit(1 as libc::c_int);
        }
        n =
            write(database_fd, outbuf.data as *const libc::c_void,
                  outbuf.length as size_t) as libc::c_int;
        krb5_free_data_contents(context, &mut inbuf);
        krb5_free_data_contents(context, &mut outbuf);
        if n < 0 as libc::c_int {
            snprintf(buf.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong,
                     b"while writing database block starting at offset %d\x00"
                         as *const u8 as *const libc::c_char, received_size);
            send_error(context, fd, *__errno_location(), buf.as_mut_ptr());
        } else if n as libc::c_uint != outbuf.length {
            snprintf(buf.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong,
                     b"incomplete write while writing database block starting at \noffset %d (%d written, %d expected)\x00"
                         as *const u8 as *const libc::c_char, received_size,
                     n, outbuf.length);
            send_error(context, fd,
                       -(1765328324 as libc::c_long) as krb5_error_code,
                       buf.as_mut_ptr());
        }
        received_size =
            (received_size as libc::c_uint).wrapping_add(outbuf.length) as
                krb5_ui_4 as krb5_ui_4
    }
    /* OK, we've seen the entire file.  Did we get too many bytes? */
    if received_size > database_size {
        snprintf(buf.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 1024]>() as
                     libc::c_ulong,
                 b"Received %d bytes, expected %d bytes for database file\x00"
                     as *const u8 as *const libc::c_char, received_size,
                 database_size);
        send_error(context, fd,
                   -(1765328324 as libc::c_long) as krb5_error_code,
                   buf.as_mut_ptr());
    }
    if debug != 0 {
        fprintf(stderr,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"Full propagation transfer finished.\n\x00" as
                             *const u8 as *const libc::c_char));
    }
    /* Create message acknowledging number of bytes received, but
     * don't send it until kdb5_util returns successfully. */
    database_size = htonl(database_size);
    inbuf.data = &mut database_size as *mut krb5_ui_4 as *mut libc::c_char;
    inbuf.length =
        ::std::mem::size_of::<krb5_ui_4>() as libc::c_ulong as libc::c_uint;
    retval =
        krb5_mk_safe(context, auth_context, &mut inbuf, confmsg,
                     0 as *mut krb5_replay_data);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                b"while encoding # of receieved bytes\x00" as *const u8 as
                    *const libc::c_char);
        send_error(context, fd, retval,
                   b"while encoding # of received bytes\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char);
        exit(1 as libc::c_int);
    };
}
#[c2rust::src_loc = "1456:1"]
unsafe extern "C" fn send_error(mut context: krb5_context,
                                mut fd: libc::c_int,
                                mut err_code: krb5_error_code,
                                mut err_text: *mut libc::c_char) {
    let mut error: krb5_error =
        krb5_error{magic: 0,
                   ctime: 0,
                   cusec: 0,
                   susec: 0,
                   stime: 0,
                   error: 0,
                   client: 0 as *mut krb5_principal_data,
                   server: 0 as *mut krb5_principal_data,
                   text:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},
                   e_data:
                       krb5_data{magic: 0,
                                 length: 0,
                                 data: 0 as *mut libc::c_char,},};
    let mut text: *const libc::c_char = 0 as *const libc::c_char;
    let mut outbuf: krb5_data =
        krb5_data{magic: 0, length: 0, data: 0 as *mut libc::c_char,};
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    memset(&mut error as *mut krb5_error as *mut libc::c_void,
           0 as libc::c_int,
           ::std::mem::size_of::<krb5_error>() as libc::c_ulong);
    krb5_us_timeofday(context, &mut error.stime, &mut error.susec);
    error.server = server;
    error.client = client;
    text =
        if !err_text.is_null() {
            err_text as *const libc::c_char
        } else { error_message(err_code as errcode_t) };
    error.error =
        (err_code as libc::c_long - -(1765328384 as libc::c_long)) as
            krb5_ui_4;
    if error.error > 127 as libc::c_int as libc::c_uint {
        error.error = 60 as libc::c_int as krb5_ui_4;
        if !err_text.is_null() {
            snprintf(buf.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong,
                     b"%s %s\x00" as *const u8 as *const libc::c_char,
                     error_message(err_code as errcode_t), err_text);
            text = buf.as_mut_ptr()
        }
    }
    error.text.length =
        strlen(text).wrapping_add(1 as libc::c_int as libc::c_ulong) as
            libc::c_uint;
    error.text.data = strdup(text);
    if !error.text.data.is_null() {
        if krb5_mk_error(context, &mut error, &mut outbuf) == 0 {
            krb5_write_message(context,
                               &mut fd as *mut libc::c_int as krb5_pointer,
                               &mut outbuf);
            krb5_free_data_contents(context, &mut outbuf);
        }
        free(error.text.data as *mut libc::c_void);
    };
}
#[c2rust::src_loc = "1492:1"]
unsafe extern "C" fn recv_error(mut context: krb5_context,
                                mut inbuf: *mut krb5_data) {
    let mut error: *mut krb5_error = 0 as *mut krb5_error;
    let mut retval: krb5_error_code = 0;
    retval = krb5_rd_error(context, inbuf, &mut error);
    if retval != 0 {
        com_err(progname, retval as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"while decoding error packet from client\x00" as
                             *const u8 as *const libc::c_char));
        exit(1 as libc::c_int);
    }
    if (*error).error == 60 as libc::c_int as libc::c_uint {
        if !(*error).text.data.is_null() {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Generic remote error: %s\n\x00" as *const u8 as
                                 *const libc::c_char), (*error).text.data);
        }
    } else if (*error).error != 0 {
        com_err(progname,
                (*error).error as krb5_error_code as libc::c_long +
                    -(1765328384 as libc::c_long),
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"signaled from server\x00" as *const u8 as
                             *const libc::c_char));
        if !(*error).text.data.is_null() {
            fprintf(stderr,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"Error text from client: %s\n\x00" as *const u8
                                 as *const libc::c_char), (*error).text.data);
        }
    }
    krb5_free_error(context, error);
    exit(1 as libc::c_int);
}
#[c2rust::src_loc = "1520:1"]
unsafe extern "C" fn load_database(mut context: krb5_context,
                                   mut kdb_util: *mut libc::c_char,
                                   mut database_file_name:
                                       *mut libc::c_char) {
    static mut edit_av: [*mut libc::c_char; 10] =
        [0 as *const libc::c_char as *mut libc::c_char; 10];
    let mut error_ret: libc::c_int = 0;
    let mut child_pid: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    /* <sys/param.h> has been included, so BSD will be defined on
     * BSD systems. */
    let mut waitb: libc::c_int = 0;
    let mut log_ctx: *mut kdb_log_context = 0 as *mut kdb_log_context;
    if debug != 0 {
        fprintf(stderr,
                b"calling kdb5_util to load database\n\x00" as *const u8 as
                    *const libc::c_char);
    }
    log_ctx = (*context).kdblog_context;
    edit_av[0 as libc::c_int as usize] = kdb_util;
    count = 1 as libc::c_int;
    if !realm.is_null() {
        let fresh2 = count;
        count = count + 1;
        edit_av[fresh2 as usize] =
            b"-r\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        let fresh3 = count;
        count = count + 1;
        edit_av[fresh3 as usize] = realm
    }
    let fresh4 = count;
    count = count + 1;
    edit_av[fresh4 as usize] =
        b"load\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if !kerb_database.is_null() {
        let fresh5 = count;
        count = count + 1;
        edit_av[fresh5 as usize] =
            b"-d\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        let fresh6 = count;
        count = count + 1;
        edit_av[fresh6 as usize] = kerb_database
    }
    if !log_ctx.is_null() &&
           (*log_ctx).iproprole as libc::c_uint ==
               IPROP_REPLICA as libc::c_int as libc::c_uint {
        let fresh7 = count;
        count = count + 1;
        edit_av[fresh7 as usize] =
            b"-i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    let fresh8 = count;
    count = count + 1;
    edit_av[fresh8 as usize] = database_file_name;
    let fresh9 = count;
    count = count + 1;
    edit_av[fresh9 as usize] = 0 as *mut libc::c_char;
    child_pid = fork();
    match child_pid {
        -1 => {
            com_err(progname, *__errno_location() as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while trying to fork %s\x00" as *const u8 as
                                 *const libc::c_char), kdb_util);
            exit(1 as libc::c_int);
        }
        0 => {
            execv(kdb_util, edit_av.as_mut_ptr() as *const *mut libc::c_char);
            com_err(progname, *__errno_location() as errcode_t,
                    dgettext(b"mit-krb5\x00" as *const u8 as
                                 *const libc::c_char,
                             b"while trying to exec %s\x00" as *const u8 as
                                 *const libc::c_char), kdb_util);
            _exit(1 as libc::c_int);
        }
        _ => {
            /*NOTREACHED*/
            if debug != 0 {
                fprintf(stderr,
                        b"Load PID is %d\n\x00" as *const u8 as
                            *const libc::c_char, child_pid);
            }
            if wait(&mut waitb) < 0 as libc::c_int {
                com_err(progname, *__errno_location() as errcode_t,
                        dgettext(b"mit-krb5\x00" as *const u8 as
                                     *const libc::c_char,
                                 b"while waiting for %s\x00" as *const u8 as
                                     *const libc::c_char), kdb_util);
                exit(1 as libc::c_int);
            }
        }
    }
    if !(waitb & 0x7f as libc::c_int == 0 as libc::c_int) {
        com_err(progname, 0 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"%s load terminated\x00" as *const u8 as
                             *const libc::c_char), kdb_util);
        exit(1 as libc::c_int);
    }
    error_ret = (waitb & 0xff00 as libc::c_int) >> 8 as libc::c_int;
    if error_ret != 0 {
        com_err(progname, 0 as libc::c_int as errcode_t,
                dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                         b"%s returned a bad exit status (%d)\x00" as
                             *const u8 as *const libc::c_char), kdb_util,
                error_ret);
        exit(1 as libc::c_int);
    };
}
/*
 * Get the host base service name for the kiprop principal. Returns
 * KADM5_OK on success. Caller must free the storage allocated
 * for host_service_name.
 */
#[c2rust::src_loc = "1596:1"]
unsafe extern "C" fn kadm5_get_kiprop_host_srv_name(mut context: krb5_context,
                                                    mut realm_name:
                                                        *const libc::c_char,
                                                    mut host_service_name:
                                                        *mut *mut libc::c_char)
 -> kadm5_ret_t {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char; /* XXX */
    let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
    host = params.admin_server;
    if asprintf(&mut name as *mut *mut libc::c_char,
                b"%s/%s\x00" as *const u8 as *const libc::c_char,
                b"kiprop\x00" as *const u8 as *const libc::c_char, host) <
           0 as libc::c_int {
        free(host as *mut libc::c_void);
        return 12 as libc::c_int as kadm5_ret_t
    }
    *host_service_name = name;
    return 0 as libc::c_int as kadm5_ret_t;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
