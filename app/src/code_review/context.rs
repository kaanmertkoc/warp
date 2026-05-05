use crate::ai::agent::DiffSetHunk;
use crate::code_review::diff_state::{DiffLineType, FileDiff, GitFileStatus};
use std::collections::HashMap;
use warp_editor::render::model::LineCount;

cfg_if::cfg_if! {
    if #[cfg(feature = "local_fs")] {
        use std::path::Path;
        use crate::ai::agent::{AIAgentAttachment, CurrentHead, DiffBase};
        use crate::ai::blocklist::BlocklistAIContextModel;
        use crate::code_review::{diff_state::DiffMode, DiffSetScope};
        use warpui::{AppContext, ModelHandle};
    }
}

/// Converts file diffs into a HashMap of file paths to DiffSetHunks
/// If repo_path is provided, file paths will be relative to the repo root
pub fn convert_file_diffs_to_diffset_hunks<'a, I>(files: I) -> HashMap<String, Vec<DiffSetHunk>>
where
    I: Iterator<Item = &'a FileDiff>,
{
    let mut file_diffs: HashMap<String, Vec<DiffSetHunk>> = HashMap::new();

    for file_diff in files {
        let file_path = file_diff.file_path.display().to_string();

        let mut file_hunks = Vec::new();
        for hunk in file_diff.hunks.iter() {
            // Format the diff content for this hunk
            let mut diff_lines = Vec::new();
            let mut lines_added = 0;
            let mut lines_removed = 0;
            for line in &hunk.lines {
                let prefix = match line.line_type {
                    DiffLineType::Add => {
                        lines_added += 1;
                        "+"
                    }
                    DiffLineType::Delete => {
                        lines_removed += 1;
                        "-"
                    }
                    DiffLineType::Context => "",
                    DiffLineType::HunkHeader => continue,
                };
                diff_lines.push(format!("{}{}", prefix, line.text));
            }
            let diff_content = diff_lines.join("\n");

            // Create line range using LineCount: Note that git lines are 1-based and LineCount is 0-based
            let line_range = LineCount::from(hunk.new_start_line.saturating_sub(1))
                ..LineCount::from(hunk.new_start_line.saturating_sub(1) + hunk.new_line_count);

            file_hunks.push(DiffSetHunk {
                line_range,
                diff_content,
                lines_added,
                lines_removed,
            });
        }

        if !file_hunks.is_empty() {
            file_diffs.insert(file_path, file_hunks);
        }
    }

    file_diffs
}

/// Formats parsed file diffs back into unified diff text suitable for copying
/// into external agents or tools.
pub fn format_file_diffs_as_unified_diff<'a, I>(files: I) -> String
where
    I: Iterator<Item = &'a FileDiff>,
{
    let mut output = String::new();

    for file_diff in files {
        if !output.is_empty() {
            output.push('\n');
        }

        let new_path = file_diff.file_path.display().to_string();
        let old_path = match &file_diff.status {
            GitFileStatus::Renamed { old_path } => old_path.as_str(),
            _ => new_path.as_str(),
        };

        output.push_str(&format!("diff --git a/{old_path} b/{new_path}\n"));
        match &file_diff.status {
            GitFileStatus::New | GitFileStatus::Untracked => {
                output.push_str("new file mode 100644\n");
            }
            GitFileStatus::Deleted => {
                output.push_str("deleted file mode 100644\n");
            }
            GitFileStatus::Renamed { old_path } => {
                output.push_str(&format!("rename from {old_path}\n"));
                output.push_str(&format!("rename to {new_path}\n"));
            }
            _ => {}
        }

        let old_header_path = match &file_diff.status {
            GitFileStatus::New | GitFileStatus::Untracked => "/dev/null".to_string(),
            _ => format!("a/{old_path}"),
        };
        let new_header_path = match &file_diff.status {
            GitFileStatus::Deleted => "/dev/null".to_string(),
            _ => format!("b/{new_path}"),
        };

        output.push_str(&format!("--- {old_header_path}\n"));
        output.push_str(&format!("+++ {new_header_path}\n"));

        if file_diff.is_binary {
            output.push_str(&format!(
                "Binary files {old_header_path} and {new_header_path} differ\n"
            ));
            continue;
        }

        for hunk in file_diff.hunks.iter() {
            output.push_str(&format!(
                "@@ -{},{} +{},{} @@\n",
                hunk.old_start_line, hunk.old_line_count, hunk.new_start_line, hunk.new_line_count
            ));

            for line in &hunk.lines {
                let prefix = match line.line_type {
                    DiffLineType::Add => '+',
                    DiffLineType::Delete => '-',
                    DiffLineType::Context => ' ',
                    DiffLineType::HunkHeader => continue,
                };
                output.push(prefix);
                output.push_str(&line.text);
                output.push('\n');
                if line.no_trailing_newline {
                    output.push_str("\\ No newline at end of file\n");
                }
            }
        }
    }

    output.trim_end().to_string()
}

/// Creates attachment reference and key for a set of changes based on scope and diff mode
#[cfg(feature = "local_fs")]
pub fn create_attachment_reference_and_key(
    scope: &DiffSetScope,
    diff_mode: &DiffMode,
    main_branch_name: Option<&str>,
    repo_path: &Path,
) -> (String, String) {
    match scope {
        DiffSetScope::All => {
            let diff_set_description = match diff_mode {
                DiffMode::Head => "uncommitted changes".to_string(),
                DiffMode::MainBranch => {
                    let main_branch = main_branch_name.unwrap_or("main");
                    format!("diffset against {main_branch}")
                }
                DiffMode::OtherBranch(branch_name) => {
                    format!("diffset against {branch_name}")
                }
            };
            let key = diff_set_description.clone();
            (format!("<change:{key}>"), key)
        }
        DiffSetScope::File(file_path) => {
            let relative_path = if file_path.is_absolute() {
                file_path
                    .strip_prefix(repo_path)
                    .unwrap_or(file_path)
                    .to_path_buf()
            } else {
                file_path.clone()
            };
            let key = relative_path.display().to_string();
            (format!("<change:{key}>"), key)
        }
    }
}

/// Registers a DiffSet attachment with the AI controller
/// This encapsulates the common logic for creating and registering diff attachments
#[cfg(feature = "local_fs")]
pub fn register_diffset_attachment(
    ai_context_model: &ModelHandle<BlocklistAIContextModel>,
    attachment_key: String,
    file_diffs: HashMap<String, Vec<DiffSetHunk>>,
    current: Option<CurrentHead>,
    base: DiffBase,
    ctx: &mut AppContext,
) {
    // Create the DiffSet attachment
    let attachment = AIAgentAttachment::DiffSet {
        file_diffs,
        current,
        base,
    };

    // Register the attachment with the AI controller
    ai_context_model.update(ctx, |context_model, _| {
        context_model.register_diff_hunk_attachment(attachment_key, attachment);
    });
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, sync::Arc};

    use crate::code_review::{
        diff_size_limits::DiffSize,
        diff_state::{DiffHunk, DiffLine, FileDiff},
    };

    use super::*;

    fn line(line_type: DiffLineType, text: &str) -> DiffLine {
        DiffLine {
            line_type,
            old_line_number: None,
            new_line_number: None,
            text: text.to_string(),
            no_trailing_newline: false,
        }
    }

    fn file_diff() -> FileDiff {
        FileDiff {
            file_path: PathBuf::from("src/main.rs"),
            status: GitFileStatus::Modified,
            hunks: Arc::new(vec![DiffHunk {
                old_start_line: 1,
                old_line_count: 3,
                new_start_line: 1,
                new_line_count: 4,
                lines: vec![
                    line(DiffLineType::Context, "fn main() {"),
                    line(DiffLineType::Delete, "    println!(\"old\");"),
                    line(DiffLineType::Add, "    println!(\"new\");"),
                    line(DiffLineType::Context, "}"),
                ],
                unified_diff_start: 0,
                unified_diff_end: 0,
            }]),
            is_binary: false,
            is_autogenerated: false,
            max_line_number: 4,
            has_hidden_bidi_chars: false,
            size: DiffSize::Normal,
        }
    }

    #[test]
    fn format_file_diffs_as_unified_diff_outputs_copyable_patch() {
        let diff = file_diff();
        let formatted = format_file_diffs_as_unified_diff([&diff].into_iter());

        assert_eq!(
            formatted,
            [
                "diff --git a/src/main.rs b/src/main.rs",
                "--- a/src/main.rs",
                "+++ b/src/main.rs",
                "@@ -1,3 +1,4 @@",
                " fn main() {",
                "-    println!(\"old\");",
                "+    println!(\"new\");",
                " }",
            ]
            .join("\n")
        );
    }
}
