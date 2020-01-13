use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:7"]
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
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:7"]
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
#[c2rust::header_src = "/usr/include/bits/types/time_t.h:7"]
pub mod time_t_h {
    #[c2rust::src_loc = "7:1"]
    pub type time_t = __time_t;
    use super::types_h::__time_t;
}
#[c2rust::header_src = "/usr/include/sys/stat.h:7"]
pub mod sys_stat_h {
    #[c2rust::src_loc = "70:1"]
    pub type off_t = __off_t;
    use super::types_h::__off_t;
    use super::stat_h::stat;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "205:1"]
        pub fn stat(__file: *const libc::c_char, __buf: *mut stat)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/stat.h:7"]
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
#[c2rust::header_src = "/usr/include/sys/types.h:8"]
pub mod sys_types_h {
    #[c2rust::src_loc = "35:1"]
    pub type u_int = __u_int;
    #[c2rust::src_loc = "108:1"]
    pub type ssize_t = __ssize_t;
    #[c2rust::src_loc = "115:1"]
    pub type caddr_t = __caddr_t;
    use super::types_h::{__u_int, __ssize_t, __caddr_t};
}
#[c2rust::header_src = "/usr/lib/clang/9.0.1/include/stddef.h:8"]
pub mod stddef_h {
    #[c2rust::src_loc = "46:1"]
    pub type size_t = libc::c_ulong;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:8"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    use super::types_h::{__int16_t, __int32_t};
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timeval.h:8"]
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
#[c2rust::header_src = "/usr/include/bits/confname.h:9"]
pub mod confname_h {
    #[c2rust::src_loc = "71:1"]
    pub type C2RustUnnamed = libc::c_uint;
    #[c2rust::src_loc = "528:5"]
    pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
    #[c2rust::src_loc = "526:5"]
    pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
    #[c2rust::src_loc = "523:5"]
    pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
    #[c2rust::src_loc = "520:5"]
    pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
    #[c2rust::src_loc = "518:5"]
    pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
    #[c2rust::src_loc = "516:5"]
    pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
    #[c2rust::src_loc = "514:5"]
    pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
    #[c2rust::src_loc = "511:5"]
    pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
    #[c2rust::src_loc = "508:5"]
    pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
    #[c2rust::src_loc = "506:5"]
    pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
    #[c2rust::src_loc = "504:5"]
    pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
    #[c2rust::src_loc = "502:5"]
    pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
    #[c2rust::src_loc = "499:5"]
    pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
    #[c2rust::src_loc = "497:5"]
    pub const _SC_IPV6: C2RustUnnamed = 235;
    #[c2rust::src_loc = "493:5"]
    pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
    #[c2rust::src_loc = "491:5"]
    pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
    #[c2rust::src_loc = "489:5"]
    pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
    #[c2rust::src_loc = "487:5"]
    pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
    #[c2rust::src_loc = "485:5"]
    pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
    #[c2rust::src_loc = "483:5"]
    pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
    #[c2rust::src_loc = "481:5"]
    pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
    #[c2rust::src_loc = "479:5"]
    pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
    #[c2rust::src_loc = "477:5"]
    pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
    #[c2rust::src_loc = "475:5"]
    pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
    #[c2rust::src_loc = "473:5"]
    pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
    #[c2rust::src_loc = "471:5"]
    pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
    #[c2rust::src_loc = "469:5"]
    pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
    #[c2rust::src_loc = "467:5"]
    pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
    #[c2rust::src_loc = "465:5"]
    pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
    #[c2rust::src_loc = "462:5"]
    pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
    #[c2rust::src_loc = "460:5"]
    pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
    #[c2rust::src_loc = "458:5"]
    pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
    #[c2rust::src_loc = "456:5"]
    pub const _SC_TRACE: C2RustUnnamed = 181;
    #[c2rust::src_loc = "454:5"]
    pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
    #[c2rust::src_loc = "451:5"]
    pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
    #[c2rust::src_loc = "449:5"]
    pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
    #[c2rust::src_loc = "447:5"]
    pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
    #[c2rust::src_loc = "445:5"]
    pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
    #[c2rust::src_loc = "442:5"]
    pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
    #[c2rust::src_loc = "440:5"]
    pub const _SC_STREAMS: C2RustUnnamed = 174;
    #[c2rust::src_loc = "438:5"]
    pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
    #[c2rust::src_loc = "436:5"]
    pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
    #[c2rust::src_loc = "434:5"]
    pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
    #[c2rust::src_loc = "432:5"]
    pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
    #[c2rust::src_loc = "430:5"]
    pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
    #[c2rust::src_loc = "428:5"]
    pub const _SC_2_PBS: C2RustUnnamed = 168;
    #[c2rust::src_loc = "426:5"]
    pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
    #[c2rust::src_loc = "424:5"]
    pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
    #[c2rust::src_loc = "422:5"]
    pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
    #[c2rust::src_loc = "420:5"]
    pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
    #[c2rust::src_loc = "418:5"]
    pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
    #[c2rust::src_loc = "416:5"]
    pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
    #[c2rust::src_loc = "414:5"]
    pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
    #[c2rust::src_loc = "412:5"]
    pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
    #[c2rust::src_loc = "410:5"]
    pub const _SC_SPAWN: C2RustUnnamed = 159;
    #[c2rust::src_loc = "408:5"]
    pub const _SC_SIGNALS: C2RustUnnamed = 158;
    #[c2rust::src_loc = "406:5"]
    pub const _SC_SHELL: C2RustUnnamed = 157;
    #[c2rust::src_loc = "404:5"]
    pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
    #[c2rust::src_loc = "402:5"]
    pub const _SC_REGEXP: C2RustUnnamed = 155;
    #[c2rust::src_loc = "400:5"]
    pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
    #[c2rust::src_loc = "398:5"]
    pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
    #[c2rust::src_loc = "396:5"]
    pub const _SC_NETWORKING: C2RustUnnamed = 152;
    #[c2rust::src_loc = "394:5"]
    pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
    #[c2rust::src_loc = "392:5"]
    pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
    #[c2rust::src_loc = "390:5"]
    pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
    #[c2rust::src_loc = "388:5"]
    pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
    #[c2rust::src_loc = "386:5"]
    pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
    #[c2rust::src_loc = "384:5"]
    pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
    #[c2rust::src_loc = "382:5"]
    pub const _SC_PIPE: C2RustUnnamed = 145;
    #[c2rust::src_loc = "380:5"]
    pub const _SC_FIFO: C2RustUnnamed = 144;
    #[c2rust::src_loc = "378:5"]
    pub const _SC_FD_MGMT: C2RustUnnamed = 143;
    #[c2rust::src_loc = "376:5"]
    pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
    #[c2rust::src_loc = "374:5"]
    pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
    #[c2rust::src_loc = "372:5"]
    pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
    #[c2rust::src_loc = "370:5"]
    pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
    #[c2rust::src_loc = "368:5"]
    pub const _SC_CPUTIME: C2RustUnnamed = 138;
    #[c2rust::src_loc = "366:5"]
    pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
    #[c2rust::src_loc = "364:5"]
    pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
    #[c2rust::src_loc = "362:5"]
    pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
    #[c2rust::src_loc = "360:5"]
    pub const _SC_BASE: C2RustUnnamed = 134;
    #[c2rust::src_loc = "358:5"]
    pub const _SC_BARRIERS: C2RustUnnamed = 133;
    #[c2rust::src_loc = "356:5"]
    pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
    #[c2rust::src_loc = "353:5"]
    pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
    #[c2rust::src_loc = "351:5"]
    pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
    #[c2rust::src_loc = "349:5"]
    pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
    #[c2rust::src_loc = "346:5"]
    pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
    #[c2rust::src_loc = "344:5"]
    pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
    #[c2rust::src_loc = "342:5"]
    pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
    #[c2rust::src_loc = "340:5"]
    pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
    #[c2rust::src_loc = "337:5"]
    pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
    #[c2rust::src_loc = "335:5"]
    pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
    #[c2rust::src_loc = "333:5"]
    pub const _SC_NL_NMAX: C2RustUnnamed = 122;
    #[c2rust::src_loc = "331:5"]
    pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
    #[c2rust::src_loc = "329:5"]
    pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
    #[c2rust::src_loc = "327:5"]
    pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
    #[c2rust::src_loc = "324:5"]
    pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
    #[c2rust::src_loc = "322:5"]
    pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
    #[c2rust::src_loc = "320:5"]
    pub const _SC_UINT_MAX: C2RustUnnamed = 116;
    #[c2rust::src_loc = "318:5"]
    pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
    #[c2rust::src_loc = "316:5"]
    pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
    #[c2rust::src_loc = "314:5"]
    pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
    #[c2rust::src_loc = "312:5"]
    pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
    #[c2rust::src_loc = "310:5"]
    pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
    #[c2rust::src_loc = "308:5"]
    pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
    #[c2rust::src_loc = "306:5"]
    pub const _SC_NZERO: C2RustUnnamed = 109;
    #[c2rust::src_loc = "304:5"]
    pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
    #[c2rust::src_loc = "302:5"]
    pub const _SC_WORD_BIT: C2RustUnnamed = 107;
    #[c2rust::src_loc = "300:5"]
    pub const _SC_LONG_BIT: C2RustUnnamed = 106;
    #[c2rust::src_loc = "298:5"]
    pub const _SC_INT_MIN: C2RustUnnamed = 105;
    #[c2rust::src_loc = "296:5"]
    pub const _SC_INT_MAX: C2RustUnnamed = 104;
    #[c2rust::src_loc = "294:5"]
    pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
    #[c2rust::src_loc = "292:5"]
    pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
    #[c2rust::src_loc = "290:5"]
    pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
    #[c2rust::src_loc = "287:5"]
    pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
    #[c2rust::src_loc = "285:5"]
    pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
    #[c2rust::src_loc = "283:5"]
    pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
    #[c2rust::src_loc = "280:5"]
    pub const _SC_2_UPE: C2RustUnnamed = 97;
    #[c2rust::src_loc = "278:5"]
    pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
    #[c2rust::src_loc = "276:5"]
    pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
    #[c2rust::src_loc = "273:5"]
    pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
    #[c2rust::src_loc = "271:5"]
    pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
    #[c2rust::src_loc = "269:5"]
    pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
    #[c2rust::src_loc = "267:5"]
    pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
    #[c2rust::src_loc = "265:5"]
    pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
    #[c2rust::src_loc = "263:5"]
    pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
    #[c2rust::src_loc = "260:5"]
    pub const _SC_PASS_MAX: C2RustUnnamed = 88;
    #[c2rust::src_loc = "258:5"]
    pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
    #[c2rust::src_loc = "256:5"]
    pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
    #[c2rust::src_loc = "254:5"]
    pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
    #[c2rust::src_loc = "252:5"]
    pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
    #[c2rust::src_loc = "250:5"]
    pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
    #[c2rust::src_loc = "247:5"]
    pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
    #[c2rust::src_loc = "245:5"]
    pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
    #[c2rust::src_loc = "243:5"]
    pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
    #[c2rust::src_loc = "241:5"]
    pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
    #[c2rust::src_loc = "239:5"]
    pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
    #[c2rust::src_loc = "237:5"]
    pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
    #[c2rust::src_loc = "235:5"]
    pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
    #[c2rust::src_loc = "233:5"]
    pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
    #[c2rust::src_loc = "231:5"]
    pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
    #[c2rust::src_loc = "229:5"]
    pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
    #[c2rust::src_loc = "227:5"]
    pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
    #[c2rust::src_loc = "225:5"]
    pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
    #[c2rust::src_loc = "223:5"]
    pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
    #[c2rust::src_loc = "221:5"]
    pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
    #[c2rust::src_loc = "219:5"]
    pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
    #[c2rust::src_loc = "217:5"]
    pub const _SC_THREADS: C2RustUnnamed = 67;
    #[c2rust::src_loc = "213:5"]
    pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
    #[c2rust::src_loc = "211:5"]
    pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
    #[c2rust::src_loc = "209:5"]
    pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
    #[c2rust::src_loc = "207:5"]
    pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
    #[c2rust::src_loc = "205:5"]
    pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
    #[c2rust::src_loc = "203:5"]
    pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
    #[c2rust::src_loc = "201:5"]
    pub const _SC_IOV_MAX: C2RustUnnamed = 60;
    #[c2rust::src_loc = "199:5"]
    pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
    #[c2rust::src_loc = "197:5"]
    pub const _SC_SELECT: C2RustUnnamed = 59;
    #[c2rust::src_loc = "195:5"]
    pub const _SC_POLL: C2RustUnnamed = 58;
    #[c2rust::src_loc = "193:5"]
    pub const _SC_PII_OSI: C2RustUnnamed = 57;
    #[c2rust::src_loc = "191:5"]
    pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
    #[c2rust::src_loc = "189:5"]
    pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
    #[c2rust::src_loc = "187:5"]
    pub const _SC_PII_XTI: C2RustUnnamed = 54;
    #[c2rust::src_loc = "185:5"]
    pub const _SC_PII: C2RustUnnamed = 53;
    #[c2rust::src_loc = "182:5"]
    pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
    #[c2rust::src_loc = "180:5"]
    pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
    #[c2rust::src_loc = "178:5"]
    pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
    #[c2rust::src_loc = "176:5"]
    pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
    #[c2rust::src_loc = "174:5"]
    pub const _SC_2_C_DEV: C2RustUnnamed = 48;
    #[c2rust::src_loc = "172:5"]
    pub const _SC_2_C_BIND: C2RustUnnamed = 47;
    #[c2rust::src_loc = "170:5"]
    pub const _SC_2_VERSION: C2RustUnnamed = 46;
    #[c2rust::src_loc = "167:5"]
    pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
    #[c2rust::src_loc = "165:5"]
    pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
    #[c2rust::src_loc = "163:5"]
    pub const _SC_LINE_MAX: C2RustUnnamed = 43;
    #[c2rust::src_loc = "161:5"]
    pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
    #[c2rust::src_loc = "159:5"]
    pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
    #[c2rust::src_loc = "157:5"]
    pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
    #[c2rust::src_loc = "155:5"]
    pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
    #[c2rust::src_loc = "153:5"]
    pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
    #[c2rust::src_loc = "151:5"]
    pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
    #[c2rust::src_loc = "149:5"]
    pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
    #[c2rust::src_loc = "144:5"]
    pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
    #[c2rust::src_loc = "142:5"]
    pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
    #[c2rust::src_loc = "140:5"]
    pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
    #[c2rust::src_loc = "138:5"]
    pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
    #[c2rust::src_loc = "136:5"]
    pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
    #[c2rust::src_loc = "133:5"]
    pub const _SC_PAGESIZE: C2RustUnnamed = 30;
    #[c2rust::src_loc = "131:5"]
    pub const _SC_VERSION: C2RustUnnamed = 29;
    #[c2rust::src_loc = "129:5"]
    pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
    #[c2rust::src_loc = "127:5"]
    pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
    #[c2rust::src_loc = "125:5"]
    pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
    #[c2rust::src_loc = "123:5"]
    pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
    #[c2rust::src_loc = "121:5"]
    pub const _SC_AIO_MAX: C2RustUnnamed = 24;
    #[c2rust::src_loc = "119:5"]
    pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
    #[c2rust::src_loc = "117:5"]
    pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
    #[c2rust::src_loc = "115:5"]
    pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
    #[c2rust::src_loc = "113:5"]
    pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
    #[c2rust::src_loc = "111:5"]
    pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
    #[c2rust::src_loc = "109:5"]
    pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
    #[c2rust::src_loc = "107:5"]
    pub const _SC_MEMLOCK: C2RustUnnamed = 17;
    #[c2rust::src_loc = "105:5"]
    pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
    #[c2rust::src_loc = "103:5"]
    pub const _SC_FSYNC: C2RustUnnamed = 15;
    #[c2rust::src_loc = "101:5"]
    pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
    #[c2rust::src_loc = "99:5"]
    pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
    #[c2rust::src_loc = "97:5"]
    pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
    #[c2rust::src_loc = "95:5"]
    pub const _SC_TIMERS: C2RustUnnamed = 11;
    #[c2rust::src_loc = "93:5"]
    pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
    #[c2rust::src_loc = "91:5"]
    pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
    #[c2rust::src_loc = "89:5"]
    pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
    #[c2rust::src_loc = "87:5"]
    pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
    #[c2rust::src_loc = "85:5"]
    pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
    #[c2rust::src_loc = "83:5"]
    pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
    #[c2rust::src_loc = "81:5"]
    pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
    #[c2rust::src_loc = "79:5"]
    pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
    #[c2rust::src_loc = "77:5"]
    pub const _SC_CLK_TCK: C2RustUnnamed = 2;
    #[c2rust::src_loc = "75:5"]
    pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
    #[c2rust::src_loc = "73:5"]
    pub const _SC_ARG_MAX: C2RustUnnamed = 0;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:12"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/krb5/krb5.h:12"]
pub mod krb5_h {
    /* typedef struct _profile_t *profile_t; */
    /*
 * begin wordsize.h
 */
    /*
 * Word-size related definition.
 */
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
    #[c2rust::src_loc = "175:1"]
    pub type krb5_msgtype = libc::c_uint;
    #[c2rust::src_loc = "176:1"]
    pub type krb5_kvno = libc::c_uint;
    #[c2rust::src_loc = "178:1"]
    pub type krb5_addrtype = krb5_int32;
    #[c2rust::src_loc = "179:1"]
    pub type krb5_enctype = krb5_int32;
    #[c2rust::src_loc = "181:1"]
    pub type krb5_authdatatype = krb5_int32;
    #[c2rust::src_loc = "185:1"]
    pub type krb5_preauthtype = krb5_int32;
    /* This may change, later on */
    #[c2rust::src_loc = "186:1"]
    pub type krb5_flags = krb5_int32;
    /* *
 * Represents a timestamp in seconds since the POSIX epoch.  This legacy type
 * is used frequently in the ABI, but cannot represent timestamps after 2038 as
 * a positive number.  Code which uses this type should cast values of it to
 * uint32_t so that negative values are treated as timestamps between 2038 and
 * 2106 on platforms with 64-bit time_t.
 */
    #[c2rust::src_loc = "195:1"]
    pub type krb5_timestamp = krb5_int32;
    #[c2rust::src_loc = "197:1"]
    pub type krb5_deltat = krb5_int32;
    /* *
 * Used to convey an operation status.  The value 0 indicates success; any
 * other values are com_err codes.  Use krb5_get_error_message() to obtain a
 * string describing the error.
 */
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
    /* Originally used to recognize AFS and default salts.  No longer used. */
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
    /*
 * Per V5 spec on definition of principal types
 */
    /* *<  Name type not known */
    /* *< Just the name of the principal
                                      as in DCE, or for users */
    /* *< Service and other unique instance (krbtgt) */
    /* *< Service with host name as instance
                                      (telnet, rcommands) */
    /* *< Service with host as remaining components */
    /* *< Unique ID */
    /* *< PKINIT */
    /* *< Name in form of SMTP email name */
    /* *< Windows 2000 UPN */
    /* *< Well-known (special) principal */
    /* *< First component of
                                                NT_WELLKNOWN principals */
    /* *< Windows 2000 UPN and SID */
    /* *< NT 4 style name */
    /* *< NT 4 style name and SID */
    /* * Constant version of krb5_principal_data */
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
    /* *< Anonymous realm */
    /* *< Anonymous principal name */
    /*
 * end "base-defs.h"
 */
    /*
 * begin "hostaddr.h"
 */
    /* * Structure for address */
    #[c2rust::src_loc = "323:1"]
    pub type krb5_address = _krb5_address;
    /* *
 * Specify a keytab to use for acquiring initial credentials.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 * @param [in] keytab           Key table handle
 *
 * This function supplies a keytab containing the client key for an initial
 * credentials request.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Get the next KDC request for acquiring initial credentials.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [in]  in              KDC response (empty on the first call)
 * @param [out] out             Next KDC request
 * @param [out] realm           Realm for next KDC request
 * @param [out] flags           Output flags
 *
 * This function constructs the next KDC request in an initial credential
 * exchange, allowing the caller to control the transport of KDC requests and
 * replies.  On the first call, @a in should be set to an empty buffer; on
 * subsequent calls, it should be set to the KDC's reply to the previous
 * request.
 *
 * If more requests are needed, @a flags will be set to
 * #KRB5_INIT_CREDS_STEP_FLAG_CONTINUE and the next request will be placed in
 * @a out.  If no more requests are needed, @a flags will not contain
 * #KRB5_INIT_CREDS_STEP_FLAG_CONTINUE and @a out will be empty.
 *
 * If this function returns @c KRB5KRB_ERR_RESPONSE_TOO_BIG, the caller should
 * transmit the next request using TCP rather than UDP.  If this function
 * returns any other error, the initial credential exchange has failed.
 *
 * @a context must be the same as the one passed to krb5_init_creds_init() for
 * this initial credentials context.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Set a password for acquiring initial credentials.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 * @param [in] password         Password
 *
 * This function supplies a password to be used to construct the client key for
 * an initial credentials request.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Specify a service principal for acquiring initial credentials.
 *
 * @param [in] context          Library context
 * @param [in] ctx              Initial credentials context
 * @param [in] service          Service principal string
 *
 * This function supplies a service principal string to acquire initial
 * credentials for instead of the default krbtgt service.  @a service is parsed
 * as a principal name; any realm part is ignored.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve ticket times from an initial credentials context.
 *
 * @param [in]  context         Library context
 * @param [in]  ctx             Initial credentials context
 * @param [out] times           Ticket times for acquired credentials
 *
 * The initial credentials context must have completed obtaining credentials
 * via either krb5_init_creds_get() or krb5_init_creds_step().
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Create a context to get credentials from a KDC's Ticket Granting Service.
 *
 * @param[in]  context          Library context
 * @param[in]  ccache           Credential cache handle
 * @param[in]  creds            Input credentials
 * @param[in]  options          @ref KRB5_GC options for this request.
 * @param[out] ctx              New TGS request context
 *
 * This function prepares to obtain credentials matching @a creds, either by
 * retrieving them from @a ccache or by making requests to ticket-granting
 * services beginning with a ticket-granting ticket for the client principal's
 * realm.
 *
 * The resulting TGS acquisition context can be used asynchronously with
 * krb5_tkt_creds_step() or synchronously with krb5_tkt_creds_get().  See also
 * krb5_get_credentials() for synchronous use.
 *
 * Use krb5_tkt_creds_free() to free @a ctx when it is no longer needed.
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Synchronously obtain credentials using a TGS request context.
 *
 * @param[in] context           Library context
 * @param[in] ctx               TGS request context
 *
 * This function synchronously obtains credentials using a context created by
 * krb5_tkt_creds_init().  On successful return, the credentials can be
 * retrieved with krb5_tkt_creds_get_creds().
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve acquired credentials from a TGS request context.
 *
 * @param[in]  context          Library context
 * @param[in]  ctx              TGS request context
 * @param[out] creds            Acquired credentials
 *
 * This function copies the acquired initial credentials from @a ctx into @a
 * creds, after the successful completion of krb5_tkt_creds_get() or
 * krb5_tkt_creds_step().  Use krb5_free_cred_contents() to free @a creds when
 * it is no longer needed.
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Free a TGS request context.
 *
 * @param[in]  context  Library context
 * @param[in]  ctx      TGS request context
 *
 * @version New in 1.9
 */
    /* *< More responses needed */
    /* *
 * Get the next KDC request in a TGS exchange.
 *
 * @param[in]  context          Library context
 * @param[in]  ctx              TGS request context
 * @param[in]  in               KDC response (empty on the first call)
 * @param[out] out              Next KDC request
 * @param[out] realm            Realm for next KDC request
 * @param[out] flags            Output flags
 *
 * This function constructs the next KDC request for a TGS exchange, allowing
 * the caller to control the transport of KDC requests and replies.  On the
 * first call, @a in should be set to an empty buffer; on subsequent calls, it
 * should be set to the KDC's reply to the previous request.
 *
 * If more requests are needed, @a flags will be set to
 * #KRB5_TKT_CREDS_STEP_FLAG_CONTINUE and the next request will be placed in @a
 * out.  If no more requests are needed, @a flags will not contain
 * #KRB5_TKT_CREDS_STEP_FLAG_CONTINUE and @a out will be empty.
 *
 * If this function returns @c KRB5KRB_ERR_RESPONSE_TOO_BIG, the caller should
 * transmit the next request using TCP rather than UDP.  If this function
 * returns any other error, the TGS exchange has failed.
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve ticket times from a TGS request context.
 *
 * @param[in]  context          Library context
 * @param[in]  ctx              TGS request context
 * @param[out] times            Ticket times for acquired credentials
 *
 * The TGS request context must have completed obtaining credentials via either
 * krb5_tkt_creds_get() or krb5_tkt_creds_step().
 *
 * @version New in 1.9
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Get initial credentials using a key table.
 *
 * @param [in]  context         Library context
 * @param [out] creds           New credentials
 * @param [in]  client          Client principal
 * @param [in]  arg_keytab      Key table handle
 * @param [in]  start_time      Time when ticket becomes valid (0 for now)
 * @param [in]  in_tkt_service  Service name of initial credentials (or NULL)
 * @param [in]  k5_gic_options  Initial credential options
 *
 * This function requests KDC for an initial credentials for @a client using a
 * client key stored in @a arg_keytab.  If @a in_tkt_service is specified, it
 * is parsed as a principal name (with the realm ignored) and used as the
 * service principal for the request; otherwise the ticket-granting service is
 * used.
 *
 * @sa krb5_verify_init_creds()
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
    /* *< boolean */
    /* *
 * Initialize a credential verification options structure.
 *
 * @param [in] k5_vic_options   Verification options structure
 */
    /* *
 * Set whether credential verification is required.
 *
 * @param [in] k5_vic_options   Verification options structure
 * @param [in] ap_req_nofail    Whether to require successful verification
 *
 * This function determines how krb5_verify_init_creds() behaves if no keytab
 * information is available.  If @a ap_req_nofail is @c FALSE, verification
 * will be skipped in this case and krb5_verify_init_creds() will return
 * successfully.  If @a ap_req_nofail is @c TRUE, krb5_verify_init_creds() will
 * not return successfully unless verification can be performed.
 *
 * If this function is not used, the behavior of krb5_verify_init_creds() is
 * determined through configuration.
 */
    /* *
 * Verify initial credentials against a keytab.
 *
 * @param [in] context          Library context
 * @param [in] creds            Initial credentials to be verified
 * @param [in] server           Server principal (or NULL)
 * @param [in] keytab           Key table (NULL to use default keytab)
 * @param [in] ccache           Credential cache for fetched creds (or NULL)
 * @param [in] options          Verification options (NULL for default options)
 *
 * This function attempts to verify that @a creds were obtained from a KDC with
 * knowledge of a key in @a keytab, or the default keytab if @a keytab is NULL.
 * If @a server is provided, the highest-kvno key entry for that principal name
 * is used to verify the credentials; otherwise, all unique "host" service
 * principals in the keytab are tried.
 *
 * If the specified keytab does not exist, or is empty, or cannot be read, or
 * does not contain an entry for @a server, then credential verification may be
 * skipped unless configuration demands that it succeed.  The caller can
 * control this behavior by providing a verification options structure; see
 * krb5_verify_init_creds_opt_init() and
 * krb5_verify_init_creds_opt_set_ap_req_nofail().
 *
 * If @a ccache is NULL, any additional credentials fetched during the
 * verification process will be destroyed.  If @a ccache points to NULL, a
 * memory ccache will be created for the additional credentials and returned in
 * @a ccache.  If @a ccache points to a valid credential cache handle, the
 * additional credentials will be stored in that cache.
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Get validated credentials from the KDC.
 *
 * @param [in]  context         Library context
 * @param [out] creds           Validated credentials
 * @param [in]  client          Client principal name
 * @param [in]  ccache          Credential cache
 * @param [in]  in_tkt_service  Server principal string (or NULL)
 *
 * This function gets a validated credential using a postdated credential from
 * @a ccache.  If @a in_tkt_service is specified, it is parsed (with the realm
 * part ignored) and used as the server principal of the credential;
 * otherwise, the ticket-granting service is used.
 *
 * If successful, the validated credential is placed in @a creds.
 *
 * @sa krb5_get_renewed_creds()
 *
 * @retval
 * 0 Success
 * @retval
 * KRB5_NO_2ND_TKT Request missing second ticket
 * @retval
 * KRB5_NO_TKT_SUPPLIED Request did not supply a ticket
 * @retval
 * KRB5_PRINC_NOMATCH Requested principal and ticket do not match
 * @retval
 * KRB5_KDCREP_MODIFIED KDC reply did not match expectations
 * @retval
 * KRB5_KDCREP_SKEW Clock skew too great in KDC reply
 * @return
 * Kerberos error codes
 */
    /* *
 * Get renewed credential from KDC using an existing credential.
 *
 * @param [in]  context         Library context
 * @param [out] creds           Renewed credentials
 * @param [in]  client          Client principal name
 * @param [in]  ccache          Credential cache
 * @param [in]  in_tkt_service  Server principal string (or NULL)
 *
 * This function gets a renewed credential using an existing one from @a
 * ccache.  If @a in_tkt_service is specified, it is parsed (with the realm
 * part ignored) and used as the server principal of the credential; otherwise,
 * the ticket-granting service is used.
 *
 * If successful, the renewed credential is placed in @a creds.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
    /* *
 * Decode an ASN.1-formatted ticket.
 *
 * @param [in]  code            ASN.1-formatted ticket
 * @param [out] rep             Decoded ticket information
 *
 * @retval 0  Success; otherwise - Kerberos error codes
 */
    /* *
 * Retrieve a string value from the appdefaults section of krb5.conf.
 *
 * @param [in]  context         Library context
 * @param [in]  appname         Application name
 * @param [in]  realm           Realm name
 * @param [in]  option          Option to be checked
 * @param [in]  default_value   Default value to return if no match is found
 * @param [out] ret_value       String value of @a option
 *
 * This function gets the application defaults for @a option based on the given
 * @a appname and/or @a realm.
 *
 * @sa krb5_appdefault_boolean()
 */
    /* *
 * Retrieve a boolean value from the appdefaults section of krb5.conf.
 *
 * @param [in]  context         Library context
 * @param [in]  appname         Application name
 * @param [in]  realm           Realm name
 * @param [in]  option          Option to be checked
 * @param [in]  default_value   Default value to return if no match is found
 * @param [out] ret_value       Boolean value of @a option
 *
 * This function gets the application defaults for @a option based on the given
 * @a appname and/or @a realm.
 *
 * @sa krb5_appdefault_string()
 */
    /*
 * Prompter enhancements
 */
/* * Prompt for password */
    /* * Prompt for new password (during password change) */
    /* * Prompt for new password again */
    /* * Prompt for preauthentication data (such as an OTP value) */
    /* *
 * Get prompt types array from a context.
 *
 * @param [in] context          Library context
 *
 * @return
 * Pointer to an array of prompt types corresponding to the prompter's @a
 * prompts arguments.  Each type has one of the following values:
 *  @li #KRB5_PROMPT_TYPE_PASSWORD
 *  @li #KRB5_PROMPT_TYPE_NEW_PASSWORD
 *  @li #KRB5_PROMPT_TYPE_NEW_PASSWORD_AGAIN
 *  @li #KRB5_PROMPT_TYPE_PREAUTH
 */
    /* Error reporting */
/* *
 * Set an extended error message for an error code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Error string for the error code
 * @param [in] ...              printf(3) style parameters
 */
    /* *
 * Set an extended error message for an error code using a va_list.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Error string for the error code
 * @param [in] args             List of vprintf(3) style arguments
 */
    /* *
 * Add a prefix to the message for an error code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] ...              printf(3) style parameters
 *
 * Format a message and prepend it to the current message for @a code.  The
 * prefix will be separated from the old message with a colon and space.
 */
    /* *
 * Add a prefix to the message for an error code using a va_list.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] args             List of vprintf(3) style arguments
 *
 * This function is similar to krb5_prepend_error_message(), but uses a
 * va_list instead of variadic arguments.
 */
    /* *
 * Add a prefix to a different error code's message.
 *
 * @param [in] ctx              Library context
 * @param [in] old_code         Previous error code
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] ...              printf(3) style parameters
 *
 * Format a message and prepend it to the message for @a old_code.  The prefix
 * will be separated from the old message with a colon and space.  Set the
 * resulting message as the extended error message for @a code.
 */
    /* *
 * Add a prefix to a different error code's message using a va_list.
 *
 * @param [in] ctx              Library context
 * @param [in] old_code         Previous error code
 * @param [in] code             Error code
 * @param [in] fmt              Format string for error message prefix
 * @param [in] args             List of vprintf(3) style arguments
 *
 * This function is similar to krb5_wrap_error_message(), but uses a
 * va_list instead of variadic arguments.
 */
    /* *
 * Copy the most recent extended error message from one context to another.
 *
 * @param [in] dest_ctx         Library context to copy message to
 * @param [in] src_ctx          Library context with current message
 */
    /* *
 * Get the (possibly extended) error message for a code.
 *
 * @param [in] ctx              Library context
 * @param [in] code             Error code
 *
 * The behavior of krb5_get_error_message() is only defined the first time it
 * is called after a failed call to a krb5 function using the same context, and
 * only when the error code passed in is the same as that returned by the krb5
 * function.
 *
 * This function never returns NULL, so its result may be used unconditionally
 * as a C string.
 *
 * The string returned by this function must be freed using
 * krb5_free_error_message()
 *
 * @note Future versions may return the same string for the second
 * and following calls.
 */
    /* *
 * Free an error message generated by krb5_get_error_message().
 *
 * @param [in] ctx              Library context
 * @param [in] msg              Pointer to error message
 */
    /* *
 * Clear the extended error message in a context.
 *
 * @param [in] ctx              Library context
 *
 * This function unsets the extended error message in a context, to ensure that
 * it is not mistakenly applied to another occurrence of the same error code.
 */
    /* *
 * Unwrap authorization data.
 *
 * @param [in]  context         Library context
 * @param [in]  type            @ref KRB5_AUTHDATA type of @a container
 * @param [in]  container       Authorization data to be decoded
 * @param [out] authdata        List of decoded authorization data
 *
 * @sa krb5_encode_authdata_container()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Wrap authorization data in a container.
 *
 * @param [in]  context         Library context
 * @param [in]  type            @ref KRB5_AUTHDATA type of @a container
 * @param [in]  authdata        List of authorization data to be encoded
 * @param [out] container       List of encoded authorization data
 *
 * The result is returned in @a container as a single-element list.
 *
 * @sa krb5_decode_authdata_container()
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /*
 * AD-KDCIssued
 */
/* *
 * Encode and sign AD-KDCIssued authorization data.
 *
 * @param [in]  context         Library context
 * @param [in]  key             Session key
 * @param [in]  issuer          The name of the issuing principal
 * @param [in]  authdata        List of authorization data to be signed
 * @param [out] ad_kdcissued    List containing AD-KDCIssued authdata
 *
 * This function wraps a list of authorization data entries @a authdata in an
 * AD-KDCIssued container (see RFC 4120 section 5.2.6.2) signed with @a key.
 * The result is returned in @a ad_kdcissued as a single-element list.
 */
    /* *
 * Unwrap and verify AD-KDCIssued authorization data.
 *
 * @param [in] context          Library context
 * @param [in] key              Session key
 * @param [in] ad_kdcissued     AD-KDCIssued authorization data to be unwrapped
 * @param [out] issuer          Name of issuing principal (or NULL)
 * @param [out] authdata        Unwrapped list of authorization data
 *
 * This function unwraps an AD-KDCIssued authdatum (see RFC 4120 section
 * 5.2.6.2) and verifies its signature against @a key.  The issuer field of the
 * authdatum element is returned in @a issuer, and the unwrapped list of
 * authdata is returned in @a authdata.
 */
    /*
 * Windows PAC
 */
    /* Microsoft defined types of data */
    /* *< Logon information */
    /* *< Credentials information */
    /* *< Server checksum */
    /* *< KDC checksum */
    /* *< Client name and ticket info */
    /* *< Constrained delegation info */
    /* *< User principal name and DNS info */
    /* * PAC data structure to convey authorization information */
    /* *
 * Add a buffer to a PAC handle.
 *
 * @param [in] context          Library context
 * @param [in] pac              PAC handle
 * @param [in] type             Buffer type
 * @param [in] data             contents
 *
 * This function adds a buffer of type @a type and contents @a data to @a pac
 * if there isn't already a buffer of this type present.
 *
 * The valid values of @a type is one of the following:
 * @li #KRB5_PAC_LOGON_INFO         -  Logon information
 * @li #KRB5_PAC_CREDENTIALS_INFO   -  Credentials information
 * @li #KRB5_PAC_SERVER_CHECKSUM    -  Server checksum
 * @li #KRB5_PAC_PRIVSVR_CHECKSUM   -  KDC checksum
 * @li #KRB5_PAC_CLIENT_INFO        -  Client name and ticket information
 * @li #KRB5_PAC_DELEGATION_INFO    -  Constrained delegation information
 * @li #KRB5_PAC_UPN_DNS_INFO       -  User principal name and DNS information
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Free a PAC handle.
 *
 * @param [in] context         Library context
 * @param [in] pac             PAC to be freed
 *
 * This function frees the contents of @a pac and the structure itself.
 */
    /* *
 * Retrieve a buffer value from a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  type            Type of buffer to retrieve
 * @param [out] data            Buffer value
 *
 * Use krb5_free_data_contents() to free @a data when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Return an array of buffer types in a PAC handle.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [out] len             Number of entries in @a types
 * @param [out] types           Array of buffer types
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Create an empty Privilege Attribute Certificate (PAC) handle.
 *
 * @param [in]  context         Library context
 * @param [out] pac             New PAC handle
 *
 * Use krb5_pac_free() to free @a pac when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Unparse an encoded PAC into a new handle.
 *
 * @param [in]  context         Library context
 * @param [in]  ptr             PAC buffer
 * @param [in]  len             Length of @a ptr
 * @param [out] pac             PAC handle
 *
 * Use krb5_pac_free() to free @a pac when it is no longer needed.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Verify a PAC.
 *
 * @param [in] context          Library context
 * @param [in] pac              PAC handle
 * @param [in] authtime         Expected timestamp
 * @param [in] principal        Expected principal name (or NULL)
 * @param [in] server           Key to validate server checksum (or NULL)
 * @param [in] privsvr          Key to validate KDC checksum (or NULL)
 *
 * This function validates @a pac against the supplied @a server, @a privsvr,
 * @a principal and @a authtime.  If @a principal is NULL, the principal and
 * authtime are not verified.  If @a server or @a privsvr is NULL, the
 * corresponding checksum is not verified.
 *
 * If successful, @a pac is marked as verified.
 *
 * @note A checksum mismatch can occur if the PAC was copied from a cross-realm
 * TGT by an ignorant KDC; also macOS Server Open Directory (as of 10.6)
 * generates PACs with no server checksum at all.  One should consider not
 * failing the whole authentication because of this reason, but, instead,
 * treating the ticket as if it did not contain a PAC or marking the PAC
 * information as non-verified.
 *
 * @retval 0 Success; otherwise - Kerberos error codes
 */
    /* *
 * Verify a PAC, possibly from a specified realm.
 *
 * @param [in] context          Library context
 * @param [in] pac              PAC handle
 * @param [in] authtime         Expected timestamp
 * @param [in] principal        Expected principal name (or NULL)
 * @param [in] server           Key to validate server checksum (or NULL)
 * @param [in] privsvr          Key to validate KDC checksum (or NULL)
 * @param [in] with_realm       If true, expect the realm of @a principal
 *
 * This function is similar to krb5_pac_verify(), but adds a parameter
 * @a with_realm.  If @a with_realm is true, the PAC_CLIENT_INFO field is
 * expected to include the realm of @a principal as well as the name.  This
 * flag is necessary to verify PACs in cross-realm S4U2Self referral TGTs.
 *
 * @version New in 1.17
 */
    /* *
 * Sign a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  authtime        Expected timestamp
 * @param [in]  principal       Expected principal name (or NULL)
 * @param [in]  server_key      Key for server checksum
 * @param [in]  privsvr_key     Key for KDC checksum
 * @param [out] data            Signed PAC encoding
 *
 * This function signs @a pac using the keys @a server_key and @a privsvr_key
 * and returns the signed encoding in @a data.  @a pac is modified to include
 * the server and KDC checksum buffers.  Use krb5_free_data_contents() to free
 * @a data when it is no longer needed.
 *
 * @version New in 1.10
 */
    /* *
 * Sign a PAC, possibly with a specified realm.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [in]  authtime        Expected timestamp
 * @param [in]  principal       Principal name (or NULL)
 * @param [in]  server_key      Key for server checksum
 * @param [in]  privsvr_key     Key for KDC checksum
 * @param [in]  with_realm      If true, include the realm of @a principal
 * @param [out] data            Signed PAC encoding
 *
 * This function is similar to krb5_pac_sign(), but adds a parameter
 * @a with_realm.  If @a with_realm is true, the PAC_CLIENT_INFO field of the
 * signed PAC will include the realm of @a principal as well as the name.  This
 * flag is necessary to generate PACs for cross-realm S4U2Self referrals.
 *
 * @version New in 1.17
 */
    /*
 * Read client information from a PAC.
 *
 * @param [in]  context         Library context
 * @param [in]  pac             PAC handle
 * @param [out] authtime_out    Authentication timestamp (NULL if not needed)
 * @param [out] princname_out   Client account name
 *
 * Read the PAC_CLIENT_INFO buffer in @a pac.  Place the client account name as
 * a string in @a princname_out.  If @a authtime_out is not NULL, place the
 * initial authentication timestamp in @a authtime_out.
 *
 * @retval 0 on success, ENOENT if no PAC_CLIENT_INFO buffer is present in @a
 * pac, ERANGE if the buffer contains invalid lengths.
 *
 * @version New in 1.18
 */
    /* *
 * Allow the appplication to override the profile's allow_weak_crypto setting.
 *
 * @param [in] context          Library context
 * @param [in] enable           Boolean flag
 *
 * This function allows an application to override the allow_weak_crypto
 * setting.  It is primarily for use by aklog.
 *
 * @retval 0  (always)
 */
    /* *
 * A wrapper for passing information to a @c krb5_trace_callback.
 *
 * Currently, it only contains the formatted message as determined
 * the the format string and arguments of the tracing macro, but it
 * may be extended to contain more fields in the future.
 */
    /* *
 * Specify a callback function for trace events.
 *
 * @param [in] context          Library context
 * @param [in] fn               Callback function
 * @param [in] cb_data          Callback data
 *
 * Specify a callback for trace events occurring in krb5 operations performed
 * within @a context.  @a fn will be invoked with @a context as the first
 * argument, @a cb_data as the last argument, and a pointer to a
 * krb5_trace_info as the second argument.  If the trace callback is reset via
 * this function or @a context is destroyed, @a fn will be invoked with a NULL
 * second argument so it can clean up @a cb_data.  Supply a NULL value for @a
 * fn to disable trace callbacks within @a context.
 *
 * @note This function overrides the information passed through the
 * @a KRB5_TRACE environment variable.
 *
 * @version New in 1.9
 *
 * @return Returns KRB5_TRACE_NOSUPP if tracing is not supported in the library
 * (unless @a fn is NULL).
 */
    /* *
 * Specify a file name for directing trace events.
 *
 * @param [in] context          Library context
 * @param [in] filename         File name
 *
 * Open @a filename for appending (creating it, if necessary) and set up a
 * callback to write trace events to it.
 *
 * @note This function overrides the information passed through the
 * @a KRB5_TRACE environment variable.
 *
 * @version New in 1.9
 *
 * @retval KRB5_TRACE_NOSUPP Tracing is not supported in the library.
 */
    /* *
 * Hook function for inspecting or modifying messages sent to KDCs.
 *
 * @param [in]  context         Library context
 * @param [in]  data            Callback data
 * @param [in]  realm           The realm the message will be sent to
 * @param [in]  message         The original message to be sent to the KDC
 * @param [out] new_message_out Optional replacement message to be sent
 * @param [out] reply_out       Optional synthetic reply
 *
 * If the hook function returns an error code, the KDC communication will be
 * aborted and the error code will be returned to the library operation which
 * initiated the communication.
 *
 * If the hook function sets @a reply_out, @a message will not be sent to the
 * KDC, and the given reply will used instead.
 *
 * If the hook function sets @a new_message_out, the given message will be sent
 * to the KDC in place of @a message.
 *
 * If the hook function returns successfully without setting either output,
 * @a message will be sent to the KDC normally.
 *
 * The hook function should use krb5_copy_data() to construct the value for
 * @a new_message_out or @a reply_out, to ensure that it can be freed correctly
 * by the library.
 *
 * @version New in 1.15
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
    /* *
 * Hook function for inspecting or overriding KDC replies.
 *
 * @param [in]  context         Library context
 * @param [in]  data            Callback data
 * @param [in]  code            Status of KDC communication
 * @param [in]  realm           The realm the reply was received from
 * @param [in]  message         The message sent to the realm's KDC
 * @param [in]  reply           The reply received from the KDC
 * @param [out] new_reply_out   Optional replacement reply
 *
 * If @a code is zero, @a reply contains the reply received from the KDC.  The
 * hook function may return an error code to simulate an error, may synthesize
 * a different reply by setting @a new_reply_out, or may simply return
 * successfully to do nothing.
 *
 * If @a code is non-zero, KDC communication failed and @a reply should be
 * ignored.  The hook function may return @a code or a different error code, or
 * may synthesize a reply by setting @a new_reply_out and return successfully.
 *
 * The hook function should use krb5_copy_data() to construct the value for
 * @a new_reply_out, to ensure that it can be freed correctly by the library.
 *
 * @version New in 1.15
 *
 * @retval 0 Success
 * @return A Kerberos error code
 */
    #[c2rust::src_loc = "8510:1"]
    pub type krb5_post_recv_fn
        =
        Option<unsafe extern "C" fn(_: krb5_context, _: *mut libc::c_void,
                                    _: krb5_error_code, _: *const krb5_data,
                                    _: *const krb5_data, _: *const krb5_data,
                                    _: *mut *mut krb5_data)
                   -> krb5_error_code>;
    /* per Kerberos v5 protocol spec */
    /* not yet in the spec... */
    /* macros to determine if a type is a local type */
    /*
 * end "hostaddr.h"
 */
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
    /*
 * begin "encryption.h"
 */
    /* * Exposed contents of a key. */
    #[c2rust::src_loc = "363:1"]
    pub type krb5_keyblock = _krb5_keyblock;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "363:16"]
    pub struct _krb5_keyblock {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* * Structure for auth data */
    #[c2rust::src_loc = "1946:1"]
    pub type krb5_authdata = _krb5_authdata;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1946:16"]
    pub struct _krb5_authdata {
        pub magic: krb5_magic,
        pub ad_type: krb5_authdatatype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    /* *< ADTYPE */
    /* *< Length of data  */
    /* *< Data */
    /* * C representation of KDC-REQ protocol message, including KDC-REQ-BODY */
    #[c2rust::src_loc = "2054:1"]
    pub type krb5_kdc_req = _krb5_kdc_req;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2054:16"]
    pub struct _krb5_kdc_req {
        pub magic: krb5_magic,
        pub msg_type: krb5_msgtype,
        pub padata: *mut *mut krb5_pa_data,
        pub kdc_options: krb5_flags,
        pub client: krb5_principal,
        pub server: krb5_principal,
        pub from: krb5_timestamp,
        pub till: krb5_timestamp,
        pub rtime: krb5_timestamp,
        pub nonce: krb5_int32,
        pub nktypes: libc::c_int,
        pub ktype: *mut krb5_enctype,
        pub addresses: *mut *mut krb5_address,
        pub authorization_data: krb5_enc_data,
        pub unenc_authdata: *mut *mut krb5_authdata,
        pub second_ticket: *mut *mut krb5_ticket,
    }
    /* *< KRB5_AS_REQ or KRB5_TGS_REQ */
    /* *< Preauthentication data */
    /* real body */
    /* *< Requested options */
    /* *< Client principal and realm */
    /* *< Server principal and realm */
    /* *< Requested start time */
    /* *< Requested end time */
    /* *< Requested renewable end time */
    /* *< Nonce to match request and response */
    /* *< Number of enctypes */
    /* *< Requested enctypes */
    /* *< Requested addresses (optional) */
    /* *< Encrypted authz data (optional) */
    /* *< Unencrypted authz data */
    /* *< Second ticket array (optional) */
    /* *
 * Ticket structure.
 *
 * The C representation of the ticket message, with a pointer to the
 * C representation of the encrypted part.
 */
    #[c2rust::src_loc = "1979:1"]
    pub type krb5_ticket = _krb5_ticket;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1979:16"]
    pub struct _krb5_ticket {
        pub magic: krb5_magic,
        pub server: krb5_principal,
        pub enc_part: krb5_enc_data,
        pub enc_part2: *mut krb5_enc_tkt_part,
    }
    /* cleartext portion */
    /* *< server name/realm */
    /* *< encryption type, kvno, encrypted encoding */
    /* *< ptr to decrypted version, if available */
    /* * Encrypted part of ticket. */
    #[c2rust::src_loc = "1961:1"]
    pub type krb5_enc_tkt_part = _krb5_enc_tkt_part;
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
    /* to-be-encrypted portion */
    /* *< flags */
    /* *< session key: includes enctype */
    /* *< client name/realm */
    /* *< list of transited realms */
    /* *< auth, start, end, renew_till */
    /* *< array of ptrs to addresses */
    /* *< auth data */
    /* KRB5_OLD_CRYPTO */
    /*
 * end "encryption.h"
 */
    /*
 * begin "fieldbits.h"
 */
    /* kdc_options for kdc_request */
/* options is 32 bits; each host is responsible to put the 4 bytes
   representing these bits into net order before transmission */
/* #define      KDC_OPT_RESERVED        0x80000000 */
    /* #define      KDC_OPT_UNUSED          0x01000000 */
    /* #define      KDC_OPT_UNUSED          0x00400000 */
/* #define      KDC_OPT_RESERVED        0x00200000 */
/* #define      KDC_OPT_RESERVED        0x00100000 */
/* #define      KDC_OPT_RESERVED        0x00080000 */
/* #define      KDC_OPT_RESERVED        0x00040000 */
    /* #define      KDC_OPT_RESERVED        0x00004000 */
/* #define      KDC_OPT_RESERVED        0x00002000 */
/* #define      KDC_OPT_RESERVED        0x00001000 */
/* #define      KDC_OPT_RESERVED        0x00000800 */
/* #define      KDC_OPT_RESERVED        0x00000400 */
/* #define      KDC_OPT_RESERVED        0x00000200 */
/* #define      KDC_OPT_RESERVED        0x00000100 */
/* #define      KDC_OPT_RESERVED        0x00000080 */
/* #define      KDC_OPT_RESERVED        0x00000040 */
    /* #define      KDC_OPT_UNUSED          0x00000004 */
    /*
 * Mask of ticket flags in the TGT which should be converted into KDC
 * options when using the TGT to get derivitive tickets.
 *
 *  New mask = KDC_OPT_FORWARDABLE | KDC_OPT_PROXIABLE |
 *             KDC_OPT_ALLOW_POSTDATE | KDC_OPT_RENEWABLE
 */
    /* definitions for ap_options fields */
    /* * @defgroup AP_OPTS AP_OPTS
 *
 * ap_options are 32 bits; each host is responsible to put the 4 bytes
 * representing these bits into net order before transmission
 * @{
 */
    /* *< Use session key */
    /* *< Perform a mutual
                                                 authentication exchange */
    /* *< Generate a subsession key
                                                 from the current session key
                                                 obtained from the
                                                 credentials */
    /* #define      AP_OPTS_RESERVED        0x10000000 */
/* #define      AP_OPTS_RESERVED        0x08000000 */
/* #define      AP_OPTS_RESERVED        0x04000000 */
/* #define      AP_OPTS_RESERVED        0x02000000 */
/* #define      AP_OPTS_RESERVED        0x01000000 */
/* #define      AP_OPTS_RESERVED        0x00800000 */
/* #define      AP_OPTS_RESERVED        0x00400000 */
/* #define      AP_OPTS_RESERVED        0x00200000 */
/* #define      AP_OPTS_RESERVED        0x00100000 */
/* #define      AP_OPTS_RESERVED        0x00080000 */
/* #define      AP_OPTS_RESERVED        0x00040000 */
/* #define      AP_OPTS_RESERVED        0x00020000 */
/* #define      AP_OPTS_RESERVED        0x00010000 */
/* #define      AP_OPTS_RESERVED        0x00008000 */
/* #define      AP_OPTS_RESERVED        0x00004000 */
/* #define      AP_OPTS_RESERVED        0x00002000 */
/* #define      AP_OPTS_RESERVED        0x00001000 */
/* #define      AP_OPTS_RESERVED        0x00000800 */
/* #define      AP_OPTS_RESERVED        0x00000400 */
/* #define      AP_OPTS_RESERVED        0x00000200 */
/* #define      AP_OPTS_RESERVED        0x00000100 */
/* #define      AP_OPTS_RESERVED        0x00000080 */
/* #define      AP_OPTS_RESERVED        0x00000040 */
/* #define      AP_OPTS_RESERVED        0x00000020 */
/* #define      AP_OPTS_RESERVED        0x00000010 */
/* #define      AP_OPTS_RESERVED        0x00000008 */
/* #define      AP_OPTS_RESERVED        0x00000004 */
    /* * @} */
    /* end of AP_OPTS group */
    /* definitions for ad_type fields. */
    /* Ticket flags */
/* flags are 32 bits; each host is responsible to put the 4 bytes
   representing these bits into net order before transmission */
/* #define      TKT_FLG_RESERVED        0x80000000 */
    /* #define      TKT_FLG_RESERVED        0x00004000 */
/* #define      TKT_FLG_RESERVED        0x00002000 */
/* #define      TKT_FLG_RESERVED        0x00001000 */
/* #define      TKT_FLG_RESERVED        0x00000800 */
/* #define      TKT_FLG_RESERVED        0x00000400 */
/* #define      TKT_FLG_RESERVED        0x00000200 */
/* #define      TKT_FLG_RESERVED        0x00000100 */
/* #define      TKT_FLG_RESERVED        0x00000080 */
/* #define      TKT_FLG_RESERVED        0x00000040 */
/* #define      TKT_FLG_RESERVED        0x00000020 */
/* #define      TKT_FLG_RESERVED        0x00000010 */
/* #define      TKT_FLG_RESERVED        0x00000008 */
/* #define      TKT_FLG_RESERVED        0x00000004 */
/* #define      TKT_FLG_RESERVED        0x00000002 */
/* #define      TKT_FLG_RESERVED        0x00000001 */
    /* definitions for lr_type fields. */
    /* definitions for msec direction bit for KRB_SAFE, KRB_PRIV */
    /*
 * end "fieldbits.h"
 */
    /*
 * begin "proto.h"
 */
    /* * Protocol version number */
    /* Message types */
    /* *< Initial authentication request */
    /* *< Response to AS request */
    /* *< Ticket granting server request */
    /* *< Response to TGS request */
    /* *< Auth req to application server */
    /* *< Response to mutual AP request */
    /* *< Safe application message */
    /* *< Private application message */
    /* *< Cred forwarding message */
    /* *< Error response */
    /* LastReq types */
    /* PADATA types */
    /* *< RFC 4120 */
    /* *< RFC 4120 */
    /* Not used */
    /* *< timestamp encrypted in key. RFC 4120 */
    /* *< SecurId passcode. RFC 4120 */
    /* *< Sesame project. RFC 4120 */
    /* *< OSF DCE. RFC 4120 */
    /* *< Cybersafe. RFC 4120 */
    /* *< Cygnus. RFC 4120, 3961 */
    /* *< Etype info for preauth. RFC 4120 */
    /* *< SAM/OTP */
    /* *< SAM/OTP */
    /* *< PKINIT */
    /* *< PKINIT */
    /* *< PKINIT. RFC 4556 */
    /* *< PKINIT. RFC 4556 */
    /* *< RFC 4120 */
    /* *< RFC 4120 */
    /* *< Windows 2000 referrals. RFC 6820 */
    /* *< SAM/OTP. RFC 4120 */
    /* *< Embedded in typed data. RFC 4120 */
    /* *< draft referral system */
    /* *< draft challenge system, updated */
    /* *< draft challenge system, updated */
    /* MS-KILE */
    /* *< include Windows PAC */
    /* *< username protocol transition request */
    /* *< certificate protocol transition request */
    /* *< AS checksum */
    /* *< RFC 6113 */
    /* *< RFC 6113 */
    /* *< RFC 6113 */
    /* *< RFC 6113 */
    /* *< RFC 6560 section 4.1 */
    /* *< RFC 6560 section 4.2 */
    /* *< RFC 6560 section 4.3 */
    /* *< RFC 6112 */
    /* *< RFC 6806 */
    /* *< RFC 8070 */
    /* *< MS-KILE and MS-SFU */
    /* *< currently must be zero */
    /* * Transited encoding types */
    /* * alternate authentication types */
    /* authorization data types. See RFC 4120 section 5.2.6 */
    /* * @defgroup KRB5_AUTHDATA KRB5_AUTHDATA
 * @{
 */
    /* *< RFC 4537 */
    /* *< formerly 142 in krb5 1.8 */
    /* * @} */
    /* end of KRB5_AUTHDATA group */
    /* password change constants */
    /* *< Success */
    /* *< Malformed request */
    /* *< Server error */
    /* *< Authentication error */
    /* *< Password change rejected */
    /* These are Microsoft's extensions in RFC 3244, and it looks like
   they'll become standardized, possibly with other additions.  */
    /* *< Not authorized */
    /* *< Unknown RPC version */
    /* * The presented credentials were not obtained using a password directly */
    /*
 * end "proto.h"
 */
    /* Time set */
/* * Ticket start time, end time, and renewal duration. */
    #[c2rust::src_loc = "1936:1"]
    pub type krb5_ticket_times = _krb5_ticket_times;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1936:16"]
    pub struct _krb5_ticket_times {
        pub authtime: krb5_timestamp,
        pub starttime: krb5_timestamp,
        pub endtime: krb5_timestamp,
        pub renew_till: krb5_timestamp,
    }
    /* *< Time at which KDC issued the initial ticket that corresponds to this ticket */
    /* XXX ? should ktime in KDC_REP == authtime
       in ticket? otherwise client can't get this */
    /* *< optional in ticket, if not present, use @a authtime */
    /* *< Ticket expiration time */
    /* *< Latest time at which renewal of ticket can be valid */
    /* * Structure for transited encoding */
    #[c2rust::src_loc = "1954:1"]
    pub type krb5_transited = _krb5_transited;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1954:16"]
    pub struct _krb5_transited {
        pub magic: krb5_magic,
        pub tr_type: krb5_octet,
        pub tr_contents: krb5_data,
    }
    #[c2rust::src_loc = "398:1"]
    pub type krb5_enc_data = _krb5_enc_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "398:16"]
    pub struct _krb5_enc_data {
        pub magic: krb5_magic,
        pub enctype: krb5_enctype,
        pub kvno: krb5_kvno,
        pub ciphertext: krb5_data,
    }
    /* *< Transited encoding type */
    /* *< Contents */
    /* * Pre-authentication data */
    #[c2rust::src_loc = "2038:1"]
    pub type krb5_pa_data = _krb5_pa_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2038:16"]
    pub struct _krb5_pa_data {
        pub magic: krb5_magic,
        pub pa_type: krb5_preauthtype,
        pub length: libc::c_uint,
        pub contents: *mut krb5_octet,
    }
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::k5_int_h::_krb5_context;
    extern "C" {
        /* *< Preauthentication data type */
        /* *< Length of data */
        /* *< Data */
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
        /* !KRB5_CONFIG__ */
        /* for *_MAX */
        /* from profile.h */
        #[c2rust::src_loc = "125:8"]
        pub type _profile_t;
        /* *
 * Convert a string principal name to a krb5_principal structure.
 *
 * @param [in]  context         Library context
 * @param [in]  name            String representation of a principal name
 * @param [out] principal_out   New principal
 *
 * Convert a string representation of a principal name to a krb5_principal
 * structure.
 *
 * A string representation of a Kerberos name consists of one or more principal
 * name components, separated by slashes, optionally followed by the \@
 * character and a realm name.  If the realm name is not specified, the local
 * realm is used.
 *
 * To use the slash and \@ symbols as part of a component (quoted) instead of
 * using them as a component separator or as a realm prefix), put a backslash
 * (\) character in front of the symbol.  Similarly, newline, tab, backspace,
 * and NULL characters can be included in a component by using @c n, @c t, @c b
 * or @c 0, respectively.
 *
 * @note The realm in a Kerberos @a name cannot contain slash, colon,
 * or NULL characters.
 *
 * Use krb5_free_principal() to free @a principal_out when it is no longer
 * needed.
 *
 * @retval
 * 0 Success
 * @return
 * Kerberos error codes
 */
        #[no_mangle]
        #[c2rust::src_loc = "3427:1"]
        pub fn krb5_parse_name(context: krb5_context,
                               name: *const libc::c_char,
                               principal_out: *mut krb5_principal)
         -> krb5_error_code;
        /* krb5_free.c */
/* *
 * Free the storage assigned to a principal.
 *
 * @param [in] context          Library context
 * @param [in] val              Principal to be freed
 */
        #[no_mangle]
        #[c2rust::src_loc = "4596:1"]
        pub fn krb5_free_principal(context: krb5_context,
                                   val: krb5_principal);
    }
    /* KRB5_KRB5_H_INCLUDED */
    /*@modifies internalState@*/
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-int.h:12"]
pub mod k5_int_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright (C) 1989,1990,1991,1992,1993,1994,1995,2000,2001,
 * 2003,2006,2007,2008,2009 by the Massachusetts Institute of Technology,
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
    /*
 * This prototype for k5-int.h (Krb5 internals include file)
 * includes the user-visible definitions from krb5.h and then
 * includes other definitions that are not user-visible but are
 * required for compiling Kerberos internal routines.
 *
 * John Gilmore, Cygnus Support, Sat Jan 21 22:45:52 PST 1995
 */
    /* KRB5_GENERAL__ */
    /*
 * Begin "k5-config.h"
 */
    /*
 * Machine-type definitions: PC Clone 386 running Microloss Windows
 */
    /* From autoconf.h */
    /* HAVE_SYS_TYPES_H */
    /* HAVE_SYS_TYPES_H */
    /* KRB5_SYSTYPES__ */
    /* one day */
    /* one week */
    /* Thu Jan  1 00:00:00 2038 UTC */
    /*
 * Windows requires a different api interface to each function. Here
 * just define it as NULL.
 */
    /* #define KRB5_OLD_CRYPTO is done in krb5.h */
    /* KRB5_CONFIG__ */
    /*
 * End "k5-config.h"
 */
    /*
 * After loading the configuration definitions, load the Kerberos definitions.
 */
    /* Get mutex support; currently used only for the replay cache.  */
    /* Get error info support.  */
    /* Get string buffer support. */
    /* Define tracing macros. */
    /* Profile variables.  Constants are named KRB5_CONF_STRING, where STRING
 * matches the variable name.  Keep these alphabetized. */
    /* Cache configuration variables */
    /* Error codes used in KRB_ERROR protocol messages.
   Return values of library routines are based on a different error table
   (which allows non-ambiguous error codes between subsystems) */
    /* KDC errors */
    /* No error */
    /* Client's entry in DB expired */
    /* Server's entry in DB expired */
    /* Requested pvno not supported */
    /* C's key encrypted in old master */
    /* S's key encrypted in old master */
    /* Client not found in Kerberos DB */
    /* Server not found in Kerberos DB */
    /* Multiple entries in Kerberos DB */
    /* The C or S has a null key */
    /* Tkt ineligible for postdating */
    /* Requested starttime > endtime */
    /* KDC policy rejects request */
    /* KDC can't do requested opt. */
    /* No support for encryption type */
    /* No support for checksum type */
    /* No support for padata type */
    /* No support for transited type */
    /* C's creds have been revoked */
    /* S's creds have been revoked */
    /* TGT has been revoked */
    /* C not yet valid */
    /* S not yet valid */
    /* Password has expired */
    /* Preauthentication failed */
    /* Additional preauthentication */
                                           /* required */
    /* Requested server and */
                                           /* ticket don't match*/
    /* Server principal valid for */
                                           /*   user2user only */
    /* KDC policy rejected transited */
                                           /*   path */
    /* A service is not
                                            * available that is
                                            * required to process the
                                            * request */
    /* Application errors */
    /* Decrypt integrity check failed */
    /* Ticket expired */
    /* Ticket not yet valid */
    /* Request is a replay */
    /* The ticket isn't for us */
    /* Ticket/authenticator don't match */
    /* Clock skew too great */
    /* Incorrect net address */
    /* Protocol version mismatch */
    /* Invalid message type */
    /* Message stream modified */
    /* Message out of order */
    /* Key version is not available */
    /* Service key not available */
    /* Mutual authentication failed */
    /* Incorrect message direction */
    /* Alternative authentication */
                                        /* method required */
    /* Incorrect sequence numnber */
                                        /* in message */
    /* Inappropriate type of */
                                        /* checksum in message */
    /* Policy rejects transited path */
    /* Response too big for UDP, */
                                        /*   retry with TCP */
    /* other errors */
    /* Generic error (description */
                                        /* in e-text) */
    /* Field is too long for impl. */
    /* PKINIT server-reported errors */
    /* client cert not trusted */
    /* client signature verify failed */
    /* invalid Diffie-Hellman parameters */
    /* client cert not verifiable to */
                                                   /* trusted root cert */
    /* client cert had invalid signature */
    /* client cert was revoked */
    /* client cert revoked, reason unknown */
    /* mismatch between client cert and */
                                                   /* principal name */
    /* bad extended key use */
    /* bad digest algorithm in client cert */
    /* missing paChecksum in PA-PK-AS-REQ */
    /* bad digest algorithm in SignedData */
    /* The IAKERB proxy could
                                                      not find a KDC */
    /* The KDC did not respond
                                                      to the IAKERB proxy */
    /* RFC 6113 */
    /* RFC 6113 */
    /* err table base max offset for protocol err codes */
    /*
 * A null-terminated array of this structure is returned by the KDC as
 * the data part of the ETYPE_INFO preauth type.  It informs the
 * client which encryption types are supported.
 * The  same data structure is used by both etype-info and etype-info2
 * but s2kparams must be null when encoding etype-info.
 */
    /*
 *  This is essentially -1 without sign extension which can screw up
 *  comparisons on 64 bit machines. If the length is this value, then
 *  the salt data is not present. This is to distinguish between not
 *  being set and being of 0 length.
 */
    /* RFC 4537 */
    /* sam_type values -- informational only */
    /*  Enigma Logic */
    /*  Digital Pathways */
    /*  S/key where  KDC has key 0 */
    /*  Traditional S/Key */
    /*  Security Dynamics */
    /*  CRYPTOCard */
    /* XXX need to figure out who has which numbers assigned */
    /*  ActivCard decimal mode */
    /*  ActivCard hex mode */
    /*  Digital Pathways hex mode */
    /* experimental */
    /* testing */
    /* special */
    /* Array of checksums */
    /* information */
    /* KRB5_SAM_* values */
    /* informational */
    /* KRB5_SAM_* values */
    /* copied */
    /* krb5_enc_sam_response_enc */
    /*
 * Keep the pkinit definitions in a separate file so that the plugin
 * only has to include k5-int-pkinit.h rather than k5-int.h
 */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* -1 for unspecified */
    /* Plain text of an encrypted PA-FX-COOKIE value produced by the KDC. */
    /* In PAC options, indicates Resource-Based Constrained Delegation support. */
    /* struct stat, stat() */
    /* MAXPATHLEN */
    /* prototypes for file-related
                                           syscalls; flags for open &
                                           friends */
    /* libos.spec */
    /* Internal structure of an opaque key identifier */
    /*
     * Cache of data private to the cipher implementation, which we
     * don't want to have to recompute for every operation.  This may
     * include key schedules, iteration counts, etc.
     *
     * The cipher implementation is responsible for setting this up
     * whenever needed, and the enc_provider key_cleanup method must
     * then be provided to dispose of it.
     */
    /* Write the SHA-256 hash of in (containing n elements) to out. */
    /* Convenience function: zap and free ptr if it is non-NULL. */
    /* Convenience function: zap and free zero-terminated str if it is non-NULL. */
    /* Convenience function: zap and free krb5_data pointer if it is non-NULL. */
    /*
 * End "los-proto.h"
 */
    /*
 * Flags for the os_flags field
 *
 * KRB5_OS_TOFFSET_VALID means that the time offset fields are valid.
 * The intention is that this facility to correct the system clocks so
 * that they reflect the "real" time, for systems where for some
 * reason we can't set the system clock.  Instead we calculate the
 * offset between the system time and real time, and store the offset
 * in the os context so that we can correct the system clock as necessary.
 *
 * KRB5_OS_TOFFSET_TIME means that the time offset fields should be
 * returned as the time by the krb5 time routines.  This should only
 * be used for testing purposes (obviously!)
 */
    /* lock mode flags */
    /*
 * Begin "preauth.h"
 *
 * (Originally written by Glen Machin at Sandia Labs.)
 */
/*
 * Sandia National Laboratories also makes no representations about the
 * suitability of the modifications, or additions to this software for
 * any purpose.  It is provided "as is" without express or implied warranty.
 */
    /* check logon hour restrictions */
    /* sign with usage 27 instead of 26 */
    /* padata from req_body is used*/
    /* Bits 0-15 are critical in FAST options (RFC 6113 section 7.3). */
    /*
 * AD-CAMMAC's other-verifiers field is a sequence of Verifier, which is an
 * extensible choice with only one selection, Verifier-MAC.  For the time being
 * we will represent this field directly as an array of krb5_verifier_mac.
 * That will have to change if other selections are added.
 */
    /* Does not return a copy; original padata sequence responsible for freeing*/
    /* Allocate a pa-data object with uninitialized contents of size len.  If len
 * is 0, set the contents field to NULL. */
    /* Free a single pa-data object. */
    /* Without copying, add single element *pa to *list, reallocating as necessary.
 * If *list is NULL, allocate a new list.  Set *pa to NULL on success. */
    /* Without copying, add a pa-data element of type pa_type to *list with the
 * contents in data.  Set *data to empty_data() on success. */
    /* Add an empty pa-data element of type pa_type to *list. */
    /* KRB5_PREAUTH__ */
    /*
 * End "preauth.h"
 */
    /* #include "krb5/wordsize.h" -- comes in through base-defs.h. */
    /* ** Plugin framework ***/
    /*
 * This framework can be used to create pluggable interfaces.  Not all existing
 * pluggable interface use this framework, but new ones should.  A new
 * pluggable interface entails:
 *
 * - An interface ID definition in the list of #defines below.
 *
 * - A name in the interface_names array in lib/krb5/krb/plugins.c.
 *
 * - An installed public header file in include/krb5.  The public header should
 *   include <krb5/plugin.h> and should declare a vtable structure for each
 *   supported major version of the interface.
 *
 * - A consumer API implementation, located within the code unit which makes
 *   use of the pluggable interface.  The consumer API should consist of:
 *
 *   . An interface-specific handle type which contains a vtable structure for
 *     the module (or a union of several such structures, if there are multiple
 *     supported major versions) and, optionally, resource data bound to the
 *     handle.
 *
 *   . An interface-specific loader function which creates a handle or list of
 *     handles.  A list of handles would be created if the interface is a
 *     one-to-many interface where the consumer wants to consult all available
 *     modules; a single handle would be created for an interface where the
 *     consumer wants to consult a specific module.  The loader function should
 *     use k5_plugin_load or k5_plugin_load_all to produce one or a list of
 *     vtable initializer functions, and should use those functions to fill in
 *     the vtable structure for the module (if necessary, trying each supported
 *     major version starting from the most recent).  The loader function can
 *     also bind resource data into the handle based on caller arguments, if
 *     appropriate.
 *
 *   . For each plugin method, a wrapper function which accepts a krb5_context,
 *     a plugin handle, and the method arguments.  Wrapper functions should
 *     invoke the method function contained in the handle's vtable.
 *
 * - Possibly, built-in implementations of the interface, also located within
 *   the code unit which makes use of the interface.  Built-in implementations
 *   must be registered with k5_plugin_register before the first call to
 *   k5_plugin_load or k5_plugin_load_all.
 *
 * A pluggable interface should have one or more currently supported major
 * versions, starting at 1.  Each major version should have a current minor
 * version, also starting at 1.  If new methods are added to a vtable, the
 * minor version should be incremented and the vtable stucture should document
 * where each minor vtable version ends.  If method signatures for a vtable are
 * changed, the major version should be incremented.
 *
 * Plugin module implementations (either built-in or dynamically loaded) should
 * define a function named <interfacename>_<modulename>_initvt, matching the
 * signature of krb5_plugin_initvt_fn as declared in include/krb5/plugin.h.
 * The initvt function should check the given maj_ver argument against its own
 * supported major versions, cast the vtable pointer to the appropriate
 * interface-specific vtable type, and fill in the vtable methods, stopping as
 * appropriate for the given min_ver.  Memory for the vtable structure is
 * allocated by the caller, not by the module.
 *
 * Dynamic plugin modules are registered with the framework through the
 * [plugins] section of the profile, as described in the admin documentation
 * and krb5.conf man page.
 */
    /* Holds krb5_context information about each pluggable interface. */
    /* A list of plugin interface IDs.  Make sure to increment
 * PLUGIN_NUM_INTERFACES when a new interface is added, and add an entry to the
 * interface_names table in lib/krb5/krb/plugin.c. */
    /* Retrieve the plugin module of type interface_id and name modname,
 * storing the result into module. */
    /* Retrieve all plugin modules of type interface_id, storing the result
 * into modules.  Free the result with k5_plugin_free_handles. */
    /* Release a module list allocated by k5_plugin_load_all. */
    /* Register a plugin module of type interface_id and name modname. */
    /*
 * Register a plugin module which is part of the krb5 tree but is built as a
 * dynamic plugin.  Look for the module in modsubdir relative to the
 * context->base_plugin_dir.
 */
    /* Destroy the module state within context; used by krb5_free_context. */
    /* private, in kdb5.h */
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
    #[inline]
    #[c2rust::src_loc = "2296:1"]
    pub unsafe extern "C" fn k5calloc(mut nmemb: size_t, mut size: size_t,
                                      mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
        ptr =
            calloc(if nmemb != 0 {
                       nmemb
                   } else { 1 as libc::c_int as libc::c_ulong },
                   if size != 0 {
                       size
                   } else { 1 as libc::c_int as libc::c_ulong });
        *code =
            if ptr.is_null() { 12 as libc::c_int } else { 0 as libc::c_int };
        return ptr;
    }
    #[inline]
    #[c2rust::src_loc = "2308:1"]
    pub unsafe extern "C" fn k5alloc(mut size: size_t,
                                     mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        return k5calloc(1 as libc::c_int as size_t, size, code);
    }
    #[inline]
    #[c2rust::src_loc = "2326:1"]
    pub unsafe extern "C" fn k5memdup0(mut in_0: *const libc::c_void,
                                       mut len: size_t,
                                       mut code: *mut krb5_error_code)
     -> *mut libc::c_void {
        let mut ptr: *mut libc::c_void =
            k5alloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    code);
        if !ptr.is_null() && len > 0 as libc::c_int as libc::c_ulong {
            memcpy(ptr, in_0, len);
        }
        return ptr;
    }
    use super::krb5_h::{krb5_magic, krb5_enctype, krb5_deltat, krb5_flags,
                        krb5_boolean, krb5_prompt_type, krb5_trace_callback,
                        krb5_pre_send_fn, krb5_post_recv_fn, krb5_int32,
                        krb5_context, krb5_error_code};
    use super::profile_h::profile_t;
    use super::k5_plugin_h::plugin_dir_handle;
    use super::k5_err_h::errinfo;
    use super::kdb_log_h::_kdb_log_context;
    use super::kdb5_h::_kdb5_dal_handle;
    use super::stddef_h::size_t;
    use super::stdlib_h::calloc;
    use super::string_h::memcpy;
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
        #[no_mangle]
        #[c2rust::src_loc = "614:1"]
        pub fn krb5_lock_file(_: krb5_context, _: libc::c_int, _: libc::c_int)
         -> krb5_error_code;
    }
    /* _KRB5_INT_H */
    /* Define shorter internal names for setting error messages. */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/kdb_log.h:17"]
pub mod kdb_log_h {
    /* Log header magic # */
    /* Kerberos database version no. */
    /* # of updates in log */
    /* Timestamp of first update */
    /* Timestamp of last update */
    /* First serial # in the update log */
    /* Last serial # in the update log */
    /* State of update log */
    /* Block size of each element */
    /* Update entry magic # */
    /* Serial # of entry */
    /* Timestamp of update */
    /* Is the entry committed or not */
    /* Size of update entry */
    /* Address of kdb_incr_update_t */
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
    #[c2rust::src_loc = "93:1"]
    pub type kdb_ent_header_t = kdb_ent_header;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "93:16"]
    pub struct kdb_ent_header {
        pub kdb_umagic: uint32_t,
        pub kdb_entry_sno: kdb_sno_t,
        pub kdb_time: kdbe_time_t,
        pub kdb_commit: libc::c_int,
        pub kdb_entry_size: uint32_t,
        pub entry_data: [uint8_t; 4],
    }
    use super::iprop_hdr_h::iprop_role;
    use super::stdint_uintn_h::{uint32_t, uint16_t, uint8_t};
    use super::iprop_h::{kdbe_time_t, kdb_sno_t, kdb_incr_update_t};
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_error_code};
    use super::kdb_h::krb5_db_entry;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "71:1"]
        pub fn ulog_conv_2dbentry(context: krb5_context,
                                  entry: *mut *mut krb5_db_entry,
                                  update: *mut kdb_incr_update_t)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "73:1"]
        pub fn ulog_free_entries(updates: *mut kdb_incr_update_t,
                                 no_of_updates: libc::c_int);
    }
    /* !_KDB_LOG_H */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop.h:17"]
pub mod iprop_h {
    /*
 * Please do not edit this file.
 * It was generated using rpcgen.
 */
    #[c2rust::src_loc = "22:1"]
    pub type kdb_sno_t = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "24:8"]
    pub struct kdbe_time_t {
        pub seconds: uint32_t,
        pub useconds: uint32_t,
    }
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
        pub k_enctype: C2RustUnnamed_1,
        pub k_contents: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:2"]
    pub struct C2RustUnnamed_0 {
        pub k_contents_len: u_int,
        pub k_contents_val: *mut utf8str_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:2"]
    pub struct C2RustUnnamed_1 {
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
        pub k_components: C2RustUnnamed_2,
        pub k_nametype: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "52:2"]
    pub struct C2RustUnnamed_2 {
        pub k_components_len: u_int,
        pub k_components_val: *mut kdbe_data_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "60:8"]
    pub struct kdbe_tl_t {
        pub tl_type: int16_t,
        pub tl_data: C2RustUnnamed_3,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:2"]
    pub struct C2RustUnnamed_3 {
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
        pub kdbe_val_t_u: C2RustUnnamed_4,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "100:2"]
    pub union C2RustUnnamed_4 {
        pub av_attrflags: uint32_t,
        pub av_max_life: uint32_t,
        pub av_max_renew_life: uint32_t,
        pub av_exp: uint32_t,
        pub av_pw_exp: uint32_t,
        pub av_last_success: uint32_t,
        pub av_last_failed: uint32_t,
        pub av_fail_auth_count: uint32_t,
        pub av_princ: kdbe_princ_t,
        pub av_keydata: C2RustUnnamed_8,
        pub av_tldata: C2RustUnnamed_7,
        pub av_len: int16_t,
        pub av_pw_last_change: uint32_t,
        pub av_mod_princ: kdbe_princ_t,
        pub av_mod_time: uint32_t,
        pub av_mod_where: utf8str_t,
        pub av_pw_policy: utf8str_t,
        pub av_pw_policy_switch: libc::c_int,
        pub av_pw_hist_kvno: uint32_t,
        pub av_pw_hist: C2RustUnnamed_6,
        pub av_extension: C2RustUnnamed_5,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "130:3"]
    pub struct C2RustUnnamed_5 {
        pub av_extension_len: u_int,
        pub av_extension_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "126:3"]
    pub struct C2RustUnnamed_6 {
        pub av_pw_hist_len: u_int,
        pub av_pw_hist_val: *mut kdbe_pw_hist_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "114:3"]
    pub struct C2RustUnnamed_7 {
        pub av_tldata_len: u_int,
        pub av_tldata_val: *mut kdbe_tl_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "110:3"]
    pub struct C2RustUnnamed_8 {
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
        pub kdb_kdcs_seen_by: C2RustUnnamed_10,
        pub kdb_futures: C2RustUnnamed_9,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "154:2"]
    pub struct C2RustUnnamed_9 {
        pub kdb_futures_len: u_int,
        pub kdb_futures_val: *mut libc::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "150:2"]
    pub struct C2RustUnnamed_10 {
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
    use super::stdint_uintn_h::uint32_t;
    use super::sys_types_h::u_int;
    use super::stdint_intn_h::{int32_t, int16_t};
    use super::xdr_h::XDR;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "243:1"]
        pub fn xdr_kdb_incr_update_t(_: *mut XDR, _: *mut kdb_incr_update_t)
         -> libc::c_int;
    }
    /* !_IPROP_H_RPCGEN */
    /* K&R C */
    /* K&R C */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/iprop_hdr.h:17"]
pub mod iprop_hdr_h {
    #[c2rust::src_loc = "32:1"]
    pub type iprop_role = libc::c_uint;
    #[c2rust::src_loc = "35:5"]
    pub const IPROP_REPLICA: iprop_role = 2;
    #[c2rust::src_loc = "34:5"]
    pub const IPROP_MASTER: iprop_role = 1;
    #[c2rust::src_loc = "33:5"]
    pub const IPROP_NULL: iprop_role = 0;
    /* !_IPROP_HDR_H */
    /*
 * Full resync dump versioning
 */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-err.h:12"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/k5-plugin.h:12"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/lib/kdb/kdb5.h:16"]
pub mod kdb5_h {
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "27:8"]
    pub struct _kdb5_dal_handle {
        pub db_context: *mut libc::c_void,
        pub lib_handle: db_library,
        pub master_keylist: *mut krb5_keylist_node,
        pub master_princ: krb5_principal,
    }
    #[c2rust::src_loc = "19:1"]
    pub type db_library = *mut _db_library;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "19:16"]
    pub struct _db_library {
        pub name: [libc::c_char; 128],
        pub reference_cnt: libc::c_int,
        pub dl_dir_handle: plugin_dir_handle,
        pub vftabl: kdb_vftabl,
        pub next: *mut _db_library,
        pub prev: *mut _db_library,
    }
    use super::kdb_h::{krb5_keylist_node, kdb_vftabl};
    use super::krb5_h::krb5_principal;
    use super::k5_plugin_h::plugin_dir_handle;
    /* end of _KRB5_KDB5_H_ */
    /* typedef kdb5_dal_handle is in k5-int.h now */
}
#[c2rust::header_src = "/home/nmavis/dev/gssapi-rs/code/src/include/kdb.h:16"]
pub mod kdb_h {
    #[c2rust::src_loc = "294:1"]
    pub type krb5_keylist_node = _krb5_keylist_node;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "294:16"]
    pub struct _krb5_keylist_node {
        pub keyblock: krb5_keyblock,
        pub kvno: krb5_kvno,
        pub next: *mut _krb5_keylist_node,
    }
    /*
 * This number indicates the date of the last incompatible change to the DAL.
 * The maj_ver field of the module's vtable structure must match this version.
 */
    /*
 * A krb5_context can hold one database object.  Modules should use
 * krb5_db_set_context and krb5_db_get_context to store state associated with
 * the database object.
 *
 * Some module functions are mandatory for KDC operation; others are optional
 * or apply only to administrative operations.  If a function is optional, a
 * module can leave the function pointer as NULL.  Alternatively, modules can
 * return KRB5_PLUGIN_OP_NOTSUPP when asked to perform an inapplicable action.
 *
 * Some module functions have default implementations which will call back into
 * the vtable interface.  Leave these functions as NULL to use the default
 * implementations.
 *
 * The documentation in these comments describes the DAL as it is currently
 * implemented and used, not as it should be.  So if anything seems off, that
 * probably means the current state of things is off.
 *
 * Modules must allocate memory for principal entries, policy entries, and
 * other structures using an allocator compatible with malloc() as seen by
 * libkdb5 and libkrb5.  Modules may link against libkdb5 and call
 * krb5_db_alloc() to be certain that the same malloc implementation is used.
 */
    #[c2rust::src_loc = "923:1"]
    pub type kdb_vftabl = _kdb_vftabl;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "923:16"]
    pub struct _kdb_vftabl {
        pub maj_ver: libc::c_short,
        pub min_ver: libc::c_short,
        pub init_library: Option<unsafe extern "C" fn() -> krb5_error_code>,
        pub fini_library: Option<unsafe extern "C" fn() -> krb5_error_code>,
        pub init_module: Option<unsafe extern "C" fn(_: krb5_context,
                                                     _: *mut libc::c_char,
                                                     _:
                                                         *mut *mut libc::c_char,
                                                     _: libc::c_int)
                                    -> krb5_error_code>,
        pub fini_module: Option<unsafe extern "C" fn(_: krb5_context)
                                    -> krb5_error_code>,
        pub create: Option<unsafe extern "C" fn(_: krb5_context,
                                                _: *mut libc::c_char,
                                                _: *mut *mut libc::c_char)
                               -> krb5_error_code>,
        pub destroy: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *mut libc::c_char,
                                                 _: *mut *mut libc::c_char)
                                -> krb5_error_code>,
        pub get_age: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *mut libc::c_char,
                                                 _: *mut time_t)
                                -> krb5_error_code>,
        pub lock: Option<unsafe extern "C" fn(_: krb5_context, _: libc::c_int)
                             -> krb5_error_code>,
        pub unlock: Option<unsafe extern "C" fn(_: krb5_context)
                               -> krb5_error_code>,
        pub get_principal: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _:
                                                           krb5_const_principal,
                                                       _: libc::c_uint,
                                                       _:
                                                           *mut *mut krb5_db_entry)
                                      -> krb5_error_code>,
        pub put_principal: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: *mut krb5_db_entry,
                                                       _:
                                                           *mut *mut libc::c_char)
                                      -> krb5_error_code>,
        pub delete_principal: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              krb5_const_principal)
                                         -> krb5_error_code>,
        pub rename_principal: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              krb5_const_principal,
                                                          _:
                                                              krb5_const_principal)
                                         -> krb5_error_code>,
        pub iterate: Option<unsafe extern "C" fn(_: krb5_context,
                                                 _: *mut libc::c_char,
                                                 _:
                                                     Option<unsafe extern "C" fn(_:
                                                                                     krb5_pointer,
                                                                                 _:
                                                                                     *mut krb5_db_entry)
                                                                ->
                                                                    libc::c_int>,
                                                 _: krb5_pointer,
                                                 _: krb5_flags)
                                -> krb5_error_code>,
        pub create_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: osa_policy_ent_t)
                                      -> krb5_error_code>,
        pub get_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: *mut libc::c_char,
                                                    _: *mut osa_policy_ent_t)
                                   -> krb5_error_code>,
        pub put_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: osa_policy_ent_t)
                                   -> krb5_error_code>,
        pub iter_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                     _: *mut libc::c_char,
                                                     _:
                                                         osa_adb_iter_policy_func,
                                                     _: *mut libc::c_void)
                                    -> krb5_error_code>,
        pub delete_policy: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: *mut libc::c_char)
                                      -> krb5_error_code>,
        pub fetch_master_key: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _: krb5_principal,
                                                          _:
                                                              *mut krb5_keyblock,
                                                          _: *mut krb5_kvno,
                                                          _:
                                                              *mut libc::c_char)
                                         -> krb5_error_code>,
        pub fetch_master_key_list: Option<unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   krb5_principal,
                                                               _:
                                                                   *const krb5_keyblock,
                                                               _:
                                                                   *mut *mut krb5_keylist_node)
                                              -> krb5_error_code>,
        pub store_master_key_list: Option<unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut libc::c_char,
                                                               _:
                                                                   krb5_principal,
                                                               _:
                                                                   *mut krb5_keylist_node,
                                                               _:
                                                                   *mut libc::c_char)
                                              -> krb5_error_code>,
        pub dbe_search_enctype: Option<unsafe extern "C" fn(_: krb5_context,
                                                            _:
                                                                *mut krb5_db_entry,
                                                            _:
                                                                *mut krb5_int32,
                                                            _: krb5_int32,
                                                            _: krb5_int32,
                                                            _: krb5_int32,
                                                            _:
                                                                *mut *mut krb5_key_data)
                                           -> krb5_error_code>,
        pub change_pwd: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: *mut krb5_keyblock,
                                                    _:
                                                        *mut krb5_key_salt_tuple,
                                                    _: libc::c_int,
                                                    _: *mut libc::c_char,
                                                    _: libc::c_int,
                                                    _: krb5_boolean,
                                                    _: *mut krb5_db_entry)
                                   -> krb5_error_code>,
        pub promote_db: Option<unsafe extern "C" fn(_: krb5_context,
                                                    _: *mut libc::c_char,
                                                    _: *mut *mut libc::c_char)
                                   -> krb5_error_code>,
        pub decrypt_key_data: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              *const krb5_keyblock,
                                                          _:
                                                              *const krb5_key_data,
                                                          _:
                                                              *mut krb5_keyblock,
                                                          _:
                                                              *mut krb5_keysalt)
                                         -> krb5_error_code>,
        pub encrypt_key_data: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              *const krb5_keyblock,
                                                          _:
                                                              *const krb5_keyblock,
                                                          _:
                                                              *const krb5_keysalt,
                                                          _: libc::c_int,
                                                          _:
                                                              *mut krb5_key_data)
                                         -> krb5_error_code>,
        pub sign_authdata: Option<unsafe extern "C" fn(_: krb5_context,
                                                       _: libc::c_uint,
                                                       _:
                                                           krb5_const_principal,
                                                       _:
                                                           krb5_const_principal,
                                                       _: *mut krb5_db_entry,
                                                       _: *mut krb5_db_entry,
                                                       _: *mut krb5_db_entry,
                                                       _: *mut krb5_db_entry,
                                                       _: *mut krb5_keyblock,
                                                       _: *mut krb5_keyblock,
                                                       _: *mut krb5_keyblock,
                                                       _: *mut krb5_keyblock,
                                                       _: *mut krb5_keyblock,
                                                       _: krb5_timestamp,
                                                       _:
                                                           *mut *mut krb5_authdata,
                                                       _: *mut libc::c_void,
                                                       _:
                                                           *mut *mut *mut krb5_data,
                                                       _:
                                                           *mut *mut *mut krb5_authdata)
                                      -> krb5_error_code>,
        pub check_transited_realms: Option<unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *const krb5_data,
                                                                _:
                                                                    *const krb5_data,
                                                                _:
                                                                    *const krb5_data)
                                               -> krb5_error_code>,
        pub check_policy_as: Option<unsafe extern "C" fn(_: krb5_context,
                                                         _: *mut krb5_kdc_req,
                                                         _:
                                                             *mut krb5_db_entry,
                                                         _:
                                                             *mut krb5_db_entry,
                                                         _: krb5_timestamp,
                                                         _:
                                                             *mut *const libc::c_char,
                                                         _:
                                                             *mut *mut *mut krb5_pa_data)
                                        -> krb5_error_code>,
        pub check_policy_tgs: Option<unsafe extern "C" fn(_: krb5_context,
                                                          _:
                                                              *mut krb5_kdc_req,
                                                          _:
                                                              *mut krb5_db_entry,
                                                          _: *mut krb5_ticket,
                                                          _:
                                                              *mut *const libc::c_char,
                                                          _:
                                                              *mut *mut *mut krb5_pa_data)
                                         -> krb5_error_code>,
        pub audit_as_req: Option<unsafe extern "C" fn(_: krb5_context,
                                                      _: *mut krb5_kdc_req,
                                                      _: *const krb5_address,
                                                      _: *const krb5_address,
                                                      _: *mut krb5_db_entry,
                                                      _: *mut krb5_db_entry,
                                                      _: krb5_timestamp,
                                                      _: krb5_error_code)
                                     -> ()>,
        pub refresh_config: Option<unsafe extern "C" fn(_: krb5_context)
                                       -> ()>,
        pub check_allowed_to_delegate: Option<unsafe extern "C" fn(_:
                                                                       krb5_context,
                                                                   _:
                                                                       krb5_const_principal,
                                                                   _:
                                                                       *const krb5_db_entry,
                                                                   _:
                                                                       krb5_const_principal)
                                                  -> krb5_error_code>,
        pub free_principal_e_data: Option<unsafe extern "C" fn(_:
                                                                   krb5_context,
                                                               _:
                                                                   *mut krb5_octet)
                                              -> ()>,
        pub get_s4u_x509_principal: Option<unsafe extern "C" fn(_:
                                                                    krb5_context,
                                                                _:
                                                                    *const krb5_data,
                                                                _:
                                                                    krb5_const_principal,
                                                                _:
                                                                    libc::c_uint,
                                                                _:
                                                                    *mut *mut krb5_db_entry)
                                               -> krb5_error_code>,
        pub allowed_to_delegate_from: Option<unsafe extern "C" fn(_:
                                                                      krb5_context,
                                                                  _:
                                                                      krb5_const_principal,
                                                                  _:
                                                                      krb5_const_principal,
                                                                  _:
                                                                      *mut libc::c_void,
                                                                  _:
                                                                      *const krb5_db_entry)
                                                 -> krb5_error_code>,
        pub get_authdata_info: Option<unsafe extern "C" fn(_: krb5_context,
                                                           _: libc::c_uint,
                                                           _:
                                                               *mut *mut krb5_authdata,
                                                           _:
                                                               krb5_const_principal,
                                                           _:
                                                               krb5_const_principal,
                                                           _:
                                                               *mut krb5_keyblock,
                                                           _:
                                                               *mut krb5_keyblock,
                                                           _:
                                                               *mut krb5_db_entry,
                                                           _: krb5_timestamp,
                                                           _:
                                                               *mut *mut libc::c_void,
                                                           _:
                                                               *mut krb5_principal)
                                          -> krb5_error_code>,
        pub free_authdata_info: Option<unsafe extern "C" fn(_: krb5_context,
                                                            _:
                                                                *mut libc::c_void)
                                           -> ()>,
    }
    /*
     * Mandatory: Invoked after the module library is loaded, when the first DB
     * using the module is opened, across all contexts.
     */
    /*
     * Mandatory: Invoked before the module library is unloaded, after the last
     * DB using the module is closed, across all contexts.
     */
    /*
     * Mandatory: Initialize a database object.  Profile settings should be
     * read from conf_section inside KDB_MODULE_SECTION.  db_args communicates
     * command-line arguments for module-specific flags.  mode will be one of
     * KRB5_KDB_OPEN_{RW,RO} or'd with one of
     * KRB5_KDB_SRV_TYPE_{KDC,ADMIN,PASSWD,OTHER}.
     */
    /*
     * Mandatory: Finalize the database object contained in a context.  Free
     * any state contained in the db_context pointer and null it out.
     */
    /*
     * Optional: Initialize a database object while creating the underlying
     * database.  conf_section and db_args have the same meaning as in
     * init_module.  This function may return an error if the database already
     * exists.  Used by kdb5_util create.
     *
     * If db_args contains the value "temporary", the module should create an
     * exclusively locked side copy of the database suitable for loading in a
     * propagation from master to replica.  This side copy will later be
     * promoted with promote_db, allowing complete updates of the DB with no
     * loss in read availability.  If the module cannot comply with this
     * architecture, it should return an error.
     */
    /*
     * Optional: Destroy a database.  conf_section and db_args have the same
     * meaning as in init_module.  Used by kdb5_util destroy.  In current
     * usage, the database is destroyed while open, so the module should handle
     * that.
     */
    /*
     * Deprecated: No longer used as of krb5 1.10; can be removed in the next
     * DAL revision.  Modules should leave as NULL.
     */
    /*
     * Optional: Lock the database, with semantics depending on the mode
     * argument:
     *
     * KRB5_DB_LOCKMODE_SHARED: Lock may coexist with other shared locks.
     * KRB5_DB_LOCKMODE_EXCLUSIVE: Lock may not coexist with other locks.
     * KRB5_DB_LOCKMODE_PERMANENT: Exclusive lock surviving process exit.
     *
     * Used by the "kadmin lock" command, incremental propagation, and
     * kdb5_util dump.  Incremental propagation support requires shared locks
     * to operate.  kdb5_util dump will continue unlocked if the module returns
     * KRB5_PLUGIN_OP_NOTSUPP.
     */
    /* Optional: Release a lock created with db_lock. */
    /*
     * Mandatory: Set *entry to an allocated entry for the principal
     * search_for.  If the principal is not found, return KRB5_KDB_NOENTRY.
     *
     * The meaning of flags are as follows:
     *
     * KRB5_KDB_FLAG_CANONICALIZE: Set by the KDC when looking up entries for
     *     an AS or TGS request with canonicalization requested.  Determines
     *     whether the module should return out-of-realm referrals.
     *
     * KRB5_KDB_FLAG_INCLUDE_PAC: Set by the KDC during an AS request when the
     *     client requested PAC information during padata, and during most TGS
     *     requests.  Indicates that the module should include PAC information
     *     when its sign_authdata method is invoked.
     *
     * KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY: Set by the KDC when looking up the
     *     client entry in an AS request.  Affects how the module should return
     *     out-of-realm referrals.
     *
     * KRB5_KDB_FLAG_MAP_PRINCIPALS: Set by the KDC when looking up the client
     *     entry during TGS requests, except for S4U TGS requests and requests
     *     where the server entry has the KRB5_KDB_NO_AUTH_DATA_REQUIRED
     *     attribute.  Indicates that the module should map foreign principals
     *     to local principals if it supports doing so.
     *
     * KRB5_KDB_FLAG_PROTOCOL_TRANSITION: Set by the KDC when looking up the
     *     client entry during an S4U2Self TGS request.  This affects the PAC
     *     information which should be included when authorization data is
     *     generated; see the Microsoft S4U specification for details.
     *
     * KRB5_KDB_FLAG_CONSTRAINED_DELEGATION: Set by the KDC when looking up the
     *     client entry during an S4U2Proxy TGS request.  Also affects PAC
     *     generation.
     *
     * KRB5_KDB_FLAG_CROSS_REALM: Set by the KDC after looking up a server
     *     entry during a TGS request, if the header ticket was issued by a
     *     different realm.
     *
     * KRB5_KDB_FLAG_ISSUING_REFERRAL: Set by the KDC after looking up a server
     *     entry during a TGS request, if the requested server principal is not
     *     part of the realm being served, and a referral or alternate TGT will
     *     be issued instead.
     *
     * A module may return an in-realm alias by setting (*entry)->princ to the
     * canonical name.  The KDC will decide based on the request whether to use
     * the requested name or the canonical name in the issued ticket.
     *
     * A module can return a referral to another realm if
     * KRB5_KDB_FLAG_CANONICALIZE is set, or if
     * KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY is set and search_for->type is
     * KRB5_NT_ENTERPRISE_PRINCIPAL.  If KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY is
     * set, the module should return a referral by simply filling in an
     * out-of-realm name in (*entry)->princ and setting all other fields to
     * NULL.  Otherwise, the module should return the entry for the cross-realm
     * TGS of the referred-to realm.  For TGS referals, the module can also
     * include tl-data of type KRB5_TL_SERVER_REFERRAL containing ASN.1-encoded
     * Windows referral data as documented in
     * draft-ietf-krb-wg-kerberos-referrals-11 appendix A; this will be
     * returned to the client as encrypted padata.
     */
    /*
     * Optional: Create or modify a principal entry.  db_args communicates
     * command-line arguments for module-specific flags.
     *
     * The mask field of an entry indicates the changed fields.  Mask values
     * are defined in kadmin's admin.h header.  If KADM5_PRINCIPAL is set in
     * the mask, the entry is new; otherwise it already exists.  All fields of
     * an entry are expected to contain correct values, regardless of whether
     * they are specified in the mask, so it is acceptable for a module to
     * ignore the mask and update the entire entry.
     */
    /*
     * Optional: Delete the entry for the principal search_for.  If the
     * principal did not exist, return KRB5_KDB_NOENTRY.
     */
    /*
     * Optional with default: Rename a principal.  If the source principal does
     * not exist, return KRB5_KDB_NOENTRY.  If the target exists, return an
     * error.
     *
     * NOTE: If the module chooses to implement a custom function for renaming
     * a principal instead of using the default, then rename operations will
     * fail if iprop logging is enabled.
     */
    /*
     * Optional: For each principal entry in the database, invoke func with the
     * argments func_arg and the entry data.  If match_entry is specified, the
     * module may narrow the iteration to principal names matching that regular
     * expression; a module may alternatively ignore match_entry.
     */
    /*
     * Optional: Create a password policy entry.  Return an error if the policy
     * already exists.
     */
    /*
     * Optional: Set *policy to the policy entry of the specified name.  If the
     * entry does not exist, return KRB5_KDB_NOENTRY.
     */
    /*
     * Optional: Modify an existing password policy entry to match the values
     * in policy.  Return an error if the policy does not already exist.
     */
    /*
     * Optional: For each password policy entry in the database, invoke func
     * with the argments data and the entry data.  If match_entry is specified,
     * the module may narrow the iteration to policy names matching that
     * regular expression; a module may alternatively ignore match_entry.
     */
    /*
     * Optional: Delete the password policy entry with the name policy.  Return
     * an error if the entry does not exist.
     */
    /*
     * Optional with default: Retrieve a master keyblock from the stash file
     * db_args, filling in *key and *kvno.  mname is the name of the master
     * principal for the realm.
     *
     * The default implementation reads the master keyblock from a keytab or
     * old-format stash file.
     */
    /*
     * Optional with default: Given a keyblock for some version of the
     * database's master key, fetch the decrypted master key values from the
     * database and store the list into *mkeys_list.  The caller will free
     * *mkeys_list using a libkdb5 function which uses the standard free()
     * function, so the module must not use a custom allocator.
     *
     * The caller may not know the version number of the master key it has, in
     * which case it will pass IGNORE_VNO.
     *
     * The default implementation ignores kvno and tries the key against the
     * current master key data and all KRB5_TL_MKEY_AUX values, which contain
     * copies of the master keys encrypted with old master keys.
     */
    /*
     * Optional with default: Save a list of master keyblocks, obtained from
     * fetch_master_key_list, into the stash file db_arg.  The caller will set
     * master_pwd to NULL, so the module should just ignore it.  mname is the
     * name of the master principal for the realm.
     *
     * The default implementation saves the list of master keys in a
     * keytab-format file.
     */
    /*
     * Optional with default: Starting at position *start, scan the key data of
     * a database entry for a key matching the enctype ktype, the salt type
     * stype, and the version kvno.  Store the resulting key into *kdatap and
     * set *start to the position after the key found.  If ktype is negative,
     * match any enctype.  If stype is negative, match any salt type.  If kvno
     * is zero or negative, find the most recent key version satisfying the
     * other constraints.
     */
    /*
     * Optional with default: Change the key data for db_entry to include keys
     * derived from the password passwd in each of the specified key-salt
     * types, at version new_kvno.  Discard the old key data if keepold is not
     * set.
     *
     * The default implementation uses the keyblock master_key to encrypt each
     * new key, via the function encrypt_key_data.
     */
    /*
     * Optional: Promote a temporary database to be the live one.  context must
     * be initialized with an exclusively locked database created with the
     * "temporary" db_arg.  On success, the database object contained in
     * context will be finalized.
     *
     * This method is used by kdb5_util load to replace the live database with
     * minimal loss of read availability.
     */
    /*
     * Optional with default: Decrypt the key in key_data with master keyblock
     * mkey, placing the result into dbkey.  Copy the salt from key_data, if
     * any, into keysalt.  Either dbkey or keysalt may be left unmodified on
     * successful return if key_data does not contain key or salt information.
     *
     * The default implementation expects the encrypted key (in krb5_c_encrypt
     * format) to be stored in key_data_contents[0], with length given by
     * key_data_length[0].  If key_data_ver is 2, it expects the salt to be
     * stored, unencrypted, in key_data_contents[1], with length given by
     * key_data_length[1].
     */
    /*
     * Optional with default: Encrypt dbkey with master keyblock mkey, placing
     * the result into key_data along with keysalt.
     *
     * The default implementation stores the encrypted key (in krb5_c_encrypt
     * format) in key_data_contents[0] and the length in key_data_length[0].
     * If keysalt is specified, it sets key_data_ver to 2, and stores the salt
     * in key_data_contents[1] and its length in key_data_length[1].  If
     * keysalt is not specified, key_data_ver is set to 1.
     */
    /*
     * Optional: Generate signed authorization data, such as a Windows PAC, for
     * the ticket to be returned to the client.  Place the signed authorization
     * data, if any, in *signed_auth_data.  This function will be invoked for
     * an AS request if the client included padata requesting a PAC.  This
     * function will be invoked for a TGS request if there is authorization
     * data in the TGT, if the client is from another realm, or if the TGS
     * request is an S4U2Self or S4U2Proxy request.  This function will not be
     * invoked during TGS requests if the server principal has the
     * no_auth_data_required attribute set.  Input parameters are:
     *
     *   flags: The flags used to look up the client principal.
     *
     *   client_princ: For S4U2Self and S4U2Proxy TGS requests, the client
     *     principal requested by the service; for regular TGS requests, the
     *     possibly-canonicalized client principal.
     *
     *   server_princ: The server principal in the request.
     *
     *   client: The DB entry of the client if it is in the local realm, NULL
     *     if not.  For S4U2Self and S4U2Proxy TGS requests, this is the DB
     *     entry for the client principal requested by the service.
     *
     *   server: The DB entry of the service principal, or of a cross-realm
     *     krbtgt principal in case of referral.
     *
     *   header_server: For S4U2Proxy requests, the DB entry of the second
     *     ticket server.  For other TGS requests, the DB entry of the header
     *     ticket server.  For AS requests, NULL.
     *
     *   local_tgt: the DB entry of the local krbtgt principal.
     *
     *   client_key: The reply key for the KDC request, before any FAST armor
     *     is applied.  For AS requests, this may be the client's long-term key
     *     or a key chosen by a preauth mechanism.  For TGS requests, this may
     *     be the subkey found in the AP-REQ or the session key of the TGT.
     *
     *   server_key: The server key used to encrypt the returned ticket.
     *
     *   header_key: For S4U2Proxy requests, the key used to decrypt the second
     *     ticket.  For TGS requests, the key used to decrypt the header
     *     ticket.  For AS requests, NULL.
     *
     *   local_tgt_key: The decrypted first key of local_tgt.
     *
     *   session_key: The session key of the ticket being granted to the
     *     requestor.
     *
     *   authtime: The timestamp of the original client authentication time.
     *     For AS requests, this is the current time.  For TGS requests, this
     *     is the authtime of the subject ticket (TGT or S4U2Proxy evidence
     *     ticket).
     *
     *   tgt_auth_data: For TGS requests, the authorization data present in the
     *     subject ticket.  For AS requests, NULL.
     *
     *   ad_info: For TGS requests, the parsed authorization data if obtained
     *     by get_authdata_info method from the authorization data present in
     *     the subject ticket.  Otherwise NULL.
     *
     *   auth_indicators: Points to NULL or a null-terminated list of krb5_data
     *     pointers, each containing an authentication indicator (RFC 8129).
     *     The method may modify this list, or free it and replace
     *     *auth_indicators with NULL, to change which auth indicators will be
     *     included in the ticket.
     */
    /*
     * Optional: Perform a policy check on a cross-realm ticket's transited
     * field.  Return 0 if the check authoritatively succeeds,
     * KRB5_PLUGIN_NO_HANDLE to use the core transited-checking mechanisms, or
     * another error (other than KRB5_PLUGIN_OP_NOTSUPP) if the check fails.
     */
    /*
     * Optional: Perform a policy check on an AS request, in addition to the
     * standard policy checks.  Return 0 if the AS request is allowed.  If the
     * AS request is not allowed:
     *   - Place a short string literal into *status.
     *   - If desired, place data into e_data.  Any data placed here will be
     *     freed by the caller using the standard free function.
     *   - Return an appropriate error (such as KRB5KDC_ERR_POLICY).
     */
    /*
     * Optional: Perform a policy check on a TGS request, in addition to the
     * standard policy checks.  Return 0 if the TGS request is allowed.  If the
     * TGS request is not allowed:
     *   - Place a short string literal into *status.
     *   - If desired, place data into e_data.  Any data placed here will be
     *     freed by the caller using the standard free function.
     *   - Return an appropriate error (such as KRB5KDC_ERR_POLICY).
     * The input parameter ticket contains the TGT used in the TGS request.
     */
    /*
     * Optional: This method informs the module of a successful or unsuccessful
     * AS request.
     */
    /* Note: there is currently no method for auditing TGS requests. */
    /*
     * Optional: This method informs the module of a request to reload
     * configuration or other state (that is, the KDC received a SIGHUP).
     */
    /*
     * Optional: Perform a policy check on server being allowed to obtain
     * tickets from client to proxy.  (Note that proxy is the target of the
     * delegation, not the delegating service; the term "proxy" is from the
     * viewpoint of the delegating service asking another service to perform
     * some of its work in the authentication context of the client.  This
     * terminology comes from the Microsoft S4U protocol documentation.)
     * Return 0 if policy allows it, or an appropriate error (such as
     * KRB5KDC_ERR_POLICY) if not.  If this method is not implemented, all
     * S4U2Proxy delegation requests will be rejected.
     */
    /*
     * Optional: Free the e_data pointer of a database entry.  If this method
     * is not implemented, the e_data pointer in principal entries will be
     * freed with free() as seen by libkdb5.
     */
    /*
     * Optional: get a principal entry for S4U2Self based on X509 certificate.
     *
     * If flags include KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY, princ->realm
     * indicates the request realm, but the data components should be ignored.
     * The module can return an out-of-realm client referral as it would for
     * get_principal().
     *
     * If flags does not include KRB5_KDB_FLAG_CLIENT_REFERRALS_ONLY, princ is
     * from PA-S4U-X509-USER.  If it contains data components (and not just a
     * realm), the module should verify that it is the same as the lookup
     * result for client_cert.  The module should not return a referral.
     */
    /*
     * Optional: Perform a policy check on server being allowed to obtain
     * tickets from client to proxy.  This method is similar to
     * check_allowed_to_delegate, but it operates on the target server DB entry
     * (called "proxy" here as in Microsoft's protocol documentation) rather
     * than the intermediate server entry.  server_ad_info represents the
     * authdata of the intermediate server, as returned by the
     * get_authdata_info method on the header ticket.  Return 0 if policy
     * allows the delegation, or an appropriate error (such as
     * KRB5KDC_ERR_POLICY) if not.
     *
     * This method is called for S4U2Proxy requests and implements the
     * resource-based constrained delegation variant, which can support
     * cross-realm delegation.  If this method is not implemented or if it
     * returns a policy error, the KDC will fall back to
     * check_allowed_to_delegate if the intermediate and target servers are in
     * the same realm and the evidence ticket is forwardable.
     */
    /*
     * Optional: Perform verification and policy checks on authorization data,
     * such as a Windows PAC, based on the request client lookup flags.  Return
     * 0 if all checks have passed.  Optionally return a representation of the
     * authdata in *ad_info_out, to be consumed by allowed_to_delegate_from and
     * sign_authdata.  If client_out is not NULL, set *client_out to the client
     * name in the PAC; this indicates the requested client principal for a
     * cross-realm S4U2Proxy request.
     *
     * This method is called for TGS requests on the authorization data from
     * the header ticket.  For S4U2Proxy requests it is also called on the
     * authorization data from the evidence ticket.  If the
     * KRB5_KDB_FLAG_PROTOCOL_TRANSITION bit is set in flags, the authdata is
     * from the header ticket of an S4U2Self referral request, and the supplied
     * client_princ is the requested client.
     */
    /* End of minor version 0 for major version 8. */
    /*
 * A principal database entry.  Extensions to this structure currently use the
 * tl_data list.  The e_data and e_length fields are not used by any calling
 * code except kdb5_util dump and load, which marshal and unmarshal the array
 * in the dump record.  KDB modules may use these fields internally as long as
 * they set e_length appropriately (non-zero if the data should be marshalled
 * across dump and load, zero if not) and handle null e_data values in
 * caller-constructed principal entries.
 */
    #[c2rust::src_loc = "191:1"]
    pub type krb5_db_entry = _krb5_db_entry_new;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "191:16"]
    pub struct _krb5_db_entry_new {
        pub magic: krb5_magic,
        pub len: krb5_ui_2,
        pub mask: krb5_ui_4,
        pub attributes: krb5_flags,
        pub max_life: krb5_deltat,
        pub max_renewable_life: krb5_deltat,
        pub expiration: krb5_timestamp,
        pub pw_expiration: krb5_timestamp,
        pub last_success: krb5_timestamp,
        pub last_failed: krb5_timestamp,
        pub fail_auth_count: krb5_kvno,
        pub n_tl_data: krb5_int16,
        pub n_key_data: krb5_int16,
        pub e_length: krb5_ui_2,
        pub e_data: *mut krb5_octet,
        pub princ: krb5_principal,
        pub tl_data: *mut krb5_tl_data,
        pub key_data: *mut krb5_key_data,
    }
    /* NOT saved */
    /* members currently changed/set */
    /* When the client expires */
    /* When its passwd expires */
    /* Last successful passwd */
    /* Last failed passwd attempt */
    /* # of failed passwd attempt */
    /* Length of extra data */
    /* Extra data to be saved */
    /* Length, data */
    /* Linked list */
    /* key_data must be sorted by kvno in descending order. */
    /* Array */
    /*
 * If this ever changes up the version number and make the arrays be as
 * big as necessary.
 *
 * Currently the first type is the enctype and the second is the salt type.
 */
    #[c2rust::src_loc = "167:1"]
    pub type krb5_key_data = _krb5_key_data;
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
    /* Version */
    /* Key Version */
    /* Array of types */
    /* Array of lengths */
    /* Array of pointers */
    /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 1990, 1991, 2016 by the Massachusetts Institute of Technology.
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
/*
 * Copyright 2009 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
    /* KDC Database interface definitions */
    /* This API is not considered as stable as the main krb5 API.
 *
 * - We may make arbitrary incompatible changes between feature
 *   releases (e.g. from 1.7 to 1.8).
 * - We will make some effort to avoid making incompatible changes for
 *   bugfix releases, but will make them if necessary.
 */
    /* This version will be incremented when incompatible changes are made to the
 * KDB API, and will be kept in sync with the libkdb major version. */
    /* Salt types */
    /* #define KRB5_KDB_SALTTYPE_V4            1 */
    /* #define KRB5_KDB_SALTTYPE_AFS3          5 */
    /* Attributes */
    /* S4U2Self OK */
    /* Creation flags */
    /* Entry get flags */
/* Name canonicalization requested */
    /* Include authorization data generated by backend */
    /* Is AS-REQ (client referrals only) */
    /* Map cross-realm principals */
    /* Protocol transition */
    /* Constrained delegation */
    /* User-to-user */
    /* Cross-realm */
    /* Issuing referral */
    /* KDB iteration flags */
    /* String attribute names recognized by krb5 */
    /*
 * Note --- these structures cannot be modified without changing the
 * database version number in libkdb.a, but should be expandable by
 * adding new tl_data types.
 */
    #[c2rust::src_loc = "147:1"]
    pub type krb5_tl_data = _krb5_tl_data;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "147:16"]
    pub struct _krb5_tl_data {
        pub tl_data_next: *mut _krb5_tl_data,
        pub tl_data_type: krb5_int16,
        pub tl_data_length: krb5_ui_2,
        pub tl_data_contents: *mut krb5_octet,
    }
    /* NOT saved */
    /* # of array elements */
    #[c2rust::src_loc = "177:1"]
    pub type krb5_keysalt = _krb5_keysalt;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "177:16"]
    pub struct _krb5_keysalt {
        pub type_0: krb5_int16,
        pub data: krb5_data,
    }
    #[c2rust::src_loc = "239:1"]
    pub type krb5_key_salt_tuple = __krb5_key_salt_tuple;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "239:16"]
    pub struct __krb5_key_salt_tuple {
        pub ks_enctype: krb5_enctype,
        pub ks_salttype: krb5_int32,
    }
    #[c2rust::src_loc = "237:1"]
    pub type osa_adb_iter_policy_func
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: osa_policy_ent_t)
                   -> ()>;
    #[c2rust::src_loc = "215:1"]
    pub type osa_policy_ent_t = *mut _osa_policy_ent_t;
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
    use super::krb5_h::{krb5_keyblock, krb5_kvno, krb5_error_code,
                        krb5_context, krb5_const_principal, krb5_pointer,
                        krb5_flags, krb5_principal, krb5_int32, krb5_boolean,
                        krb5_timestamp, krb5_authdata, krb5_data,
                        krb5_kdc_req, krb5_pa_data, krb5_ticket, krb5_address,
                        krb5_octet, krb5_magic, krb5_ui_2, krb5_ui_4,
                        krb5_deltat, krb5_int16, krb5_enctype};
    use super::time_t_h::time_t;
    use super::k5_int_h::_krb5_context;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "358:1"]
        pub fn krb5_db_open(kcontext: krb5_context,
                            db_args: *mut *mut libc::c_char,
                            mode: libc::c_int) -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "374:1"]
        pub fn krb5_db_free_principal(kcontext: krb5_context,
                                      entry: *mut krb5_db_entry);
    }
    /* Length, data */
    /* no longer used */
    /* Only valid if version > 1 */
    /* pwdMaxFailure */
    /* pwdFailureCountInterval */
    /* pwdLockoutDuration */
    /* Only valid if version > 2 */
    /* KRB5_KDB5__ */
    /* !defined(_WIN32) */
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/include/profile.h:12"]
pub mod profile_h {
    /*
 * profile.h
 */
    #[c2rust::src_loc = "24:1"]
    pub type profile_t = *mut _profile_t;
    use super::krb5_h::_profile_t;
    /*@modifies internalState@*/
}
#[c2rust::header_src = "/usr/include/sys/time.h:12"]
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/types.h:17"]
pub mod gssrpc_types_h {
    #[c2rust::src_loc = "93:1"]
    pub type rpc_inline_t = int32_t;
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
  "/home/nmavis/dev/gssapi-rs/code/src/include/gssrpc/xdr.h:17"]
pub mod xdr_h {
    #[c2rust::src_loc = "81:1"]
    pub type xdr_op = libc::c_uint;
    #[c2rust::src_loc = "84:2"]
    pub const XDR_FREE: xdr_op = 2;
    #[c2rust::src_loc = "83:2"]
    pub const XDR_DECODE: xdr_op = 1;
    #[c2rust::src_loc = "82:2"]
    pub const XDR_ENCODE: xdr_op = 0;
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
    extern "C" {
        /* XDR using memory buffers */
        #[no_mangle]
        #[c2rust::src_loc = "315:1"]
        pub fn gssrpc_xdrmem_create(_: *mut XDR, _: caddr_t, _: u_int,
                                    _: xdr_op);
        #[no_mangle]
        #[c2rust::src_loc = "276:1"]
        pub fn gssrpc_xdr_sizeof(_: xdrproc_t, _: *mut libc::c_void)
         -> libc::c_ulong;
    }
    /* !defined(GSSRPC_XDR_H) */
}
#[c2rust::header_src = "/usr/include/unistd.h:9"]
pub mod unistd_h {
    use super::stddef_h::size_t;
    use super::sys_types_h::ssize_t;
    use super::types_h::__off_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "366:1"]
        pub fn write(__fd: libc::c_int, __buf: *const libc::c_void,
                     __n: size_t) -> ssize_t;
        #[no_mangle]
        #[c2rust::src_loc = "619:1"]
        pub fn sysconf(__name: libc::c_int) -> libc::c_long;
        #[no_mangle]
        #[c2rust::src_loc = "353:1"]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "334:1"]
        pub fn lseek(__fd: libc::c_int, __offset: __off_t,
                     __whence: libc::c_int) -> __off_t;
    }
}
#[c2rust::header_src = "/usr/include/fcntl.h:10"]
pub mod fcntl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "195:1"]
        pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...)
         -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/mman.h:11"]
pub mod mman_h {
    use super::stddef_h::size_t;
    use super::types_h::__off_t;
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "57:1"]
        pub fn mmap(__addr: *mut libc::c_void, __len: size_t,
                    __prot: libc::c_int, __flags: libc::c_int,
                    __fd: libc::c_int, __offset: __off_t)
         -> *mut libc::c_void;
        #[no_mangle]
        #[c2rust::src_loc = "76:1"]
        pub fn munmap(__addr: *mut libc::c_void, __len: size_t)
         -> libc::c_int;
        #[no_mangle]
        #[c2rust::src_loc = "89:1"]
        pub fn msync(__addr: *mut libc::c_void, __len: size_t,
                     __flags: libc::c_int) -> libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libintl.h:12"]
pub mod libintl_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "44:1"]
        pub fn dgettext(__domainname: *const libc::c_char,
                        __msgid: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:12"]
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
    }
}
#[c2rust::header_src = "/usr/include/assert.h:12"]
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
#[c2rust::header_src = "/usr/include/errno.h:12"]
pub mod errno_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "37:1"]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:12"]
pub mod stdlib_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "591:13"]
        pub fn abort() -> !;
        #[no_mangle]
        #[c2rust::src_loc = "565:1"]
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        #[c2rust::src_loc = "542:14"]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/usr/include/sys/syslog.h:15"]
pub mod syslog_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "190:1"]
        pub fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
    }
}
#[c2rust::header_src =
  "/home/nmavis/dev/gssapi-rs/code/src/lib/kdb/kdb5int.h:18"]
pub mod kdb5int_h {
    use super::k5_int_h::_krb5_context;
    use super::krb5_h::{krb5_context, krb5_error_code, krb5_principal_data,
                        krb5_principal};
    use super::kdb_h::krb5_db_entry;
    extern "C" {
        /* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/* lib/kdb/kdb5int.h - Private header for kdb5 library */
/*
 * Copyright (C) 2008 by the Massachusetts Institute of Technology.
 * All rights reserved.
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
        #[no_mangle]
        #[c2rust::src_loc = "32:1"]
        pub fn krb5int_put_principal_no_log(kcontext: krb5_context,
                                            entries: *mut krb5_db_entry)
         -> krb5_error_code;
        #[no_mangle]
        #[c2rust::src_loc = "36:1"]
        pub fn krb5int_delete_principal_no_log(kcontext: krb5_context,
                                               search_for: krb5_principal)
         -> krb5_error_code;
    }
    /* __KDB5INT_H__ */
}
pub use self::types_h::{__u_int, __uint8_t, __int16_t, __uint16_t, __int32_t,
                        __uint32_t, __dev_t, __uid_t, __gid_t, __ino_t,
                        __mode_t, __nlink_t, __off_t, __time_t, __suseconds_t,
                        __blksize_t, __blkcnt_t, __ssize_t, __syscall_slong_t,
                        __caddr_t};
pub use self::struct_timespec_h::timespec;
pub use self::time_t_h::time_t;
pub use self::sys_stat_h::{off_t, stat};
pub use self::stat_h::stat;
pub use self::sys_types_h::{u_int, ssize_t, caddr_t};
pub use self::stddef_h::size_t;
pub use self::stdint_intn_h::{int16_t, int32_t};
pub use self::struct_timeval_h::timeval;
pub use self::confname_h::{C2RustUnnamed, _SC_THREAD_ROBUST_PRIO_PROTECT,
                           _SC_THREAD_ROBUST_PRIO_INHERIT, _SC_XOPEN_STREAMS,
                           _SC_TRACE_USER_EVENT_MAX, _SC_TRACE_SYS_MAX,
                           _SC_TRACE_NAME_MAX, _SC_TRACE_EVENT_NAME_MAX,
                           _SC_SS_REPL_MAX, _SC_V7_LPBIG_OFFBIG,
                           _SC_V7_LP64_OFF64, _SC_V7_ILP32_OFFBIG,
                           _SC_V7_ILP32_OFF32, _SC_RAW_SOCKETS, _SC_IPV6,
                           _SC_LEVEL4_CACHE_LINESIZE, _SC_LEVEL4_CACHE_ASSOC,
                           _SC_LEVEL4_CACHE_SIZE, _SC_LEVEL3_CACHE_LINESIZE,
                           _SC_LEVEL3_CACHE_ASSOC, _SC_LEVEL3_CACHE_SIZE,
                           _SC_LEVEL2_CACHE_LINESIZE, _SC_LEVEL2_CACHE_ASSOC,
                           _SC_LEVEL2_CACHE_SIZE, _SC_LEVEL1_DCACHE_LINESIZE,
                           _SC_LEVEL1_DCACHE_ASSOC, _SC_LEVEL1_DCACHE_SIZE,
                           _SC_LEVEL1_ICACHE_LINESIZE,
                           _SC_LEVEL1_ICACHE_ASSOC, _SC_LEVEL1_ICACHE_SIZE,
                           _SC_TRACE_LOG, _SC_TRACE_INHERIT,
                           _SC_TRACE_EVENT_FILTER, _SC_TRACE,
                           _SC_HOST_NAME_MAX, _SC_V6_LPBIG_OFFBIG,
                           _SC_V6_LP64_OFF64, _SC_V6_ILP32_OFFBIG,
                           _SC_V6_ILP32_OFF32, _SC_2_PBS_CHECKPOINT,
                           _SC_STREAMS, _SC_SYMLOOP_MAX, _SC_2_PBS_TRACK,
                           _SC_2_PBS_MESSAGE, _SC_2_PBS_LOCATE,
                           _SC_2_PBS_ACCOUNTING, _SC_2_PBS, _SC_USER_GROUPS_R,
                           _SC_USER_GROUPS, _SC_TYPED_MEMORY_OBJECTS,
                           _SC_TIMEOUTS, _SC_SYSTEM_DATABASE_R,
                           _SC_SYSTEM_DATABASE, _SC_THREAD_SPORADIC_SERVER,
                           _SC_SPORADIC_SERVER, _SC_SPAWN, _SC_SIGNALS,
                           _SC_SHELL, _SC_REGEX_VERSION, _SC_REGEXP,
                           _SC_SPIN_LOCKS, _SC_READER_WRITER_LOCKS,
                           _SC_NETWORKING, _SC_SINGLE_PROCESS,
                           _SC_MULTI_PROCESS, _SC_MONOTONIC_CLOCK,
                           _SC_FILE_SYSTEM, _SC_FILE_LOCKING,
                           _SC_FILE_ATTRIBUTES, _SC_PIPE, _SC_FIFO,
                           _SC_FD_MGMT, _SC_DEVICE_SPECIFIC_R,
                           _SC_DEVICE_SPECIFIC, _SC_DEVICE_IO,
                           _SC_THREAD_CPUTIME, _SC_CPUTIME,
                           _SC_CLOCK_SELECTION, _SC_C_LANG_SUPPORT_R,
                           _SC_C_LANG_SUPPORT, _SC_BASE, _SC_BARRIERS,
                           _SC_ADVISORY_INFO, _SC_XOPEN_REALTIME_THREADS,
                           _SC_XOPEN_REALTIME, _SC_XOPEN_LEGACY,
                           _SC_XBS5_LPBIG_OFFBIG, _SC_XBS5_LP64_OFF64,
                           _SC_XBS5_ILP32_OFFBIG, _SC_XBS5_ILP32_OFF32,
                           _SC_NL_TEXTMAX, _SC_NL_SETMAX, _SC_NL_NMAX,
                           _SC_NL_MSGMAX, _SC_NL_LANGMAX, _SC_NL_ARGMAX,
                           _SC_USHRT_MAX, _SC_ULONG_MAX, _SC_UINT_MAX,
                           _SC_UCHAR_MAX, _SC_SHRT_MIN, _SC_SHRT_MAX,
                           _SC_SCHAR_MIN, _SC_SCHAR_MAX, _SC_SSIZE_MAX,
                           _SC_NZERO, _SC_MB_LEN_MAX, _SC_WORD_BIT,
                           _SC_LONG_BIT, _SC_INT_MIN, _SC_INT_MAX,
                           _SC_CHAR_MIN, _SC_CHAR_MAX, _SC_CHAR_BIT,
                           _SC_XOPEN_XPG4, _SC_XOPEN_XPG3, _SC_XOPEN_XPG2,
                           _SC_2_UPE, _SC_2_C_VERSION, _SC_2_CHAR_TERM,
                           _SC_XOPEN_SHM, _SC_XOPEN_ENH_I18N, _SC_XOPEN_CRYPT,
                           _SC_XOPEN_UNIX, _SC_XOPEN_XCU_VERSION,
                           _SC_XOPEN_VERSION, _SC_PASS_MAX, _SC_ATEXIT_MAX,
                           _SC_AVPHYS_PAGES, _SC_PHYS_PAGES,
                           _SC_NPROCESSORS_ONLN, _SC_NPROCESSORS_CONF,
                           _SC_THREAD_PROCESS_SHARED, _SC_THREAD_PRIO_PROTECT,
                           _SC_THREAD_PRIO_INHERIT,
                           _SC_THREAD_PRIORITY_SCHEDULING,
                           _SC_THREAD_ATTR_STACKSIZE,
                           _SC_THREAD_ATTR_STACKADDR, _SC_THREAD_THREADS_MAX,
                           _SC_THREAD_STACK_MIN, _SC_THREAD_KEYS_MAX,
                           _SC_THREAD_DESTRUCTOR_ITERATIONS, _SC_TTY_NAME_MAX,
                           _SC_LOGIN_NAME_MAX, _SC_GETPW_R_SIZE_MAX,
                           _SC_GETGR_R_SIZE_MAX, _SC_THREAD_SAFE_FUNCTIONS,
                           _SC_THREADS, _SC_T_IOV_MAX, _SC_PII_OSI_M,
                           _SC_PII_OSI_CLTS, _SC_PII_OSI_COTS,
                           _SC_PII_INTERNET_DGRAM, _SC_PII_INTERNET_STREAM,
                           _SC_IOV_MAX, _SC_UIO_MAXIOV, _SC_SELECT, _SC_POLL,
                           _SC_PII_OSI, _SC_PII_INTERNET, _SC_PII_SOCKET,
                           _SC_PII_XTI, _SC_PII, _SC_2_LOCALEDEF,
                           _SC_2_SW_DEV, _SC_2_FORT_RUN, _SC_2_FORT_DEV,
                           _SC_2_C_DEV, _SC_2_C_BIND, _SC_2_VERSION,
                           _SC_CHARCLASS_NAME_MAX, _SC_RE_DUP_MAX,
                           _SC_LINE_MAX, _SC_EXPR_NEST_MAX,
                           _SC_EQUIV_CLASS_MAX, _SC_COLL_WEIGHTS_MAX,
                           _SC_BC_STRING_MAX, _SC_BC_SCALE_MAX,
                           _SC_BC_DIM_MAX, _SC_BC_BASE_MAX, _SC_TIMER_MAX,
                           _SC_SIGQUEUE_MAX, _SC_SEM_VALUE_MAX,
                           _SC_SEM_NSEMS_MAX, _SC_RTSIG_MAX, _SC_PAGESIZE,
                           _SC_VERSION, _SC_MQ_PRIO_MAX, _SC_MQ_OPEN_MAX,
                           _SC_DELAYTIMER_MAX, _SC_AIO_PRIO_DELTA_MAX,
                           _SC_AIO_MAX, _SC_AIO_LISTIO_MAX,
                           _SC_SHARED_MEMORY_OBJECTS, _SC_SEMAPHORES,
                           _SC_MESSAGE_PASSING, _SC_MEMORY_PROTECTION,
                           _SC_MEMLOCK_RANGE, _SC_MEMLOCK, _SC_MAPPED_FILES,
                           _SC_FSYNC, _SC_SYNCHRONIZED_IO, _SC_PRIORITIZED_IO,
                           _SC_ASYNCHRONOUS_IO, _SC_TIMERS,
                           _SC_PRIORITY_SCHEDULING, _SC_REALTIME_SIGNALS,
                           _SC_SAVED_IDS, _SC_JOB_CONTROL, _SC_TZNAME_MAX,
                           _SC_STREAM_MAX, _SC_OPEN_MAX, _SC_NGROUPS_MAX,
                           _SC_CLK_TCK, _SC_CHILD_MAX, _SC_ARG_MAX};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
pub use self::krb5_h::{krb5_octet, krb5_int16, krb5_ui_2, krb5_int32,
                       krb5_ui_4, krb5_boolean, krb5_msgtype, krb5_kvno,
                       krb5_addrtype, krb5_enctype, krb5_authdatatype,
                       krb5_preauthtype, krb5_flags, krb5_timestamp,
                       krb5_deltat, krb5_error_code, krb5_magic, _krb5_data,
                       krb5_data, krb5_pointer, krb5_principal_data,
                       krb5_principal, krb5_const_principal, _krb5_address,
                       krb5_address, krb5_post_recv_fn, krb5_context,
                       krb5_pre_send_fn, krb5_trace_callback, krb5_trace_info,
                       _krb5_trace_info, krb5_prompt_type, krb5_keyblock,
                       _krb5_keyblock, krb5_authdata, _krb5_authdata,
                       krb5_kdc_req, _krb5_kdc_req, krb5_ticket, _krb5_ticket,
                       krb5_enc_tkt_part, _krb5_enc_tkt_part,
                       krb5_ticket_times, _krb5_ticket_times, krb5_transited,
                       _krb5_transited, krb5_enc_data, _krb5_enc_data,
                       krb5_pa_data, _krb5_pa_data, _profile_t,
                       krb5_parse_name, krb5_free_principal};
pub use self::k5_int_h::{_krb5_context, plugin_interface, dns_canonhost,
                         CANONHOST_FALLBACK, CANONHOST_TRUE, CANONHOST_FALSE,
                         krb5_preauth_context, kdb5_dal_handle,
                         _krb5_os_context, k5calloc, k5alloc, k5memdup0,
                         plugin_mapping, k5_tls_vtable_st,
                         hostrealm_module_handle, localauth_module_handle,
                         ccselect_module_handle, krb5_preauth_context_st,
                         krb5_lock_file};
pub use self::kdb_log_h::{_kdb_log_context, kdb_hlog_t, kdb_hlog,
                          kdb_log_context, kdb_ent_header_t, kdb_ent_header,
                          ulog_conv_2dbentry, ulog_free_entries};
pub use self::iprop_h::{kdb_sno_t, kdbe_time_t, utf8str_t, kdbe_key_t,
                        C2RustUnnamed_0, C2RustUnnamed_1, kdbe_data_t,
                        kdbe_princ_t, C2RustUnnamed_2, kdbe_tl_t,
                        C2RustUnnamed_3, kdbe_pw_hist_t, kdbe_attr_type_t,
                        AT_PW_HIST, AT_PW_HIST_KVNO, AT_PW_POLICY_SWITCH,
                        AT_PW_POLICY, AT_PW_LAST_CHANGE, AT_MOD_WHERE,
                        AT_MOD_TIME, AT_MOD_PRINC, AT_LEN, AT_TL_DATA,
                        AT_KEYDATA, AT_PRINC, AT_FAIL_AUTH_COUNT,
                        AT_LAST_FAILED, AT_LAST_SUCCESS, AT_PW_EXP, AT_EXP,
                        AT_MAX_RENEW_LIFE, AT_MAX_LIFE, AT_ATTRFLAGS,
                        kdbe_val_t, C2RustUnnamed_4, C2RustUnnamed_5,
                        C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
                        kdbe_t, kdb_incr_update_t, C2RustUnnamed_9,
                        C2RustUnnamed_10, kdb_ulog_t, update_status_t,
                        UPDATE_PERM_DENIED, UPDATE_NIL, UPDATE_BUSY,
                        UPDATE_FULL_RESYNC_NEEDED, UPDATE_ERROR, UPDATE_OK,
                        kdb_last_t, kdb_incr_result_t, xdr_kdb_incr_update_t};
pub use self::iprop_hdr_h::{iprop_role, IPROP_REPLICA, IPROP_MASTER,
                            IPROP_NULL};
pub use self::k5_err_h::errinfo;
pub use self::k5_plugin_h::{plugin_dir_handle, plugin_file_handle};
pub use self::kdb5_h::{_kdb5_dal_handle, db_library, _db_library};
pub use self::kdb_h::{krb5_keylist_node, _krb5_keylist_node, kdb_vftabl,
                      _kdb_vftabl, krb5_db_entry, _krb5_db_entry_new,
                      krb5_key_data, _krb5_key_data, krb5_tl_data,
                      _krb5_tl_data, krb5_keysalt, _krb5_keysalt,
                      krb5_key_salt_tuple, __krb5_key_salt_tuple,
                      osa_adb_iter_policy_func, osa_policy_ent_t,
                      _osa_policy_ent_t, krb5_db_open,
                      krb5_db_free_principal};
pub use self::profile_h::profile_t;
pub use self::time_h::{timezone, __timezone_ptr_t, gettimeofday};
pub use self::gssrpc_types_h::rpc_inline_t;
pub use self::xdr_h::{xdr_op, XDR_FREE, XDR_DECODE, XDR_ENCODE, xdrproc_t,
                      XDR, xdr_ops, gssrpc_xdrmem_create, gssrpc_xdr_sizeof};
use self::unistd_h::{write, sysconf, close, lseek};
use self::fcntl_h::open;
use self::mman_h::{mmap, munmap, msync};
use self::libintl_h::dgettext;
use self::string_h::{memcpy, memset};
use self::assert_h::__assert_fail;
use self::errno_h::__errno_location;
use self::stdlib_h::{abort, free, calloc};
use self::syslog_h::syslog;
use self::kdb5int_h::{krb5int_put_principal_no_log,
                      krb5int_delete_principal_no_log};
/* -*- mode: c; c-basic-offset: 4; indent-tabs-mode: nil -*- */
/*
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */
/* This module includes all the necessary functions that create and modify the
 * Kerberos principal update and header logs. */
#[c2rust::src_loc = "29:12"]
static mut pagesize: libc::c_int = 0 as libc::c_int;
/* Initialize context->kdblog_context if it does not yet exist, and return it.
 * Return NULL on allocation failure. */
#[c2rust::src_loc = "39:1"]
unsafe extern "C" fn create_log_context(mut context: krb5_context)
 -> *mut kdb_log_context {
    let mut log_ctx: *mut kdb_log_context = 0 as *mut kdb_log_context;
    if !(*context).kdblog_context.is_null() {
        return (*context).kdblog_context
    }
    log_ctx =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<kdb_log_context>() as libc::c_ulong) as
            *mut kdb_log_context;
    if log_ctx.is_null() { return 0 as *mut kdb_log_context }
    (*log_ctx).ulogfd = -(1 as libc::c_int);
    (*context).kdblog_context = log_ctx;
    return log_ctx;
}
#[inline]
#[c2rust::src_loc = "54:1"]
unsafe extern "C" fn time_equal(mut a: *const kdbe_time_t,
                                mut b: *const kdbe_time_t) -> krb5_boolean {
    return ((*a).seconds == (*b).seconds && (*a).useconds == (*b).useconds) as
               libc::c_int as krb5_boolean;
}
#[c2rust::src_loc = "60:1"]
unsafe extern "C" fn time_current(mut out: *mut kdbe_time_t) {
    let mut timestamp: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    gettimeofday(&mut timestamp, 0 as *mut timezone);
    (*out).seconds = timestamp.tv_sec as uint32_t;
    (*out).useconds = timestamp.tv_usec as uint32_t;
}
/* Sync update entry to disk. */
#[c2rust::src_loc = "71:1"]
unsafe extern "C" fn sync_update(mut ulog: *mut kdb_hlog_t,
                                 mut upd: *mut kdb_ent_header_t) {
    let mut start: libc::c_ulong = 0;
    let mut end: libc::c_ulong = 0;
    let mut size: libc::c_ulong = 0;
    if pagesize == 0 {
        pagesize = sysconf(_SC_PAGESIZE as libc::c_int) as libc::c_int
    }
    start =
        upd as libc::c_ulong &
            !(pagesize - 1 as libc::c_int) as libc::c_ulong;
    end =
        (upd as
             libc::c_ulong).wrapping_add((*ulog).kdb_block as
                                             libc::c_ulong).wrapping_add((pagesize
                                                                              -
                                                                              1
                                                                                  as
                                                                                  libc::c_int)
                                                                             as
                                                                             libc::c_ulong)
            & !(pagesize - 1 as libc::c_int) as libc::c_ulong;
    size = end.wrapping_sub(start);
    if msync(start as caddr_t as *mut libc::c_void, size, 4 as libc::c_int) !=
           0 {
        /* Couldn't sync to disk, let's panic. */
        syslog(3 as libc::c_int,
               dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"could not sync ulog update to disk\x00" as *const u8
                            as *const libc::c_char));
        abort();
    };
}
/* Sync memory to disk for the update log header. */
#[c2rust::src_loc = "93:1"]
unsafe extern "C" fn sync_header(mut ulog: *mut kdb_hlog_t) {
    if pagesize == 0 {
        pagesize = sysconf(_SC_PAGESIZE as libc::c_int) as libc::c_int
    }
    if msync(ulog as caddr_t as *mut libc::c_void, pagesize as size_t,
             4 as libc::c_int) != 0 {
        /* Couldn't sync to disk, let's panic. */
        syslog(3 as libc::c_int,
               dgettext(b"mit-krb5\x00" as *const u8 as *const libc::c_char,
                        b"could not sync ulog header to disk\x00" as *const u8
                            as *const libc::c_char));
        abort();
    };
}
/* Return true if the ulog entry for sno matches sno and timestamp. */
#[c2rust::src_loc = "107:1"]
unsafe extern "C" fn check_sno(mut log_ctx: *mut kdb_log_context,
                               mut sno: kdb_sno_t,
                               mut timestamp: *const kdbe_time_t)
 -> krb5_boolean {
    let mut indx: libc::c_uint =
        sno.wrapping_sub(1 as libc::c_int as
                             libc::c_uint).wrapping_rem((*log_ctx).ulogentries);
    let mut ent: *mut kdb_ent_header_t =
        ((*log_ctx).ulog as
             *mut libc::c_char).offset(::std::mem::size_of::<kdb_hlog_t>() as
                                           libc::c_ulong as
                                           isize).offset(indx.wrapping_mul((*(*log_ctx).ulog).kdb_block
                                                                               as
                                                                               libc::c_uint)
                                                             as isize) as
            *mut libc::c_void as *mut kdb_ent_header_t;
    return ((*ent).kdb_entry_sno == sno &&
                time_equal(&mut (*ent).kdb_time, timestamp) != 0) as
               libc::c_int as krb5_boolean;
}
/*
 * Check last against our ulog and determine whether it is up to date
 * (UPDATE_NIL), so far out of date that a full dump is required
 * (UPDATE_FULL_RESYNC_NEEDED), or okay to update with ulog entries
 * (UPDATE_OK).
 */
#[c2rust::src_loc = "123:1"]
unsafe extern "C" fn get_sno_status(mut log_ctx: *mut kdb_log_context,
                                    mut last: *const kdb_last_t)
 -> update_status_t {
    let mut ulog: *mut kdb_hlog_t = (*log_ctx).ulog;
    /* If last matches the ulog's last serial number and time exactly, it are
     * up to date even if the ulog is empty. */
    if (*last).last_sno == (*ulog).kdb_last_sno &&
           time_equal(&(*last).last_time, &mut (*ulog).kdb_last_time) != 0 {
        return UPDATE_NIL
    }
    /* If our ulog is empty or does not contain last_sno, a full resync is
     * required. */
    if (*ulog).kdb_num == 0 as libc::c_int as libc::c_uint ||
           (*last).last_sno > (*ulog).kdb_last_sno ||
           (*last).last_sno < (*ulog).kdb_first_sno {
        return UPDATE_FULL_RESYNC_NEEDED
    }
    /* If the timestamp in our ulog entry does not match last, then sno was
     * reused and a full resync is required. */
    if check_sno(log_ctx, (*last).last_sno, &(*last).last_time) == 0 {
        return UPDATE_FULL_RESYNC_NEEDED
    }
    /* last is not fully up to date, but can be updated using our ulog. */
    return UPDATE_OK;
}
/* Extend update log file. */
#[c2rust::src_loc = "150:1"]
unsafe extern "C" fn extend_file_to(mut fd: libc::c_int,
                                    mut new_size: libc::c_uint)
 -> krb5_error_code {
    let mut current_offset: off_t = 0;
    static mut zero: [libc::c_char; 512] = [0; 512];
    let mut wrote_size: ssize_t = 0;
    let mut write_size: size_t = 0;
    current_offset = lseek(fd, 0 as libc::c_int as __off_t, 2 as libc::c_int);
    if current_offset < 0 as libc::c_int as libc::c_long {
        return *__errno_location()
    }
    if new_size > 2147483647 as libc::c_int as libc::c_uint {
        return 22 as libc::c_int
    }
    while current_offset < new_size as off_t {
        write_size = (new_size as libc::c_long - current_offset) as size_t;
        if write_size > 512 as libc::c_int as libc::c_ulong {
            write_size = 512 as libc::c_int as size_t
        }
        wrote_size =
            write(fd, zero.as_ptr() as *const libc::c_void, write_size);
        if wrote_size < 0 as libc::c_int as libc::c_long {
            return *__errno_location()
        }
        if wrote_size == 0 as libc::c_int as libc::c_long {
            return 22 as libc::c_int
        }
        current_offset += wrote_size;
        write_size = (new_size as libc::c_long - current_offset) as size_t
    }
    return 0 as libc::c_int;
}
/*
 * Resize the array elements.  We reinitialize the update log rather than
 * unrolling the the log and copying it over to a temporary log for obvious
 * performance reasons.  Replicas will subsequently do a full resync, but the
 * need for resizing should be very small.
 */
#[c2rust::src_loc = "184:1"]
unsafe extern "C" fn resize(mut ulog: *mut kdb_hlog_t,
                            mut ulogentries: uint32_t,
                            mut ulogfd: libc::c_int,
                            mut recsize: libc::c_uint) -> krb5_error_code {
    let mut new_block: libc::c_uint = 0;
    let mut new_size: libc::c_uint = 0;
    if ulog.is_null() {
        return -(1780008406 as libc::c_long) as krb5_error_code
    }
    new_size =
        ::std::mem::size_of::<kdb_hlog_t>() as libc::c_ulong as libc::c_uint;
    new_block =
        recsize.wrapping_div(2048 as libc::c_int as
                                 libc::c_uint).wrapping_add(1 as libc::c_int
                                                                as
                                                                libc::c_uint);
    new_block = new_block.wrapping_mul(2048 as libc::c_int as libc::c_uint);
    new_size = new_size.wrapping_add(ulogentries.wrapping_mul(new_block));
    if new_size > 0x10000000 as libc::c_int as libc::c_uint {
        return -(1780008406 as libc::c_long) as krb5_error_code
    }
    /* Reinit log with new block size. */
    memset(ulog as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<kdb_hlog_t>() as libc::c_ulong);
    (*ulog).kdb_hmagic = 0x6662323 as libc::c_int as uint32_t;
    (*ulog).db_version_num = 1 as libc::c_int as uint16_t;
    (*ulog).kdb_state = 1 as libc::c_int as uint16_t;
    (*ulog).kdb_block = new_block as uint16_t;
    sync_header(ulog);
    /* Expand log considering new block size. */
    return extend_file_to(ulogfd, new_size);
}
/* Set the ulog to contain only a dummy entry with the given serial number and
 * timestamp. */
#[c2rust::src_loc = "215:1"]
unsafe extern "C" fn set_dummy(mut log_ctx: *mut kdb_log_context,
                               mut sno: kdb_sno_t,
                               mut kdb_time: *const kdbe_time_t) {
    let mut ulog: *mut kdb_hlog_t = (*log_ctx).ulog;
    let mut ent: *mut kdb_ent_header_t =
        (ulog as
             *mut libc::c_char).offset(::std::mem::size_of::<kdb_hlog_t>() as
                                           libc::c_ulong as
                                           isize).offset(sno.wrapping_sub(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint).wrapping_rem((*log_ctx).ulogentries).wrapping_mul((*ulog).kdb_block
                                                                                                                                                  as
                                                                                                                                                  libc::c_uint)
                                                             as isize) as
            *mut libc::c_void as *mut kdb_ent_header_t;
    memset(ent as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<kdb_ent_header_t>() as libc::c_ulong);
    (*ent).kdb_umagic = 0x6661212 as libc::c_int as uint32_t;
    (*ent).kdb_entry_sno = sno;
    (*ent).kdb_time = *kdb_time;
    sync_update(ulog, ent);
    (*ulog).kdb_num = 1 as libc::c_int as uint32_t;
    (*ulog).kdb_last_sno = sno;
    (*ulog).kdb_first_sno = (*ulog).kdb_last_sno;
    (*ulog).kdb_last_time = *kdb_time;
    (*ulog).kdb_first_time = (*ulog).kdb_last_time;
}
/* Reinitialize the ulog header, starting from sno 1 with the current time. */
#[c2rust::src_loc = "233:1"]
unsafe extern "C" fn reset_ulog(mut log_ctx: *mut kdb_log_context) {
    let mut kdb_time: kdbe_time_t = kdbe_time_t{seconds: 0, useconds: 0,};
    let mut ulog: *mut kdb_hlog_t = (*log_ctx).ulog;
    memset(ulog as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<kdb_hlog_t>() as libc::c_ulong);
    (*ulog).kdb_hmagic = 0x6662323 as libc::c_int as uint32_t;
    (*ulog).db_version_num = 1 as libc::c_int as uint16_t;
    (*ulog).kdb_block = 2048 as libc::c_int as uint16_t;
    /* Create a dummy entry to remember the timestamp for downstreams. */
    time_current(&mut kdb_time);
    set_dummy(log_ctx, 1 as libc::c_int as kdb_sno_t, &mut kdb_time);
    (*ulog).kdb_state = 1 as libc::c_int as uint16_t;
    sync_header(ulog);
}
/*
 * If any database operations will be invoked while the ulog lock is held, the
 * caller must explicitly lock the database before locking the ulog, or
 * deadlock may result.
 */
#[c2rust::src_loc = "256:1"]
unsafe extern "C" fn lock_ulog(mut context: krb5_context,
                               mut mode: libc::c_int) -> krb5_error_code {
    let mut log_ctx: *mut kdb_log_context = 0 as *mut kdb_log_context;
    let mut ulog: *mut kdb_hlog_t = 0 as *mut kdb_hlog_t;
    log_ctx = (*context).kdblog_context;
    if !log_ctx.is_null() {
    } else {
        __assert_fail(b"log_ctx != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"kdb_log.c\x00" as *const u8 as *const libc::c_char,
                      262 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"krb5_error_code lock_ulog(krb5_context, int)\x00")).as_ptr());
    }
    ulog = (*log_ctx).ulog;
    if !ulog.is_null() {
    } else {
        __assert_fail(b"ulog != NULL\x00" as *const u8 as *const libc::c_char,
                      b"kdb_log.c\x00" as *const u8 as *const libc::c_char,
                      262 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 45],
                                                &[libc::c_char; 45]>(b"krb5_error_code lock_ulog(krb5_context, int)\x00")).as_ptr());
    }
    return krb5_lock_file(context, (*log_ctx).ulogfd, mode);
}
#[c2rust::src_loc = "266:1"]
unsafe extern "C" fn unlock_ulog(mut context: krb5_context) {
    lock_ulog(context, 0x8 as libc::c_int);
}
/*
 * Add an update to the log.  The update's kdb_entry_sno and kdb_time fields
 * must already be set.  The layout of the update log looks like:
 *
 * header log -> [ update header -> xdr(kdb_incr_update_t) ], ...
 */
#[c2rust::src_loc = "278:1"]
unsafe extern "C" fn store_update(mut log_ctx: *mut kdb_log_context,
                                  mut upd: *mut kdb_incr_update_t)
 -> krb5_error_code {
    let mut xdrs: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    let mut indx_log: *mut kdb_ent_header_t = 0 as *mut kdb_ent_header_t;
    let mut i: libc::c_uint = 0;
    let mut recsize: libc::c_uint = 0;
    let mut upd_size: libc::c_ulong = 0;
    let mut retval: krb5_error_code = 0;
    let mut ulog: *mut kdb_hlog_t = (*log_ctx).ulog;
    let mut ulogentries: uint32_t = (*log_ctx).ulogentries;
    upd_size =
        gssrpc_xdr_sizeof(::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                  *mut XDR,
                                                                              _:
                                                                                  *mut kdb_incr_update_t)
                                                             -> libc::c_int>,
                                                  xdrproc_t>(Some(xdr_kdb_incr_update_t
                                                                      as
                                                                      unsafe extern "C" fn(_:
                                                                                               *mut XDR,
                                                                                           _:
                                                                                               *mut kdb_incr_update_t)
                                                                          ->
                                                                              libc::c_int)),
                          upd as *mut libc::c_void);
    recsize =
        (::std::mem::size_of::<kdb_ent_header_t>() as
             libc::c_ulong).wrapping_add(upd_size) as libc::c_uint;
    if recsize > (*ulog).kdb_block as libc::c_uint {
        retval = resize(ulog, ulogentries, (*log_ctx).ulogfd, recsize);
        if retval != 0 { return retval }
    }
    (*ulog).kdb_state = 2 as libc::c_int as uint16_t;
    i =
        (*upd).kdb_entry_sno.wrapping_sub(1 as libc::c_int as
                                              libc::c_uint).wrapping_rem(ulogentries);
    indx_log =
        (ulog as
             *mut libc::c_char).offset(::std::mem::size_of::<kdb_hlog_t>() as
                                           libc::c_ulong as
                                           isize).offset(i.wrapping_mul((*ulog).kdb_block
                                                                            as
                                                                            libc::c_uint)
                                                             as isize) as
            *mut libc::c_void as *mut kdb_ent_header_t;
    memset(indx_log as *mut libc::c_void, 0 as libc::c_int,
           (*ulog).kdb_block as libc::c_ulong);
    (*indx_log).kdb_umagic = 0x6661212 as libc::c_int as uint32_t;
    (*indx_log).kdb_entry_size = upd_size as uint32_t;
    (*indx_log).kdb_entry_sno = (*upd).kdb_entry_sno;
    (*indx_log).kdb_time = (*upd).kdb_time;
    (*indx_log).kdb_commit = 0 as libc::c_int;
    gssrpc_xdrmem_create(&mut xdrs,
                         (*indx_log).entry_data.as_mut_ptr() as
                             *mut libc::c_char, (*indx_log).kdb_entry_size,
                         XDR_ENCODE);
    if xdr_kdb_incr_update_t(&mut xdrs, upd) == 0 {
        return -(1780008409 as libc::c_long) as krb5_error_code
    }
    (*indx_log).kdb_commit = 1 as libc::c_int;
    sync_update(ulog, indx_log);
    /* Modify the ulog header to reflect the new update. */
    (*ulog).kdb_last_sno = (*upd).kdb_entry_sno;
    (*ulog).kdb_last_time = (*upd).kdb_time;
    if (*ulog).kdb_num == 0 as libc::c_int as libc::c_uint {
        /* We should only see this in old ulogs. */
        (*ulog).kdb_num = 1 as libc::c_int as uint32_t;
        (*ulog).kdb_first_sno = (*upd).kdb_entry_sno;
        (*ulog).kdb_first_time = (*upd).kdb_time
    } else if (*ulog).kdb_num < ulogentries {
        (*ulog).kdb_num = (*ulog).kdb_num.wrapping_add(1)
    } else {
        /* We are circling; set kdb_first_sno and time to the next update. */
        i = (*upd).kdb_entry_sno.wrapping_rem(ulogentries);
        indx_log =
            (ulog as
                 *mut libc::c_char).offset(::std::mem::size_of::<kdb_hlog_t>()
                                               as libc::c_ulong as
                                               isize).offset(i.wrapping_mul((*ulog).kdb_block
                                                                                as
                                                                                libc::c_uint)
                                                                 as isize) as
                *mut libc::c_void as *mut kdb_ent_header_t;
        (*ulog).kdb_first_sno = (*indx_log).kdb_entry_sno;
        (*ulog).kdb_first_time = (*indx_log).kdb_time
    }
    (*ulog).kdb_state = 1 as libc::c_int as uint16_t;
    sync_header(ulog);
    return 0 as libc::c_int;
}
/* Add an entry to the update log. */
#[no_mangle]
#[c2rust::src_loc = "343:1"]
pub unsafe extern "C" fn ulog_add_update(mut context: krb5_context,
                                         mut upd: *mut kdb_incr_update_t)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut log_ctx: *mut kdb_log_context = 0 as *mut kdb_log_context;
    let mut ulog: *mut kdb_hlog_t = 0 as *mut kdb_hlog_t;
    log_ctx = (*context).kdblog_context;
    if !log_ctx.is_null() {
    } else {
        __assert_fail(b"log_ctx != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"kdb_log.c\x00" as *const u8 as *const libc::c_char,
                      350 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 67],
                                                &[libc::c_char; 67]>(b"krb5_error_code ulog_add_update(krb5_context, kdb_incr_update_t *)\x00")).as_ptr());
    }
    ulog = (*log_ctx).ulog;
    if !ulog.is_null() {
    } else {
        __assert_fail(b"ulog != NULL\x00" as *const u8 as *const libc::c_char,
                      b"kdb_log.c\x00" as *const u8 as *const libc::c_char,
                      350 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 67],
                                                &[libc::c_char; 67]>(b"krb5_error_code ulog_add_update(krb5_context, kdb_incr_update_t *)\x00")).as_ptr());
    }
    ret = lock_ulog(context, 0x2 as libc::c_int);
    if ret != 0 { return ret }
    /* If we have reached the last possible serial number, reinitialize the
     * ulog and start over.  Replicas will do a full resync. */
    if (*ulog).kdb_last_sno == -(1 as libc::c_int) as kdb_sno_t {
        reset_ulog(log_ctx);
    }
    (*upd).kdb_entry_sno =
        (*ulog).kdb_last_sno.wrapping_add(1 as libc::c_int as libc::c_uint);
    time_current(&mut (*upd).kdb_time);
    ret = store_update(log_ctx, upd);
    unlock_ulog(context);
    return ret;
}
/* Used by the replica to update its hash db from the incr update log. */
#[no_mangle]
#[c2rust::src_loc = "368:1"]
pub unsafe extern "C" fn ulog_replay(mut context: krb5_context,
                                     mut incr_ret: *mut kdb_incr_result_t,
                                     mut db_args: *mut *mut libc::c_char)
 -> krb5_error_code {
    let mut entry: *mut krb5_db_entry = 0 as *mut krb5_db_entry;
    let mut upd: *mut kdb_incr_update_t = 0 as *mut kdb_incr_update_t;
    let mut fupd: *mut kdb_incr_update_t = 0 as *mut kdb_incr_update_t;
    let mut i: libc::c_int = 0;
    let mut no_of_updates: libc::c_int = 0;
    let mut retval: krb5_error_code = 0;
    let mut dbprinc: krb5_principal = 0 as *mut krb5_principal_data;
    let mut dbprincstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut log_ctx: *mut kdb_log_context = 0 as *mut kdb_log_context;
    let mut ulog: *mut kdb_hlog_t = 0 as *mut kdb_hlog_t;
    log_ctx = (*context).kdblog_context;
    if !log_ctx.is_null() {
    } else {
        __assert_fail(b"log_ctx != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"kdb_log.c\x00" as *const u8 as *const libc::c_char,
                      380 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 72],
                                                &[libc::c_char; 72]>(b"krb5_error_code ulog_replay(krb5_context, kdb_incr_result_t *, char **)\x00")).as_ptr());
    }
    ulog = (*log_ctx).ulog;
    if !ulog.is_null() {
    } else {
        __assert_fail(b"ulog != NULL\x00" as *const u8 as *const libc::c_char,
                      b"kdb_log.c\x00" as *const u8 as *const libc::c_char,
                      380 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 72],
                                                &[libc::c_char; 72]>(b"krb5_error_code ulog_replay(krb5_context, kdb_incr_result_t *, char **)\x00")).as_ptr());
    }
    retval =
        krb5_db_open(context, db_args,
                     0 as libc::c_int | 0x200 as libc::c_int);
    if retval != 0 { return retval }
    no_of_updates = (*incr_ret).updates.kdb_ulog_t_len as libc::c_int;
    upd = (*incr_ret).updates.kdb_ulog_t_val;
    fupd = upd;
    i = 0 as libc::c_int;
    while i < no_of_updates {
        if !((*upd).kdb_commit == 0) {
            /* Replay this update in the database. */
            if (*upd).kdb_deleted != 0 {
                dbprincstr =
                    k5memdup0((*upd).kdb_princ_name.utf8str_t_val as
                                  *const libc::c_void,
                              (*upd).kdb_princ_name.utf8str_t_len as size_t,
                              &mut retval) as *mut libc::c_char;
                if dbprincstr.is_null() { break ; }
                retval = krb5_parse_name(context, dbprincstr, &mut dbprinc);
                free(dbprincstr as *mut libc::c_void);
                if retval != 0 { break ; }
                retval = krb5int_delete_principal_no_log(context, dbprinc);
                krb5_free_principal(context, dbprinc);
                if retval as libc::c_long == -(1780008443 as libc::c_long) {
                    retval = 0 as libc::c_int
                }
                if retval != 0 { break ; }
            } else {
                retval = ulog_conv_2dbentry(context, &mut entry, upd);
                if retval != 0 { break ; }
                retval = krb5int_put_principal_no_log(context, entry);
                krb5_db_free_principal(context, entry);
                if retval != 0 { break ; }
            }
            retval = lock_ulog(context, 0x2 as libc::c_int);
            if retval != 0 { break ; }
            /* If (unexpectedly) this update does not follow the last one we
         * stored, discard any previous ulog state. */
            if (*ulog).kdb_num != 0 as libc::c_int as libc::c_uint &&
                   (*upd).kdb_entry_sno !=
                       (*ulog).kdb_last_sno.wrapping_add(1 as libc::c_int as
                                                             libc::c_uint) {
                reset_ulog(log_ctx);
            }
            /* Store this update in the ulog for any downstream KDCs. */
            retval = store_update(log_ctx, upd);
            unlock_ulog(context);
            if retval != 0 { break ; }
            upd = upd.offset(1)
        }
        i += 1
    }
    if retval != 0 { ulog_init_header(context); }
    if !fupd.is_null() { ulog_free_entries(fupd, no_of_updates); }
    return retval;
}
/* Reinitialize the log header. */
#[no_mangle]
#[c2rust::src_loc = "451:1"]
pub unsafe extern "C" fn ulog_init_header(mut context: krb5_context)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut log_ctx: *mut kdb_log_context = 0 as *mut kdb_log_context;
    let mut ulog: *mut kdb_hlog_t = 0 as *mut kdb_hlog_t;
    log_ctx = (*context).kdblog_context;
    if !log_ctx.is_null() {
    } else {
        __assert_fail(b"log_ctx != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"kdb_log.c\x00" as *const u8 as *const libc::c_char,
                      458 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"krb5_error_code ulog_init_header(krb5_context)\x00")).as_ptr());
    }
    ulog = (*log_ctx).ulog;
    if !ulog.is_null() {
    } else {
        __assert_fail(b"ulog != NULL\x00" as *const u8 as *const libc::c_char,
                      b"kdb_log.c\x00" as *const u8 as *const libc::c_char,
                      458 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 47],
                                                &[libc::c_char; 47]>(b"krb5_error_code ulog_init_header(krb5_context)\x00")).as_ptr());
    }
    ret = lock_ulog(context, 0x2 as libc::c_int);
    if ret != 0 { return ret }
    reset_ulog(log_ctx);
    unlock_ulog(context);
    return 0 as libc::c_int;
}
/* Map the log file to memory for performance and simplicity. */
#[no_mangle]
#[c2rust::src_loc = "468:1"]
pub unsafe extern "C" fn ulog_map(mut context: krb5_context,
                                  mut logname: *const libc::c_char,
                                  mut ulogentries: uint32_t)
 -> krb5_error_code {
    let mut current_block: u64;
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
    let mut retval: krb5_error_code = 0;
    let mut filesize: uint32_t = 0;
    let mut log_ctx: *mut kdb_log_context = 0 as *mut kdb_log_context;
    let mut ulog: *mut kdb_hlog_t = 0 as *mut kdb_hlog_t;
    let mut locked: krb5_boolean = 0 as libc::c_int as krb5_boolean;
    log_ctx = create_log_context(context);
    if log_ctx.is_null() { return 12 as libc::c_int }
    if stat(logname, &mut st) == -(1 as libc::c_int) {
        (*log_ctx).ulogfd =
            open(logname, 0o2 as libc::c_int | 0o100 as libc::c_int,
                 0o600 as libc::c_int);
        if (*log_ctx).ulogfd == -(1 as libc::c_int) {
            retval = *__errno_location();
            current_block = 14432450054309240504;
        } else {
            filesize =
                (::std::mem::size_of::<kdb_hlog_t>() as
                     libc::c_ulong).wrapping_add(ulogentries.wrapping_mul(2048
                                                                              as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_uint)
                                                     as libc::c_ulong) as
                    uint32_t;
            retval = extend_file_to((*log_ctx).ulogfd, filesize);
            if retval != 0 {
                current_block = 14432450054309240504;
            } else { current_block = 8457315219000651999; }
        }
    } else {
        (*log_ctx).ulogfd =
            open(logname, 0o2 as libc::c_int, 0o600 as libc::c_int);
        if (*log_ctx).ulogfd == -(1 as libc::c_int) {
            retval = *__errno_location();
            current_block = 14432450054309240504;
        } else { current_block = 8457315219000651999; }
    }
    match current_block {
        8457315219000651999 => {
            ulog =
                mmap(0 as *mut libc::c_void,
                     0x10000000 as libc::c_int as size_t,
                     0x1 as libc::c_int | 0x2 as libc::c_int,
                     0x1 as libc::c_int, (*log_ctx).ulogfd,
                     0 as libc::c_int as __off_t) as *mut kdb_hlog_t;
            if ulog ==
                   -(1 as libc::c_int) as *mut libc::c_void as *mut kdb_hlog_t
               {
                retval = *__errno_location()
            } else {
                (*log_ctx).ulog = ulog;
                (*log_ctx).ulogentries = ulogentries;
                retval = lock_ulog(context, 0x2 as libc::c_int);
                if !(retval != 0) {
                    locked = 1 as libc::c_int as krb5_boolean;
                    if (*ulog).kdb_hmagic !=
                           0x6662323 as libc::c_int as libc::c_uint {
                        if (*ulog).kdb_hmagic !=
                               0 as libc::c_int as libc::c_uint {
                            retval =
                                -(1780008407 as libc::c_long) as
                                    krb5_error_code;
                            current_block = 14432450054309240504;
                        } else {
                            reset_ulog(log_ctx);
                            current_block = 8693738493027456495;
                        }
                    } else { current_block = 8693738493027456495; }
                    match current_block {
                        14432450054309240504 => { }
                        _ => {
                            /* Reinit ulog if ulogentries changed such that we have too many entries or
     * our first or last entry was written to the wrong location. */
                            if (*ulog).kdb_num !=
                                   0 as libc::c_int as libc::c_uint &&
                                   ((*ulog).kdb_num > ulogentries ||
                                        check_sno(log_ctx,
                                                  (*ulog).kdb_first_sno,
                                                  &mut (*ulog).kdb_first_time)
                                            == 0 ||
                                        check_sno(log_ctx,
                                                  (*ulog).kdb_last_sno,
                                                  &mut (*ulog).kdb_last_time)
                                            == 0) {
                                reset_ulog(log_ctx);
                            }
                            if (*ulog).kdb_num != ulogentries {
                                /* Expand the ulog file if it isn't big enough. */
                                filesize =
                                    (::std::mem::size_of::<kdb_hlog_t>() as
                                         libc::c_ulong).wrapping_add(ulogentries.wrapping_mul((*ulog).kdb_block
                                                                                                  as
                                                                                                  libc::c_uint)
                                                                         as
                                                                         libc::c_ulong)
                                        as uint32_t;
                                retval =
                                    extend_file_to((*log_ctx).ulogfd,
                                                   filesize);
                                (retval) != 0;
                            }
                        }
                    }
                }
            }
        }
        _ => { }
    }
    if locked != 0 { unlock_ulog(context); }
    if retval != 0 { ulog_fini(context); }
    return retval;
}
/* Get the last set of updates seen, (last+1) to n is returned. */
#[no_mangle]
#[c2rust::src_loc = "548:1"]
pub unsafe extern "C" fn ulog_get_entries(mut context: krb5_context,
                                          mut last: *const kdb_last_t,
                                          mut ulog_handle:
                                              *mut kdb_incr_result_t)
 -> krb5_error_code {
    let mut current_block: u64;
    let mut xdrs: XDR =
        XDR{x_op: XDR_ENCODE,
            x_ops: 0 as *mut xdr_ops,
            x_public: 0 as *mut libc::c_char,
            x_private: 0 as *mut libc::c_void,
            x_base: 0 as *mut libc::c_char,
            x_handy: 0,};
    let mut indx_log: *mut kdb_ent_header_t = 0 as *mut kdb_ent_header_t;
    let mut upd: *mut kdb_incr_update_t = 0 as *mut kdb_incr_update_t;
    let mut indx: libc::c_uint = 0;
    let mut count: libc::c_uint = 0;
    let mut sno: uint32_t = 0;
    let mut retval: krb5_error_code = 0;
    let mut log_ctx: *mut kdb_log_context = 0 as *mut kdb_log_context;
    let mut ulog: *mut kdb_hlog_t = 0 as *mut kdb_hlog_t;
    let mut ulogentries: uint32_t = 0;
    log_ctx = (*context).kdblog_context;
    if !log_ctx.is_null() {
    } else {
        __assert_fail(b"log_ctx != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"kdb_log.c\x00" as *const u8 as *const libc::c_char,
                      562 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 88],
                                                &[libc::c_char; 88]>(b"krb5_error_code ulog_get_entries(krb5_context, const kdb_last_t *, kdb_incr_result_t *)\x00")).as_ptr());
    }
    ulog = (*log_ctx).ulog;
    if !ulog.is_null() {
    } else {
        __assert_fail(b"ulog != NULL\x00" as *const u8 as *const libc::c_char,
                      b"kdb_log.c\x00" as *const u8 as *const libc::c_char,
                      562 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 88],
                                                &[libc::c_char; 88]>(b"krb5_error_code ulog_get_entries(krb5_context, const kdb_last_t *, kdb_incr_result_t *)\x00")).as_ptr());
    }
    ulogentries = (*log_ctx).ulogentries;
    retval = lock_ulog(context, 0x1 as libc::c_int);
    if retval != 0 { return retval }
    /* If another process terminated mid-update, reset the ulog and force full
     * resyncs. */
    if (*ulog).kdb_state as libc::c_int != 1 as libc::c_int {
        reset_ulog(log_ctx);
    }
    (*ulog_handle).ret = get_sno_status(log_ctx, last);
    if !((*ulog_handle).ret as libc::c_uint !=
             UPDATE_OK as libc::c_int as libc::c_uint) {
        sno = (*last).last_sno;
        count = (*ulog).kdb_last_sno.wrapping_sub(sno);
        upd =
            calloc(count as libc::c_ulong,
                   ::std::mem::size_of::<kdb_incr_update_t>() as
                       libc::c_ulong) as *mut kdb_incr_update_t;
        if upd.is_null() {
            (*ulog_handle).ret = UPDATE_ERROR;
            retval = 12 as libc::c_int
        } else {
            (*ulog_handle).updates.kdb_ulog_t_val = upd;
            loop  {
                if !(sno < (*ulog).kdb_last_sno) {
                    current_block = 3275366147856559585;
                    break ;
                }
                indx = sno.wrapping_rem(ulogentries);
                indx_log =
                    (ulog as
                         *mut libc::c_char).offset(::std::mem::size_of::<kdb_hlog_t>()
                                                       as libc::c_ulong as
                                                       isize).offset(indx.wrapping_mul((*ulog).kdb_block
                                                                                           as
                                                                                           libc::c_uint)
                                                                         as
                                                                         isize)
                        as *mut libc::c_void as *mut kdb_ent_header_t;
                memset(upd as *mut libc::c_void, 0 as libc::c_int,
                       ::std::mem::size_of::<kdb_incr_update_t>() as
                           libc::c_ulong);
                gssrpc_xdrmem_create(&mut xdrs,
                                     (*indx_log).entry_data.as_mut_ptr() as
                                         *mut libc::c_char,
                                     (*indx_log).kdb_entry_size, XDR_DECODE);
                if xdr_kdb_incr_update_t(&mut xdrs, upd) == 0 {
                    (*ulog_handle).ret = UPDATE_ERROR;
                    retval = -(1780008409 as libc::c_long) as krb5_error_code;
                    current_block = 5498835644851925448;
                    break ;
                } else {
                    /* Mark commitment since we didn't want to decode and encode the incr
         * update record the first time. */
                    (*upd).kdb_commit = (*indx_log).kdb_commit;
                    upd = upd.offset(1);
                    sno = sno.wrapping_add(1)
                }
            }
            match current_block {
                5498835644851925448 => { }
                _ => {
                    (*ulog_handle).updates.kdb_ulog_t_len = count;
                    (*ulog_handle).lastentry.last_sno = (*ulog).kdb_last_sno;
                    (*ulog_handle).lastentry.last_time.seconds =
                        (*ulog).kdb_last_time.seconds;
                    (*ulog_handle).lastentry.last_time.useconds =
                        (*ulog).kdb_last_time.useconds;
                    (*ulog_handle).ret = UPDATE_OK
                }
            }
        }
    }
    unlock_ulog(context);
    return retval;
}
#[no_mangle]
#[c2rust::src_loc = "619:1"]
pub unsafe extern "C" fn ulog_set_role(mut ctx: krb5_context,
                                       mut role: iprop_role)
 -> krb5_error_code {
    if create_log_context(ctx).is_null() { return 12 as libc::c_int }
    (*(*ctx).kdblog_context).iproprole = role;
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "628:1"]
pub unsafe extern "C" fn ulog_get_sno_status(mut context: krb5_context,
                                             mut last: *const kdb_last_t)
 -> update_status_t {
    let mut status: update_status_t = UPDATE_OK;
    if lock_ulog(context, 0x1 as libc::c_int) != 0 as libc::c_int {
        return UPDATE_ERROR
    }
    status = get_sno_status((*context).kdblog_context, last);
    unlock_ulog(context);
    return status;
}
#[no_mangle]
#[c2rust::src_loc = "640:1"]
pub unsafe extern "C" fn ulog_get_last(mut context: krb5_context,
                                       mut last_out: *mut kdb_last_t)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut log_ctx: *mut kdb_log_context = 0 as *mut kdb_log_context;
    let mut ulog: *mut kdb_hlog_t = 0 as *mut kdb_hlog_t;
    log_ctx = (*context).kdblog_context;
    if !log_ctx.is_null() {
    } else {
        __assert_fail(b"log_ctx != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"kdb_log.c\x00" as *const u8 as *const libc::c_char,
                      647 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 58],
                                                &[libc::c_char; 58]>(b"krb5_error_code ulog_get_last(krb5_context, kdb_last_t *)\x00")).as_ptr());
    }
    ulog = (*log_ctx).ulog;
    if !ulog.is_null() {
    } else {
        __assert_fail(b"ulog != NULL\x00" as *const u8 as *const libc::c_char,
                      b"kdb_log.c\x00" as *const u8 as *const libc::c_char,
                      647 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 58],
                                                &[libc::c_char; 58]>(b"krb5_error_code ulog_get_last(krb5_context, kdb_last_t *)\x00")).as_ptr());
    }
    ret = lock_ulog(context, 0x1 as libc::c_int);
    if ret != 0 { return ret }
    (*last_out).last_sno = (*(*log_ctx).ulog).kdb_last_sno;
    (*last_out).last_time = (*(*log_ctx).ulog).kdb_last_time;
    unlock_ulog(context);
    return 0 as libc::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "657:1"]
pub unsafe extern "C" fn ulog_set_last(mut context: krb5_context,
                                       mut last: *const kdb_last_t)
 -> krb5_error_code {
    let mut ret: krb5_error_code = 0;
    let mut log_ctx: *mut kdb_log_context = 0 as *mut kdb_log_context;
    let mut ulog: *mut kdb_hlog_t = 0 as *mut kdb_hlog_t;
    log_ctx = (*context).kdblog_context;
    if !log_ctx.is_null() {
    } else {
        __assert_fail(b"log_ctx != NULL\x00" as *const u8 as
                          *const libc::c_char,
                      b"kdb_log.c\x00" as *const u8 as *const libc::c_char,
                      664 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 64],
                                                &[libc::c_char; 64]>(b"krb5_error_code ulog_set_last(krb5_context, const kdb_last_t *)\x00")).as_ptr());
    }
    ulog = (*log_ctx).ulog;
    if !ulog.is_null() {
    } else {
        __assert_fail(b"ulog != NULL\x00" as *const u8 as *const libc::c_char,
                      b"kdb_log.c\x00" as *const u8 as *const libc::c_char,
                      664 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 64],
                                                &[libc::c_char; 64]>(b"krb5_error_code ulog_set_last(krb5_context, const kdb_last_t *)\x00")).as_ptr());
    }
    ret = lock_ulog(context, 0x2 as libc::c_int);
    if ret != 0 { return ret }
    set_dummy(log_ctx, (*last).last_sno, &(*last).last_time);
    sync_header(ulog);
    unlock_ulog(context);
    return 0 as libc::c_int;
}
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
#[c2rust::src_loc = "675:1"]
pub unsafe extern "C" fn ulog_fini(mut context: krb5_context) {
    let mut log_ctx: *mut kdb_log_context = (*context).kdblog_context;
    if log_ctx.is_null() { return }
    if !(*log_ctx).ulog.is_null() {
        munmap((*log_ctx).ulog as *mut libc::c_void,
               0x10000000 as libc::c_int as size_t);
    }
    if (*log_ctx).ulogfd != -(1 as libc::c_int) { close((*log_ctx).ulogfd); }
    free(log_ctx as *mut libc::c_void);
    (*context).kdblog_context = 0 as *mut _kdb_log_context;
}
