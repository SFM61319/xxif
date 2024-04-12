//! A library to generate image hashes and add them to the images' EXIF data.

use std::ffi::OsStr;
use std::fs::read;
use std::io::Result as IoResult;
use std::path::{Path, PathBuf};

use little_exif::{exif_tag::ExifTag, metadata::Metadata};
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use uuid::Uuid;
use walkdir::{DirEntry, WalkDir};
use xxhash_rust::xxh3::xxh3_128;

pub mod constants;

/// Returns `true` if `extension` is an [acceptable image extension](constants::ACCEPTABLE_IMAGE_EXTENSIONS).
#[inline]
fn is_acceptable(extension: &str) -> bool {
    constants::ACCEPTABLE_IMAGE_EXTENSIONS
        .iter()
        .any(|&acceptable_extension| extension.eq_ignore_ascii_case(acceptable_extension))
}

/// Returns `true` if `entry` is an image.
#[inline]
fn is_image(entry: &DirEntry) -> bool {
    entry
        .path()
        .extension()
        .and_then(OsStr::to_str)
        .map_or(false, is_acceptable)
}

/// Hashes the image at `path` using [`XXH3-128`](xxh3_128) and returns it.
fn hash_image(path: &Path) -> IoResult<u128> {
    tracing::debug!(image = ?path, "Hashing image");

    let bytes = read(path)?;
    let hash_value = xxh3_128(&bytes);

    tracing::debug!(image = ?path, hash = ?hash_value, "Hashed image");
    Ok(hash_value)
}

/// Sets the hash of image at `path` in the image's EXIF data.
fn process_image(path: PathBuf) -> IoResult<()> {
    tracing::debug!(image = ?path, "Processing image");

    let hash_value = hash_image(&path)?;
    let mut metadata = Metadata::new_from_path(&path)?;

    let image_uid = Uuid::from_u128(hash_value).hyphenated().to_string();
    let image_uid = ExifTag::ImageUniqueID(image_uid);

    metadata.set_tag(image_uid);
    metadata.write_to_file(&path)?;

    tracing::info!(image = ?path, "Processed image");
    Ok(())
}

/// Sets the hashes of all images in `path` in the images' EXIF data recursively
/// and returns the number of successfully processed images.
#[inline]
fn process_all_images<P>(path: P) -> usize
where
    P: AsRef<Path>,
{
    WalkDir::new(path)
        .into_iter()
        .map(|entry| entry.inspect_err(|err| tracing::warn!(error = ?err, "Failed to walk path")))
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file() && is_image(entry))
        .map(DirEntry::into_path)
        .par_bridge()
        .map(process_image)
        .filter_map(Result::ok)
        .count()
}

/// Sets the hashes of all images in `path` in the images' EXIF data recursively
/// in parallel using `core` threads and returns the number of successfully processed images.
#[inline]
pub fn process_images<P>(path: P, cores: usize) -> usize
where
    P: AsRef<Path> + Send,
{
    ThreadPoolBuilder::new()
        .num_threads(cores)
        .build()
        .inspect_err(|err| tracing::error!(error = ?err, "Failed to build thread pool"))
        .map(|pool| pool.install(|| process_all_images(path)))
        .unwrap_or_default()
}
