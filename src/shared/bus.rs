use crate::entity_sharing::entity_sharing_model::EntitySharing;
use crate::entity_subscription::entity_subscription_model::EntitySubscription;
use serde_json::Value;

pub enum Commands {
    EntitySharingCreated {
        entity_sharing: EntitySharing,
    },
    NewEntityData {
        entity_sharing_id: String,
        data: Value,
    },
    EntitySubscriptionCreated {
        entity_subscription: EntitySubscription,
    },
}

#[derive(PartialEq, Clone)]
pub enum TopicIds {
    EntitySharingCreated,
    NewEntityData,
    EntitySubscriptionCreated,
}
