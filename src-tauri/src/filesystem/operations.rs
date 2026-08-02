use std::path::Path;
use std::io::{self, Write};
use std::fs::{create_dir, rename, remove_file,remove_dir,write, copy};
use std::fs::File;

pub fn create_folder(path: &Path) -> io::Result<()> {
      create_dir(path) 
}

pub fn create_file(path: &Path) -> io::Result<()> {
    let mut file = File::create(path)?;
        file.write_all(b"")?;
        Ok(())
}

pub fn rename_file(path: &Path, new_name: &str) -> io::Result<()> {
    let parent = path.parent().unwrap();
    let new_path = parent.join(new_name);
    rename(path, &new_path)
}

pub fn copy_file(src: &Path, dest: &Path) -> io::Result<()> {
     copy(src, dest)?;
     Ok(())
}

pub fn move_file(src: &Path, dest: &Path) -> io::Result<()> {
    rename(src, dest)
}

pub fn delete(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        remove_dir(path)
    } else {
        remove_file(path)
    }
}