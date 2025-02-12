pub const SYSTEM_PROMPT: &str =
    "I want you to act as a commit message generator. I will provide you with information about the task and the prefix for the task code, and I would like you to generate an appropriate commit message using the conventional commit format. Do not write any explanations or other words, just reply with the commit message.";
pub const MODEL: &str = "llama3.2:latest";
pub const LOCALHOST: &str = "http://localhost";
pub const LLM_PORT: u16 = 11434;
pub const DEFAULT_DIFF_TYPE: &str = "--cached";
pub const TEMPERATURE: f32 = 0.7;
pub const REPEAT_PENALTY: f32 = 1.3;
pub const TOP_K: u32 = 35;
pub const TOP_P: f32 = 0.25;
pub const TARGET_DIRECTORY: &str =
    "Please enter the target directory containing the Git repository:";
pub const CHANGES_TYPE_INQUERY: &str = "What type of changes would you like to commit?";
pub const CHANGES_NATURE_INQUERY: &str =
    "What is the nature of the changes? bugfix, feature, chore, docs, style, refactor, perf, test, build, or feat!";
pub const PUSH_CHANGES_INQUERY: &str = "Do you want to push the changes?";
pub const APPROVE_COMMIT_INQUERY: &str = "Do you approve this commit message?";
pub const DIRECTORY_NOT_GIT_REPO: &str = "The target directory is not a Git repository.";
pub const NO_COMMIT_CHANGES: &str = "There are no changes to commit.";
pub const CHANGES_PUSHED: &str = "Changes committed and pushed.";
pub const CHANGES_NOT_PUSHED: &str = "Changes committed, but not pushed.";
