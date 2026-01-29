use crazyflie_lib::Crazyflie;
use rclrs::{Node, Service, Context, CreateBasicExecutor, SpinOptions, ServiceInfo};
use std::sync::Arc;

use crazyflie_lib::subsystems::log::LogPeriod;
use std::time::Duration;
use workstation_ros_rs::log_blocks::register_log_blocks;


struct CrazyflieROS {
    node: Node,
    cf: Arc<crazyflie_lib::Crazyflie>,
}

impl CrazyflieROS {
    pub async fn new(node: Node, link_context: &crazyflie_link::LinkContext, uri: &str) -> Self {
        let cf = crazyflie_lib::Crazyflie::connect_from_uri(
                &link_context,
                uri).await.unwrap();
        println!("Connected!");
        let cfarc = Arc::<Crazyflie>::new(cf);

        let result = CrazyflieROS {
            node: node.clone(),
            cf: cfarc.clone(),
        };

        
        // let firmware_version = result.cf.platform.firmware_version().await.unwrap();
        let protocol_version = result.cf.platform.protocol_version().await.unwrap();
        println!(
            "protocol {}", protocol_version
        );

        // let device_type = result.cf.platform.device_type_name().await.unwrap();
        // println!("Device type:          {}", device_type);

        // println!("Number of params var: {}", result.cf.param.names().len());
        // println!("Number of log var:    {}", result.cf.log.names().len());


        result
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut executor = Context::default_from_env()?.create_basic_executor();
    let node = executor.create_node("crazyflie_server_rs")?;


    // let broadcasts_num_repeats_ = node
    //     .declare_parameter("all.broadcasts.num_repeats")
    //     .default(15).mandatory().unwrap();


    // println!("{}", broadcasts_num_repeats_.get());

    // println!("{}", node.use_undeclared_parameters().get::<bool>("robots.cf231.enabled").unwrap());


    let link_context = crazyflie_link::LinkContext::new();

    let cfros = CrazyflieROS::new(node, &link_context, "radio://0/80/2M/E7C2C2C201").await;

    println!("Registering log blocks...");
    let blocks = register_log_blocks(&cfros.cf).await?;
    println!("Register finished! Starting log stream...");

    // 第一段：200ms
    let stream_states_and_desired_states = blocks
        .states_and_desired_states
        .start(Duration::from_millis(200).try_into()?)
        .await?;
    let stream_error_and_actions = blocks
        .error_and_actions
        .start(LogPeriod::from_millis(200)?).await?;

    for _ in 0..20 {
        let data_pos_vel = stream_states_and_desired_states.next().await?;
        let data_orientation = stream_error_and_actions.next().await?;
        println!("{:?}", data_pos_vel);
        println!("{:?}", data_orientation);
    }
        
    
    executor.spin(SpinOptions::default());

    drop(cfros);

    Ok(())
}