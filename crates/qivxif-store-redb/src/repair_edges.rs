use crate::{
    StoreResult,
    codec::decode,
    repair::{RepairFinding, finding},
    tables,
};
use qivxif_core::{EdgeId, NodeId};
use qivxif_graph::EdgeRecord;
use redb::ReadableTable;

pub(crate) fn check_edges(
    tx: &redb::ReadTransaction,
    findings: &mut Vec<RepairFinding>,
) -> StoreResult<()> {
    let edges = tx.open_table(tables::EDGES)?;
    let nodes = tx.open_table(tables::NODES)?;
    let by_from = tx.open_table(tables::EDGES_BY_FROM)?;
    let by_to = tx.open_table(tables::EDGES_BY_TO)?;
    for item in edges.iter()? {
        let (key, bytes) = item?;
        let key = key.value().to_owned();
        let Some(edge) = decode_edge(bytes.value(), "edges", &key, findings) else {
            continue;
        };
        if edge.tombstone.is_none() {
            require_node(&nodes, &edge.from_node, "edges", &key, findings)?;
            require_node(&nodes, &edge.to_node, "edges", &key, findings)?;
        }
        require_index(
            &by_from,
            &edge.from_node,
            &edge.id,
            "edge_from_index_missing",
            findings,
        )?;
        require_index(
            &by_to,
            &edge.to_node,
            &edge.id,
            "edge_to_index_missing",
            findings,
        )?;
    }
    check_index(tx, tables::EDGES_BY_FROM, "edges_by_from", true, findings)?;
    check_index(tx, tables::EDGES_BY_TO, "edges_by_to", false, findings)?;
    Ok(())
}

fn check_index(
    tx: &redb::ReadTransaction,
    table_def: redb::TableDefinition<&str, &[u8]>,
    table_name: &str,
    from_index: bool,
    findings: &mut Vec<RepairFinding>,
) -> StoreResult<()> {
    let index = tx.open_table(table_def)?;
    let edges = tx.open_table(tables::EDGES)?;
    for item in index.iter()? {
        let (key, edge_id_bytes) = item?;
        let key = key.value().to_owned();
        let Ok(edge_id) = decode::<EdgeId>(edge_id_bytes.value()) else {
            findings.push(finding(
                "decode_failed",
                table_name,
                &key,
                "edge id did not decode",
            ));
            continue;
        };
        let Some(edge_bytes) = edges.get(edge_id.as_str())? else {
            findings.push(finding(
                index_code(from_index, "dangling"),
                table_name,
                &key,
                "index points at missing edge",
            ));
            continue;
        };
        let Some(edge) = decode_edge(edge_bytes.value(), table_name, &key, findings) else {
            continue;
        };
        let endpoint = if from_index {
            &edge.from_node
        } else {
            &edge.to_node
        };
        if key != edge_key(endpoint, &edge.id) {
            findings.push(finding(
                index_code(from_index, "wrong_endpoint"),
                table_name,
                &key,
                "index key does not match edge endpoint",
            ));
        }
    }
    Ok(())
}

fn require_node(
    nodes: &redb::ReadOnlyTable<&str, &[u8]>,
    node_id: &NodeId,
    table: &str,
    key: &str,
    findings: &mut Vec<RepairFinding>,
) -> StoreResult<()> {
    if nodes.get(node_id.as_str())?.is_none() {
        findings.push(finding(
            "edge_target_missing",
            table,
            key,
            "edge endpoint node is missing",
        ));
    }
    Ok(())
}

fn require_index(
    index: &redb::ReadOnlyTable<&str, &[u8]>,
    node_id: &NodeId,
    edge_id: &EdgeId,
    code: &str,
    findings: &mut Vec<RepairFinding>,
) -> StoreResult<()> {
    if index.get(edge_key(node_id, edge_id).as_str())?.is_none() {
        findings.push(finding(
            code,
            "edges",
            edge_id.as_str(),
            "edge index entry is missing",
        ));
    }
    Ok(())
}

fn decode_edge(
    bytes: &[u8],
    table: &str,
    key: &str,
    findings: &mut Vec<RepairFinding>,
) -> Option<EdgeRecord> {
    match decode(bytes) {
        Ok(edge) => Some(edge),
        Err(_) => {
            findings.push(finding("decode_failed", table, key, "edge did not decode"));
            None
        }
    }
}

fn edge_key(node_id: &NodeId, edge_id: &EdgeId) -> String {
    format!("{}:{}", node_id.as_str(), edge_id.as_str())
}

fn index_code(from_index: bool, suffix: &str) -> &'static str {
    match (from_index, suffix) {
        (true, "dangling") => "edge_from_index_dangling",
        (false, "dangling") => "edge_to_index_dangling",
        (true, _) => "edge_from_index_wrong_endpoint",
        (false, _) => "edge_to_index_wrong_endpoint",
    }
}
