use crate::AuthContext;
use qivxif_core::Visibility;
use qivxif_graph::NodeRecord;

pub fn can_read(auth: &AuthContext, node: &NodeRecord) -> bool {
    node.visibility == Visibility::Public
        || auth.is_admin()
        || auth
            .user_id()
            .is_some_and(|user_id| user_id == &node.owner_user_id)
}

pub fn can_write(auth: &AuthContext, node: &NodeRecord) -> bool {
    auth.is_admin()
        || auth
            .user_id()
            .is_some_and(|user_id| user_id == &node.owner_user_id)
}

pub fn can_link(auth: &AuthContext, from: &NodeRecord, to: &NodeRecord) -> bool {
    can_write(auth, from) && can_read(auth, to)
}

pub fn can_publish(auth: &AuthContext, node: &NodeRecord) -> bool {
    can_write(auth, node)
}

pub fn can_moderate(auth: &AuthContext, _target: &NodeRecord) -> bool {
    auth.is_admin()
}

pub fn can_administer(auth: &AuthContext, _target: &NodeRecord) -> bool {
    auth.is_admin()
}
