use chrono::{Local, NaiveDate};
use percent_encoding::percent_decode_str;
use regex::Regex;
use serde_json::Value;
use std::{
    collections::BTreeMap,
    fs,
    io::{self, Write},
    path::{Component, Path, PathBuf},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const STARTER: &str = "Draft starter: replace before review.";
const TEMPLATES: [(&str, &str); 3] = [
    ("brief", include_str!("../templates/brief.md")),
    ("sources", include_str!("../templates/sources.md")),
    ("review", include_str!("../templates/review.md")),
];

fn regex(pattern: &str) -> Regex {
    Regex::new(pattern).expect("static regex")
}

pub fn create(root: &Path, slug: &str, title: &str, language: &str) -> Result<PathBuf> {
    if !regex(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").is_match(slug) {
        return Err("Slug must contain lowercase ASCII words separated by hyphens".into());
    }
    if title.trim().is_empty() || language.trim().is_empty() || title.contains(['\n', '\r']) {
        return Err("Provide a nonempty single-line title and a language".into());
    }
    fs::create_dir_all(root)?;
    let target = root.canonicalize()?.join(slug);
    fs::create_dir(&target)?;
    for folder in [
        "assets/images",
        "assets/videos",
        "assets/gifs",
        "examples",
        "editorial",
    ] {
        fs::create_dir_all(target.join(folder))?;
    }
    let today = Local::now().date_naive().to_string();
    let fields = [
        ("title", Value::from(title)),
        (
            "description",
            Value::from("Draft article; description pending."),
        ),
        ("tags", Value::Array(vec![])),
        ("language", Value::from(language)),
        ("created", Value::from(today.clone())),
        ("updated", Value::from(today)),
        ("status", Value::from("draft")),
    ];
    let front = fields
        .iter()
        .map(|(k, v)| format!("{k}: {v}"))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(
        target.join("index.md"),
        format!("---\n{front}\n---\n\n# {title}\n\n{STARTER}\n"),
    )?;
    for (name, content) in TEMPLATES {
        fs::write(target.join(format!("editorial/{name}.md")), content)?;
    }
    Ok(target)
}

fn prose_only(body: &str) -> (String, bool) {
    let fence_pattern = regex(r"^ {0,3}(`{3,}|~{3,})(.*)$");
    let mut fence: Option<String> = None;
    let mut lines = vec![];
    for line in body.lines() {
        let matched = fence_pattern.captures(line);
        if let Some(open) = &fence {
            if let Some(c) = matched
                && c[1].starts_with(&open[..1])
                && c[1].len() >= open.len()
                && c[2].trim().is_empty()
            {
                fence = None;
            }
            continue;
        }
        if let Some(c) = matched {
            fence = Some(c[1].to_owned());
        } else {
            lines.push(line);
        }
    }
    // Match code-span delimiter lengths without regex backreferences.
    let text = lines.join("\n");
    let mut clean = String::new();
    let mut rest = text.as_str();
    while let Some(start) = rest.find('`') {
        clean.push_str(&rest[..start]);
        rest = &rest[start..];
        let width = rest.bytes().take_while(|b| *b == b'`').count();
        let delimiter = &rest[..width];
        if let Some(end) = rest[width..].find(delimiter) {
            rest = &rest[width + end + width..];
        } else {
            clean.push_str(rest);
            rest = "";
        }
    }
    clean.push_str(rest);
    (clean, fence.is_some())
}

// Walk without following directory symlinks; check every link before reading Markdown.
fn inventory(
    dir: &Path,
    root: &Path,
    files: &mut Vec<PathBuf>,
    errors: &mut Vec<String>,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let kind = entry.file_type()?;
        if kind.is_symlink() {
            match path.canonicalize() {
                Ok(resolved) if resolved.starts_with(root) => {
                    if resolved.is_file() {
                        files.push(path);
                    }
                }
                _ => errors.push(format!(
                    "Symlink escapes article folder or is broken: {}",
                    path.display()
                )),
            }
        } else if kind.is_dir() {
            inventory(&path, root, files, errors)?;
        } else {
            files.push(path);
        }
    }
    Ok(())
}

fn normalize(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for part in path.components() {
        match part {
            Component::ParentDir => {
                result.pop();
            }
            Component::CurDir => {}
            _ => result.push(part.as_os_str()),
        }
    }
    result
}

fn local_links(text: &str, file: &Path, root: &Path) -> Vec<String> {
    let mut errors = vec![];
    let mut urls = vec![];
    let key = |s: &str| {
        s.split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase()
    };
    let mut refs = BTreeMap::new();
    for c in regex(r"(?m)^ {0,3}\[([^\]]+)\]:\s*(<[^>]+>|\S+)").captures_iter(text) {
        let url = c[2].trim_matches(['<', '>']).to_owned();
        refs.insert(key(&c[1]), url.clone());
        urls.push(url);
    }
    for c in regex(r"(!?)\[([^\]]*)\]\(\s*(<[^>]*>|(?:[^\s()]|\([^()]*\))*)[^\n)]*\)")
        .captures_iter(text)
    {
        urls.push(c[3].trim_matches(['<', '>']).to_owned());
        if !c[1].is_empty() && c[2].trim().is_empty() {
            errors.push("image needs alt text".to_owned());
        }
    }
    for c in regex(r"(!?)\[([^\]]*)\]\[([^\]]*)\]").captures_iter(text) {
        let reference = key(if c[3].is_empty() { &c[2] } else { &c[3] });
        if !refs.contains_key(&reference) {
            errors.push(format!("undefined reference [{reference}]"));
        }
        if !c[1].is_empty() && c[2].trim().is_empty() {
            errors.push("image needs alt text".to_owned());
        }
    }
    for c in regex(r#"(?i)\b(?:src|href|poster)\s*=\s*["']([^"']+)["']"#).captures_iter(text) {
        urls.push(c[1].to_owned());
    }
    let scheme = regex(r"^([a-zA-Z][a-zA-Z0-9+.-]*):");
    for url in urls {
        if url.starts_with("//") {
            continue;
        }
        if let Some(c) = scheme.captures(&url) {
            if !["http", "https", "mailto", "tel"].contains(&c[1].to_lowercase().as_str()) {
                errors.push(format!("unsupported link scheme: {url}"));
            }
            continue;
        }
        let raw = url.split(['?', '#']).next().unwrap_or_default();
        let Ok(decoded) = percent_decode_str(raw).decode_utf8() else {
            errors.push(format!("invalid URL: {url}"));
            continue;
        };
        if decoded.is_empty() {
            continue;
        }
        if Path::new(decoded.as_ref()).is_absolute() || decoded.contains('\\') {
            errors.push(format!("use a relative portable path: {url}"));
            continue;
        }
        let path = normalize(
            &file
                .parent()
                .expect("article file parent")
                .join(decoded.as_ref()),
        );
        if !path.starts_with(root) {
            errors.push(format!("link escapes article folder: {url}"));
        } else {
            match path.canonicalize() {
                Ok(resolved) if !resolved.starts_with(root) => {
                    errors.push(format!("link escapes article folder: {url}"))
                }
                Err(_) => errors.push(format!("missing local resource: {url}")),
                _ => {}
            }
        }
    }
    errors
        .into_iter()
        .map(|e| format!("{}: {e}", file.strip_prefix(root).unwrap_or(file).display()))
        .collect()
}

pub fn validate(folder: &Path) -> Result<Vec<String>> {
    if !folder.join("index.md").is_file() {
        return Ok(vec!["Missing index.md".into()]);
    }
    let root = folder.canonicalize()?;
    let index = root.join("index.md");
    let mut errors = vec![];
    let mut files = vec![];
    inventory(&root, &root, &mut files, &mut errors)?;
    if !errors.is_empty() {
        return Ok(errors);
    }
    let source = fs::read_to_string(&index)?.replace("\r\n", "\n");
    let fm = regex(r"(?s)\A---\n(.*?)\n---(?:\n|$)(.*)\z");
    let captures = fm.captures(&source);
    let body = if let Some(c) = &captures {
        let mut metadata = BTreeMap::<String, Value>::new();
        for line in c[1].trim().lines() {
            if let Some((key, value)) = line.split_once(':') {
                if metadata.contains_key(key) {
                    errors.push(format!("Duplicate frontmatter key: {key}"));
                }
                match serde_json::from_str(value) {
                    Ok(value) => {
                        metadata.insert(key.to_owned(), value);
                    }
                    Err(_) => errors.push(format!("{key}: expected a JSON-compatible YAML value")),
                }
            } else {
                errors.push(format!("Invalid frontmatter line: {line}"));
            }
        }
        for key in [
            "title",
            "description",
            "tags",
            "language",
            "created",
            "updated",
            "status",
        ] {
            if !metadata.contains_key(key) {
                errors.push(format!("Missing frontmatter key: {key}"));
            }
        }
        let string = |key: &str| {
            metadata
                .get(key)
                .and_then(Value::as_str)
                .unwrap_or_default()
        };
        for key in ["title", "description", "language"] {
            if string(key).trim().is_empty() {
                errors.push(format!("{key}: expected nonempty string"));
            }
        }
        if !metadata
            .get("tags")
            .and_then(Value::as_array)
            .is_some_and(|tags| {
                tags.iter()
                    .all(|v| v.as_str().is_some_and(|s| !s.trim().is_empty()))
            })
        {
            errors.push("tags: expected an array of nonempty strings".into());
        }
        if !["draft", "needs-revision", "reviewed"].contains(&string("status")) {
            errors.push("status: expected draft, needs-revision, or reviewed".into());
        }
        let mut dates = BTreeMap::new();
        for key in ["created", "updated"] {
            match NaiveDate::parse_from_str(string(key), "%Y-%m-%d") {
                Ok(date) if date.to_string() == string(key) => {
                    dates.insert(key, date);
                }
                _ => errors.push(format!("{key}: expected a quoted ISO date")),
            }
        }
        if dates.len() == 2 && dates["updated"] < dates["created"] {
            errors.push("updated date precedes created date".into());
        }
        c.get(2).unwrap().as_str()
    } else {
        errors.push("Missing or malformed frontmatter".into());
        &source
    };
    for (name, _) in TEMPLATES {
        if !root.join(format!("editorial/{name}.md")).is_file() {
            errors.push(format!("Missing editorial/{name}.md"));
        }
    }
    let (prose, unclosed) = prose_only(body);
    if unclosed {
        errors.push("index.md: unclosed code fence".into());
    }
    if regex(r"(?m)^#\s+\S").find_iter(&prose).count() != 1 {
        errors.push("index.md: expected exactly one H1 title".into());
    }
    if body.contains(STARTER) {
        errors.push("index.md: unfinished draft starter".into());
    }
    for file in files
        .iter()
        .filter(|p| p.extension().is_some_and(|ext| ext == "md"))
    {
        let text = if *file == index {
            body.to_owned()
        } else {
            fs::read_to_string(file)?
        };
        let (clean, _) = prose_only(&text);
        errors.extend(local_links(&clean, file, &root));
    }
    Ok(errors)
}

pub fn agent_definitions() -> Vec<(String, String)> {
    let standards = include_str!("../references/technical-standards.md");
    [
        ("idea-generator", "blogger_idea_generator", include_str!("../agents/idea-generator.md")),
        ("writer", "blogger_writer", include_str!("../agents/writer.md")),
        ("reviewer", "blogger_reviewer", include_str!("../agents/reviewer.md")),
    ].into_iter().map(|(role, name, prompt)| {
        let instructions = format!("{prompt}\n\n{standards}\n\nFollow the parent command workflow and return evidence and changed paths. The parent owns report persistence.\n");
        let mut content = format!("name = {}\ndescription = {}\ndeveloper_instructions = {}\n", Value::from(name), Value::from(format!("Technical publishing {role} specialist.")), Value::from(instructions));
        if role == "reviewer" { content.push_str("sandbox_mode = \"read-only\"\n"); }
        (format!("{name}.toml"), content)
    }).collect()
}

pub fn setup_agents(project: &Path) -> Result<PathBuf> {
    let target = project.canonicalize()?.join(".codex/agents");
    fs::create_dir_all(&target)?;
    let definitions = agent_definitions();
    for (name, content) in &definitions {
        let path = target.join(name);
        if path.is_symlink() || (path.exists() && fs::read_to_string(&path)? != *content) {
            return Err(format!("Refusing to overwrite existing agent: {}", path.display()).into());
        }
    }
    for (name, content) in definitions {
        let path = target.join(name);
        if !path.exists() {
            fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path)?
                .write_all(content.as_bytes())?;
        }
    }
    Ok(target)
}
