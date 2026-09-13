use codex_blogger::{create, setup_agents, validate};
use std::{fs, path::PathBuf, process::Command};
use tempfile::TempDir;

struct Fixture {
    _temp: TempDir,
    root: PathBuf,
    article: PathBuf,
}
impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().to_path_buf();
        let article = create(&root, "sample", "A title --- with punctuation", "en").unwrap();
        let this = Self {
            _temp: temp,
            root,
            article,
        };
        this.replace(
            "Draft starter: replace before review.",
            "A complete example.",
        );
        this
    }
    fn replace(&self, from: &str, to: &str) {
        let path = self.article.join("index.md");
        fs::write(&path, fs::read_to_string(&path).unwrap().replace(from, to)).unwrap();
    }
    fn append(&self, text: &str) {
        let path = self.article.join("index.md");
        fs::write(
            &path,
            format!("{}\n{text}\n", fs::read_to_string(&path).unwrap()),
        )
        .unwrap();
    }
    fn errors(&self) -> Vec<String> {
        validate(&self.article).unwrap()
    }
}

#[test]
fn valid_and_read_only() {
    let f = Fixture::new();
    let before = fs::read(f.article.join("index.md")).unwrap();
    assert_eq!(f.errors(), Vec::<String>::new());
    assert_eq!(before, fs::read(f.article.join("index.md")).unwrap());
}
#[test]
fn draft_is_not_complete() {
    let f = Fixture::new();
    let draft = create(&f.root, "draft", "Draft", "en").unwrap();
    assert!(
        validate(&draft)
            .unwrap()
            .iter()
            .any(|e| e.contains("unfinished"))
    );
}
#[test]
fn collision_preserves_article() {
    let f = Fixture::new();
    let before = fs::read(f.article.join("index.md")).unwrap();
    assert!(create(&f.root, "sample", "Replacement", "en").is_err());
    assert_eq!(before, fs::read(f.article.join("index.md")).unwrap());
}
#[test]
fn bad_slug_and_title() {
    let f = Fixture::new();
    for slug in ["../escape", "/tmp/escape", "a/b", "", "UPPER"] {
        assert!(create(&f.root, slug, "Title", "en").is_err());
    }
    assert!(create(&f.root, "valid", "bad\ntitle", "en").is_err());
}
#[test]
fn missing_resource_and_empty_alt() {
    let f = Fixture::new();
    f.append("![](assets/images/missing.png)");
    assert!(
        f.errors()
            .iter()
            .any(|e| e.contains("missing local resource"))
    );
    assert!(f.errors().iter().any(|e| e.contains("alt text")));
}
#[test]
fn local_media_and_references() {
    let f = Fixture::new();
    fs::write(f.article.join("assets/images/diagram (1).svg"), "<svg/>").unwrap();
    f.append("![Diagram](<assets/images/diagram (1).svg>)\n[Diagram][d]\n[d]: <assets/images/diagram (1).svg>");
    assert!(f.errors().is_empty(), "{:?}", f.errors());
}
#[test]
fn encoded_escape_and_absolute_path() {
    let f = Fixture::new();
    fs::write(f.root.join("outside.txt"), "outside").unwrap();
    f.append("[escape](%2e%2e/outside.txt)\n[absolute](/tmp/outside.txt)");
    assert!(f.errors().iter().any(|e| e.contains("escapes")));
    assert!(f.errors().iter().any(|e| e.contains("relative portable")));
}
#[cfg(unix)]
#[test]
fn escaping_symlink_is_rejected_before_reading() {
    let f = Fixture::new();
    std::os::unix::fs::symlink(&f.root, f.article.join("assets/escape")).unwrap();
    assert!(f.errors().iter().any(|e| e.contains("Symlink escapes")));
}
#[test]
fn code_links_ignored_and_fence_checked() {
    let f = Fixture::new();
    f.append("```md\n[example](missing.txt)\n```\n`[sample](missing.txt)`");
    assert!(f.errors().is_empty());
    f.append("```rust\nfn main() {}");
    assert!(f.errors().iter().any(|e| e.contains("unclosed")));
}
#[test]
fn metadata_validation() {
    let f = Fixture::new();
    f.replace("status: \"draft\"", "status: \"published\"");
    f.replace("tags: []", "tags: \"go\"");
    assert!(f.errors().iter().any(|e| e.contains("status:")));
    assert!(f.errors().iter().any(|e| e.contains("tags:")));
}
#[test]
fn malformed_dates_and_duplicates() {
    let f = Fixture::new();
    f.replace("tags: []", "tags: []\ntags: []");
    let original = fs::read_to_string(f.article.join("index.md")).unwrap();
    let updated = original
        .lines()
        .find(|l| l.starts_with("updated:"))
        .unwrap();
    f.replace(updated, "updated: \"2026-02-30\"");
    assert!(f.errors().iter().any(|e| e.contains("Duplicate")));
    assert!(f.errors().iter().any(|e| e.contains("ISO date")));
}
#[test]
fn external_and_html_links() {
    let f = Fixture::new();
    f.append(
        "[Source](https://example.com/docs)\n<video src=\"assets/videos/missing.mp4\"></video>",
    );
    assert_eq!(f.errors().len(), 1);
    assert!(f.errors()[0].contains("missing.mp4"));
}
#[test]
fn undefined_reference() {
    let f = Fixture::new();
    f.append("[Source][missing]");
    assert!(f.errors().iter().any(|e| e.contains("undefined reference")));
}
#[test]
fn agent_setup_is_idempotent_and_preserves_changes() {
    let dir = tempfile::tempdir().unwrap();
    let target = setup_agents(dir.path()).unwrap();
    assert_eq!(fs::read_dir(&target).unwrap().count(), 3);
    assert_eq!(setup_agents(dir.path()).unwrap(), target);
    let file = target.join("blogger_writer.toml");
    let content = fs::read_to_string(&file).unwrap();
    // These generated TOML basic strings are also JSON strings.
    for line in content.lines() {
        let (_, value) = line.split_once(" = ").unwrap();
        let _: String = serde_json::from_str(value).unwrap();
    }
    fs::write(&file, "custom = true\n").unwrap();
    assert!(setup_agents(dir.path()).is_err());
    assert_eq!(fs::read_to_string(file).unwrap(), "custom = true\n");
}
#[test]
fn embedded_templates_work_outside_repository() {
    let temp = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_codex-blogger"))
        .current_dir(temp.path())
        .args([
            "create",
            "--root",
            "articles",
            "--slug",
            "rust-demo",
            "--title",
            "Rust CLI",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        temp.path()
            .join("articles/rust-demo/editorial/review.md")
            .is_file()
    );
    let check = Command::new(env!("CARGO_BIN_EXE_codex-blogger"))
        .current_dir(temp.path())
        .args(["validate", "articles/rust-demo"])
        .output()
        .unwrap();
    assert_eq!(check.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&check.stderr).contains("unfinished"));
}
#[test]
fn cli_validates_and_reports_argument_errors() {
    let f = Fixture::new();
    let cli = env!("CARGO_BIN_EXE_codex-blogger");
    assert!(
        Command::new(cli)
            .arg("validate")
            .arg(&f.article)
            .output()
            .unwrap()
            .status
            .success()
    );
    assert_eq!(
        Command::new(cli)
            .args(["create", "--unknown"])
            .output()
            .unwrap()
            .status
            .code(),
        Some(2)
    );
}
