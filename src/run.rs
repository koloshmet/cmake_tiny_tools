use tokio::io::AsyncWriteExt;

fn get_cmake_lists_content(exe_name: &str) -> String {
    return format!("cmake_minimum_required(VERSION 3.22.1)
project({exe_name})

add_executable({exe_name} main.cpp)
target_compile_features({exe_name} PRIVATE cxx_std_23)
target_compile_options({exe_name} PRIVATE -Wall -Wextra -Wpedantic)");
}

fn get_cmake_presets_content(preset_name: &str, bin_dir: &str) -> String {
    return format!(r#"{{
    "version": 3,
    "configurePresets": [
        {{
            "name": "{preset_name}",
            "binaryDir": "{bin_dir}",
            "cacheVariables": {{
                "CMAKE_BUILD_TYPE": "Debug"
            }}
        }}
    ],
    "buildPresets": [
        {{
            "name": "{preset_name}",
            "configurePreset": "{preset_name}",
            "configuration": "debug"
        }}
    ]
}}"#);
}

async fn get_source_content(source_file: impl AsRef<std::path::Path>) -> std::io::Result<String> {
    let mut src_path = std::env::current_dir()?;
    src_path.push(source_file);
    return tokio::fs::read_to_string(src_path).await;
}

struct SandboxDir {
    pub path: std::path::PathBuf
}

impl Drop for SandboxDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

impl SandboxDir {
    async fn create() -> std::io::Result<Self> {
        let mut tmp_dir = std::env::temp_dir();
        tmp_dir.push("cmake-tiny-tools");
        tokio::fs::create_dir(&tmp_dir).await?;
        return Ok(Self{ path: tmp_dir });
    }

    async fn add_file(&self, file: impl AsRef<std::path::Path>, content: &[u8]) -> std::io::Result<()> {
        let path = self.path.join(file);
        let mut file = tokio::fs::File::create(path).await?;
        file.write_all(content).await?;
        return Ok(());
    }

    async fn build_and_run(&self, name: &str, bin_dir: &str) -> std::io::Result<()> {
        let mut cmake = tokio::process::Command::new("cmake")
            .current_dir(&self.path).args(["--preset", name])
            .kill_on_drop(true).spawn()?;
        cmake.wait().await?;

        let mut cmake_build = tokio::process::Command::new("cmake")
            .current_dir(&self.path).args(["--build", "--preset", name])
            .kill_on_drop(true).spawn()?;
        cmake_build.wait().await?;

        let mut exe_path = self.path.join(bin_dir);
        exe_path.push(name);
        let mut executable = tokio::process::Command::new(exe_path)
            .kill_on_drop(true).spawn()?;
        executable.wait().await?;
        return Ok(());
    }
}

pub async fn run(source_file: impl AsRef<std::path::Path>) -> std::io::Result<()> {
    let sandbox = SandboxDir::create().await?;
    let name = "tmp";
    let bin_dir = "build";

    let cmake_lists = get_cmake_lists_content(name);
    let cmake_presets = get_cmake_presets_content(name, bin_dir);

    tokio::try_join!(
        sandbox.add_file("CMakeLists.txt", cmake_lists.as_bytes()),
        sandbox.add_file("CMakePresets.json", cmake_presets.as_bytes()),
        async {
            let source = get_source_content(source_file).await?;
            sandbox.add_file("main.cpp", source.as_bytes()).await?;
            let result: std::io::Result<()> = Ok(());
            return result;
        })?;

    sandbox.build_and_run(name, bin_dir).await?;
    return Ok(());
}
