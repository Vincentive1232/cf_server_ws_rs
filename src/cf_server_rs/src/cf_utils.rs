use crazyflie_lib::Crazyflie;
use std::sync::Arc;

pub async fn connect_cf(link_context: &crazyflie_link::LinkContext, uri: &str) -> Result<Arc<Crazyflie>, Box<dyn std::error::Error>> {
    let cf = Crazyflie::connect_from_uri(link_context, uri).await?;
    println!("Connected!");

    let protocol_version = cf.platform.protocol_version().await?;
    println!("protocol {}", protocol_version);

    Ok(Arc::new(cf))
}