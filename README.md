# Yooz Lang Database Engine

---

## **Overview**

**Yooz** is a multi-layer database engine designed for efficient hierarchical data storage, management, and retrieval. It stores data in .yooz files, allowing flexible structured and nested layers for dynamic data organization. **`Yooz`** offers a command-line interface (CLI) with intuitive commands like **`CREATE`**, **`ADD`**, **`FIND`**, **`REMOVE`**, and **`CHANGE`** to interact with your data seamlessly.

---

## **Key Features**

**Hierarchical Data Management:** Store, organize, and retrieve data in nested layers with flexible add, delete, and search operations.

**Lightweight & Fast CLI:** Optimized for developers who prefer working with efficient, minimalistic command-line tools.

**Cross-Platform Support:** Available for Linux (Debian/Ubuntu, Red Hat/CentOS) and Windows, with .deb, .rpm, and .msi installation packages.

---

## **Prerequisites**

- **Supported Operating Systems:**
  - Linux (Debian/Ubuntu and Red Hat/CentOS)
  - Windows
- **Required Tools:**
  - `Cargo` and Rust for building the project
  - `dpkg` and `rpm` for package management (Linux)

---

## **Installation Guide**

### **1. Debian/Ubuntu**

1. Download the ZIP package for Debian from the link below then extract it
2. Run the installation script:
   ```bash
   ./install-debian.sh
   ```

3. Verify the installation:
```bash
yooz --version
```

### **2. Red Hat/CentOS**

1. Download the ZIP package for `Red Hat` from the link below then extract it.
2. Run the installation script:
   ```bash
   ./install-redhat.sh
   ```
3. Verify the installation:
```bash
yooz --version
```

### **3. Windows**

1. Download the ZIP package for `Windows` from the link below then extract it.
2. Execute the `.msi` installer file in the ZIP
3. The `yooz`  will be automatically added to your `PATH`.
4. Verify the installation:

```powershell
yooz --version
```

---

### **Database Structure**

**`Yooz`** stores data in .yooz files using a hierarchical, multi-layered structure. Data can be organized in any order, mixing key-value pairs with nested layers at different levels.

Data Format:

* Keys are marked with +

* Values are marked with -

* Each layer is enclosed in parentheses ()

---

Example Structure:

```
(
  +name
  -mohammad
  (
    +country
    -iran
  )
  +age
  -22
)
```

---

## **Queries**

**`Yooz`** provides a set of queries for database management:

| **Query** | **Description**       |      **Example**  |
| ----------------- | ----------------------|-------------------------------------------------------------------- |
| `CREATE`        | Creates a new database. | ` yooz CREATE dbname`                                    |
| `ADD`           | Adds data to the database. | ` yooz "ADD TO dbname(key(layer)) value"`             |
| `FIND`          | Searches for data in the database. | ` yooz "FIND IN dbname(key(layer))"`          |
| `REMOVE`        | Removes data from the database. | ` yooz "REMOVE FROM dbname(key(layer))"`         |
| `CHANGE`        | Updates data in the database. | ` yooz "CHANGE FROM dbname(key(layer)) new-value"` |
| `help`          | Displays command help information.      | `yooz help`                                                   |
| `--version`     | Displays the installed version of `yooz`.   | `yooz --version`                                             |

---

## **Examples**

### **1. Creating a Database:**

```bash
yooz CREATE mydb
```

### **2. Adding Data:**

```bash
yooz "ADD TO mydb(name(1)) mohammad"
```
```bash
yooz "ADD TO mydb(country(2)) iran"
```

### **3. Finding Data:**

```bash
yooz "FIND IN mydb(name(1))"
```

4. Updating Data:

```bash
yooz "CHANGE FROM mydb(age(1)) 23"
```

### **5. Removing Data:**

```bash
yooz "REMOVE FROM mydb(name(1))"
```

--- 

### Download Links

<p style="display:flex;gap:2rem;margin-left:3rem">
  <a href="https://github.com/user-attachments/files/18624125/yooz_0.1.0_debian.zip">
    <img src="https://img.icons8.com/color/48/000000/debian.png" alt="Debian" />
  </a>
  <a href="https://github.com/user-attachments/files/18624112/yooz-0.1.0-1.redhat.zip">
    <img src="https://img.icons8.com/color/48/000000/red-hat.png" alt="RedHat" />
  </a>
  <a href="https://github.com/user-attachments/files/18624097/yooz-0.1.0-x86_64.zip">
    <img src="https://img.icons8.com/color/48/000000/windows-10.png" alt="Windows" />
  </a>
</p>

---

### **Maintainer**

* **Developer:** **Mohammad Hasan Farrahi** ([mohammad.hfarrahi@gmail.com](mailto:mohammad.hfarrahi@gmail.com))
* **Team:** **Yooz-lang Team** ([GitHub](https://github.com/yooz-lang))
* **Website:** [Yooz](http://yooz.run)
* **Repository:** [Yooz-lang on GitHub](https://github.com/yooz-lang/yooz-engine)

---