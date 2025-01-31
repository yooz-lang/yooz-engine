# Yooz Lang Database Engine

---

## **Overview**

`Yooz` is a multi-layer database engine designed for hierarchical data storage, management, and retrieval. This tool provides a command-line interface (CLI) with commands like `CREATE`, `ADD`, `FIND`, `REMOVE`, and `CHANGE` for working with data.

---

## **Features**

- **Hierarchical Data Storage:** Supports data storage in layers with add, delete, and search functionalities.
- **Simple and Fast CLI:** Designed for developers and users who prefer command-line tools.
- **Cross-Platform Installation:** Includes `.deb`, `.rpm`, and `.msi` packages along with installation  scripts.

---

## **Prerequisites**

- **Supported Operating Systems:**
  - Linux (Debian/Ubuntu and Red Hat/CentOS)
  - Windows
- **Required Tools:**
  - `Cargo` and Rust for building the project
  - `dpkg` and `rpm` for package management (Linux)

---

## **Installation Instructions**

### **1. Installing on Debian/Ubuntu**

1. Transfer the generated `.deb` package to your system.
2. Run the installation script:
   ```bash
   ./install-yooz.sh
   ```

3.After installation, you can verify the installation:

```bash
yooz --version
```

### **2. Installing on Red Hat/CentOS**

1. Transfer the generated `.rpm` package to your system.
2. Run the installation script:
   ```bash
   ./install-yooz-rpm.sh
   ```
3. Verify the installation:
```bash
yooz --version
```

### **3. Installing on Windows**

1. Run the install-windows.ps1 as administrator
2. Run the following command to verify the installation:
```bash
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

## **Contact Us**

* **Developers:**
  Mohammad Hasan Farrahi ([mohammad.hfarrahi@gmail.com]())
  Yooz-lang Team ([GitHub](https://github.com/yooz-lang))
* **Website:** [http://yooz.run](http://yooz.run)
* **Repository:** [Yooz-lang on GitHub](https://github.com/yooz-lang/yooz-engine)
