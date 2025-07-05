import { cn } from "@/lib/utils";
import { Loader2Icon } from "lucide-react";

interface LoadingProps {
  className?: string;
}
function Loading({ className }: LoadingProps) {
  return <Loader2Icon className={cn("animate-spin", className)} />;
}

export { Loading };
