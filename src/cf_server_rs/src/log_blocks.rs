use crazyflie_lib::{subsystems::log::LogPeriod, subsystems::log::LogBlock, Crazyflie};
use rclrs::vendor::action_msgs::msg::GoalStatusArray;

pub const STATES_AND_DESIRED_STATES: &[&str] = &[
    "pose.x",
    "pose.y",
    "pose.yaw",
    "pose.x_des",
    "pose.y_des",
    "pose.yaw_des",
];

pub const ERROR_AND_ACTIONS: &[&str] = &[
    "error.x_err",
    "error.y_err",
    "error.yaw_err",
    "error.v_ff",
    "error.w_ff",
    "error.reserved",
];

pub const WHEEL_DATA: &[&str] = &[
    "wheel.duty_l",
    "wheel.duty_r",
    "wheel.omega_l_cmd",
    "wheel.omega_r_cmd",
    "wheel.omega_l_meas",
    "wheel.omega_r_meas",
];

pub const ROBOT_STATUS: &[&str] = &[
    "status.mode",
    "status.running",
    "status.t_ms",
    "status.rsv1",
    "status.rsv2",
    "status.rsv3",
];

pub struct LogBlocks {
    pub states_and_desired_states: LogBlock,
    pub error_and_actions: LogBlock,
    pub wheel_data: LogBlock,
    pub robot_status: LogBlock,
}

pub async fn register_log_blocks(cf: &Crazyflie) -> Result<LogBlocks, Box<dyn std::error::Error>> {
    let mut states_and_desired_states = cf.log.create_block().await?;
    let mut error_and_actions = cf.log.create_block().await?;
    let mut wheel_data = cf.log.create_block().await?;
    let mut robot_status = cf.log.create_block().await?;

    add_vars(&mut states_and_desired_states, STATES_AND_DESIRED_STATES).await?;
    add_vars(&mut error_and_actions, ERROR_AND_ACTIONS).await?;
    add_vars(&mut wheel_data, WHEEL_DATA).await?;
    add_vars(&mut robot_status, ROBOT_STATUS).await?;

    Ok(LogBlocks { states_and_desired_states, error_and_actions, wheel_data, robot_status })
}

async fn add_vars(
    block: &mut LogBlock,
    vars: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    for &var in vars {
        block.add_variable(var).await?;
    }
    Ok(())
}