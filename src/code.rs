use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

// pub enum Lang {
//     Java,
//     Javascript,
//     Lua,
//     Python,
//     Ruby,
//     Shell,
// }

#[derive(Copy, Clone, Debug)]
pub enum Tag {
    SH,
    CMD,
    RS,
    NIRS,
    BS,
    NIBS,
    FU,
    FD,
    FW,
    FR,
    LL,
    SUID,
    SUDO,
    CB,
    LSUID,
}
#[derive(Copy, Clone, Debug)]
pub struct Code<'a> {
    pub title: &'a str,
    pub code: &'a str,
    pub tag: Tag,
    // pub lang: Lang,
    // pub is_copy: bool,
    // pub is_write: bool,
}

impl<'a> Code<'a> {
    fn new(title: &'a str, code: &'a str, tag: Tag) -> Code<'a> {
        Code { title, code, tag }
    }
}
