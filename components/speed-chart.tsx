'use client';

import { AreaChart, Area, ResponsiveContainer } from 'recharts';

const NET_COLOR = 'hsl(217.2 91.2% 59.8%)';
const DISK_COLOR = 'hsl(142.1 70% 45%)';

export type SpeedPoint = { t: number; net: number; disk: number };

export function SpeedChart({ data }: { data: SpeedPoint[] }) {
  return (
    <div className="pointer-events-none h-8">
      <ResponsiveContainer width="100%" height="100%">
        <AreaChart data={data}>
          <defs>
            <linearGradient id="netFill" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stopColor={NET_COLOR} stopOpacity={0.35} />
              <stop offset="100%" stopColor={NET_COLOR} stopOpacity={0} />
            </linearGradient>
            <linearGradient id="diskFill" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stopColor={DISK_COLOR} stopOpacity={0.25} />
              <stop offset="100%" stopColor={DISK_COLOR} stopOpacity={0} />
            </linearGradient>
          </defs>

          <Area
            type="monotone"
            dataKey="net"
            stroke={NET_COLOR}
            fill="url(#netFill)"
            strokeWidth={1.5}
            dot={false}
            activeDot={false}
            isAnimationActive={false}
          />
          <Area
            type="monotone"
            dataKey="disk"
            stroke={DISK_COLOR}
            fill="url(#diskFill)"
            strokeWidth={1.5}
            dot={false}
            activeDot={false}
            isAnimationActive={false}
          />
        </AreaChart>
      </ResponsiveContainer>
    </div>
  );
}
