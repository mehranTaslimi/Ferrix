"use client";

import * as React from "react";
import { cn } from "@/lib/utils";

export type ChartConfig = Record<
  string,
  { label: string; color?: string; icon?: React.ComponentType<{ className?: string }> }
>;

export function ChartContainer({
  className,
  children,
  config,
}: {
  className?: string;
  config: ChartConfig;
  children: React.ReactNode;
}) {

  return (
    <div
      data-chart
      className={cn("rounded-xl border bg-card p-3", className)}
      style={
        Object.keys(config).reduce((vars, key, i) => {
          const c = config[key]?.color;
          if (c) (vars as any)[`--chart-${key}`] = c;
          return vars;
        }, {} as React.CSSProperties)
      }
    >
      {children}
    </div>
  );
}
