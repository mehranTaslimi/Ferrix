export interface DownloadType {
  id: number;
  url: string;
  total_bytes: number;
  status: Status;
  created_at: Date;
  downloaded_bytes: number;
  chunk_count: number;
  file_path: string;
  file_name: string;
  content_type: string;
  extension: Extension;
}

export enum ContentType {
  ApplicationXRarCompressed = "application/x-rar-compressed",
  ImageJPEG = "image/jpeg",
  VideoMp4 = "video/mp4",
}

export enum Extension {
  Jpg = "jpg",
  Mp4 = "mp4",
  Rar = "rar",
}

export enum Status {
  Downloading = "downloading",
  Completed = "completed",
  Queued = "queued",
  Paused = "paused",
  Failed = "failed",
  Writing = "writing",
  Error = "error",
}
