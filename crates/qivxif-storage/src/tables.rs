use redb::TableDefinition;

pub(crate) const DB_FILE: &str = "world.redb";
pub(crate) const META: TableDefinition<&str, &[u8]> = TableDefinition::new("meta");
pub(crate) const SECTIONS: TableDefinition<&str, &[u8]> = TableDefinition::new("sections");
pub(crate) const META_WORLD: &str = "world";
