use crate::model;
use colored::*;
use std::path::Path;

pub struct Printer {}

impl Printer {
    pub fn new() -> Self {
        return Printer {};
    }

    pub fn begin_delete(&self, bucket_name: &model::BucketName) {
        println!("{} {}", "Deleting existing files on s3".bright_yellow(), bucket_name.0);
    }

    pub fn on_delete(&self, key: &model::ObjectKey) {
        println!("{} {}", "Deleted".bright_yellow(), key.0);
    }

    pub fn on_exclude(&self, path: &Path) {
        println!("{} {}", "Excluded".green(), path.to_str().unwrap().black());
    }

    pub fn on_upload(&self, path: &Path, meta: &model::Meta) {
        println!("{} {}", "Uploaded".bright_cyan(), path.to_str().unwrap());
        if let Some(content_type) = &meta.content_type {
            println!("  {} {}", "Content-Type".white(), content_type.0);
        }
        if let Some(cache_control) = &meta.cache_control {
            println!("  {} {}", "Cache-Control".white(), cache_control.0);
        }
    }

    pub fn on_ignore(&self, path: &Path) {
        println!("{} {}", "Ignored".yellow(), path.to_str().unwrap().black());
    }
}
