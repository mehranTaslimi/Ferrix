'use client';

import { memo } from 'react';

import { Status } from './types';

function getStatusColor(status: Status) {
  switch (status) {
    case Status.Downloading:
      return 'bg-blue-500';
    case Status.Completed:
      return 'bg-green-500';
    case Status.Failed:
      return 'bg-red-500';
    case Status.Paused:
      return 'bg-yellow-500';
    case Status.Queued:
      return 'bg-gray-500';
    case Status.Writing:
      return 'bg-purple-500';
    default:
      return 'bg-gray-500';
  }
}
interface StatusProps {
  status: Status;
}

function StatusIndicator({ status }: StatusProps) {
  return (
    <div className="flex items-center gap-2">
      <div className={`h-2 w-2 rounded-full ${getStatusColor(status)}`} />
      <span className="text-muted-foreground text-xs font-medium tracking-wide uppercase">
        {status}
      </span>
    </div>
  );
}

export default memo(StatusIndicator);
