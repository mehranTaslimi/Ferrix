pub enum RegistryAction {
    AddNewDownload(
        /* Download Url */ String,
        super::download::DownloadOptions,
    ),
    AddDownloadToQueue(/*Download Id */ i64),
}

impl super::Registry {
    pub async fn dispatch(action: RegistryAction) -> Result<(), String> {
        match action {
            RegistryAction::AddNewDownload(url, options) => {
                return Self::add_new_download(url, options).await;
            }
            RegistryAction::AddDownloadToQueue(download_id) => {
                Self::add_download_queue(download_id);
                Ok(())
            }
        }
    }
}
