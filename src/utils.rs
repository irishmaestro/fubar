use arboard::Clipboard;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{prelude::*, Result};
use std::path::Path;

pub enum Lang {
    Java,
    Javascript,
    Python,
    Ruby,
    Shell,
}

pub fn fmt_payload(payload: &str) -> String {
    // lang: Option<Lang>
    let mut x = String::new();
    // if let Some(_) = lang {
    //     match lang.unwrap() {
    //         lang::Shell => x.push_str("#!/bin/bash\n"),
    //         lang::Python => x.push_str("#!/usr/bin/env python3\n"),
    //         _ => p.push_str(""),
    //     }
    // }
    for z in payload.trim().lines() {
        if z.starts_with("    ") {
            x.push_str(z.strip_prefix("    ").unwrap());
        } else {
            x.push_str(z);
        }
        x.push_str("\n");
    }
    x
}

pub fn copy_payload(payload: &str) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(fmt_payload(payload).as_str()).unwrap();
}

pub fn write_payload(payload: &str, lang: Lang) -> Result<()> {
    let ext = match lang {
        Lang::Python => "py",
        Lang::Shell => "sh",
        Lang::Ruby => "rb",
        Lang::Javascript => "js",
        Lang::Java => "java",
    };
    let path = Path::new("payload").with_extension(ext);
    let mut file = File::create(path)?;
    write!(file, "{}", payload)?;
    Ok(())
}
