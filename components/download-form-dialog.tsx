import React from "react";
import { Input } from "@/components/ui/input";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
} from "@/components/ui/form";
import { Button } from "@/components/ui/button";
import { Download } from "lucide-react";

import { useForm } from "react-hook-form";

interface DownLoadFormDialogProps {
  isOpen: boolean;
  setIsDialogOpen: (open: boolean) => void;
  onAddDownload: (url: string, chunkCount: number) => Promise<void>;
}
export const DownLoadFormDialog = ({
  isOpen,
  setIsDialogOpen,
  onAddDownload,
}: DownLoadFormDialogProps) => {
  const form = useForm({
    defaultValues: {
      downloadUrl:
        "https://wallpaperswide.com/download/most_beautiful_mountain_scenery-wallpaper-3840x2160.jpg",
      chunkCount: 6,
    },
  });
  async function onSubmitForm(values: {
    downloadUrl: string;
    chunkCount: number;
  }) {
    try {
      await onAddDownload(values.downloadUrl, values.chunkCount);
      form.reset({ downloadUrl: "", chunkCount: 6 });
      setIsDialogOpen(false);
    } catch (e) {
      console.log(e);
    }
  }
  return (
    <Dialog open={isOpen} onOpenChange={setIsDialogOpen}>
      <DialogTrigger asChild>
        <Button>
          <Download className="w-4 h-4 mr-2" /> Add Download
        </Button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-[425px] frosted-glass">
        <DialogHeader>
          <DialogTitle>Add New Download</DialogTitle>
          <DialogDescription>
            Enter the download URL and configure the number of chunks for
            parallel downloading.
          </DialogDescription>
        </DialogHeader>
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmitForm)}
            className="space-y-4"
          >
            <FormField
              control={form.control}
              name="downloadUrl"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Download URL</FormLabel>
                  <FormControl>
                    <Input placeholder="Enter download URL" {...field} />
                  </FormControl>
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="chunkCount"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Chunk Count</FormLabel>
                  <FormControl>
                    <Input
                      type="number"
                      min="1"
                      max="32"
                      placeholder="6"
                      {...field}
                      onChange={(e) =>
                        field.onChange(Number.parseInt(e.target.value) || 6)
                      }
                    />
                  </FormControl>
                  <p className="text-xs text-muted-foreground">
                    Number of parallel chunks for download (1-32). Higher values
                    may increase speed but use more resources.
                  </p>
                </FormItem>
              )}
            />

            <DialogFooter>
              <Button
                type="button"
                variant="outline"
                onClick={() => setIsDialogOpen(false)}
              >
                Cancel
              </Button>
              <Button type="submit">
                <Download className="w-4 h-4 mr-2" /> Start Download
              </Button>
            </DialogFooter>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  );
};
