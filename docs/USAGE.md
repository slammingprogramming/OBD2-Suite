# Usage Guide for Universal OBD-II Debugging Suite

The Universal OBD-II Debugging Suite provides a comprehensive set of tools for diagnosing and debugging vehicles equipped with OBD-II systems. This guide will walk you through the steps to effectively use the suite.

## Getting Started

1. **Connect Your OBD-II Adapter**

   Before using the suite, ensure your OBD-II adapter is properly connected to the vehicle's OBD-II port. This port is typically located under the dashboard, near the driver's seat.

2. **Launch the Application**

   Open a terminal or command prompt and navigate to the project directory where the application is located. Run the application using the following command:

   ./target/release/obd2_suite

   On Windows, use:

   .\target\release\obd2_suite.exe

3. **Establish a Connection**

   Upon launching the application, follow the on-screen prompts to select your OBD-II adapter (if applicable) and establish a connection with the vehicle. Ensure that the vehicle's ignition is in the "ON" position, but the engine does not need to be running.

## Navigating the Interface

The user interface is designed for ease of use, with the following key sections:

- **Home Screen**: Displays options to access different functionalities such as data retrieval, error code interpretation, and live monitoring.
- **Data Categories**: Choose from various categories such as Engine, Transmission, ABS, SRS, and more to retrieve specific diagnostic information.
- **Real-time Monitoring**: Access live data streams to view sensor readings such as RPM, vehicle speed, and coolant temperature.

## Retrieving Data

1. **Select Data Category**

   From the main menu, select the data category you wish to analyze. Options include:
   - Engine Data
   - Transmission Data
   - ABS Data
   - SRS Data
   - DTCs (Diagnostic Trouble Codes)
   - Emissions Data

2. **View Retrieved Data**

   Once you select a category, the suite will retrieve and display relevant information. This may include sensor readings, system statuses, and error codes.

3. **Error Code Interpretation**

   If DTCs are present, you can click on each code to view detailed interpretations and possible solutions. The suite provides context and suggestions for resolving issues.

## Real-time Monitoring

To monitor live sensor data:

1. Navigate to the **Real-time Monitoring** section.
2. Select the desired parameters you wish to monitor (e.g., RPM, coolant temperature).
3. View the live data displayed on the screen, which updates in real-time.

## Generating Diagnostic Reports

1. After retrieving and analyzing the data, you can generate a diagnostic report.
2. Select the option to create a report from the main menu.
3. Choose the data you want to include, such as error codes and sensor readings.
4. Save the report in your desired format (e.g., PDF, text file).

## Exiting the Application

To exit the Universal OBD-II Debugging Suite, simply close the application window or press Ctrl + C in the terminal.

## Conclusion

You are now equipped to utilize the Universal OBD-II Debugging Suite effectively. For further assistance or to report issues, please refer to the Support section of our GitHub repository.
