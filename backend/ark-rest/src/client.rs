use crate::apis::ark_service_api::ark_service_get_info;
use crate::models::V1GetInfoResponse;
use bitcoin::address::NetworkUnchecked;
use bitcoin::Address;
use std::error::Error as StdError;

pub struct Client {
    configuration: crate::apis::configuration::Configuration,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Error(Box<dyn StdError + Send + Sync + 'static>);

impl<T: std::fmt::Debug + Send + Sync + 'static> From<crate::apis::Error<T>> for Error {
    fn from(value: crate::apis::Error<T>) -> Self {
        Self(value.into())
    }
}

impl Client {
    pub fn new(ark_server_url: String) -> Self {
        let configuration = crate::apis::configuration::Configuration {
            base_path: ark_server_url,
            ..Default::default()
        };

        Self { configuration }
    }

    pub async fn get_info(&self) -> Result<ark_core::server::Info, Error> {
        let _info = ark_service_get_info(&self.configuration).await?;

        // TODO: Mapping from the `ark-rest` generated types to the `ark_core` types is too much of
        // a pain given that every field is currently optional. We are waiting for an update to the
        // swagger file to continue.

        //let info = info.try_into()?;

        // Ok(info)

        todo!()
    }
}
