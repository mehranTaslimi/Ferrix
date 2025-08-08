"use client";

import { FormMessage as RHFMessage } from "../ui/form";

export default function FormMessage() {
  return (
    <div className="min-h-2.5">
      <RHFMessage className="text-xs leading-2.5" />
    </div>
  );
}
