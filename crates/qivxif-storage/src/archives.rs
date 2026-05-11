use crate::StoreError;
use object_store::{ObjectStore, local::LocalFileSystem, path::Path as ObjectPath};
use std::{path::Path, sync::Arc};

pub struct ArchiveStore {
    store: Arc<dyn ObjectStore>,
}

impl ArchiveStore {
    pub fn local(root: &Path) -> Result<Self, StoreError> {
        let store = LocalFileSystem::new_with_prefix(root)?;
        Ok(Self::from_object_store(Arc::new(store)))
    }

    pub fn from_object_store(store: Arc<dyn ObjectStore>) -> Self {
        Self { store }
    }

    pub async fn put_manifest(&self, name: &str, bytes: Vec<u8>) -> Result<(), StoreError> {
        self.store
            .put(&manifest_path(name)?, bytes.into())
            .await
            .map_err(StoreError::from)?;
        Ok(())
    }

    pub async fn get_manifest(&self, name: &str) -> Result<Vec<u8>, StoreError> {
        let result = self
            .store
            .get(&manifest_path(name)?)
            .await
            .map_err(StoreError::from)?;
        Ok(result.bytes().await.map_err(StoreError::from)?.to_vec())
    }

    pub async fn list_manifests(&self) -> Result<Vec<String>, StoreError> {
        let result = self
            .store
            .list_with_delimiter(Some(&ObjectPath::from("manifests")))
            .await
            .map_err(StoreError::from)?;
        Ok(result
            .objects
            .into_iter()
            .map(|meta| meta.location.to_string())
            .collect())
    }
}

fn manifest_path(name: &str) -> Result<ObjectPath, StoreError> {
    Ok(ObjectPath::from(format!(
        "manifests/{}.json",
        manifest_name(name)?
    )))
}

fn manifest_name(name: &str) -> Result<&str, StoreError> {
    let valid = !name.is_empty()
        && !name.starts_with('.')
        && !name.contains("..")
        && name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'.' | b'-'));
    if valid {
        Ok(name)
    } else {
        Err(StoreError::InvalidArchiveName)
    }
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
        assert!(root.path().join("manifests/smoke.json").is_file());
        assert!(!root.path().join(crate::tables::DB_FILE).exists());
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

    #[test]
    fn rejects_ambiguous_manifest_names() {
        for name in ["", ".", "..", "../x", "a/b", "a\\b", "line\nbreak"] {
            assert!(matches!(
                manifest_path(name),
                Err(StoreError::InvalidArchiveName)
            ));
        }
    }
}
