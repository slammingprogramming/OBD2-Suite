# Installation Guide for Universal OBD-II Debugging Suite

The Universal OBD-II Debugging Suite is designed to be user-friendly and easy to install. Follow the steps below to set up the application on your system.

## Prerequisites

Before installing the Universal OBD-II Debugging Suite, ensure that you have the following:

- **Rust**: You need to have Rust and Cargo (Rust's package manager and build system) installed. If you haven't installed Rust yet, you can download and install it from [rustup.rs](https://rustup.rs/).
- **OBD-II Adapter**: A compatible OBD-II adapter to connect your computer to the vehicle's OBD-II port. This can be a wired or wireless adapter.
- **Required Dependencies**: The suite may require additional dependencies based on your operating system.

## Installation Steps

1. **Clone the Repository**

   Open your terminal or command prompt and run the following command to clone the repository:

   git clone https://github.com/slammingprogramming/OBD2-Suite.git

   Navigate into the project directory:

   cd OBD2-Suite

2. **Build the Project**

   Once you are in the project directory, run the following command to build the project:

   cargo build --release

   This command compiles the project and generates the executable in the `target/release` directory.

3. **Install Additional Dependencies**

   Depending on your system, you may need to install additional dependencies. Follow the instructions for your operating system below:

   ### Windows

   - Make sure you have the latest version of the Visual Studio Build Tools installed. You can download it from [here](https://visualstudio.microsoft.com/visual-cpp-build-tools/).

   ### macOS

   - Install Xcode Command Line Tools by running:

     xcode-select --install

   ### Linux

   - Ensure you have the necessary build tools and libraries installed. On Ubuntu, you can run:

     sudo apt update
     sudo apt install build-essential libssl-dev

4. **Run the Application**

   After building the project successfully, you can run the application using the following command:

   ./target/release/obd2_suite

   If you are on Windows, you can run:

   .\target\release\obd2_suite.exe

5. **Connect to Your Vehicle**

   Ensure your OBD-II adapter is connected to the vehicle's OBD-II port and your computer. Follow the on-screen instructions to establish a connection and start diagnosing.

## Troubleshooting

If you encounter any issues during installation or while running the application, please refer to the [Troubleshooting](https://github.com/slammingprogramming/OBD2-Suite/issues) section of our GitHub repository or create a new issue for assistance.

## Conclusion

You are now ready to use the Universal OBD-II Debugging Suite to diagnose and debug your vehicle's systems. For further usage instructions and features, please refer to the [README.md](../README.md) file.

Happy troubleshooting!
