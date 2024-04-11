//! A library to generate image hashes and add them to the images' EXIF data.

use std::ffi::OsStr;
use std::fs::read;
use std::path::{Path, PathBuf};

use little_exif::{exif_tag::ExifTag, metadata::Metadata};
use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};
use walkdir::{DirEntry, WalkDir};
use xxhash_rust::xxh3::xxh3_64;

pub mod constants;

/// Processes all images in `directory` in parallel using `cores` threads.
pub fn process_images(directory: &str, cores: usize) {
    let image_paths = walk_images(directory).map(DirEntry::into_path);
    let pool = create_thread_pool(cores);

    pool.install(|| image_paths.par_bridge().for_each(process_image));
}

/// Returns an iterator over all files in `directory` that are images.
#[inline]
fn walk_images(directory: &str) -> impl Iterator<Item = DirEntry> {
    WalkDir::new(directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(is_file)
        .filter(is_image)
}

/// Returns `true` if `entry` is a file.
#[inline]
fn is_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
}

/// Returns `true` if `entry` is an image.
#[inline]
fn is_image(entry: &DirEntry) -> bool {
    let image_extension = constants::IMAGE_EXTENSION;
    entry
        .path()
        .extension()
        .and_then(OsStr::to_str)
        .map_or(false, |ext| ext.eq_ignore_ascii_case(image_extension))
}

/// Creates a new thread pool with `cores` threads.
#[inline]
#[tracing::instrument(level = tracing::Level::DEBUG)]
fn create_thread_pool(cores: usize) -> ThreadPool {
    ThreadPoolBuilder::new()
        .num_threads(cores)
        .build()
        .expect("Failed to create thread pool")
}

/// Processes the image at `path`.
#[tracing::instrument(level = tracing::Level::DEBUG)]
fn process_image(path: PathBuf) {
    tracing::info!("Processing image");

    let hash_value = hash_image(&path);
    let mut metadata = Metadata::new();

    metadata.set_tag(ExifTag::ImageUniqueID(hash_value.to_string()));
    metadata
        .write_to_file(&path)
        .expect("Failed to write image metadata");

    tracing::info!("Processed image");
}

/// Hashes the image at `path`.
fn hash_image(path: &Path) -> u64 {
    tracing::debug!("Hashing image");

    let bytes = read(path).expect("Failed to open and read image");
    let hash_value = xxh3_64(&bytes);

    tracing::debug!("Hashed image");
    hash_value
}
