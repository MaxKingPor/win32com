use std::{os::windows::prelude::OsStrExt, path::Path};

use dm::Dmsoft;

use windows::Win32::System::Com;
#[allow(unused_labels)]

fn main() {
    unsafe {

        let s = r#"D:\Project\Rust-Project\win32com\dm.dll"#;

        dm::set_dll_path(s, "DmReg.dll");
        println!("#######################");

        Com::CoInitializeEx(None, Default::default()).unwrap();

        let dm = Dmsoft::new().unwrap();

        let s = dm.Ver().unwrap();
        println!("Ver: {}", s);
        let result = dm.SetPath("./").unwrap();

        println!("SetPath: {:?}", result);

        let result = dm.Ocr(0, 0, 2000, 2000, "ffffff-000000", 1.0);
        println!("Ocr: {:?}", result);

        let (mut x, mut y) = (0, 0);
        let result = dm.FindStr(0, 0, 2000, 2000, "1", "000000-000000", 1.0, &mut x, &mut y);
        println!("FindStr: {:?} x:{}, y:{}", result, x, y);
    }

    println!("#################################")
}
