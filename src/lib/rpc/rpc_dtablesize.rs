use ::libc;
#[c2rust::header_src = "/usr/include/bits/confname.h:38"]
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
#[c2rust::header_src = "/usr/include/unistd.h:38"]
pub mod unistd_h {
    extern "C" {
        #[no_mangle]
        #[c2rust::src_loc = "619:1"]
        pub fn sysconf(__name: libc::c_int) -> libc::c_long;
    }
}
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
use self::unistd_h::sysconf;
/* @(#)rpc.h	2.3 88/08/10 4.0 RPCSRC; from 1.9 88/02/08 SMI */
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
 * rpc.h, Just includes the billions of rpc header files necessary to
 * do remote procedure calling.
 */
/* external data representation interfaces */
/* Client side only authentication */
/* Client side (mostly) remote procedure call */
/* semi-private protocol headers */
/* Server side only remote procedure callee */
/*
 * get the local host's IP address without consulting
 * name service library functions
 */
/* @(#)rpc_dtablesize.c	2.1 88/07/29 4.0 RPCSRC */
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
 * Cache the result of getdtablesize(), so we don't have to do an
 * expensive system call every time.
 */
#[no_mangle]
#[c2rust::src_loc = "45:1"]
pub unsafe extern "C" fn gssrpc__rpc_dtablesize() -> libc::c_int {
    static mut size: libc::c_int = 0;
    if size == 0 as libc::c_int {
        size = sysconf(_SC_OPEN_MAX as libc::c_int) as libc::c_int;
        /* sysconf() can return a number larger than what will fit in an
   fd_set.  we can't use fd's larger than this, anyway. */
        if size >= 1024 as libc::c_int {
            size = 1024 as libc::c_int - 1 as libc::c_int
        }
    }
    return size;
}
