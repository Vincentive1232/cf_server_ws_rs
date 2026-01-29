# Since the ROS2-Rust is still a mess on crate.io and the official tutorial is out-dated, I wrote a tutorial here to help people do a quickstart.

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

## create workspace and build
1. Create normal ros workspace:
    ```bash
    mkdir -p workspace_name/src && cd workspace_name
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
3. Create a new rust package:
    ```bash
    cd src
    cargo new package_name
    ```
4. Add a `package.xml` file in the new package folder with(just an example):
    ```xml
    <?xml version="1.0"?>
    <?xml-model href="http://download.ros.org/schema/package_format3.xsd" schematypens="http://www.w3.org/2001/XMLSchema"?>
    <package format="3">
    <name>package</name>
    <version>1.0.3</version>
    <description>ROS 2 Package for Bitcraze Crazyflie robots</description>
    <maintainer email="name@email.com">Name</maintainer>
    <license>MIT</license>

    <depend>rclrs</depend>
    <depend>rosidl_runtime_rs</depend>
    <depend>std_msgs</depend>

    <export>
        <build_type>ament_cargo</build_type>
    </export>
    </package>
    ```
    Here you can define you package name, version, denpendencies, etc. Noted that the build_type must be `ament_cargo` for rust packages.
5. Include `rclrs = "0.7"` in your `Cargo.toml` dependencies.
6. Build the rosidl_generator_rs first by:
   ```bash
   colcon build --packages-select rosidl_generator_rs
   ```
7. Then build the `std_msgs` package **(must be built after `rosidl_generator_rs`)**:
   ```bash
   colcon build --packages-select std_msgs
   ```
8. Finally build the workspace including your rust package:
   ```bash
    colcon build
    ```
    Here some errors might occur but if you don't use those packages that cause errors, you can ignore them.

    Or you could build your package only by:
    ```bash
    colcon build --packages-select package_name
    ```
9. Finally, source the workspace:
    ```bash
    source install/setup.bash
    ```
