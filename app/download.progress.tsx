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
    <div className="space-y-6 w-full max-w-md">
      <div className="space-y-2">
        <div className="flex justify-between">
          <h3 className="font-medium">Download Progress</h3>
          <span>{total?.toFixed(2) ?? 0}%</span>
        </div>
        <Progress value={total ?? 0} className="h-2" />
      </div>
      <div className="space-y-3">
        {Object.entries(progress).map(([c, p]) => (
          <div className="flex items-center space-x-3" key={c}>
            <div className="w-6 h-6 flex items-center justify-center">
              {p < 100 ? (
                <Loader2 className="h-4 w-4 animate-spin text-blue-500" />
              ) : (
                <Check className="h-4 w-4 text-green-500" />
              )}
            </div>
            <div className="flex-1">
              <div className="flex justify-between">
                <span className="text-sm">Chunk {c}</span>
                <span className="text-sm text-gray-500">{p}%</span>
              </div>
              <Progress value={p} className="h-1" />
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
