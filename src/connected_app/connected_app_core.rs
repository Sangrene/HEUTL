use crate::connected_app::{
    connected_app_model::ConnectedApp, connected_app_repository::{ConnectedAppRepository, CreateConnectedAppParams},
};
use crate::shared::errors::Error;

pub struct ConnectedAppCore<'a> {
    pub connected_app_repository: Box<dyn ConnectedAppRepository + 'a>,
}

impl<'a> ConnectedAppCore<'a> {
    pub async fn create_connected_app(&self, params: &CreateConnectedAppParams) -> Result<ConnectedApp, Error> {
        return self.connected_app_repository.create_connected_app(params).await;
    }

    pub async fn get_connected_app(&self, id: &String) -> Result<ConnectedApp, Error> {
        return self.connected_app_repository.get_connected_app(id).await;
    }
}
