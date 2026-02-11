use std::{fmt::Display, fs::File, io::{BufRead, BufReader}, path::{Path, PathBuf}};

/// 
/// A type for an add-on.
/// In an effort to make this type extensible, the Option type has been used for optional parameters.
/// For example, an attachment add-on likely has no version or update-site property, as the primary add-on would handle this.
pub struct Addon {
    pub title: String,
    pub version: Option<String>,
    pub path: PathBuf,
    /// Probably want to change this eventually to a URL type.
    pub update_site: Option<String>,

    ///
    ///  These are child add-ons associated with existing add-ons, nicknamed "attachments".
    ///  For an easy example, take the Bagnon add-on, which has three "attachments":
    ///     - Bagnon_Bank
    ///     - Bagnon_Config
    ///     - Bagnon_GuildBank
    pub attachments: Option<Vec<Addon>>
}

/// Constructs an Addon type via a borrowed path of that addon's directory.
/// 
/// This process goes as follows:
/// - First, determine the location of the addon's "toc" file.
/// - Second, determine whether the addon is a primary addon or an attachment via separator charactor parsing.
/// - Third, construct the add-on type.
pub fn construct_addon(path: &Path) -> Addon {

    let mut title: Option<String> = None;
    let mut version: Option<String> = None;

    for entry in path.read_dir().expect("Unknown entry.") {
        if let Ok(entry) = entry {
            
            let file_name = entry.file_name().to_str().expect("Cannot convert from type OsStr to type &str").to_string();
            let file_type = entry.file_type().expect("Unknown file type");

            if file_type.is_file() && file_name.ends_with(".toc") {
                let file = File::open(entry.path()).expect("Cannot open TOC file");

                let buf = BufReader::new(file);

                for line in buf.lines() {
                    if let Ok(line) = line {
                        if let Ok(line) = find_and_replace_detail("## ", &line) {

                            if let Ok(found_title) = find_and_replace_detail("Title: ", &line) {
                                title = Some(found_title);
                            }

                            if let Ok(found_version) = find_and_replace_detail("Version: ", &line) {
                                version = Some(found_version);
                            }
                        }
                    }
                }
            }
        }
    }


    Addon { 
        title: title.unwrap(),
        version: version,
        path: path.to_path_buf(),
        update_site: None,
        attachments: None,
    }
}

/// Result type which uses the toc file detail error.
type Result<T> = std::result::Result<T, DetailNotFoundError>;

/// Error definition for when we are unable to find a toc file detail.
#[derive(Debug, Clone)]
pub struct DetailNotFoundError;

impl Display for DetailNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Detail not found")
    }
}

///
/// Checks if a line starts with the detail, then replaces the detail string in the line.
/// 
/// Returns the replaced line.
/// 
/// ie:
/// Line is "## Hey" , which starts with detail "## ", "## " is then removed from the line and "Hey" is returned.
/// 
fn find_and_replace_detail(detail: &str, line: &str) -> Result<String> {
    if line.starts_with(detail) {
        let line = line.replace(detail, "");

        return Ok(line)    
    }

    return Err(DetailNotFoundError)
}