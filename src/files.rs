use std::{fs, path::PathBuf};

use phf::phf_set;
use sdl3::rect::Rect;

use crate::images::Image;

static EXTENTIONS: phf::Set<&'static str> = phf_set!(
    "cur",
    "ico",
    "bmp",
    "pnm",
    "xpm",
    "xcf",
    "pcx",
    "gif",
    "jpg" | "jpeg",
    "tif" | "tiff",
    "png",
    "tga",
    "lbm",
    "xv",
    "webp",
);

pub struct FileEntry {
    pub filename: PathBuf,
    pub name: String,
    pub thumbnail_file: PathBuf,
    pub thumbnail: Image,
    pub tile_rect: Rect,
}

pub fn load_filelist(path: &PathBuf) -> anyhow::Result<Vec<FileEntry>> {
    let mut result: Vec<FileEntry> = vec![];
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_file()
            && let Some(ext) = path.extension()
            && let Some(ext) = ext.to_str()
            && EXTENTIONS.contains(ext)
        {
            let thumbnail_file = path.join("_preview").join(path.file_name().unwrap());
            result.push(FileEntry {
                filename: path.clone(),
                name: path.file_name().unwrap().to_str().unwrap().into(),
                thumbnail_file,
                thumbnail: Image::default(),
                tile_rect: Rect::new(0, 0, 1, 1),
            });
        }
    }
    Ok(result)
}
