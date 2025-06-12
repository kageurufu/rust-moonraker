use std::collections::HashMap;

use eserde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UpdateManagerResponse {
    /// The name of the software currently updating.
    application: String,
    /// A unique ID associated with the current update.
    proc_id: usize,
    /// A message containing status and/or information about the current update.
    message: String,
    /// When set to true it indicates that the update has finished and this will be the last status response notification sent for this update.
    complete: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UpdateManagerRefreshed {
    ///Set to true if an update is currently in progress.
    pub busy: bool,
    ///The maximum number of GitHub API requests allowed. An unauthenticated user is typically allowed 60 requests per hour.
    pub github_rate_limit: usize,
    ///The number of GitHub API requests remaining until the reset time is reached.
    pub github_requests_remaining: usize,
    ///The time when the rate limit will reset, reported in unix time.
    pub github_limit_reset_time: usize,
    ///A Version Info object containing the update status for each configured software updater.
    pub version_info: HashMap<String, Update>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "configured_type")]
pub enum Update {
    System {
        /// The name of the software to manage updates for. Will always be system.
        name: String,
        /// The number of system packages that require updating.
        package_count: usize,
        /// An array of package names that require updating.
        package_list: Vec<String>,
    },

    GitRepo {
        /// The name of the software to manage updates for.
        name: String,
        /// DEPRECATED. Will always report git_repo.
        detected_type: String,
        /// The configured update channel.
        channel: String,
        /// A value of true indicates that the current channel configuration is not supported by the type. Will always be false for git_repo types as all channels are supported.
        channel_invalid: bool,
        /// Set to true when Moonraker's debug features are enabled. In this condition updates may proceed when the repo's HEAD is detached.
        debug_enabled: bool,
        /// Set to true when repo detection completes and passes all validity checks.
        is_valid: bool,
        /// The current detected version.
        version: String,
        /// The latest version available on the remote.
        remote_version: String,
        /// The version prior to the last update. This version is used during a rollback request.
        rollback_version: String,
        /// The complete version string reported by git describe. Generally includes an abbreviated hash of the current commit and tags such as "dirty" when appropriate.
        full_version_string: String,
        /// The latest available commit hash on the remote.
        remote_hash: String,
        /// The commit hash the local repo is currently on.
        current_hash: String,
        /// The git alias of the remote. The git default for the primary alias is origin.
        remote_alias: String,
        /// Full URL of the git remote matching the current remote_alias.
        remote_url: String,
        /// The origin git remote URL for this repo. This URL is used to perform a hard recovery when requested.
        recovery_url: String,
        /// The owner of the remote repo as detected from the remote URL.
        owner: String,
        /// The name of the current git branch.
        branch: String,
        /// The name of the remote repo as detected from the remote URL.
        repo_name: String,
        /// Set to true if the repo is "dirty", ie: if one or more files in the repo have been modified.
        is_dirty: bool,
        /// Set to true if the repo is corrupt. This indicates that the local repo is broken and needs to be recovered.
        corrupt: bool,
        /// Set to true when the repo is clean and no untracked files exist in the repo.
        pristine: bool,
        /// Set to true when the git repo's HEAD is detached.
        detached: bool,
        /// An array of strings containing the output from a failed git command during initialization or an update. This array will be empty if all git commands succeed.
        git_messages: Vec<String>,
        /// An array of strings that describe anomalies found during initialization. An anomaly can be defined as an unexpected condition that does not result in an invalid repo state. Updates may proceed when anomalies are detected. An example of an anomaly is the presence of "untracked files" in the repo.
        anomalies: Vec<String>,
        /// An array of strings that describe warnings detected during repo initialization. When a warning is present the repo is marked invalid and updates are disabled.
        warnings: Vec<String>,
        /// An array of Commit Info objects providing commit data on upstream commits available for update. This array is limited to a size of 30 untagged commits. Any tagged commits within 100 commits behind are included.
        commits_behind: Vec<GitCommitInfo>,
        /// The total number of commits the current repo is behind the next update. This number may be greater than the length of the commits_behind array.
        commits_behind_count: usize,
        /// An object containing custom tags added to the updater's configuration in moonraker.conf. The values will always be strings. Client developers may define what tags, if any, users will configure. The software can then choose to display information or perform a specific action pre/post update if necessary.
        info_tags: Vec<String>,
    },

    Python {
        /// The name of the software to manage updates for.
        name: String,
        /// The configured update channel.
        channel: String,
        /// A value of true indicates that the current channel configuration is not supported by the type. are supported.
        channel_invalid: bool,
        /// Set to true when Moonraker's debug features are enabled.
        debug_enabled: bool,
        /// The owner of the GitHub repo hosting the software. Will be a ? when no repo owner is detected.
        owner: String,
        /// The name of the GitHub repo hosting the software. Will be a ? when no repo name is detected.
        repo_name: String,
        /// The name of the branch on the GitHub remote to build dev updates from. Will be null if no primary branch is configured.
        branch: Option<String>,
        /// The current detected version.
        version: String,
        /// The version of the latest available release on GitHub.
        remote_version: String,
        /// The version prior to the last update. This version is used during a rollback request.
        rollback_version: String,
        /// The complete version string extracted from the python package's metadata.
        full_version_string: String,
        /// The hash of the commit used to build the current version of the package. A placeholder of not-specified is used when the the current hash is not provided in the package metadata.
        current_hash: String,
        /// The hash of the latest update available. A placeholder of update-available is used when the remote hash is not provided by the remote host and updates are available.
        remote_hash: String,
        /// Set to true when the updater has completed initialization and all validity checks passed.
        is_valid: bool,
        /// Set to true if the repo was modified at the time the package was built.
        is_dirty: bool,
        /// A URL to the software's changelog. Will be an empty string if no changelog URL is detected.
        changelog_url: String,
        /// An array of strings that describe anomalies found during initialization. An anomaly can be defined as an unexpected condition that does not result in an invalid updater state. Updates may proceed when anomalies are detected.
        anomalies: Vec<String>,
        /// An array of strings that describe warnings detected during initialization. When a warning is present the updater is marked invalid and updates are disabled.
        warnings: Vec<String>,
        /// An object containing custom tags added to the updater's configuration in moonraker.conf. The values will always be strings. Client developers may define what tags, if any, users will configure. The software can then choose to display information or perform a specific action pre/post update if necessary.
        info_tags: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GitCommitInfo {
    /// The author of the commit.
    author: String,
    /// The date of the commit in unix time. Note that the date is extracted from the git log as a string value. It should be converted to an integer prior to processing from unix time.
    date: String,
    /// The commit hash.
    sha: String,
    /// The title of the commit.
    subject: String,
    /// The content in the body of the commit.
    message: String,
    /// The name of the associated tag if present. Will be null if the commit has no tag.
    tag: Option<String>,
}
