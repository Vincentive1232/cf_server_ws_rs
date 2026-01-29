use rclrs::{Context, CreateBasicExecutor, SpinOptions};

pub struct RosRuntime {
    pub executor: rclrs::Executor,
    pub node: rclrs::Node,
}

pub fn init_ros(node_name: &str) -> Result<RosRuntime, Box<dyn std::error::Error>> {
    let mut executor = Context::default_from_env()?.create_basic_executor();
    let node = executor.create_node(node_name)?;

    Ok(RosRuntime { executor, node })
}

pub async fn spin_blocking(mut executor: rclrs::Executor) {
    tokio::task::spawn_blocking(move || { executor.spin(SpinOptions::default()); })
        .await
        .ok();
}