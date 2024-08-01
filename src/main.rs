use std::env::temp_dir;
use std::fs::remove_dir_all;
use std::path::{Path, PathBuf};
use git2::build::RepoBuilder;
use clap::Parser;
use regex::Regex;
use which::which;


#[derive(Parser)]
#[command(version = "0.1.0")]
struct Args {
    /// The repository to clone from GitHub (ex: "https://github.com/yunusey/gosu")
    #[structopt(name = "repository")]
    repository: String,

    /// The path to the Godot executable (tries to find it atomatically, if not specified)
    /// (ex: --godot-path="/home/yunusey/.nix-profile/bin/godot4")
    #[arg(long, default_value = "")]
    godot_path: String,

    /// Extra arguments to pass to the Godot executable (learn more at
    /// https://docs.godotengine.org/en/stable/tutorials/editor/command_line_tutorial.html)
    /// (ex: --extra-arguments="--resolution 1920x1080")
    #[arg(short, long, default_value = "")]
    extra_arguments: String,
}

#[derive(Debug, Clone, PartialEq)]
struct RepositoryProps {
    owner: String,
    repo: String,
    branch: Option<String>,
    subpath: Option<Vec<String>>,
}

fn parse_repository(repository: &str) -> Option<RepositoryProps> {
    let re = Regex::new(r"^((https?|ftp|smtp)://)?(www.)?(github\.com/)?(?P<username>[^/]+)/(?P<repo>[^/]+)(?:/tree/(?P<branch>[^/]+)(/(?P<subpath>.+))?)?$").unwrap();

    if let Some(caps) = re.captures(repository) {
        return Some(RepositoryProps {
            owner: caps.name("username").unwrap().as_str().to_string(),
            repo: caps.name("repo").unwrap().as_str().to_string(),
            branch: caps.name("branch").map(|branch| branch.as_str().to_string()),
            subpath: caps.name("subpath").map(|subpath| subpath.as_str().to_string().split('/').map(|s| s.to_string()).collect()),
        })
    }
    else {
        None
    }
}

fn find_godot() -> Option<PathBuf> {
    const EXECUTABLE_NAMES: [&str; 3] = ["godot", "godot4", "godot3"];
    for executable in &EXECUTABLE_NAMES {
        match which(executable) {
            Ok(path) => {
                return Some(path.to_path_buf());
            },
            Err(_) => continue,
        }
    }

    None
}

fn main() {
    let args = Args::parse();

    let godot_path = if args.godot_path.is_empty() {
        find_godot().expect("Failed to find Godot executable")
    }
    else {
        Path::new(&args.godot_path).to_path_buf()
    };
    println!("Using Godot executable at {}", godot_path.display());

    let tmpdir = temp_dir();
    println!("Cloning repository to {}", tmpdir.display());
    let Some(repository_props) = parse_repository(&args.repository) else { return };
    let clone_path = Path::new(&tmpdir)
        .join(&repository_props.owner)
        .join(&repository_props.repo)
        .join(&repository_props.branch.clone().unwrap_or("default".to_string()));

    // just delete the old clone if it exists
    // TODO(yunusey): maybe add a feature to cache the clone if there's no update(s)
    if clone_path.exists() {
        remove_dir_all(&clone_path).unwrap();
    }

    let mut repository = RepoBuilder::new();
    if repository_props.branch.is_some() {
        repository
            .branch(&repository_props.branch.clone().unwrap_or("".to_string()));
    }
    repository
        .clone(&format!("https://github.com/{}/{}", repository_props.owner, repository_props.repo), &clone_path)
        .expect("Failed to clone repository");
    println!("Repository cloned to {}", clone_path.display());

    let project_path = clone_path
        .join(&repository_props.subpath.unwrap_or(vec![]).join("/")); // subpaths if specified

    println!("Opening the project in the editor...");
    let _ = std::process::Command::new(&godot_path)
        .arg("--path")      // we need to open the project in the editor at least once
        .arg(&project_path)
        .arg("--headless")  // no need to wait for the editor
        .arg("--quit")      // quit once the first iteration is done
        .spawn()            // quit causes the whole process to exit, so use spawn instead of exec
        .expect("Failed to open project in editor")
        .wait();

    println!("Running the game now");
    let _ = std::process::Command::new(&godot_path)
        .arg("--path")
        .arg(&project_path)
        .args(&mut args.extra_arguments.split(' '))
        .spawn() // we can use exec instead of spawn too, but just in case we need more control
        .expect("Failed to run the game")
        .wait();

    println!("Quitting...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_repository_test(){
        let repo = parse_repository("https://github.com/yunusey/gosu");
        assert_eq!(repo, Some(RepositoryProps {
            owner: "yunusey".to_string(),
            repo: "gosu".to_string(),
            branch: None,
            subpath: None
        }));

        let repo = parse_repository("https://www.github.com/yunusey/gosu");
        assert_eq!(repo, Some(RepositoryProps {
            owner: "yunusey".to_string(),
            repo: "gosu".to_string(),
            branch: None,
            subpath: None
        }));

        let repo = parse_repository("github.com/yunusey/gosu");
        assert_eq!(repo, Some(RepositoryProps {
            owner: "yunusey".to_string(),
            repo: "gosu".to_string(),
            branch: None,
            subpath: None
        }));

        let repo = parse_repository("yunusey/gosu");
        assert_eq!(repo, Some(RepositoryProps {
            owner: "yunusey".to_string(),
            repo: "gosu".to_string(),
            branch: None,
            subpath: None
        }));

        let repo = parse_repository("https://github.com/godotengine/godot-demo-projects/tree/master/gui/theming_override");
        assert_eq!(repo, Some(RepositoryProps {
            owner: "godotengine".to_string(),
            repo: "godot-demo-projects".to_string(),
            branch: Some("master".to_string()),
            subpath: Some(vec!["gui".to_string(), "theming_override".to_string()])
        }));
    }
}
