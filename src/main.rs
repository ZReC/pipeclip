/**
 * Copyright (c) 2020-2022 ZReC and others
 * MIT License
 */
use arboard::Clipboard;
use std::error::Error;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let mut cb = Clipboard::new()?;

    if let Ok(img) = cb.get_image() {
        print!(
            "P7\nWIDTH {}\nHEIGHT {}\nDEPTH 4\nMAXVAL 255\nTUPLTYPE RGB_ALPHA\nENDHDR\n",
            img.width, img.height
        );
        io::stdout().write_all(&img.bytes)?;
    } else {
        print!("{}", cb.get_text()?);
    }

    Ok(())
}