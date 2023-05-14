pub static BIN_NAME: &'static str = "file";
pub static FR_DESC_1: &'static str = "Each input line is treated as a filename for the `file` command and the output is corrupted by a suffix `:` followed by the result or the error of the operation, so this may not be suitable for binary files.";
pub static FR_CODE_1: &'static str = r#"
    
    LFILE=file_to_read
    file -f $LFILE
"#;
pub static FR_DESC_2: &'static str = "Each line is corrupted by a prefix string and wrapped inside quotes, so this may not be suitable for binary files.

If a line in the target file begins with a `#``, it will not be printed as these lines are parsed as comments.

It can also be provided with a directory and will read each file in the directory.";
pub static FR_CODE_2: &'static str = r#"
    
    LFILE=file_to_read
    file -m $LFILE
"#;
pub static SUID_DESC: &'static str = "Each input line is treated as a filename for the `file` command and the output is corrupted by a suffix `:` followed by the result or the error of the operation, so this may not be suitable for binary files.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which file) .

    LFILE=file_to_read
    ./file -f $LFILE
"#;
pub static SUDO_DESC: &'static str = "Each input line is treated as a filename for the `file` command and the output is corrupted by a suffix `:` followed by the result or the error of the operation, so this may not be suitable for binary files.";
pub static SUDO_CODE: &'static str = r#"
    
    LFILE=file_to_read
    sudo file -f $LFILE
"#;
use crate::code::{Code, Tag};
pub static FR_1: Code<'static> = Code {
    title: "FR_CODE_1",
    code: FR_CODE_1,
    tag: Tag::FR,
};
pub static FR_2: Code<'static> = Code {
    title: "FR_CODE_2",
    code: FR_CODE_2,
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
