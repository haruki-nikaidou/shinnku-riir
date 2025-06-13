use crate::file_meta::FileMeta;
use compact_str::CompactString;
use std::collections::HashMap;
use std::sync::Arc;
use kanau::processor::Processor;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub enum FsNode {
    File(FileMeta),
    Directory {
        name: CompactString,
        children: HashMap<CompactString, Arc<RwLock<FsNode>>>,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum JsonFsNode {
    File(FileMeta),
    Directory {
        name: CompactString,
        children: Vec<JsonFsNode>,
    },
}

impl FsNode {
    pub fn name(&self) -> &str {
        match self {
            FsNode::File(meta) => meta.file_name.as_str(),
            FsNode::Directory { name, .. } => name.as_str(),
        }
    }

    pub async fn get_node(&self, name: &str) -> Option<Arc<RwLock<FsNode>>> {
        match self {
            FsNode::File(_) => None,
            FsNode::Directory { children, .. } => children.get(name).cloned(),
        }
    }

    pub async fn to_json(&self) -> JsonFsNode {
        match self {
            FsNode::File(meta) => JsonFsNode::File(meta.clone()),
            FsNode::Directory { name, children } => {
                let mut ser_children = Vec::new();
                for child in children.values() {
                    ser_children.push(Box::pin(child.read().await.to_json()).await);
                }
                JsonFsNode::Directory {
                    name: name.clone(),
                    children: ser_children,
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FsTree {
    root: Arc<RwLock<FsNode>>,
}

impl FsTree {
    pub fn new(root: FsNode) -> Self {
        Self {
            root: Arc::new(RwLock::new(root)),
        }
    }

    pub async fn get_node(&self, path: &[CompactString]) -> Option<Arc<RwLock<FsNode>>> {
        let mut current = self.root.clone();
        for name in path {
            let owner = current;
            current = owner.read().await.get_node(name.as_str()).await?;
        }
        Some(current)
    }

    pub async fn insert_node(&mut self, path: &[CompactString], node: FsNode) {
        let mut current = self.root.clone();

        for name in &path[..path.len().saturating_sub(1)] {
            let next = {
                let mut guard = current.write().await;
                match &mut *guard {
                    FsNode::File(_) => {
                        // If a file is encountered in path, replace it with a Directory
                        *guard = FsNode::Directory {
                            name: name.clone(),
                            children: HashMap::new(),
                        };
                        None
                    }
                    FsNode::Directory { children, .. } => {
                        let owner = children
                            .entry(name.clone())
                            .or_insert_with(|| {
                                Arc::new(RwLock::new(FsNode::Directory {
                                    name: name.clone(),
                                    children: HashMap::new(),
                                }))
                            })
                            .clone();
                        Some(owner)
                    }
                }
            };

            if let Some(next) = next {
                current = next;
            } else {
                break;
            }
        }

        // Insert or overwrite at the target position
        if let Some(last_name) = path.last() {
            let mut guard = current.write().await;
            match &mut *guard {
                FsNode::File(_) => {
                    // If target parent is a file, replace it with Directory first
                    *guard = FsNode::Directory {
                        name: last_name.clone(),
                        children: HashMap::new(),
                    };
                }
                FsNode::Directory { children, .. } => {
                    children.insert(last_name.clone(), Arc::new(RwLock::new(node)));
                }
            }
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccessRequest {
    pub path: String
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccessResponse {
    pub node: JsonFsNode,
}

impl Processor<AccessRequest, AccessResponse> for FsTree {
    async fn process(&self, request: AccessRequest) -> AccessResponse {
        let path = request.path.split('/').map(|s| CompactString::new(s)).collect::<Vec<_>>();
        let node = self.get_node(&path).await.unwrap();
        let node = node.read().await;
        AccessResponse {
            node: node.to_json().await,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_meta::FileMeta;
    use compact_str::CompactString;
    use smallvec::SmallVec;
    use std::time::SystemTime;

    #[tokio::test]
    async fn test_fs_tree_dir() {
        let root = FsNode::Directory {
            name: CompactString::new("root"),
            children: HashMap::new(),
        };
        let mut tree = FsTree::new(root);
        tree.insert_node(
            &[
                CompactString::new("dir1"),
                CompactString::new("dir2"),
                CompactString::new("file1"),
            ],
            FsNode::File(FileMeta {
                categories: SmallVec::new(),
                file_name: CompactString::new("file1"),
                file_size: 100,
                upload_timestamp: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                path: "dir1/dir2/file1".to_string(),
            }),
        )
        .await;
        
        let node = tree.get_node(&[CompactString::new("dir1"), CompactString::new("dir2"), CompactString::new("file1")]).await.unwrap();
        let node = node.read().await;
        assert_eq!(node.name(), "file1");
    }
}
