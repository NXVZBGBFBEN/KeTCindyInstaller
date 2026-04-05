use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone, Copy, strum::Display)]
pub(crate) enum PackageKind {
    KeTCindy,
    Cinderella,
    R,
    Maxima,
}

#[derive(Clone)]
pub(crate) enum PackageState {
    Initialized,
    Fetching,
    Fetched {
        versions: Vec<String>,
        selected_index: Option<usize>,
    },
    Downloading {
        progress: f32,
    },
    Downloaded {
        path: PathBuf,
    },
    Installing,
    Installed,
    Error(String),
}

#[derive(Clone)]
pub(crate) struct Package {
    pub(crate) kind: PackageKind,
    pub(crate) state: PackageState,
}

impl Package {
    pub(crate) fn new(kind: PackageKind) -> Self {
        Self {
            kind: kind,
            state: PackageState::Initialized,
        }
    }
}
