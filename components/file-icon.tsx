import {
  FileText, FileJson,
  FileCode,
  FileSpreadsheet,
  File,
  FileAudio,
  FileVideo,
  FileImage,
  FileArchive,
  FileText as TextFile,
  FileType2, FileX,
  FileSymlink,
  BookOpen, FilePlus,
  FileMinus,
  FileKey,
  FileLock2,
  Database,
  FileCog,
  FileChartColumn, FileSignature, Presentation, FileOutput, FileDiff,
  FileWarning
} from "lucide-react";

export function FileIcon({ extension }: { extension: string }) {
  const iconClass = "w-5 h-5";
  const ext = extension?.toLowerCase()?.replace(/^\./, "") || "";

  switch (ext) {
    case "mp4":
    case "mov":
    case "avi":
    case "mkv":
    case "webm":
      return <FileVideo className={iconClass} />;

    case "jpg":
    case "jpeg":
    case "png":
    case "gif":
    case "bmp":
    case "webp":
    case "svg":
      return <FileImage className={iconClass} />;

    case "mp3":
    case "wav":
    case "ogg":
    case "flac":
    case "aac":
      return <FileAudio className={iconClass} />;

    case "rar":
    case "zip":
    case "7z":
    case "tar":
    case "gz":
      return <FileArchive className={iconClass} />;

    case "txt":
      return <TextFile className={iconClass} />;
    case "pdf":
      return <FileText className={iconClass} />;
    case "doc":
    case "docx":
    case "odt":
      return <FileType2 className={iconClass} />;
    case "rtf":
      return <FileSignature className={iconClass} />;

    case "xls":
    case "xlsx":
    case "ods":
    case "csv":
      return <FileSpreadsheet className={iconClass} />;

    case "ppt":
    case "pptx":
    case "odp":
      return <Presentation className={iconClass} />;

    case "json":
      return <FileJson className={iconClass} />;
    case "js":
    case "ts":
    case "tsx":
    case "jsx":
    case "html":
    case "css":
    case "scss":
    case "rs":
    case "py":
    case "go":
    case "java":
    case "cpp":
    case "c":
    case "h":
    case "sh":
      return <FileCode className={iconClass} />;
    case "lock":
      return <FileLock2 className={iconClass} />;
    case "key":
      return <FileKey className={iconClass} />;
    case "toml":
    case "yaml":
    case "yml":
      return <FileCog className={iconClass} />;

    case "db":
    case "sqlite":
    case "sql":
      return <Database className={iconClass} />;

    case "chart":
    case "graph":
      return <FileChartColumn className={iconClass} />;

    case "epub":
    case "mobi":
      return <BookOpen className={iconClass} />;

    case "log":
      return <FileWarning className={iconClass} />;

    case "diff":
      return <FileDiff className={iconClass} />;
    case "patch":
      return <FilePlus className={iconClass} />;
    case "bak":
      return <FileMinus className={iconClass} />;
    case "symlink":
    case "lnk":
      return <FileSymlink className={iconClass} />;
    case "out":
      return <FileOutput className={iconClass} />;
    case "bin":
      return <FileX className={iconClass} />;

    default:
      return <File className={iconClass} />;
  }
}