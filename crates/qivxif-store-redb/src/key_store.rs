use crate::{
    StoreError, StoreResult,
    codec::{decode, encode},
    store::QivxifStore,
    tables,
};
use qivxif_core::UserId;
use rand::{RngCore, rngs::OsRng};
use redb::ReadableTable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InviteCodeRecord {
    pub id: String,
    pub secret_hash: String,
    pub role: String,
    pub expires_in: Option<String>,
    pub max_uses: u64,
    pub uses: u64,
    pub revoked: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AccessKeyRecord {
    pub id: String,
    pub user_id: UserId,
    pub secret_hash: String,
    pub scopes: Vec<String>,
    pub expires_in: Option<String>,
    pub revoked: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct KeyAuditRecord {
    pub id: String,
    pub action: String,
    pub target_id: String,
    pub detail: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct IssuedSecret<T> {
    pub record: T,
    pub secret: String,
}

impl QivxifStore {
    pub fn issue_invite(
        &self,
        role: String,
        expires_in: Option<String>,
        max_uses: u64,
    ) -> StoreResult<IssuedSecret<InviteCodeRecord>> {
        let secret = prefixed_secret("qxi_inv");
        let record = InviteCodeRecord {
            id: record_id("inv"),
            secret_hash: secret_hash(&secret),
            role,
            expires_in,
            max_uses,
            uses: 0,
            revoked: false,
        };
        let tx = self.database.begin_write()?;
        tx.open_table(tables::INVITE_CODES)?
            .insert(record.id.as_str(), encode(&record)?.as_slice())?;
        write_audit(&tx, "invite.issue", &record.id, &record.role)?;
        tx.commit()?;
        Ok(IssuedSecret { record, secret })
    }

    pub fn list_invites(&self) -> StoreResult<Vec<InviteCodeRecord>> {
        list_table(&self.database.begin_read()?, tables::INVITE_CODES)
    }

    pub fn revoke_invite(&self, id: &str) -> StoreResult<InviteCodeRecord> {
        let tx = self.database.begin_write()?;
        let mut record = {
            let table = tx.open_table(tables::INVITE_CODES)?;
            let Some(bytes) = table.get(id)? else {
                return Err(StoreError::KeyMissing);
            };
            decode::<InviteCodeRecord>(bytes.value())?
        };
        record.revoked = true;
        {
            let mut table = tx.open_table(tables::INVITE_CODES)?;
            table.insert(id, encode(&record)?.as_slice())?;
        }
        write_audit(&tx, "invite.revoke", id, "revoked")?;
        tx.commit()?;
        Ok(record)
    }

    pub fn issue_access_key(
        &self,
        user_id: UserId,
        scopes: Vec<String>,
        expires_in: Option<String>,
    ) -> StoreResult<IssuedSecret<AccessKeyRecord>> {
        let secret = prefixed_secret("qxi_key");
        let record = AccessKeyRecord {
            id: record_id("key"),
            user_id,
            secret_hash: secret_hash(&secret),
            scopes,
            expires_in,
            revoked: false,
        };
        let tx = self.database.begin_write()?;
        tx.open_table(tables::ACCESS_KEYS)?
            .insert(record.id.as_str(), encode(&record)?.as_slice())?;
        write_audit(&tx, "key.issue", &record.id, &record.user_id.to_string())?;
        tx.commit()?;
        Ok(IssuedSecret { record, secret })
    }

    pub fn list_access_keys(&self) -> StoreResult<Vec<AccessKeyRecord>> {
        list_table(&self.database.begin_read()?, tables::ACCESS_KEYS)
    }

    pub fn revoke_access_key(&self, id: &str) -> StoreResult<AccessKeyRecord> {
        let tx = self.database.begin_write()?;
        let mut record = {
            let table = tx.open_table(tables::ACCESS_KEYS)?;
            let Some(bytes) = table.get(id)? else {
                return Err(StoreError::KeyMissing);
            };
            decode::<AccessKeyRecord>(bytes.value())?
        };
        record.revoked = true;
        {
            let mut table = tx.open_table(tables::ACCESS_KEYS)?;
            table.insert(id, encode(&record)?.as_slice())?;
        }
        write_audit(&tx, "key.revoke", id, "revoked")?;
        tx.commit()?;
        Ok(record)
    }

    pub fn key_audit(&self) -> StoreResult<Vec<KeyAuditRecord>> {
        list_table(&self.database.begin_read()?, tables::KEY_AUDIT)
    }
}

fn list_table<T: for<'de> Deserialize<'de>>(
    tx: &redb::ReadTransaction,
    table: redb::TableDefinition<&str, &[u8]>,
) -> StoreResult<Vec<T>> {
    let table = tx.open_table(table)?;
    let mut records = Vec::new();
    for item in table.iter()? {
        records.push(decode(item?.1.value())?);
    }
    Ok(records)
}

fn write_audit(
    tx: &redb::WriteTransaction,
    action: &str,
    target_id: &str,
    detail: &str,
) -> StoreResult<()> {
    let record = KeyAuditRecord {
        id: record_id("aud"),
        action: action.to_owned(),
        target_id: target_id.to_owned(),
        detail: detail.to_owned(),
    };
    tx.open_table(tables::KEY_AUDIT)?
        .insert(record.id.as_str(), encode(&record)?.as_slice())?;
    Ok(())
}

fn prefixed_secret(prefix: &str) -> String {
    format!("{prefix}_{}", random_hex(32))
}

fn record_id(prefix: &str) -> String {
    format!("{prefix}_{}", random_hex(16))
}

fn secret_hash(secret: &str) -> String {
    blake3::hash(secret.as_bytes()).to_hex().to_string()
}

fn random_hex(len: usize) -> String {
    let mut bytes = vec![0u8; len];
    OsRng.fill_bytes(&mut bytes);
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
