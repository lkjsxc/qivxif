use crate::{StoreResult, repair_edges::check_edges, repair_feed::check_feed, store::QivxifStore};
use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RepairReport {
    pub ok: bool,
    pub findings: Vec<RepairFinding>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RepairFinding {
    pub code: String,
    pub table: String,
    pub key: String,
    pub message: String,
}

impl QivxifStore {
    pub fn repair_check(&self) -> StoreResult<RepairReport> {
        let tx = self.database.begin_read()?;
        let mut findings = Vec::new();
        check_edges(&tx, &mut findings)?;
        check_feed(&tx, &mut findings)?;
        Ok(RepairReport {
            ok: findings.is_empty(),
            findings,
        })
    }
}

pub(crate) fn finding(code: &str, table: &str, key: &str, message: &str) -> RepairFinding {
    RepairFinding {
        code: code.to_owned(),
        table: table.to_owned(),
        key: key.to_owned(),
        message: message.to_owned(),
    }
}
