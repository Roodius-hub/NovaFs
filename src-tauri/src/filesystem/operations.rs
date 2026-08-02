use std::path::Path;
use std::io::{self, Write};
use std::fs::{copy, create_dir, remove_dir_all, remove_file, rename};
use std::fs::File;

pub fn create_folder(path: &Path) -> io::Result<()> {
      create_dir(path) 
}

pub fn create_file(path: &Path) -> io::Result<()> {
     File::create(path)?;
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
    let meta = path.metadata()?;
    if meta.is_dir() {
        remove_dir_all(path)
    } else {
        remove_file(path)
    }
}