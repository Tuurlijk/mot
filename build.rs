use progenitor::GenerationSettings;
use progenitor::InterfaceStyle;

fn main() {
    generate_moneybird_api_library();
    println!("cargo:rustc-env=GIT_INFO={}", get_version());
}

fn get_version() -> String {
    let git_output = std::process::Command::new("git")
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

fn generate_moneybird_api_library() {
    let src = "moneybird-openapi.snipped.yaml";
    println!("cargo:rerun-if-changed={}", src);
    let file = std::fs::File::open(src).unwrap();
    let spec = serde_yaml_ng::from_reader(file).unwrap();

    // Create GenerationSettings with Builder style
    let mut binding = GenerationSettings::default();
    let settings = binding.with_interface(InterfaceStyle::Builder);

    // Pass the settings to the Generator
    let mut generator = progenitor::Generator::new(settings);

    let tokens = generator.generate_tokens(&spec).unwrap();
    let ast = syn::parse2(tokens).unwrap();
    let content = prettyplease::unparse(&ast);

    let mut out_file = std::path::Path::new("src").to_path_buf();
    out_file.push("moneybird.rs");

    std::fs::write(out_file, content).unwrap();
}
