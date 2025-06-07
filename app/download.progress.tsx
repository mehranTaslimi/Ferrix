import { Progress } from "@/components/ui/progress";
import { Check, Loader2 } from "lucide-react";

export default function DownloadProgress({
  progress,
}: {
  progress: Record<number, number>;
}) {
  const total =
    Object.values(progress).reduce((prev, curr) => prev + curr, 0) /
    Object.keys(progress).length;

  return (
    <div>
      <Progress className="h-1" value={50} />
    </div>
  );
}
