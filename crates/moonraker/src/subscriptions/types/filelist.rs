use eserde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum FilelistChange {
    /// A file has been created within the watched root.
    CreateFile { item: FilelistChangeItem },

    /// A subdirectory has been created within the watched root.
    CreateDir { item: FilelistChangeItem },

    /// A file has been deleted within the watched root.
    DeleteFile { item: FilelistChangeItem },

    /// A subdirectory has been deleted within the watched root.
    DeleteDir { item: FilelistChangeItem },

    /// A file in a watched root has been moved.
    MoveFile {
        source_item: FilelistChangeSourceItem,
        item: FilelistChangeItem,
    },

    /// A subdirectory in a watched root has been moved.
    MoveDir {
        source_item: FilelistChangeSourceItem,
        item: FilelistChangeItem,
    },

    /// A file in a watched root has been modified.
    ModifyFile { item: FilelistChangeItem },

    /// A root folder's location on disk has changed.
    RootUpdate { item: FilelistChangeItem },
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FilelistChangeItem {
    /// The path of the destination item relative to the root directory.
    pub path: String,
    /// The root node of the destination item.
    pub root: String,
    /// The last modified date in Unix Time (seconds).
    /// TODO: chrono::DateTime from float
    pub modified: f64,
    /// The size of the destination item.
    pub size: usize,
    /// Permissions available on the changed item (if applicable).
    pub permissions: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FilelistChangeSourceItem {
    /// The path of the source item relative to the root directory.
    pub path: String,
    /// The root node of the source item.
    pub root: String,
}

#[cfg(test)]
mod test {
    use super::FilelistChange;
    use crate::subscriptions::types::{FilelistChangeItem, FilelistChangeSourceItem};
    use serde_json::{from_value, json};

    #[test]
    fn test_example() {
        let example = json!(
            {
                "item": {
                    "root": "gcodes",
                    "path": "subdir/my_file.gcode",
                    "modified": 1676940082.8595376,
                    "size": 384096,
                    "permissions": "rw"
                },
                "source_item": {
                    "path": "testdir/my_file.gcode",
                    "root": "gcodes"
                },
                "action": "move_file"
            }
        );

        let value: FilelistChange = from_value(example).unwrap();

        assert_eq!(
            value,
            FilelistChange::MoveFile {
                item: FilelistChangeItem {
                    root: "gcodes".to_string(),
                    path: "subdir/my_file.gcode".to_string(),
                    modified: 1676940082.8595376,
                    size: 384096,
                    permissions: "rw".to_string()
                },
                source_item: FilelistChangeSourceItem {
                    path: "testdir/my_file.gcode".to_string(),
                    root: "gcodes".to_string()
                },
            }
        );
    }
}
