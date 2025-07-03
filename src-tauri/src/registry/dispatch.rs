pub enum RegistryAction {
    AddNewDownload(
        /* Download Url */ String,
        super::download::DownloadOptions,
    ),
}

impl super::Registry {
    pub async fn dispatch(action: RegistryAction) -> Result<(), String> {
        match action {
            RegistryAction::AddNewDownload(url, options) => {
                return Self::add_new_download(url, options).await;
            }
        }
    }
}
