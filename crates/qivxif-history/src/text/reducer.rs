use super::{
    TextAtom, TextCharId, TextDelete, TextDocState, TextEdit, TextInsert, TextOperation,
    TextRestore, TextSnapshot,
};
use crate::{HistoryError, HistoryResult};
use qivxif_core::{ActorId, OperationId, TextDocId};
use std::collections::BTreeMap;

pub fn apply_text_op(mut state: TextDocState, op: TextOperation) -> HistoryResult<TextDocState> {
    if state.applied_operations.contains(&op.op_id) {
        return Ok(state);
    }
    match op.edit {
        TextEdit::Insert(insert) => apply_insert(&mut state, insert)?,
        TextEdit::Delete(delete) => apply_delete(&mut state, delete)?,
        TextEdit::Restore(restore) => apply_restore(&mut state, restore),
    }
    state.applied_operations.push(op.op_id);
    state.content = render_atoms(&state.atoms);
    Ok(state)
}

pub fn snapshot_text(
    state: &TextDocState,
    doc_id: TextDocId,
    after_operation: OperationId,
) -> TextSnapshot {
    TextSnapshot {
        doc_id,
        after_operation,
        content: state.content.clone(),
    }
}

pub fn restore_text(
    snapshot: TextSnapshot,
    op_id: OperationId,
    actor_id: ActorId,
) -> TextOperation {
    TextOperation {
        op_id,
        doc_id: snapshot.doc_id,
        edit: TextEdit::Restore(TextRestore {
            content: snapshot.content,
            actor_id,
            first_seq: 1,
        }),
    }
}

fn apply_insert(state: &mut TextDocState, insert: TextInsert) -> HistoryResult<()> {
    if let Some(anchor) = &insert.after
        && !state.atoms.iter().any(|atom| &atom.id == anchor)
    {
        return Err(HistoryError::MissingTextCharacter);
    }
    for item in insert.chars {
        if state.atoms.iter().any(|atom| atom.id == item.id) {
            continue;
        }
        state.atoms.push(TextAtom {
            id: item.id,
            after: insert.after.clone(),
            value: item.value,
            deleted: false,
        });
    }
    Ok(())
}

fn apply_delete(state: &mut TextDocState, delete: TextDelete) -> HistoryResult<()> {
    for id in &delete.ids {
        if !state.atoms.iter().any(|atom| &atom.id == id) {
            return Err(HistoryError::MissingTextCharacter);
        }
    }
    for atom in &mut state.atoms {
        if delete.ids.contains(&atom.id) {
            atom.deleted = true;
        }
    }
    Ok(())
}

fn apply_restore(state: &mut TextDocState, restore: TextRestore) {
    for atom in &mut state.atoms {
        atom.deleted = true;
    }
    for (offset, value) in restore.content.chars().enumerate() {
        state.atoms.push(TextAtom {
            id: TextCharId {
                actor_id: restore.actor_id.clone(),
                seq: restore.first_seq + offset as u64,
            },
            after: None,
            value,
            deleted: false,
        });
    }
}

fn render_atoms(atoms: &[TextAtom]) -> String {
    let mut children: BTreeMap<Option<TextCharId>, Vec<TextAtom>> = BTreeMap::new();
    for atom in atoms {
        children
            .entry(atom.after.clone())
            .or_default()
            .push(atom.clone());
    }
    for group in children.values_mut() {
        group.sort_by(|left, right| left.id.cmp(&right.id));
    }
    let mut out = String::new();
    render_after(None, &children, &mut out);
    out
}

fn render_after(
    anchor: Option<TextCharId>,
    children: &BTreeMap<Option<TextCharId>, Vec<TextAtom>>,
    out: &mut String,
) {
    if let Some(group) = children.get(&anchor) {
        for atom in group {
            if !atom.deleted {
                out.push(atom.value);
            }
            render_after(Some(atom.id.clone()), children, out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::TextInsertedChar;

    fn cid(actor: &ActorId, seq: u64) -> TextCharId {
        TextCharId {
            actor_id: actor.clone(),
            seq,
        }
    }

    fn insert(actor: &ActorId, seq: u64, text: &str) -> TextOperation {
        TextOperation {
            op_id: OperationId::generate(),
            doc_id: TextDocId::generate(),
            edit: TextEdit::Insert(TextInsert {
                after: None,
                chars: text
                    .chars()
                    .enumerate()
                    .map(|(offset, value)| TextInsertedChar {
                        id: cid(actor, seq + offset as u64),
                        value,
                    })
                    .collect(),
            }),
        }
    }

    #[test]
    fn inserts_and_deduplicates() {
        let actor = ActorId::generate();
        let op = insert(&actor, 1, "hi");
        let state = apply_text_op(TextDocState::default(), op.clone()).unwrap();
        assert_eq!(state.content, "hi");
        assert_eq!(apply_text_op(state.clone(), op).unwrap(), state);
    }

    #[test]
    fn deletes_known_characters() {
        let actor = ActorId::generate();
        let state = apply_text_op(TextDocState::default(), insert(&actor, 1, "hi")).unwrap();
        let op = TextOperation {
            op_id: OperationId::generate(),
            doc_id: TextDocId::generate(),
            edit: TextEdit::Delete(TextDelete {
                ids: vec![cid(&actor, 1)],
            }),
        };
        assert_eq!(apply_text_op(state, op).unwrap().content, "i");
    }

    #[test]
    fn restore_creates_new_visible_content() {
        let actor = ActorId::generate();
        let state = apply_text_op(TextDocState::default(), insert(&actor, 1, "old")).unwrap();
        let snapshot = TextSnapshot {
            doc_id: TextDocId::generate(),
            after_operation: OperationId::generate(),
            content: "new".to_owned(),
        };
        let op = restore_text(snapshot, OperationId::generate(), actor);
        assert_eq!(apply_text_op(state, op).unwrap().content, "new");
    }
}
