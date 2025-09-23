use crate::entity_sharing::{
    entity_sharing_core::EntitySharingCore, entity_sharing_model::EntitySharing,
};
use crate::entity_subscription::entity_subscription_core::EntitySubscriptionCore;
use crate::entity_subscription::entity_subscription_model::EntitySubscription;
use crate::shared::errors::Error;
use reqwest;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub struct EntityPollingHandler {
    handles: Vec<JoinHandle<()>>,
}

impl EntityPollingHandler {
    pub fn new() -> Self {
        Self { handles: vec![] }
    }

    pub async fn init_entity_sharings_polling(
        &mut self,
        entity_subscription_core: &EntitySubscriptionCore<'static>,
        entity_sharing_core: &EntitySharingCore<'static>,
        should_stop: &Arc<AtomicBool>,
    ) -> Result<(), Error> {
        let all_entity_sharings = entity_sharing_core
            .get_all_polling_entity_sharings()
            .await?;

        for entity_sharing in all_entity_sharings {
            if let Some(polling_infos) = entity_sharing.polling_infos {
                if let Err(e) = self
                    .setup_new_entity_sharing_polling(
                        entity_sharing.id,
                        entity_subscription_core,
                        entity_sharing_core,
                        should_stop,
                    )
                    .await
                {
                    eprintln!("Error setting up new entity sharing polling: {:?}", e);
                }
            }
        }
        for handle in self.handles.drain(..) {
            handle.join().unwrap();
        }
        Ok(())
    }

    async fn setup_new_entity_sharing_polling(
        &mut self,
        entity_sharing_id: String,
        entity_subscription_core: &EntitySubscriptionCore<'static>,
        entity_sharing_core: &EntitySharingCore<'static>,
        should_stop: &Arc<AtomicBool>,
    ) -> Result<(), Error> {
        let entity_sharing = entity_sharing_core
            .get_entity_sharing(&entity_sharing_id)
            .await?;
        let entity_subscriptions = entity_subscription_core
            .get_all_entity_subscriptions_for_entity_sharing(&entity_sharing_id)
            .await?;
        let should_stop_clone = Arc::clone(&should_stop);
        let handle = thread::spawn(move || {
            run_entity_sharing_polling(entity_sharing, entity_subscriptions, &should_stop_clone);
        });
        self.handles.push(handle);
        Ok(())
    }
}

fn run_entity_sharing_polling(
    entity_sharing: EntitySharing,
    entity_subscriptions: Vec<EntitySubscription>,
    should_stop: &Arc<AtomicBool>,
) {
    println!("Starting entity sharing thread: {:?}", entity_sharing);

    while !should_stop.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_secs(1));
        if let Some(polling_infos) = &entity_sharing.polling_infos {
            // reqwest::get(polling_infos.polling_url).await.unwrap();
            println!("Entity sharing: {:?}", polling_infos.polling_url);
            
        }

        if should_stop.load(Ordering::Relaxed) {
            break;
        }
    }

    println!("Stopping entity subscription thread: {}", entity_sharing.id);
}
