use crate::entity_sharing::entity_sharing_model::EntitySharing;
use crate::entity_subscription::entity_subscription_core::EntitySubscriptionCore;
use crate::shared::bus::{Commands, TopicIds};
use crate::shared::errors::Error;
use crate::shared::python_runner::run_python_script_output_json;
use futures::future::join_all;
use pubsub_bus::BusEvent;
use pubsub_bus::Subscriber;
use serde_json::json;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use tokio;
use tokio::sync::broadcast;

pub struct EntityPollingHandler {
    handles: Vec<JoinHandle<()>>,
    should_stop: Arc<AtomicBool>,
    entity_subscription_core: Arc<EntitySubscriptionCore<'static>>,
    channel: (
        broadcast::Sender<EntitySharing>,
        broadcast::Receiver<EntitySharing>,
    ),
}

impl Subscriber<Commands, TopicIds> for EntityPollingHandler {
    fn on_event(&mut self, event: &BusEvent<Commands, TopicIds>) {
        match event.get_content() {
            Commands::EntitySharingCreated { entity_sharing } => {
                let entity_sharing = entity_sharing.clone();
                let entity_subscription_core = Arc::clone(&self.entity_subscription_core);
                let should_stop = Arc::clone(&self.should_stop);

                let handle = setup_new_entity_sharing_polling(
                    entity_sharing,
                    entity_subscription_core,
                    should_stop,
                    self.channel.0.subscribe()
                );

                self.handles.push(handle);
            }
            Commands::EntitySharingUpdated { entity_sharing } => {
                self.channel.0.send(entity_sharing.clone()).unwrap();
            }
            _ => {}
        }
    }

    fn is_subscribed_to(&self, topic_id: &TopicIds) -> bool {
        match topic_id {
            TopicIds::EntitySharingCreated => true,
            TopicIds::EntitySharingUpdated => true,
            _ => false,
        }
    }
}

impl EntityPollingHandler {
    pub fn new(
        entity_subscription_core: Arc<EntitySubscriptionCore<'static>>,
        should_stop: Arc<AtomicBool>,
    ) -> Self {
        Self {
            handles: vec![],
            entity_subscription_core,
            should_stop,
            channel: broadcast::channel(16),
        }
    }
}

fn setup_new_entity_sharing_polling(
    entity_sharing: EntitySharing,
    entity_subscription_core: Arc<EntitySubscriptionCore<'static>>,
    should_stop: Arc<AtomicBool>,
    receiver: broadcast::Receiver<EntitySharing>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        if let Err(e) = rt.block_on(run_entity_sharing_polling(
            entity_sharing,
            entity_subscription_core,
            &should_stop,
            receiver,
        )) {
            eprintln!("Error in entity sharing polling: {:?}", e);
        }
    })
}

async fn run_entity_sharing_polling(
    mut entity_sharing: EntitySharing,
    entity_subscription_core: Arc<EntitySubscriptionCore<'static>>,
    should_stop: &Arc<AtomicBool>,
    mut receiver: broadcast::Receiver<EntitySharing>,
) -> Result<(), Error> {
    if entity_sharing.polling_infos.is_none() {
        return Ok(());
    }

    println!(
        "Starting entity sharing polling thread: {:?}",
        entity_sharing.name
    );
    while !should_stop.load(Ordering::Relaxed) {
        if !receiver.is_empty() {
            if let Ok(new_entity_sharing) = receiver.recv().await {
                entity_sharing = new_entity_sharing;
            }
        }
        
        let entity_subscriptions = entity_subscription_core
            .get_all_entity_subscriptions_for_entity_sharing(&entity_sharing.id)
            .await
            .unwrap();
        if let Some(polling_infos) = &entity_sharing.polling_infos {
            if let Some(python_script) = &entity_sharing.python_script {
                //TODO: set the input of the python script
                let result = match run_python_script_output_json(python_script, &json!({})) {
                    Ok(result) => result,
                    Err(e) => {
                        eprintln!(
                            "Error polling entity sharing: {:?} - {:?}",
                            entity_sharing.name, e
                        );
                        tokio::time::sleep(Duration::from_millis(10000)).await;
                        continue;
                    }
                };
                join_all(entity_subscriptions.into_iter().map(async |sub| {
                    entity_subscription_core
                        .notify_subscription_of_new_entity_list(&sub, &result)
                        .await
                }))
                .await;
            }
            tokio::time::sleep(Duration::from_millis(polling_infos.polling_interval)).await;
        }

        if should_stop.load(Ordering::Relaxed) {
            break;
        }
    }

    println!(
        "Stopping entity sharing polling thread: {}",
        entity_sharing.id
    );
    Ok(())
}
