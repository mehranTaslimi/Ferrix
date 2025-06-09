"use client";
import DownloadForm from "@/app/download.form";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24">
      <div className="w-1/2">
        <DownloadForm />
      </div>
    </main>
  );
}
