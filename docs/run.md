## Run HashPool library

To create a documentation run for the HashPool library written in Rust, you can follow these steps:

### 1. **Clone the Repository**
   First, you'll need to clone the repository to your local machine.

   ```bash
   git clone https://github.com/AreaLayer/HashPool.git
   cd HashPool
   ```

### 2. **Generate Documentation Using `cargo doc`**
   Rust provides a built-in tool to generate documentation for a project. Use `cargo doc` to generate the documentation.

   ```bash
   cargo doc --no-deps
   ```

   This command will generate HTML documentation for your project in the `target/doc` directory. The `--no-deps` flag ensures that only your project’s documentation is generated, without including the documentation of dependencies.

