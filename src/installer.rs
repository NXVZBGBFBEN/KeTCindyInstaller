use crate::package::Package;
use crate::package::PackageKind;
use crate::package::PackageState;
use anyhow::Context;

enum BackendEvent {
    Fetched {
        package_kind: PackageKind,
        versions: Vec<String>,
    },
    Error {
        package_kind: PackageKind,
        message: String,
    },
}

pub(crate) struct Installer {
    packages: Vec<Package>,
    worker_event_tx: tokio::sync::mpsc::Sender<BackendEvent>,
    worker_event_rx: tokio::sync::mpsc::Receiver<BackendEvent>,
    async_runtime: tokio::runtime::Runtime,
}

impl Installer {
    pub fn new() -> Self {
        let packages = vec![
            Package::new(PackageKind::KeTCindy),
            Package::new(PackageKind::Cinderella),
            Package::new(PackageKind::R),
            Package::new(PackageKind::Maxima),
        ];
        let (worker_event_tx, worker_event_rx) = tokio::sync::mpsc::channel(8);
        let async_runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        Self { packages, worker_event_tx, worker_event_rx, async_runtime }
    }

    // バージョン一覧の取得（UI側から呼ぶ）
    pub fn fetch_versions(&mut self, package_kind: PackageKind) {
        use crate::worker;

        self.package(package_kind).state = PackageState::Fetching;

        let tx = self.worker_event_tx.clone();
        self.async_runtime.spawn(async move {
            let versions = worker::fetch_versions(package_kind).await.context("Failed to retrieve version list.");

            let event = match versions {
                Ok(versions) => BackendEvent::Fetched { package_kind: package_kind, versions },
                Err(error) => BackendEvent::Error { package_kind: package_kind, message: error.to_string() },
            };
            tx.send(event).await.ok();
        });
    }

    // バージョン選択（UI側から呼ぶ）
    pub fn select_version(&mut self, package_kind: PackageKind, index: Option<usize>) {
        let package = self.package(package_kind);
        if let PackageState::Fetched { selected_index, .. } = &mut package.state {
            *selected_index = index;
        }
    }

    // TODO: インストール処理書く
    pub fn start_installation(&mut self) {
        panic!("oh shit");
    }

    // workerからのイベントがあるかを確認して，ある場合は処理
    pub fn poll_events(&mut self) -> bool {
        let mut had_events = false;
        // Receiverのキューを全部処理
        while let Ok(event) = self.worker_event_rx.try_recv() {
            had_events = true;
            self.handle_event(event);
        }
        had_events
    }

    // Eventに基づいてPackageの状態を更新する
    fn handle_event(&mut self, event: BackendEvent) {
        match event {
            BackendEvent::Fetched { package_kind, versions } => {
                self.package(package_kind).state = PackageState::Fetched {
                    versions,
                    selected_index: Some(0),    // デフォルトで最新を選択
                };
            },
            BackendEvent::Error { package_kind, message } => {
                // TODO: エラーハンドリング
                panic!("aaa!")
            },
        }
    }

    // packagesを複製して返す（描画用）
    pub fn packages(&self) -> Vec<Package> {
        self.packages.clone()
    }

    // PackageKindに対応するPackageの可変参照を返す（内部用）
    fn package(&mut self, package_kind: PackageKind) -> &mut Package {
        // TODO:
        // テストを書いてunwrap()の正当性を保証するか，
        // パッケージごとにフィールドを用意してコンパイル時に保証する
        self.packages.iter_mut().find(|package| package.kind == package_kind).unwrap()
    }
}
