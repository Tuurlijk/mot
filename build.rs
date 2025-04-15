use progenitor::GenerationSettings;
use progenitor::InterfaceStyle;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // Set up cargo rerun-if-changed conditions
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=FORCE_MONEYBIRD_GENERATION");

    // Always set the GIT_INFO environment variable
    println!("cargo:rustc-env=GIT_INFO={}", get_version());

    // Check if we should (re)generate the MoneyBird API
    if should_generate_moneybird_api() {
        match generate_moneybird_api_library() {
            Ok(_) => println!("Successfully generated MoneyBird API library"),
            Err(e) => eprintln!("Failed to generate MoneyBird API library: {}", e),
        }
    }
}

/// Determines if the MoneyBird API should be regenerated
fn should_generate_moneybird_api() -> bool {
    let src_yaml = "moneybird-openapi.snipped.yaml";
    let target_rs = Path::new("src").join("moneybird.rs");

    // Report dependency on the source file for cargo
    println!("cargo:rerun-if-changed={}", src_yaml);

    // Force generation if environment variable is set
    if env::var("FORCE_MONEYBIRD_GENERATION").is_ok() {
        return true;
    }

    // Check if source YAML file exists
    if !Path::new(src_yaml).exists() {
        return false;
    }

    // Generate if target file doesn't exist (this will ensure the file gets generated if missing)
    if !target_rs.exists() {
        println!(
            "Target file {} does not exist, triggering generation",
            target_rs.display()
        );
        return true;
    }

    // Compare modification times
    let src_modified = fs::metadata(src_yaml).ok().and_then(|m| m.modified().ok());
    let target_modified = fs::metadata(&target_rs)
        .ok()
        .and_then(|m| m.modified().ok());

    match (src_modified, target_modified) {
        (Some(src_time), Some(target_time)) => src_time > target_time,
        _ => true, // If we can't compare modification times, regenerate to be safe
    }
}

fn get_version() -> String {
    let git_output = Command::new("git")
        .args(["describe", "--always", "--tags", "--long", "--dirty"])
        .output()
        .ok();
    let git_info = git_output
        .as_ref()
        .and_then(|output| std::str::from_utf8(&output.stdout).ok().map(str::trim));
    let cargo_pkg_version = env!("CARGO_PKG_VERSION");

    // Default git_describe to cargo_pkg_version
    let mut git_describe = String::from(cargo_pkg_version);

    if let Some(git_info) = git_info {
        if git_info.contains(cargo_pkg_version) {
            // Remove the 'g' only if followed by at least 7 hexadecimal characters
            let git_info = regex::Regex::new(r"g([0-9a-f]{7,})")
                .unwrap()
                .replace(git_info, |caps: &regex::Captures| {
                    caps.get(1).unwrap().as_str().to_string()
                });
            git_describe = git_info.to_string();
        } else {
            git_describe = format!("v{}-{}", cargo_pkg_version, git_info);
        }
    }
    if git_describe.ends_with('-') {
        git_describe.pop();
    }
    git_describe
}

fn generate_moneybird_api_library() -> Result<(), String> {
    let src = "moneybird-openapi.snipped.yaml";
    let out_file_path = PathBuf::from("src").join("moneybird.rs");

    // Open and parse the OpenAPI spec
    let file = fs::File::open(src).map_err(|e| format!("Failed to open OpenAPI spec: {}", e))?;

    let spec = serde_yaml_ng::from_reader(file)
        .map_err(|e| format!("Failed to parse OpenAPI spec: {}", e))?;

    // Create GenerationSettings with Builder style
    let mut binding = GenerationSettings::default();
    let settings = binding.with_interface(InterfaceStyle::Builder);

    // Generate the API code
    let mut generator = progenitor::Generator::new(settings);
    let tokens = generator
        .generate_tokens(&spec)
        .map_err(|e| format!("Failed to generate tokens: {}", e))?;

    let ast =
        syn::parse2(tokens).map_err(|e| format!("Failed to parse generated tokens: {}", e))?;

    let content = prettyplease::unparse(&ast);

    // Write the generated code to the output file
    fs::write(&out_file_path, &content)
        .map_err(|e| format!("Failed to write output file: {}", e))?;

    // Format the generated code with rustfmt
    format_generated_file(&out_file_path)?;

    Ok(())
}

fn format_generated_file(file_path: &Path) -> Result<(), String> {
    println!("Formatting generated file: {}", file_path.display());

    // Try cargo fmt first, which will use the project's rustfmt configuration
    let cargo_fmt_output = Command::new("cargo")
        .args(["fmt", "--", file_path.to_str().unwrap_or("")])
        .output();

    match cargo_fmt_output {
        Ok(output) if output.status.success() => {
            println!("Successfully formatted using cargo fmt");
            return Ok(());
        }
        Ok(output) => {
            println!("cargo fmt failed with status: {}", output.status);
            // Continue to fallback method if cargo fmt fails
        }
        Err(e) => {
            println!(
                "cargo fmt command failed: {}, falling back to direct rustfmt",
                e
            );
            // Continue to fallback method
        }
    }

    // Fallback: Try direct rustfmt if cargo fmt failed
    println!("Trying direct rustfmt as a fallback...");
    if Command::new("rustfmt").arg("--version").output().is_err() {
        return Err("rustfmt not found. Install with 'rustup component add rustfmt'".to_string());
    }

    let output = Command::new("rustfmt")
        .arg(file_path)
        .output()
        .map_err(|e| format!("Failed to execute rustfmt: {}", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("rustfmt failed: {}", error));
    }

    println!("Successfully formatted using direct rustfmt");
    Ok(())
}
