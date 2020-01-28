use crate::model;
use rusoto_core::Region;
use rusoto_s3::{
    Delete, DeleteObjectsRequest, ListObjectsV2Request, ObjectIdentifier, PutObjectRequest,
    S3Client, StreamingBody, S3 as Rusoto_S3,
};
use std::fs::File;
use std::io::Read;
use std::path::Path;

type Result<T> = std::result::Result<T, failure::Error>;

pub struct S3 {
    client: S3Client,
    bucket_name: model::BucketName,
}

impl S3 {
    pub fn new(bucket_name: &model::BucketName) -> Self {
        return Self {
            client: S3Client::new(Region::default()),
            bucket_name: bucket_name.clone(),
        };
    }

    pub fn clear_bucket(&self) -> Result<Vec<model::ObjectKey>> {
        let identifiers: Vec<ObjectIdentifier> = self
            .list_objects()?
            .iter()
            .map(|key| {
                return ObjectIdentifier {
                    key: key.clone(),
                    ..ObjectIdentifier::default()
                };
            })
            .collect();

        if identifiers.is_empty() {
            return Ok(Vec::new());
        }

        let output = self
            .client
            .delete_objects(DeleteObjectsRequest {
                bucket: self.bucket_name.0.clone(),
                delete: Delete {
                    objects: identifiers,
                    ..Delete::default()
                },
                ..DeleteObjectsRequest::default()
            })
            .sync()?;

        let deleted_keys = output.deleted.map(|x| {
            x.iter()
                .flat_map(|x| x.key.as_ref().map(|x| model::ObjectKey(x.clone())))
                .collect()
        });

        if let Some(errors) = output.errors {
            if let Some(error) = errors.first() {
                return Err(failure::err_msg(
                    error
                        .message
                        .clone()
                        .unwrap_or("unknown s3 error".to_string()),
                ));
            }
        }

        Ok(deleted_keys.unwrap_or(Vec::new()))
    }

    pub fn put_object(
        &self,
        path: &Path,
        key: &model::ObjectKey,
        meta: &model::Meta,
    ) -> Result<()> {
        let mut buf = Vec::new();
        File::open(path)?.read_to_end(&mut buf)?;

        let _output = self
            .client
            .put_object(PutObjectRequest {
                bucket: self.bucket_name.0.clone(),
                key: key.0.clone(),
                body: Some(StreamingBody::from(buf)),
                cache_control: meta.cache_control.as_ref().map(|x| x.0.clone()),
                content_type: meta.content_type.as_ref().map(|x| x.0.clone()),
                ..PutObjectRequest::default()
            })
            .sync()?;

        Ok(())
    }

    fn list_objects(&self) -> Result<(Vec<String>)> {
        let output = self
            .client
            .list_objects_v2(ListObjectsV2Request {
                bucket: self.bucket_name.0.clone(),
                ..ListObjectsV2Request::default()
            })
            .sync()?;
        if let Some(is_truncated) = output.is_truncated {
            if is_truncated {
                return Err(failure::err_msg(format!(
                    "too many objects in {}",
                    self.bucket_name.0
                )));
            }
        }

        if let Some(contents) = output.contents {
            return Ok(contents.iter().flat_map(|x| x.key.clone()).collect());
        }

        Ok(Vec::new())
    }
}
