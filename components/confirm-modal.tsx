import { useState } from "react";
import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import { Checkbox } from "@/components/ui/checkbox";

export function RemoveDownloadDialog({
    open,
    onOpenChange,
    onConfirm,
    filename,
    fileExist,
}: {
    open: boolean;
    onOpenChange: (open: boolean) => void;
    onConfirm: (removeFile: boolean) => void;
    filename: string;
    fileExist: boolean;
}) {
    const [removeFileChecked, setRemoveFileChecked] = useState(false);

    return (
        <AlertDialog open={open} onOpenChange={onOpenChange}>
            <AlertDialogContent>
                <AlertDialogHeader>
                    <AlertDialogTitle>Remove download?</AlertDialogTitle>
                    <AlertDialogDescription>
                        This will remove the download entry
                        {fileExist && " and optionally delete the file from your disk"}.
                    </AlertDialogDescription>
                </AlertDialogHeader>

                {fileExist && (
                    <div className="flex items-center space-x-2 mt-4">
                        <Checkbox
                            id="remove-file"
                            checked={removeFileChecked}
                            onCheckedChange={(checked) => setRemoveFileChecked(!!checked)}
                        />
                        <label htmlFor="remove-file" className="text-sm">
                            Also delete <b>{filename}</b> from disk
                        </label>
                    </div>
                )}

                <AlertDialogFooter>
                    <AlertDialogCancel>Cancel</AlertDialogCancel>
                    <AlertDialogAction
                        onClick={() => onConfirm(removeFileChecked)}
                    >
                        Remove
                    </AlertDialogAction>
                </AlertDialogFooter>
            </AlertDialogContent>
        </AlertDialog>
    );
}
