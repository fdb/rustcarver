use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use walkdir::WalkDir;

fn file_copy(from: &Path, ext: &str) -> std::io::Result<()> {
    let file_stem = from.file_stem();
    let dst_filename = format!("results/{}.{}", file_stem.unwrap().to_string_lossy(), ext);
    fs::copy(from, Path::new(&dst_filename))?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let home_dir = dirs::home_dir().unwrap();
    let safari_cache_dir = "cache";
    let walker = WalkDir::new(safari_cache_dir).into_iter();
    for entry in walker {
        let entry = entry?;
        if !entry.path().is_file() {
            continue;
        }
        let f = File::open(entry.path())?;
        let mut reader = BufReader::new(f);
        let mut buf: [u8; 4] = [0; 4];
        let result = reader.read_exact(&mut buf);
        if result.is_err() {
            continue;
        }
        if buf[0] == 0xFF && buf[1] == 0xD8 && buf[2] == 0xFF {
            // JPEG file header
            println!("JPEG: {}", entry.path().display());
            file_copy(entry.path(), "jpg")?;
        } else if buf[0] == 0x47 && buf[1] == 0x49 && buf[2] == 0x46 && buf[3] == 0x38 {
            // GIF file header
            println!("GIF: {}", entry.path().display());
            file_copy(entry.path(), "gif")?;
        } else if buf[0] == 0x89 && buf[1] == 0x50 && buf[2] == 0x4e && buf[3] == 0x47 {
            // PNG file header
            println!("PNG: {}", entry.path().display());
            file_copy(entry.path(), "png")?;
        }
    }
    Ok(())
}
