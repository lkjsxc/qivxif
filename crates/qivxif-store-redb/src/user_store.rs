use crate::{
    StoreError, StoreResult,
    codec::{decode, encode},
    records::{StoredSession, StoredUser},
    store::QivxifStore,
    tables,
};
use qivxif_auth::{AuthRole, PasswordHashString};
use qivxif_core::{ActorId, SessionId, UserId};
use redb::ReadableTable;

impl QivxifStore {
    pub fn create_admin_user(
        &self,
        name: String,
        password_hash: PasswordHashString,
    ) -> StoreResult<StoredUser> {
        let tx = self.database.begin_write()?;
        {
            let names = tx.open_table(tables::USER_NAMES)?;
            if names.iter()?.next().is_some() {
                return Err(StoreError::AdminExists);
            }
        }
        let user = user_record(name, password_hash, vec![AuthRole::Owner, AuthRole::Admin]);
        insert_user(&tx, &user)?;
        tx.commit()?;
        Ok(user)
    }

    pub fn create_user(
        &self,
        name: String,
        password_hash: PasswordHashString,
        roles: Vec<AuthRole>,
    ) -> StoreResult<StoredUser> {
        let tx = self.database.begin_write()?;
        let user = user_record(name, password_hash, roles);
        insert_user(&tx, &user)?;
        tx.commit()?;
        Ok(user)
    }

    pub fn find_user_by_name(&self, name: &str) -> StoreResult<Option<StoredUser>> {
        let tx = self.database.begin_read()?;
        let names = tx.open_table(tables::USER_NAMES)?;
        let Some(user_id_bytes) = names.get(name)? else {
            return Ok(None);
        };
        let user_id: UserId = decode(user_id_bytes.value())?;
        self.get_user(&user_id)
    }

    pub fn get_user(&self, user_id: &UserId) -> StoreResult<Option<StoredUser>> {
        let tx = self.database.begin_read()?;
        let users = tx.open_table(tables::USERS)?;
        users
            .get(user_id.as_str())?
            .map(|bytes| decode(bytes.value()))
            .transpose()
    }

    pub fn create_session(&self, session: StoredSession) -> StoreResult<StoredSession> {
        let tx = self.database.begin_write()?;
        {
            let mut sessions = tx.open_table(tables::SESSIONS)?;
            sessions.insert(session.id.as_str(), encode(&session)?.as_slice())?;
        }
        tx.commit()?;
        Ok(session)
    }

    pub fn get_session(&self, session_id: &SessionId) -> StoreResult<Option<StoredSession>> {
        let tx = self.database.begin_read()?;
        let sessions = tx.open_table(tables::SESSIONS)?;
        sessions
            .get(session_id.as_str())?
            .map(|bytes| decode(bytes.value()))
            .transpose()
    }

    pub fn delete_session(&self, session_id: &SessionId) -> StoreResult<()> {
        let tx = self.database.begin_write()?;
        {
            let mut sessions = tx.open_table(tables::SESSIONS)?;
            sessions.remove(session_id.as_str())?;
        }
        tx.commit()?;
        Ok(())
    }
}

fn user_record(
    name: String,
    password_hash: PasswordHashString,
    roles: Vec<AuthRole>,
) -> StoredUser {
    StoredUser {
        id: UserId::generate(),
        actor_id: ActorId::generate(),
        name,
        password_hash,
        roles,
    }
}

fn insert_user(tx: &redb::WriteTransaction, user: &StoredUser) -> StoreResult<()> {
    let mut names = tx.open_table(tables::USER_NAMES)?;
    if names.get(user.name.as_str())?.is_some() {
        return Err(StoreError::DuplicateUserName);
    }
    let mut users = tx.open_table(tables::USERS)?;
    users.insert(user.id.as_str(), encode(user)?.as_slice())?;
    names.insert(user.name.as_str(), encode(&user.id)?.as_slice())?;
    Ok(())
}
