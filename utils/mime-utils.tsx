import {
  FileText,
  ImageIcon,
  VideoIcon,
  Archive,
  Music,
  Code,
  FileIcon,
  Database,
  Presentation,
  FileSpreadsheet,
  FileCog,
  FileKey,
  FileLock2,
} from 'lucide-react';

import type { JSX } from 'react';

const iconClass = 'w-4 h-4';

const contains = (s: string, ...subs: string[]) => subs.some((x) => s.includes(x));

export function iconForMime(mimeRaw?: string): JSX.Element {
  const mime = (mimeRaw || '').toLowerCase();

  if (mime.startsWith('image/')) return <ImageIcon className={iconClass} />;

  if (mime.startsWith('video/')) return <VideoIcon className={iconClass} />;

  if (mime.startsWith('audio/')) return <Music className={iconClass} />;

  if (
    contains(
      mime,
      'zip',
      'rar',
      '7z',
      'x-7z-compressed',
      'x-rar-compressed',
      'x-tar',
      'x-gtar',
      'x-gzip',
      'x-bzip',
      'x-bzip2',
      'zstd',
      'xz',
    )
  ) {
    return <Archive className={iconClass} />;
  }

  if (mime === 'application/pdf') return <FileText className={iconClass} />;

  if (contains(mime, 'msword', 'vnd.openxmlformats-officedocument.wordprocessingml')) {
    return <FileText className={iconClass} />;
  }
  if (contains(mime, 'vnd.ms-excel', 'vnd.openxmlformats-officedocument.spreadsheetml')) {
    return <FileSpreadsheet className={iconClass} />;
  }
  if (contains(mime, 'vnd.ms-powerpoint', 'vnd.openxmlformats-officedocument.presentationml')) {
    return <Presentation className={iconClass} />;
  }

  if (mime.startsWith('text/') || contains(mime, 'markdown', 'rtf')) {
    if (
      contains(
        mime,
        'javascript',
        'typescript',
        'tsx',
        'jsx',
        'css',
        'scss',
        'html',
        'xml',
        'yaml',
        'yml',
        'toml',
        'x-shellscript',
      )
    ) {
      return <Code className={iconClass} />;
    }
    return <FileText className={iconClass} />;
  }

  if (contains(mime, 'json')) return <Code className={iconClass} />;
  if (contains(mime, 'yaml', 'yml', 'toml', 'ini')) return <FileCog className={iconClass} />;
  if (contains(mime, 'pgp-signature', 'x-pem-file', 'pkcs', 'x-x509'))
    return <FileKey className={iconClass} />;
  if (contains(mime, 'pkcs7-signature')) return <FileLock2 className={iconClass} />;

  if (
    contains(
      mime,
      'x-python',
      'x-rust',
      'x-go',
      'x-c',
      'x-c++',
      'x-java',
      'x-csharp',
      'x-php',
      'x-ruby',
    )
  ) {
    return <Code className={iconClass} />;
  }

  if (
    contains(
      mime,
      'x-sqlite3',
      'vnd.sqlite3',
      'sql',
      'x-msaccess',
      'x-dbf',
      'x-parquet',
      'x-feather',
    )
  ) {
    return <Database className={iconClass} />;
  }

  if (
    contains(
      mime,
      'x-msdownload',
      'x-dosexec',
      'x-apple-diskimage',
      'x-deb',
      'x-rpm',
      'vnd.android.package-archive',
      'x-appimage',
      'octet-stream',
    )
  ) {
    return <FileCog className={iconClass} />;
  }

  return <FileIcon className={iconClass} />;
}

export function labelForMime(mimeRaw?: string): string {
  const mime = (mimeRaw || '').toLowerCase();

  if (mime.startsWith('image/')) return 'Images';
  if (mime.startsWith('video/')) return 'Videos';
  if (mime.startsWith('audio/')) return 'Audio';

  if (
    contains(
      mime,
      'zip',
      'rar',
      '7z',
      'x-7z-compressed',
      'x-rar-compressed',
      'x-tar',
      'x-gtar',
      'x-gzip',
      'x-bzip',
      'x-bzip2',
      'zstd',
      'xz',
    )
  )
    return 'Archives';

  if (mime === 'application/pdf') return 'PDFs';

  if (contains(mime, 'msword', 'vnd.openxmlformats-officedocument.wordprocessingml'))
    return 'Documents';

  if (contains(mime, 'vnd.ms-excel', 'vnd.openxmlformats-officedocument.spreadsheetml'))
    return 'Spreadsheets';

  if (contains(mime, 'vnd.ms-powerpoint', 'vnd.openxmlformats-officedocument.presentationml'))
    return 'Presentations';

  if (mime.startsWith('text/') || contains(mime, 'markdown', 'rtf', 'xml')) return 'Documents';

  if (contains(mime, 'json')) return 'Data / JSON';
  if (contains(mime, 'yaml', 'yml', 'toml', 'ini')) return 'Config';
  if (
    contains(
      mime,
      'javascript',
      'typescript',
      'tsx',
      'jsx',
      'x-python',
      'x-rust',
      'x-go',
      'x-c',
      'x-c++',
      'x-java',
      'x-csharp',
      'x-php',
      'x-ruby',
      'html',
      'css',
      'scss',
    )
  )
    return 'Code';

  if (
    contains(
      mime,
      'x-sqlite3',
      'vnd.sqlite3',
      'sql',
      'x-msaccess',
      'x-dbf',
      'x-parquet',
      'x-feather',
    )
  )
    return 'Databases';

  if (
    contains(
      mime,
      'x-msdownload',
      'x-dosexec',
      'x-apple-diskimage',
      'x-deb',
      'x-rpm',
      'vnd.android.package-archive',
      'x-appimage',
      'octet-stream',
    )
  )
    return 'Applications';

  if (mime.startsWith('application/')) return 'Applications';

  return mime.split('/')[0] || 'Other';
}
