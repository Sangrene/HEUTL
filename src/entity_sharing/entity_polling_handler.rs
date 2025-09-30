use crate::entity_sharing::{
    entity_sharing_core::EntitySharingCore, entity_sharing_model::EntitySharing,
};
use crate::entity_subscription::entity_subscription_core::EntitySubscriptionCore;
use crate::entity_subscription::entity_subscription_model::EntitySubscription;
use crate::shared::bus::{Commands, TopicIds};
use crate::shared::errors::Error;
use pubsub_bus::BusEvent;
use pubsub_bus::Subscriber;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use tokio;

pub struct EntityPollingHandler {
    handles: Vec<JoinHandle<()>>,
    should_stop: Arc<AtomicBool>,
    entity_subscription_core: Arc<EntitySubscriptionCore<'static>>,
    entity_sharing_core: Arc<Mutex<EntitySharingCore<'static>>>,
}

impl Subscriber<Commands, TopicIds> for EntityPollingHandler {
    fn on_event(&mut self, event: &BusEvent<Commands, TopicIds>) {
        let should_stop = Arc::clone(&self.should_stop);
        let entity_subscription_core = Arc::clone(&self.entity_subscription_core);
        let entity_sharing_core = Arc::clone(&self.entity_sharing_core);

        match event.get_content() {
            Commands::EntitySharingCreated { entity_sharing } => {
                let entity_sharing = entity_sharing.clone();
                let entity_subscription_core_clone = Arc::clone(&entity_subscription_core);
                let should_stop_clone = Arc::clone(&should_stop);

                tokio::task::spawn(async move {
                    let _ = setup_new_entity_sharing_polling(
                        entity_sharing,
                        entity_subscription_core_clone,
                        &should_stop_clone,
                    )
                    .await;
                });
            }
            _ => {}
        }
    }

    fn is_subscribed_to(&self, topic_id: &TopicIds) -> bool {
        match topic_id {
            TopicIds::EntitySharingCreated => true,
            _ => false,
        }
    }
}

impl EntityPollingHandler {
    pub fn new(
        entity_subscription_core: Arc<EntitySubscriptionCore<'static>>,
        entity_sharing_core: Arc<Mutex<EntitySharingCore<'static>>>,
        should_stop: Arc<AtomicBool>,
    ) -> Self {
        Self {
            handles: vec![],
            entity_subscription_core,
            entity_sharing_core,
            should_stop,
        }
    }

    pub async fn init_entity_sharings_polling(&mut self) -> Result<(), Error> {
        let all_entity_sharings = self
            .entity_sharing_core
            .lock()
            .unwrap()
            .get_all_polling_entity_sharings()
            .await?;
        let should_stop = Arc::clone(&self.should_stop);
        for entity_sharing in all_entity_sharings {
            if let Err(e) = setup_new_entity_sharing_polling(
                entity_sharing,
                Arc::clone(&self.entity_subscription_core),
                &should_stop,
            )
            .await
            {
                eprintln!("Error setting up new entity sharing polling: {:?}", e);
            }
        }
        for handle in self.handles.drain(..) {
            handle.join().unwrap();
        }
        Ok(())
    }
}

async fn setup_new_entity_sharing_polling(
    entity_sharing: EntitySharing,
    entity_subscription_core: Arc<EntitySubscriptionCore<'static>>,
    should_stop: &Arc<AtomicBool>,
) -> Result<(), Error> {
    if entity_sharing.polling_infos.is_none() {
        return Ok(());
    }

    let should_stop_clone = Arc::clone(&should_stop);
    let handle = thread::spawn(async move || {
        run_entity_sharing_polling(entity_sharing, entity_subscription_core, &should_stop_clone)
            .await;
    });
    handle.join().unwrap().await;
    Ok(())
}

async fn run_entity_sharing_polling(
    entity_sharing: EntitySharing,
    entity_subscription_core: Arc<EntitySubscriptionCore<'static>>,
    should_stop: &Arc<AtomicBool>,
) {
    println!("Starting entity sharing thread: {:?}", entity_sharing);

    while !should_stop.load(Ordering::Relaxed) {
        let entity_subscriptions = entity_subscription_core
            .get_all_entity_subscriptions_for_entity_sharing(&entity_sharing.id)
            .await;
        if let Some(polling_infos) = &entity_sharing.polling_infos {
            reqwest::get(&polling_infos.polling_url).await.unwrap();
            println!("Entity sharing: {:?}", polling_infos.polling_url);
            println!("Entity subscriptions: {:?}", entity_subscriptions);
        }

        if should_stop.load(Ordering::Relaxed) {
            break;
        }
    }

    println!(
        "Stopping entity sharing polling thread: {}",
        entity_sharing.id
    );
}
