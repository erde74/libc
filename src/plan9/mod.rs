use core::option::Option;

pub use ffi::c_void;

pub type c_char = u8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type c_ulong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;

pub type uint = c_uint;
pub type uintptr = u64;

pub type Rune = u32;
pub type jmp_buf = [usize; 10usize];
pub type mpdigit = c_uint;

pub type va_list = __builtin_va_list;
pub type __builtin_va_list = [__va_list_tag; 1usize];

s! {
    pub struct __va_list_tag {
        pub gp_offset: c_uint,
        pub fp_offset: c_uint,
        pub overflow_arg_area: *mut c_void,
        pub reg_save_area: *mut c_void,
    }

    pub struct IOchunk {
        pub addr: *mut c_void,
        pub len: u32,
    }

    pub struct timespec {
        pub tv_sec: i64,
        pub tv_nsec: i64,
    }

    pub struct Fmt {
        pub runes: u8,
        pub start: *mut c_void,
        pub to: *mut c_void,
        pub stop: *mut c_void,
        pub flush:
            Option<unsafe extern "C" fn(arg1: *mut Fmt) -> c_int>,
        pub farg: *mut c_void,
        pub nfmt: c_int,
        pub args: va_list,
        pub r: c_int,
        pub width: c_int,
        pub prec: c_int,
        pub flags: u32,
    }

    pub struct Tm {
        pub sec: c_int,
        pub min: c_int,
        pub hour: c_int,
        pub mday: c_int,
        pub mon: c_int,
        pub year: c_int,
        pub wday: c_int,
        pub yday: c_int,
        pub zone: [c_char; 4usize],
        pub tzoff: c_int,
    }

    pub struct Lock {
        pub key: i32,
        pub sem: i32,
    }

    pub struct QLp {
        pub inuse: c_int,
        pub next: *mut QLp,
        pub state: c_char,
    }

    pub struct QLock {
        pub lock: Lock,
        pub locked: c_int,
        pub head: *mut QLp,
        pub tail: *mut QLp,
    }

    pub struct RWLock {
        pub lock: Lock,
        pub _readers: c_int,
        pub writer: c_int,
        pub head: *mut QLp,
        pub tail: *mut QLp,
    }

    pub struct Rendez {
        pub l: *mut QLock,
        pub head: *mut QLp,
        pub tail: *mut QLp,
    }

    pub struct NetConnInfo {
        pub dir: *mut c_char,
        pub root: *mut c_char,
        pub spec: *mut c_char,
        pub lsys: *mut c_char,
        pub lserv: *mut c_char,
        pub rsys: *mut c_char,
        pub rserv: *mut c_char,
        pub laddr: *mut c_char,
        pub raddr: *mut c_char,
    }

    pub struct Qid {
        pub path: u64,
        pub vers: u32,
        pub type_: u8,
    }

    pub struct Dir {
        pub type_: u16,
        pub dev: uint,
        pub qid: Qid,
        pub mode: u32,
        pub atime: u32,
        pub mtime: u32,
        pub length: i64,
        pub name: *mut c_char,
        pub uid: *mut c_char,
        pub gid: *mut c_char,
        pub muid: *mut c_char,
    }

    pub struct Waitmsg {
        pub pid: c_int,
        pub time: [u32; 3usize],
        pub msg: *mut c_char,
    }

    pub struct PSlice {
        pub ptrs: *mut *mut c_void,
        pub len: size_t,
        pub capacity: size_t,
    }
    // stdio
    pub struct FILE {
        pub fd: c_int,
        pub flags: c_char,
        pub state: c_char,
        pub buf: *mut c_char,
        pub rp: *mut c_char,
        pub wp: *mut c_char,
        pub lp: *mut c_char,
        pub bufl: i32,
        pub unbuf: [c_char; 1usize],
    }
}

pub const EBADF: i32 = 9;

// stdio file descriptor numbers
pub const STDIN_FILENO: ::c_int = 0;
pub const STDOUT_FILENO: ::c_int = 1;
pub const STDERR_FILENO: ::c_int = 2;

pub const UTFmax: c_uint = 4;
pub const Runesync: c_uint = 128;
pub const Runeself: c_uint = 128;
pub const Runeerror: c_uint = 65533;
pub const Runemax: c_uint = 1114111;
pub const Runemask: c_uint = 2097151;

pub const FmtWidth: c_uint = 1;
pub const FmtLeft: c_uint = 2;
pub const FmtPrec: c_uint = 4;
pub const FmtSharp: c_uint = 8;
pub const FmtSpace: c_uint = 16;
pub const FmtSign: c_uint = 32;
pub const FmtZero: c_uint = 64;
pub const FmtUnsigned: c_uint = 128;
pub const FmtShort: c_uint = 256;
pub const FmtLong: c_uint = 512;
pub const FmtVLong: c_uint = 1024;
pub const FmtComma: c_uint = 2048;
pub const FmtByte: c_uint = 4096;
pub const FmtFlag: c_uint = 8192;

pub const Huge: f64 = 3.4028234e38;
pub const PIO2: f64 = 1.570796326794896619231e0;
pub const PI: f64 = PIO2 + PIO2;

pub const PNPROC: c_uint = 1;
pub const PNGROUP: c_uint = 2;

pub const RFNAMEG: c_uint = 1;
pub const RFENVG: c_uint = 2;
pub const RFFDG: c_uint = 4;
pub const RFNOTEG: c_uint = 8;
pub const RFPROC: c_uint = 16;
pub const RFMEM: c_uint = 32;
pub const RFNOWAIT: c_uint = 64;
pub const RFCNAMEG: c_uint = 1024;
pub const RFCENVG: c_uint = 2048;
pub const RFCFDG: c_uint = 4096;
pub const RFREND: c_uint = 8192;
pub const RFNOMNT: c_uint = 16384;

pub const STATMAX: u32 = 65535;
pub const ERRMAX: u32 = 128;

pub const MORDER: u32 = 3;
pub const MREPL: u32 = 0;
pub const MBEFORE: u32 = 1;
pub const MAFTER: u32 = 2;
pub const MCREATE: u32 = 4;
pub const MCACHE: u32 = 16;
pub const MMASK: u32 = 23;

pub const OREAD: u32 = 0;
pub const OWRITE: u32 = 1;
pub const ORDWR: u32 = 2;
pub const OEXEC: u32 = 3;
pub const OTRUNC: u32 = 16;
pub const OCEXEC: u32 = 32;
pub const ORCLOSE: u32 = 64;
pub const OEXCL: u32 = 4096;

pub const AEXIST: u32 = 0;
pub const AEXEC: u32 = 1;
pub const AWRITE: u32 = 2;
pub const AREAD: u32 = 4;

pub const SG_RONLY: u32 = 32;
pub const SG_CEXEC: u32 = 64;

pub const NCONT: u32 = 0;
pub const NDFLT: u32 = 1;
pub const NSAVE: u32 = 2;
pub const NRSTR: u32 = 3;

pub const QTDIR: u32 = 128;
pub const QTAPPEND: u32 = 64;
pub const QTEXCL: u32 = 32;
pub const QTMOUNT: u32 = 16;
pub const QTAUTH: u32 = 8;
pub const QTTMP: u32 = 4;
pub const QTSYMLINK: u32 = 2;
pub const QTFILE: u32 = 0;

// Dir.mode bits
pub const DMDIR: u32 = 0x80000000; // directories
pub const DMAPPEND: u32 = 0x40000000; // append only files
pub const DMEXCL: u32 = 0x20000000; // exclusive use files
pub const DMMOUNT: u32 = 0x10000000; // mounted channel
pub const DMAUTH: u32 = 0x08000000; // authentication file
pub const DMTMP: u32 = 0x04000000; // non-backed-up files
pub const DMSYMLINK: u32 = 0x02000000; // symlnk
pub const DMREAD: u32 = 0x4; // read permission
pub const DMWRITE: u32 = 0x2; // write permission
pub const DMEXEC: u32 = 0x1; // execute permission

extern "C" {

    // mem routines
    pub fn memccpy(
        dest: *mut c_void,
        src: *const c_void,
        c: c_int,
        n: u32,
    ) -> *mut c_void;

    pub fn memset(dest: *mut c_void, c: c_int, n: c_ulong) -> *mut c_void;

    pub fn memcmp(cx: *const c_void, ct: *const c_void, n: u32) -> c_int;

    pub fn memcpy(
        dest: *mut c_void,
        src: *const c_void,
        n: c_ulong,
    ) -> *mut c_void;

    pub fn memmove(
        dest: *mut c_void,
        src: *const c_void,
        n: c_ulong,
    ) -> *mut c_void;

    pub fn memchr(cx: *const c_void, c: c_int, n: u32) -> *mut c_void;

    // string routines

    pub fn strcat(s: *mut c_char, ct: *const c_char) -> *mut c_char;

    pub fn strchr(cs: *const c_char, c: c_int) -> *mut c_char;

    pub fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;

    pub fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;

    pub fn strecpy(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *const c_char,
    ) -> *mut c_char;

    pub fn strdup(cs: *const c_char) -> *mut c_char;

    pub fn strlcat(
        arg1: *mut c_char,
        arg2: *const c_char,
        arg3: c_ulong,
    ) -> c_ulong;

    pub fn strlcpy(
        arg1: *mut c_char,
        arg2: *const c_char,
        arg3: c_ulong,
    ) -> c_ulong;

    pub fn strncat(
        arg1: *mut c_char,
        arg2: *const c_char,
        arg3: i32,
    ) -> *mut c_char;

    pub fn strncpy(
        arg1: *mut c_char,
        arg2: *const c_char,
        arg3: u32,
    ) -> *mut c_char;

    pub fn strncmp(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: i32,
    ) -> c_int;

    pub fn strpbrk(arg1: *const c_char, arg2: *const c_char) -> *mut c_char;

    pub fn strrchr(arg1: *const c_char, arg2: c_int) -> *mut c_char;

    pub fn strtok(arg1: *mut c_char, arg2: *mut c_char) -> *mut c_char;

    pub fn strlen(arg1: *const c_char) -> c_int;

    pub fn strspn(arg1: *const c_char, arg2: *const c_char) -> c_int;

    pub fn strcspn(arg1: *const c_char, arg2: *const c_char) -> c_int;

    pub fn strstr(arg1: *const c_char, arg2: *const c_char) -> *mut c_char;

    pub fn cistrncmp(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: c_int,
    ) -> c_int;

    pub fn cistrcmp(arg1: *const c_char, arg2: *const c_char) -> c_int;

    pub fn cistrstr(arg1: *const c_char, arg2: *const c_char) -> *mut c_char;

    pub fn tokenize(
        arg1: *mut c_char,
        arg2: *mut *mut c_char,
        arg3: c_int,
    ) -> c_int;

    // rune routines

    pub fn runetochar(arg1: *mut c_char, arg2: *const Rune) -> c_int;

    pub fn chartorune(arg1: *mut Rune, arg2: *const c_char) -> c_int;

    pub fn runelen(arg1: Rune) -> c_int;
    pub fn runenlen(arg1: *const Rune, arg2: c_int) -> c_int;

    pub fn fullrune(arg1: *const c_char, arg2: c_int) -> c_int;

    pub fn utflen(arg1: *const c_char) -> c_int;
    pub fn utfnlen(arg1: *const c_char, arg2: i32) -> c_int;
    pub fn utfrune(arg1: *const c_char, arg2: Rune) -> *mut c_char;
    pub fn utfrrune(arg1: *const c_char, arg2: Rune) -> *mut c_char;

    pub fn utfutf(arg1: *const c_char, arg2: *const c_char) -> *mut c_char;

    pub fn utfecpy(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *const c_char,
    ) -> *mut c_char;

    pub fn runestrcat(arg1: *mut Rune, arg2: *const Rune) -> *mut Rune;
    pub fn runestrchr(arg1: *const Rune, arg2: Rune) -> *mut Rune;
    pub fn runestrcmp(arg1: *const Rune, arg2: *const Rune) -> c_int;
    pub fn runestrcpy(arg1: *mut Rune, arg2: *const Rune) -> *mut Rune;
    pub fn runestrncpy(
        arg1: *mut Rune,
        arg2: *const Rune,
        arg3: i32,
    ) -> *mut Rune;
    pub fn runestrecpy(
        arg1: *mut Rune,
        arg2: *mut Rune,
        arg3: *const Rune,
    ) -> *mut Rune;
    pub fn runestrdup(arg1: *const Rune) -> *mut Rune;
    pub fn runestrncat(
        arg1: *mut Rune,
        arg2: *const Rune,
        arg3: i32,
    ) -> *mut Rune;
    pub fn runestrncmp(
        arg1: *const Rune,
        arg2: *const Rune,
        arg3: i32,
    ) -> c_int;
    pub fn runestrrchr(arg1: *const Rune, arg2: Rune) -> *mut Rune;
    pub fn runestrlen(arg1: *const Rune) -> i32;
    pub fn runestrstr(arg1: *const Rune, arg2: *const Rune) -> *mut Rune;
    pub fn tolowerrune(arg1: Rune) -> Rune;
    pub fn totitlerune(arg1: Rune) -> Rune;
    pub fn toupperrune(arg1: Rune) -> Rune;
    pub fn tobaserune(arg1: Rune) -> Rune;
    pub fn isalpharune(arg1: Rune) -> c_int;
    pub fn isbaserune(arg1: Rune) -> c_int;
    pub fn isdigitrune(arg1: Rune) -> c_int;
    pub fn islowerrune(arg1: Rune) -> c_int;
    pub fn isspacerune(arg1: Rune) -> c_int;
    pub fn istitlerune(arg1: Rune) -> c_int;
    pub fn isupperrune(arg1: Rune) -> c_int;

    // malloc
    pub fn malloc(size: usize) -> *mut c_void;
    pub fn mallocz(arg1: u32, arg2: c_int) -> *mut c_void;
    pub fn free(v: *mut c_void);
    pub fn msize(arg1: *mut c_void) -> u32;
    pub fn mallocalign(
        size: u32,
        align: u32,
        offset: i32,
        span: u32,
    ) -> *mut c_void;
    pub fn calloc(arg1: u32, arg2: size_t) -> *mut c_void;
    pub fn realloc(v: *mut c_void, size: usize) -> *mut c_void;
    pub fn setmalloctag(arg1: *mut c_void, arg2: usize);
    pub fn setrealloctag(arg1: *mut c_void, arg2: usize);
    pub fn getmalloctag(arg1: *mut c_void) -> usize;
    pub fn getrealloctag(arg1: *mut c_void) -> usize;
    pub fn malloctopoolblock(arg1: *mut c_void) -> *mut c_void;

    // print routines

    pub fn print(format: *const c_char, ...) -> c_int;

    pub fn seprint(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *const c_char,
        ...
    ) -> *mut c_char;

    pub fn vseprint(
        arg1: *mut c_char,
        arg2: *mut c_char,
        arg3: *const c_char,
        arg4: *mut __va_list_tag,
    ) -> *mut c_char;

    pub fn snprint(
        arg1: *mut c_char,
        arg2: c_int,
        arg3: *const c_char,
        ...
    ) -> c_int;

    pub fn vsnprint(
        arg1: *mut c_char,
        arg2: c_int,
        arg3: *const c_char,
        arg4: *mut __va_list_tag,
    ) -> c_int;

    pub fn smprint(arg1: *const c_char, ...) -> *mut c_char;

    pub fn vsmprint(
        arg1: *const c_char,
        arg2: *mut __va_list_tag,
    ) -> *mut c_char;

    pub fn sprint(arg1: *mut c_char, arg2: *const c_char, ...) -> c_int;

    pub fn fprint(arg1: c_int, arg2: *const c_char, ...) -> c_int;

    pub fn vfprint(
        arg1: c_int,
        arg2: *const c_char,
        arg3: *mut __va_list_tag,
    ) -> c_int;

    pub fn runesprint(arg1: *mut Rune, arg2: *const c_char, ...) -> c_int;

    pub fn runesnprint(
        arg1: *mut Rune,
        arg2: c_int,
        arg3: *const c_char,
        ...
    ) -> c_int;

    pub fn runevsnprint(
        arg1: *mut Rune,
        arg2: c_int,
        arg3: *const c_char,
        arg4: *mut __va_list_tag,
    ) -> c_int;

    pub fn runeseprint(
        arg1: *mut Rune,
        arg2: *mut Rune,
        arg3: *const c_char,
        ...
    ) -> *mut Rune;

    pub fn runevseprint(
        arg1: *mut Rune,
        arg2: *mut Rune,
        arg3: *const c_char,
        arg4: *mut __va_list_tag,
    ) -> *mut Rune;

    pub fn runesmprint(arg1: *const c_char, ...) -> *mut Rune;
    pub fn runevsmprint(
        arg1: *const c_char,
        arg2: *mut __va_list_tag,
    ) -> *mut Rune;

    pub fn fmtfdinit(
        arg1: *mut Fmt,
        arg2: c_int,
        arg3: *mut c_char,
        arg4: c_int,
    ) -> c_int;

    pub fn fmtfdflush(arg1: *mut Fmt) -> c_int;
    pub fn fmtstrinit(arg1: *mut Fmt) -> c_int;
    pub fn fmtstrflush(arg1: *mut Fmt) -> *mut c_char;
    pub fn runefmtstrinit(arg1: *mut Fmt) -> c_int;
    pub fn runefmtstrflush(arg1: *mut Fmt) -> *mut Rune;

    pub fn fmtinstall(
        arg1: c_int,
        arg2: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut Fmt) -> c_int,
        >,
    ) -> c_int;

    pub fn dofmt(arg1: *mut Fmt, arg2: *const c_char) -> c_int;
    pub fn dorfmt(arg1: *mut Fmt, arg2: *const Rune) -> c_int;

    pub fn fmtprint(arg1: *mut Fmt, arg2: *const c_char, ...) -> c_int;

    pub fn fmtvprint(
        arg1: *mut Fmt,
        arg2: *const c_char,
        arg3: *mut __va_list_tag,
    ) -> c_int;

    pub fn fmtrune(arg1: *mut Fmt, arg2: c_int) -> c_int;
    pub fn fmtstrcpy(arg1: *mut Fmt, arg2: *const c_char) -> c_int;
    pub fn fmtrunestrcpy(arg1: *mut Fmt, arg2: *const Rune) -> c_int;
    pub fn errfmt(f: *mut Fmt) -> c_int;

    // quoted strings

    pub fn unquotestrdup(arg1: *const c_char) -> *mut c_char;
    pub fn unquoterunestrdup(arg1: *const Rune) -> *mut Rune;
    pub fn quotestrdup(arg1: *const c_char) -> *mut c_char;
    pub fn quoterunestrdup(arg1: *const Rune) -> *mut Rune;
    pub fn quotestrfmt(arg1: *mut Fmt) -> c_int;
    pub fn quoterunestrfmt(arg1: *mut Fmt) -> c_int;
    pub fn quotefmtinstall();

    pub static mut doquote:
        ::core::option::Option<unsafe extern "C" fn(arg1: c_int) -> c_int>;

    pub fn needsrcquote(arg1: c_int) -> c_int;

    // random number

    pub fn srand(arg1: i32);
    pub fn rand() -> c_int;
    pub fn nrand(arg1: c_int) -> c_int;
    pub fn lrand() -> i32;
    pub fn lnrand(arg1: i32) -> i32;
    pub fn frand() -> f64;
    pub fn truerand() -> u32;
    pub fn ntruerand(arg1: u32) -> u32;

    // math

    pub fn getfcr() -> u32;
    pub fn setfsr(arg1: u32);
    pub fn getfsr() -> u32;
    pub fn setfcr(arg1: u32);
    pub fn NaN() -> f64;
    pub fn Inf(arg1: c_int) -> f64;
    pub fn isNaN(arg1: f64) -> c_int;
    pub fn isInf(arg1: f64, arg2: c_int) -> c_int;
    pub fn umuldiv(arg1: u32, arg2: u32, arg3: u32) -> u32;
    pub fn muldiv(arg1: i32, arg2: i32, arg3: i32) -> i32;
    pub fn pow(arg1: f64, arg2: f64) -> f64;
    pub fn atan2(arg1: f64, arg2: f64) -> f64;
    pub fn fabs(arg1: f64) -> f64;
    pub fn atan(arg1: f64) -> f64;
    pub fn log(arg1: f64) -> f64;
    pub fn log10(arg1: f64) -> f64;
    pub fn exp(arg1: f64) -> f64;
    pub fn floor(arg1: f64) -> f64;
    pub fn ceil(arg1: f64) -> f64;
    pub fn hypot(arg1: f64, arg2: f64) -> f64;
    pub fn sin(arg1: f64) -> f64;
    pub fn cos(arg1: f64) -> f64;
    pub fn tan(arg1: f64) -> f64;
    pub fn asin(arg1: f64) -> f64;
    pub fn acos(arg1: f64) -> f64;
    pub fn sinh(arg1: f64) -> f64;
    pub fn tanh(arg1: f64) -> f64;
    pub fn sqrt(arg1: f64) -> f64;
    pub fn fmod(arg1: f64, arg2: f64) -> f64;

    // Time-of-day

    pub fn gmtime(arg1: i32) -> *mut Tm;
    pub fn localtime(arg1: i32) -> *mut Tm;
    pub fn asctime(arg1: *mut Tm) -> *mut c_char;
    pub fn ctime(arg1: i32) -> *mut c_char;
    pub fn cputime() -> f64;
    pub fn times(arg1: *mut i32) -> i32;
    pub fn tm2sec(arg1: *mut Tm) -> i32;
    pub fn nsec() -> i64;
    pub fn cycles(arg1: *mut u64);

    pub fn _assert(arg1: *mut c_char);
    pub fn abs(arg1: c_int) -> c_int;
    pub fn atexit(
        arg1: ::core::option::Option<unsafe extern "C" fn()>,
    ) -> c_int;
    pub fn atexitdont(arg1: ::core::option::Option<unsafe extern "C" fn()>);

    pub fn atnotify(
        arg1: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *mut c_void,
                arg2: *mut c_char,
            ) -> c_int,
        >,
        arg2: c_int,
    ) -> c_int;

    pub fn atof(arg1: *const c_char) -> f64;
    pub fn atoi(arg1: *const c_char) -> c_int;
    pub fn atol(arg1: *const c_char) -> i32;
    pub fn atoll(arg1: *const c_char) -> i64;

    pub fn charstod(
        arg1: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut c_void) -> c_int,
        >,
        arg2: *mut c_void,
    ) -> f64;

    pub fn cleanname(arg1: *mut c_char) -> *mut c_char;

    pub fn decrypt(arg1: *mut c_void, arg2: *mut c_void, arg3: c_int)
        -> c_int;

    pub fn encrypt(arg1: *mut c_void, arg2: *mut c_void, arg3: c_int)
        -> c_int;

    pub fn dec64(
        arg1: *mut u8,
        arg2: c_int,
        arg3: *const c_char,
        arg4: c_int,
    ) -> c_int;

    pub fn enc64(
        arg1: *mut c_char,
        arg2: c_int,
        arg3: *const u8,
        arg4: c_int,
    ) -> c_int;

    pub fn dec32(
        arg1: *mut u8,
        arg2: c_int,
        arg3: *const c_char,
        arg4: c_int,
    ) -> c_int;

    pub fn enc32(
        arg1: *mut c_char,
        arg2: c_int,
        arg3: *const u8,
        arg4: c_int,
    ) -> c_int;

    pub fn dec16(
        arg1: *mut u8,
        arg2: c_int,
        arg3: *const c_char,
        arg4: c_int,
    ) -> c_int;

    pub fn enc16(
        arg1: *mut c_char,
        arg2: c_int,
        arg3: *const u8,
        arg4: c_int,
    ) -> c_int;

    pub fn encodefmt(arg1: *mut Fmt) -> c_int;
    pub fn exits(arg1: *const c_char);
    pub fn frexp(arg1: f64, arg2: *mut c_int) -> f64;
    pub fn getcallstack(arg1: *mut uintptr, arg2: size_t);
    pub fn getenv(arg1: *const c_char) -> *mut c_char;

    pub fn getfields(
        arg1: *mut c_char,
        arg2: *mut *mut c_char,
        arg3: c_int,
        arg4: c_int,
        arg5: *const c_char,
    ) -> c_int;

    pub fn gettokens(
        arg1: *mut c_char,
        arg2: *mut *mut c_char,
        arg3: c_int,
        arg4: *const c_char,
    ) -> c_int;

    pub fn getuser() -> *mut c_char;

    pub fn getwd(arg1: *mut c_char, arg2: c_int) -> *mut c_char;

    pub fn iounit(arg1: c_int) -> c_int;
    pub fn labs(arg1: i32) -> i32;
    pub fn ldexp(arg1: f64, arg2: c_int) -> f64;
    pub fn longjmp(arg1: *mut u64, arg2: c_int);
    pub fn mktemp(arg1: *mut c_char) -> *mut c_char;
    pub fn modf(arg1: f64, arg2: *mut f64) -> f64;

    pub fn netcrypt(arg1: *mut c_void, arg2: *mut c_void) -> c_int;

    pub fn notejmp(arg1: *mut c_void, arg2: *mut u64, arg3: c_int);
    pub fn perror(arg1: *const c_char);

    pub fn postnote(arg1: c_int, arg2: c_int, arg3: *const c_char) -> c_int;

    pub fn pow10(arg1: c_int) -> f64;

    pub fn putenv(arg1: *const c_char, arg2: *const c_char) -> c_int;

    pub fn qsort(
        arg1: *mut c_void,
        arg2: i32,
        arg3: i32,
        arg4: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *const c_void,
                arg2: *const c_void,
            ) -> c_int,
        >,
    );

    pub fn setjmp(arg1: *mut u64) -> c_int;

    pub fn strtod(arg1: *const c_char, arg2: *mut *mut c_char) -> f64;

    pub fn strtol(
        arg1: *const c_char,
        arg2: *mut *mut c_char,
        arg3: c_int,
    ) -> i32;

    pub fn strtoul(
        arg1: *const c_char,
        arg2: *mut *mut c_char,
        arg3: c_int,
    ) -> u32;

    pub fn strtoll(
        arg1: *const c_char,
        arg2: *mut *mut c_char,
        arg3: c_int,
    ) -> c_longlong;

    pub fn strtoull(
        arg1: *const c_char,
        arg2: *mut *mut c_char,
        arg3: c_int,
    ) -> c_ulonglong;

    pub fn sysfatal(arg1: *const c_char, ...);

    pub fn syslog(arg1: c_int, arg2: *const c_char, arg3: *const c_char, ...);

    pub fn time(arg1: *mut i32) -> i32;
    pub fn tolower(arg1: c_int) -> c_int;
    pub fn toupper(arg1: c_int) -> c_int;

    // atomic

    pub fn ainc(arg1: *mut i32) -> i32;
    pub fn adec(arg1: *mut i32) -> i32;
    pub fn cas32(arg1: *mut u32, arg2: u32, arg3: u32) -> c_int;

    pub fn casp(
        arg1: *mut *mut c_void,
        arg2: *mut c_void,
        arg3: *mut c_void,
    ) -> c_int;

    pub fn casl(arg1: *mut u32, arg2: u32, arg3: u32) -> c_int;

    // synchronization

    pub fn _tas(arg1: *mut c_int) -> c_int;
    pub fn _tas32(arg1: *mut c_int) -> c_int;

    pub fn lock(arg1: *mut Lock);
    pub fn unlock(arg1: *mut Lock);
    pub fn canlock(arg1: *mut Lock) -> c_int;

    pub fn qlock(arg1: *mut QLock);
    pub fn qunlock(arg1: *mut QLock);
    pub fn canqlock(arg1: *mut QLock) -> c_int;

    pub fn _qlockinit(
        arg1: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *mut c_void,
                arg2: *mut c_void,
            ) -> *mut c_void,
        >,
    );

    pub fn rlock(arg1: *mut RWLock);
    pub fn runlock(arg1: *mut RWLock);
    pub fn canrlock(arg1: *mut RWLock) -> c_int;
    pub fn wlock(arg1: *mut RWLock);
    pub fn wunlock(arg1: *mut RWLock);
    pub fn canwlock(arg1: *mut RWLock) -> c_int;

    pub fn rsleep(arg1: *mut Rendez);
    pub fn rwakeup(arg1: *mut Rendez) -> c_int;
    pub fn rwakeupall(arg1: *mut Rendez) -> c_int;
    pub fn privalloc() -> *mut *mut c_void;
    pub fn privfree(arg1: *mut *mut c_void);

    // network dialing

    pub fn accept(arg1: c_int, arg2: *const c_char) -> c_int;

    pub fn announce(arg1: *const c_char, arg2: *mut c_char) -> c_int;

    pub fn dial(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: *mut c_char,
        arg4: *mut c_int,
    ) -> c_int;

    pub fn setnetmtpt(arg1: *mut c_char, arg2: c_int, arg3: *const c_char);

    pub fn hangup(arg1: c_int) -> c_int;

    pub fn listen(arg1: *const c_char, arg2: *mut c_char) -> c_int;

    pub fn netmkaddr(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: *const c_char,
    ) -> *mut c_char;

    pub fn reject(
        arg1: c_int,
        arg2: *const c_char,
        arg3: *const c_char,
    ) -> c_int;

    // encryption

    pub fn pushssl(
        arg1: c_int,
        arg2: *const c_char,
        arg3: *const c_char,
        arg4: *const c_char,
        arg5: *mut c_int,
    ) -> c_int;

    pub fn pushtls(
        arg1: c_int,
        arg2: *const c_char,
        arg3: *const c_char,
        arg4: c_int,
        arg5: *const c_char,
        arg6: *mut c_char,
    ) -> c_int;

    pub fn getnetconninfo(
        arg1: *const c_char,
        arg2: c_int,
    ) -> *mut NetConnInfo;

    pub fn freenetconninfo(arg1: *mut NetConnInfo);

    // system calls

    pub fn _exits(arg1: *const c_char);
    pub fn abort() -> !;

    pub fn access(arg1: *const c_char, arg2: c_int) -> c_int;

    pub fn alarm(arg1: u64) -> i64;

    pub fn await(arg1: *mut c_char, arg2: c_int) -> c_int;

    pub fn bind(
        arg1: *const c_char,
        arg2: *const c_char,
        arg3: c_int,
    ) -> c_int;

    pub fn brk(arg1: *mut c_void) -> c_int;
    pub fn chdir(arg1: *const c_char) -> c_int;
    pub fn close(arg1: c_int) -> c_int;

    pub fn create(arg1: *const c_char, arg2: c_int, arg3: u32) -> c_int;

    pub fn dup(arg1: c_int, arg2: c_int) -> c_int;
    pub fn errno2str(errno: uint) -> *mut c_char;
    pub fn errstr(arg1: *mut c_char, arg2: uint) -> c_int;

    pub fn exec(arg1: *const c_char, arg2: *const *mut c_char) -> c_int;

    pub fn execl(arg1: *const c_char, ...) -> c_int;
    pub fn fork() -> c_int;
    pub fn rfork(flags: c_int) -> c_int;

    pub fn fauth(arg1: c_int, arg2: *const c_char) -> c_int;

    pub fn fstat(arg1: c_int, arg2: *mut u8, arg3: c_int) -> c_int;

    pub fn fwstat(arg1: c_int, arg2: *mut u8, arg3: c_int) -> c_int;

    pub fn fversion(
        arg1: c_int,
        arg2: c_int,
        arg3: *mut c_char,
        arg4: c_int,
    ) -> c_int;

    pub fn mount(
        arg1: c_int,
        arg2: c_int,
        arg3: *const c_char,
        arg4: c_int,
        arg5: *const c_char,
        arg6: c_int,
    ) -> c_int;

    pub fn unmount(arg1: *const c_char, arg2: *const c_char) -> c_int;

    pub fn noted(arg1: c_int) -> c_int;

    pub fn notify(
        arg1: ::core::option::Option<
            unsafe extern "C" fn(arg1: *mut c_void, arg2: *mut c_char),
        >,
    ) -> c_int;

    pub fn open(arg1: *const c_char, arg2: c_int) -> c_int;

    pub fn fd2path(arg1: c_int, arg2: *mut c_char, arg3: c_int) -> c_int;

    pub fn pipe(fd: *mut c_int) -> c_int;

    pub fn pread(arg1: c_int, arg2: *mut c_void, arg3: i32, arg4: i64) -> i32;

    pub fn read(fd: c_int, buf: *mut c_void, count: i32) -> i32;

    pub fn preadv(
        arg1: c_int,
        arg2: *mut IOchunk,
        arg3: c_int,
        arg4: i64,
    ) -> i32;

    pub fn pwrite(
        arg1: c_int,
        arg2: *const c_void,
        arg3: i32,
        arg4: i64,
    ) -> i32;

    pub fn write(fd: c_int, buf: *const c_void, count: i32) -> i32;

    pub fn pwritev(
        arg1: c_int,
        arg2: *mut IOchunk,
        arg3: c_int,
        arg4: i64,
    ) -> i32;

    pub fn r0() -> i32;
    pub fn readn(arg1: c_int, arg2: *mut c_void, arg3: i32) -> i32;

    pub fn readv(arg1: c_int, arg2: *mut IOchunk, arg3: c_int) -> i32;

    pub fn remove(arg1: *const c_char) -> c_int;
    pub fn sbrk(arg1: u32) -> *mut c_void;
    pub fn oseek(arg1: c_int, arg2: i32, arg3: c_int) -> i32;
    pub fn seek(arg1: c_int, arg2: i64, arg3: c_int) -> i64;

    pub fn segattach(
        arg1: c_int,
        arg2: *const c_char,
        arg3: *mut c_void,
        arg4: u32,
    ) -> *mut c_void;

    pub fn segbrk(arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;

    pub fn segdetach(arg1: *mut c_void) -> c_int;
    pub fn segflush(arg1: *mut c_void, arg2: u32) -> c_int;
    pub fn segfree(arg1: *mut c_void, arg2: u32) -> c_int;
    pub fn semacquire(arg1: *mut i32, arg2: c_int) -> c_int;
    pub fn semrelease(arg1: *mut i32, arg2: i32) -> i32;
    pub fn sleep(arg1: i32) -> c_int;

    pub fn stat(arg1: *const c_char, arg2: *mut u8, arg3: c_int) -> c_int;

    pub fn tsemacquire(arg1: *mut i32, arg2: u64) -> c_int;
    pub fn wait() -> *mut Waitmsg;
    pub fn waitpid() -> c_int;

    pub fn writev(fd: ::c_int, iov: *const ::IOchunk, iovcnt: ::c_int) -> i32;

    pub fn wstat(arg1: *const c_char, arg2: *mut u8, arg3: c_int) -> c_int;

    pub fn rendezvous(arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;

    pub fn dirstat(arg1: *const c_char) -> *mut Dir;
    pub fn dirfstat(arg1: c_int) -> *mut Dir;
    pub fn dirwstat(arg1: *const c_char, arg2: *mut Dir) -> c_int;
    pub fn dirfwstat(arg1: c_int, arg2: *mut Dir) -> c_int;
    pub fn dirread(arg1: c_int, arg2: *mut *mut Dir) -> i32;
    pub fn nulldir(arg1: *mut Dir);
    pub fn dirreadall(arg1: c_int, arg2: *mut *mut Dir) -> i32;
    pub fn getpid() -> c_int;
    pub fn getppid() -> c_int;
    pub fn rerrstr(arg1: *mut c_char, arg2: uint);
    pub fn sysname() -> *mut c_char;
    pub fn werrstr(arg1: *const c_char, ...);

    pub static mut argv0: *mut c_char;
    pub static mut end: [c_char; 0usize];

    pub fn reallocarray(
        base: *mut c_void,
        nel: size_t,
        size: size_t,
    ) -> *mut c_void;

    pub fn psliceinit(slice: *mut PSlice);
    pub fn psliceclear(slice: *mut PSlice);
    pub fn psliceget(slice: *mut PSlice, i: size_t) -> *mut c_void;

    pub fn psliceput(slice: *mut PSlice, i: size_t, p: *mut c_void) -> c_int;

    pub fn pslicedel(slice: *mut PSlice, i: size_t) -> c_int;
    pub fn psliceappend(s: *mut PSlice, p: *mut c_void);
    pub fn pslicelen(slice: *mut PSlice) -> size_t;
    pub fn pslicefinalize(slice: *mut PSlice) -> *mut *mut c_void;
    pub fn pslicedestroy(slice: *mut PSlice);

    pub fn query_mem(
        config_string: *const c_char,
        base: *mut usize,
        size: *mut size_t,
    );

    pub fn query_rtc(config_string: *const c_char, mtime: *mut usize);

    pub fn query_uint(
        configstring: *const c_char,
        name: *mut c_char,
        m: *mut usize,
    ) -> c_int;

}
