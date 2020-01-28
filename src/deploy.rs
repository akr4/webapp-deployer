use crate::config;
use crate::model;
use crate::printer;
use crate::s3;
use std::path::Path;
use walkdir::WalkDir;

type Result<T> = std::result::Result<T, failure::Error>;

pub fn run(
    printer: &printer::Printer,
    app_dir: &Path,
    bucket_name: &model::BucketName,
    config_path: &Path,
    dry_run: bool,
) -> Result<()> {
    let config = config::load_config(config_path)?;
    let s3 = s3::S3::new(&bucket_name);

    printer.begin_delete(&bucket_name);
    if !dry_run {
        let deleted_keys = s3.clear_bucket()?;
        for key in deleted_keys {
            printer.on_delete(&key);
        }
    }

    for entry in WalkDir::new(app_dir) {
        let entry = entry?;

        if entry.file_type().is_dir() {
            continue;
        }

        if let Ok(path_from_project_root) = entry.path().strip_prefix(app_dir) {
            let instruction = config.instructions.iter().find(|x| {
                x.pattern.is_match(path_from_project_root.to_str().unwrap())
            });

            if let Some(instruction) = instruction {
                match instruction.action {
                    model::Action::Exclude => {
                        printer.on_exclude(&path_from_project_root);
                        continue;
                    }
                    model::Action::Upload => {
                        if !dry_run {
                            s3.put_object(
                                entry.path(),
                                &model::ObjectKey(path_from_project_root.to_str().unwrap().to_string()),
                                &instruction.meta,
                            )?;
                        }
                        printer.on_upload(&path_from_project_root, &instruction.meta);
                        continue;
                    }
                }
            }

            printer.on_ignore(&path_from_project_root);
        }
    }

    Ok(())
}
