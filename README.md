<div align="center">
  <img style="text-align:center;border-radius:50%;" src="https://github.com/yooz-lang.png" height="128px" width="128px">

  # Yooz Language Database Engine

  [Website](http://yooz.run) |
  [Docs](http://yooz.run/yooz.pdf) |
  [Email](mailto:mohammad.hfarrahi@gmail.com)
  
  [![](https://img.shields.io/github/v/tag/yooz-lang/yooz-engine)](https://github.com/yooz-lang/yooz-engine/releases)

</div>

## **Overview**

`Yooz` is a high-performance, multi-layer database engine designed for hierarchical data storage, management, and retrieval. It offers a powerful yet simple command-line interface (CLI) that enables users to create, modify, and query their databases with intuitive commands like `CREATE`, `ADD`, `FIND`, `REMOVE`, and `CHANGE`.

Built with efficiency and scalability in mind, `Yooz` is a perfect solution for developers who require structured yet flexible data storage with minimal overhead.

## **Key Features**

- 🚀 **Hierarchical Data Storage** – Organize and manage data in a structured, multi-layered format.
- 🖥 **Intuitive CLI** – Perform database operations with easy-to-use commands.
- 🌍 **Cross-Platform Support** – Available for Linux (Debian/Ubuntu, Red Hat/CentOS) and Windows.
- 📦 **Pre-built Packages** – Distributable as `.deb`, `.rpm`, and `.msi` files for seamless installation.
- ⚡ **Fast & Lightweight** – Optimized for speed and low resource consumption.

## **Prerequisites**

- **Supported Operating Systems:**
  - Linux (Debian/Ubuntu and Red Hat/CentOS)
  - Windows
- **Required Tools:**
  - `Cargo` and Rust for building the project
  - `dpkg` and `rpm` for package management (Linux)

## **System Requirements**

### **Supported Operating Systems:**
- **Linux:** Debian-based (Ubuntu), Red Hat-based (CentOS, Fedora)
- **Windows:** Windows 10 and later

### **Dependencies:**
- `Cargo` and Rust (for building the project)
- `dpkg` (for Debian-based systems)
- `rpm` (for Red Hat-based systems)

---

## **Installation Guide**

### **1. Installing on Debian/Ubuntu**

1. Download the `.deb` package and transfer it to your system.
2. Run the installation script:
   ```bash
   ./install-yooz.sh
   ```
3. Verify the installation:
   ```bash
   yooz --version
   ```

### **2. Installing on Red Hat/CentOS**

1. Download the `.rpm` package and transfer it to your system.
2. Run the installation script:
   ```bash
   ./install-yooz-rpm.sh
   ```
3. Verify the installation:
   ```bash
   yooz --version
   ```

### **3. Installing on Windows**

1. Run the `.msi` installer.
2. The `yooz` CLI will be automatically added to your system `PATH`.
3. Verify the installation:
   ```powershell
   yooz --version
   ```

## **Queries**

`Yooz` provides a set of queries for database management:

| **Command** | **Description**                                                                      |
| ----------------- | ------------------------------------------------------------------------------------------ |
| `CREATE`        | Creates a new database. Example:` yooz CREATE dbname`                                    |
| `ADD`           | Adds data to the database. Example:` yooz "ADD TO dbname(key(layer)) value"`             |
| `FIND`          | Searches for data in the database. Example:` yooz "FIND IN dbname(key(layer))"`          |
| `REMOVE`        | Removes data from the database. Example:` yooz "REMOVE FROM dbname(key(layer))"`         |
| `CHANGE`        | Updates data in the database. Example:` yooz "CHANGE FROM dbname(key(layer)) new-value"` |
| `help`          | Displays command help information.                                                         |
| `--version`     | Displays the installed version of `yooz`.                                                |
You can find `Yooz` documentation, [Here](http://yooz.run/yooz.pdf).

## **Getting Involved**
We welcome contributions! Whether it’s reporting bugs, improving documentation, or adding features, feel free to [open an issue](https://github.com/yooz-lang/yooz-engine/issues/new/choose) or [fork and submit a pull request](https://github.com/yooz-lang/yooz-engine/fork) on GitHub.

