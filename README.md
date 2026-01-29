# ROS2 Communication Node for Crazyflie and Pololu Robot in RUST
This is a ROS2 communication node written in Rust for controlling Bitcraze Crazyflie drones and Pololu robots. It leverages the `crazyflie-link` and `crazyflie-lib` to interact with the Crazyflie drones and Pololu robots, providing a robust and efficient way to manage these devices within a ROS2 ecosystem. CRTP(Crazy Real-Time Protocol) is used as the communication protocol between the node and the robots.

# Quickstart
## install prerequisites
1. Install Rust (see https://rustup.rs/)
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
2. Install required system packages
    ```bash
    sudo apt install -y git libclang-dev python3-pip python3-vcstool
    ```
3. Install colcon plugins for Rust
    ```bash
    pip install colcon-cargo colcon-ros-cargo
    ```
4. Install tw2 extra packages due to some issues in rclrs
    ```bash
    sudo apt install -y ros-$ROS_DISTRO-example-interfaces
    sudo apt install -y ros-$ROS_DISTRO-test-msgs
    ```
5. Install empy
   ```bash
    python3 -m pip install -U empy==3.3.4
    ```
6. Install other cmake related dependencies
   ```bash
   python3 -m pip install -U catkin_pkg lark-parser
   ```


## Build
1. clone this repo:
    ```bash
    git clone git@github.com:Vincentive1232/cf_server_ws_rs.git
    cd cf_server_rs
    ```

2. Clone all requiered repos(not all of them are necessary, just in case you need them in the future):
    ```bash
    git clone -b jazzy https://github.com/ros2/common_interfaces.git src/common_interfaces
    git clone -b jazzy https://github.com/ros2/example_interfaces.git src/example_interfaces
    git clone -b jazzy https://github.com/ros2/rcl_interfaces.git src/rcl_interfaces
    git clone -b jazzy https://github.com/ros2/rosidl_core.git src/rosidl_core
    git clone -b jazzy https://github.com/ros2/rosidl_defaults.git src/rosidl_defaults
    git clone -b jazzy https://github.com/ros2/unique_identifier_msgs.git src/unique_identifier_msgs
    git clone https://github.com/ros2-rust/rosidl_rust.git src/rosidl_rust
    ```
3. Build the rosidl_generator_rs first by:
   ```bash
   colcon build --packages-select rosidl_generator_rs
   ```
4. Then build the `std_msgs` package **(must be built after `rosidl_generator_rs`)**:
   ```bash
   colcon build --packages-select std_msgs
   ```
5. Finally build the workspace including your rust package:
   ```bash
    colcon build
    ```
    Here some errors might occur but if you don't use those packages that cause errors, you can ignore them.

    Or you could build your package only by:
    ```bash
    colcon build --packages-select cf_server_rs
    ```
6. Finally, source the workspace:
    ```bash
    source install/setup.bash
    ```

## run
```bash
ros2 run cf_server_rs cf_server_rs
```