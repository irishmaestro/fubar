pub static BIN_NAME: &'static str = "dosbox";
pub static BIN_DESC: &'static str = "Basically `dosbox` allows to mount the local file system, so that it can be altered using DOS commands. Note that the DOS filename convention (`8.3`) is used.";
pub static FW_DESC: &'static str = "Note that the name of the written file in the following example will be `FILE_TO_`. Also note that `echo` terminates the string with a DOS-style line terminator (`\r\n`), if that’s a problem and your scenario allows it, you can create the file outside `dosbox`, then use `copy` to do the actual write.";
pub static FW_CODE: &'static str = r#"
    
    LFILE='\path\to\file_to_write'
    dosbox -c 'mount c /' -c "echo DATA >c:$LFILE" -c exit
"#;
pub static FR_DESC_1: &'static str =
    "The file content will be displayed in the DOSBox graphical window.";
pub static FR_CODE_1: &'static str = r#"
    
    LFILE='\path\to\file_to_read'
    dosbox -c 'mount c /' -c "type c:$LFILE"
"#;
pub static FR_DESC_2: &'static str = "The file is copied to a readable location.";
pub static FR_CODE_2: &'static str = r#"
    
    LFILE='\path\to\file_to_read'
    dosbox -c 'mount c /' -c "copy c:$LFILE c:\tmp\output" -c exit
    cat '/tmp/OUTPUT'
"#;
pub static SUID_DESC: &'static str = "Note that the name of the written file in the following example will be `FILE_TO_`. Also note that `echo` terminates the string with a DOS-style line terminator (`\r\n`), if that’s a problem and your scenario allows it, you can create the file outside `dosbox`, then use `copy` to do the actual write.";
pub static SUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which dosbox) .

    LFILE='\path\to\file_to_write'
    ./dosbox -c 'mount c /' -c "echo DATA >c:$LFILE" -c exit
"#;
pub static SUDO_DESC: &'static str = "Note that the name of the written file in the following example will be `FILE_TO_`. Also note that `echo` terminates the string with a DOS-style line terminator (`\r\n`), if that’s a problem and your scenario allows it, you can create the file outside `dosbox`, then use `copy` to do the actual write.";
pub static SUDO_CODE: &'static str = r#"
    
    LFILE='\path\to\file_to_write'
    sudo dosbox -c 'mount c /' -c "echo DATA >c:$LFILE" -c exit
"#;
use crate::code::{Code, Tag};
pub static FW: Code<'static> = Code {
    title: "FW_CODE",
    code: FW_CODE,
    tag: Tag::FW,
};
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
