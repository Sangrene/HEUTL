use crate::entity_sharing::entity_sharing_model::EntitySharing;

#[derive(Debug)]
pub enum Commands {
    EntitySharingCreated { entity_sharing: EntitySharing },
    EntitySharingUpdated { entity_sharing: EntitySharing },
}

#[derive(PartialEq, Clone)]
pub enum TopicIds {
    EntitySharingCreated,
    EntitySharingUpdated,
}
