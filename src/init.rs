fn get_gitignore_content() -> &'static str {
    const GITIGNORE: &str = "cmake-build*
build*
.idea
.vs
.vscode
CMakeUserPresets.json";
    return GITIGNORE;
}

fn get_cmake_lists_lib_main_content(project_name: &str) -> String {
    let capitalized_project_name = project_name.to_uppercase();
    return format!(r#"cmake_minimum_required(VERSION 3.22.1)
project({project_name} VERSION 0.1)

include(cmake/CPM.cmake)

set({capitalized_project_name}_OPTIONS "" CACHE STRING "Compiler options for {project_name} targets")

add_subdirectory(src)
add_library({project_name}::{project_name} ALIAS {project_name}_impl)

option({capitalized_project_name}_ENABLE_TESTS "Enable test targets" OFF)
option({capitalized_project_name}_ENABLE_SANDBOX "Enable sandbox target" OFF)

if ({capitalized_project_name}_ENABLE_TESTS AND BUILD_TESTING)
    enable_testing()
    add_subdirectory(tests)
endif ()
if ({capitalized_project_name}_ENABLE_SANDBOX)
    add_subdirectory(sandbox)
endif ()
"#);
}

fn get_cmake_lists_src_content(project_name: &str) -> String {
    let capitalized_project_name = project_name.to_uppercase();
    return format!(r#"
add_library({project_name}_impl lib.cpp)
target_compile_features({project_name}_impl PRIVATE cxx_std_23)
target_compile_options({project_name}_impl PRIVATE ${{{capitalized_project_name}_OPTIONS}})
target_include_directories({project_name}_impl PUBLIC ${{PROJECT_SOURCE_DIR}}/include)
"#);
}

fn get_cmake_lists_sandbox_content(project_name: &str) -> String {
    let capitalized_project_name = project_name.to_uppercase();
    return format!(r#"
add_executable(sandbox main.cpp)
target_compile_features(sandbox PRIVATE cxx_std_23)
target_compile_options(sandbox PRIVATE ${{{capitalized_project_name}_OPTIONS}})
target_link_libraries(sandbox PRIVATE {project_name}::{project_name})
"#);
}

fn get_cmake_lists_tests_content(project_name: &str) -> String {
    return format!(r#"include(CTest)

add_library({project_name}_test_common INTERFACE)
target_compile_features({project_name}_test_common INTERFACE cxx_std_23)
target_link_libraries({project_name}_test_common INTERFACE {project_name}_impl)

add_executable({project_name}_test test.cpp)
target_link_libraries({project_name}_test PRIVATE {project_name}_test_common)
add_test(Test {project_name}_test)
"#);
}

fn get_cmake_presets_content(project_name: &str, bin_dir: &str) -> String {
    let capitalized_project_name = project_name.to_uppercase();
    return format!(r#"{{
    "version": 3,
    "configurePresets": [
        {{
            "name": "dev",
            "hidden": true,
            "cacheVariables": {{
                "BUILD_TESTING": "ON",
                "{capitalized_project_name}_ENABLE_TESTS": "ON",
                "{capitalized_project_name}_ENABLE_SANDBOX": "ON"
            }}
        }},
        {{
            "name": "dev-debug",
            "inherits": "dev",
            "binaryDir": "{bin_dir}-debug",
            "cacheVariables": {{
                "CMAKE_BUILD_TYPE": "Debug"
            }}
        }},
        {{
            "name": "dev-release",
            "inherits": "dev",
            "binaryDir": "{bin_dir}-relwithdebinfo",
            "cacheVariables": {{
                "CMAKE_BUILD_TYPE": "RelWithDebInfo"
            }}
        }}
    ],
    "buildPresets": [
        {{
            "name": "dev-debug",
            "configurePreset": "dev-debug",
            "configuration": "Debug"
        }},
        {{
            "name": "dev-release",
            "configurePreset": "dev-release",
            "configuration": "RelWithDebInfo"
        }}
    ],
    "testPresets": [
        {{
            "name": "common",
            "hidden": true,
            "output": {{"outputOnFailure": true}},
            "execution": {{"noTestsAction": "error", "stopOnFailure": true}}
        }},
        {{
            "name": "dev-debug",
            "inherits": "common",
            "configurePreset": "dev-debug",
            "configuration": "Debug"
        }},
        {{
            "name": "dev-release",
            "inherits": "common",
            "configurePreset": "dev-release",
            "configuration": "RelWithDebInfo"
        }}
    ]
}}"#);
}

fn get_cmake_user_presets_content(bin_dir: &str) -> String {
    return format!(r#"{{
    "version": 3,
    "configurePresets": [
        {{
            "name": "local",
            "inherits": "dev",
            "binaryDir": "{bin_dir}",
            "generator": "Ninja",
            "cacheVariables": {{
                "CMAKE_EXPORT_COMPILE_COMMANDS": "ON",
                "CMAKE_BUILD_TYPE": "Debug",
                "CMAKE_C_COMPILER": "clang",
                "CMAKE_CXX_COMPILER": "clang++",
                "CMAKE_CXX_FLAGS": "-stdlib=libc++"
            }}
        }}
    ],
    "buildPresets": [
        {{
            "name": "local",
            "configurePreset": "local",
            "configuration": "Debug"
        }}
    ],
    "testPresets": [
        {{
            "name": "local",
            "inherits": "common",
            "configurePreset": "local",
            "configuration": "Debug"
        }}
    ]
}}"#);
}

fn get_cpm_content() -> &'static str {
    const CPM: &str = r#"# SPDX-License-Identifier: MIT
#
# SPDX-FileCopyrightText: Copyright (c) 2019-2023 Lars Melchior and contributors

set(CPM_DOWNLOAD_VERSION 0.40.0)
set(CPM_HASH_SUM "7b354f3a5976c4626c876850c93944e52c83ec59a159ae5de5be7983f0e17a2a")

if(CPM_SOURCE_CACHE)
  set(CPM_DOWNLOAD_LOCATION "${CPM_SOURCE_CACHE}/cpm/CPM_${CPM_DOWNLOAD_VERSION}.cmake")
elseif(DEFINED ENV{CPM_SOURCE_CACHE})
  set(CPM_DOWNLOAD_LOCATION "$ENV{CPM_SOURCE_CACHE}/cpm/CPM_${CPM_DOWNLOAD_VERSION}.cmake")
else()
  set(CPM_DOWNLOAD_LOCATION "${CMAKE_BINARY_DIR}/cmake/CPM_${CPM_DOWNLOAD_VERSION}.cmake")
endif()

# Expand relative path. This is important if the provided path contains a tilde (~)
get_filename_component(CPM_DOWNLOAD_LOCATION ${CPM_DOWNLOAD_LOCATION} ABSOLUTE)

file(DOWNLOAD
     https://github.com/cpm-cmake/CPM.cmake/releases/download/v${CPM_DOWNLOAD_VERSION}/CPM.cmake
     ${CPM_DOWNLOAD_LOCATION} EXPECTED_HASH SHA256=${CPM_HASH_SUM}
)

include(${CPM_DOWNLOAD_LOCATION})
"#;
    return CPM;
}

fn get_executable_main_content() -> &'static str {
    const MAIN_CPP: &str = "
auto main() -> int {
    return 0;
}
";
    return MAIN_CPP;
}

fn get_lib_content() -> &'static str {
    const LIB_CPP: &str = "
namespace NHidden {

void Hidden() {}

}
";
    return LIB_CPP;
}

use tokio::io::AsyncWriteExt;

struct ProjectDir {
    path: std::path::PathBuf
}

impl ProjectDir {
    fn new() -> std::io::Result<Self> {
        let project_dir = std::env::current_dir()?;
        return Ok(Self{ path: project_dir });
    }

    fn project_name(&self) -> std::io::Result<&str> {
        if let Some(project_name) = self.path.file_name().and_then(|s| s.to_str()) {
            return Ok(project_name);
        } else {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        };
    }

    async fn add_file(&self, file: impl AsRef<std::path::Path>, content: &[u8]) -> std::io::Result<()> {
        let path = self.path.join(file);
        let mut file = tokio::fs::File::create(path).await?;
        file.write_all(content).await?;
        return Ok(());
    }

    async fn add_dir(&self, dir: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        let path = self.path.join(dir);
        tokio::fs::create_dir_all(path).await?;
        return Ok(());
    }

    async fn init_git(&self) -> std::io::Result<()> {
        let mut git_init = tokio::process::Command::new("git")
            .current_dir(&self.path).args(["init", "-q"])
            .kill_on_drop(true).spawn()?;
        git_init.wait().await?;
        return Ok(());
    }

}

fn subdir_cmake_lists(dir: impl AsRef<std::path::Path>) -> std::path::PathBuf {
    return dir.as_ref().join("CMakeLists.txt");
}

pub enum ProjectType {
    Library,
    Executable
}

pub async fn init(_project_type: ProjectType) -> std::io::Result<()> {
    let project_dir = ProjectDir::new()?;
    let project_name = project_dir.project_name()?;
    let bin_dir = "build";

    let gitignore = get_gitignore_content();
    let cmake_lists = get_cmake_lists_lib_main_content(project_name);
    let cmake_presets = get_cmake_presets_content(project_name, bin_dir);
    let cmake_user_presets = get_cmake_user_presets_content(bin_dir);
    let executable_main = get_executable_main_content();

    tokio::try_join!(
        project_dir.add_file(".gitignore", gitignore.as_bytes()),
        project_dir.add_file("CMakeLists.txt", cmake_lists.as_bytes()),
        project_dir.add_file("CMakePresets.json", cmake_presets.as_bytes()),
        project_dir.add_file("CMakeUserPresets.json", cmake_user_presets.as_bytes()),
        project_dir.add_dir(std::path::Path::new("include").join(project_name)),
        async {
            project_dir.add_dir("cmake").await?;
            let cpm = get_cpm_content();
            project_dir.add_file(std::path::Path::new("cmake").join("CPM.cmake"), cpm.as_bytes()).await?;
            let result: std::io::Result<()> = Ok(());
            return result;
        },
        async {
            project_dir.add_dir("sandbox").await?;
            let sandbox_cmake = get_cmake_lists_sandbox_content(project_name);
            tokio::try_join!(
                project_dir.add_file(std::path::Path::new("sandbox").join("main.cpp"), executable_main.as_bytes()),
                project_dir.add_file(subdir_cmake_lists("sandbox"), sandbox_cmake.as_bytes())
            )?;
            let result: std::io::Result<()> = Ok(());
            return result;
        },
        async {
            project_dir.add_dir("src").await?;
            let src_cmake = get_cmake_lists_src_content(project_name);
            tokio::try_join!(
                project_dir.add_file(std::path::Path::new("src").join("lib.cpp"), get_lib_content().as_bytes()),
                project_dir.add_file(subdir_cmake_lists("src"), src_cmake.as_bytes())
            )?;
            let result: std::io::Result<()> = Ok(());
            return result;
        },
        async {
            project_dir.add_dir("tests").await?;
            let tests_cmake = get_cmake_lists_tests_content(project_name);
            tokio::try_join!(
                project_dir.add_file(std::path::Path::new("tests").join("test.cpp"), executable_main.as_bytes()),
                project_dir.add_file(subdir_cmake_lists("tests"), tests_cmake.as_bytes())
            )?;
            let result: std::io::Result<()> = Ok(());
            return result;
        }
    )?;
    project_dir.init_git().await?;
    return Ok(());
}
