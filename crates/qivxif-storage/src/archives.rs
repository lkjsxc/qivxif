use crate::errors::{StoreError, archive_error};
use object_store::{ObjectStore, local::LocalFileSystem, path::Path as ObjectPath};
use std::path::Path;

pub struct ArchiveStore {
    store: LocalFileSystem,
}

impl ArchiveStore {
    pub fn local(root: &Path) -> Result<Self, StoreError> {
        let store = LocalFileSystem::new_with_prefix(root).map_err(archive_error)?;
        Ok(Self { store })
    }

    pub async fn put_manifest(&self, name: &str, bytes: Vec<u8>) -> Result<(), StoreError> {
        self.store
            .put(&manifest_path(name), bytes.into())
            .await
            .map_err(archive_error)?;
        Ok(())
    }

    pub async fn get_manifest(&self, name: &str) -> Result<Vec<u8>, StoreError> {
        let result = self
            .store
            .get(&manifest_path(name))
            .await
            .map_err(archive_error)?;
        Ok(result.bytes().await.map_err(archive_error)?.to_vec())
    }

    pub async fn list_manifests(&self) -> Result<Vec<String>, StoreError> {
        let result = self
            .store
            .list_with_delimiter(Some(&ObjectPath::from("manifests")))
            .await
            .map_err(archive_error)?;
        Ok(result
            .objects
            .into_iter()
            .map(|meta| meta.location.to_string())
            .collect())
    }
}

fn manifest_path(name: &str) -> ObjectPath {
    ObjectPath::from(format!("manifests/{name}.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn writes_reads_and_lists_manifest() {
        let root = tempfile::tempdir().unwrap();
        let archives = ArchiveStore::local(root.path()).unwrap();
        archives
            .put_manifest("smoke", br#"{"epoch":"local"}"#.to_vec())
            .await
            .unwrap();
        assert_eq!(
            archives.get_manifest("smoke").await.unwrap(),
            br#"{"epoch":"local"}"#.to_vec()
        );
        assert_eq!(
            archives.list_manifests().await.unwrap(),
            vec!["manifests/smoke.json".to_string()]
        );
    }

    #[tokio::test]
    async fn missing_manifest_returns_archive_error() {
        let root = tempfile::tempdir().unwrap();
        let archives = ArchiveStore::local(root.path()).unwrap();
        assert!(matches!(
            archives.get_manifest("missing").await,
            Err(StoreError::Archive(_))
        ));
    }
}
