import { platform as getPlatform } from "@tauri-apps/plugin-os";

export type OS = "windows" | "macos" | "linux" | "unknown";

let cachedOS: OS | null = null;


export async function getCurrentOS(): Promise<OS> {
    if (cachedOS) {
        return cachedOS;
    }

    try {
        const p = await getPlatform();
        if (p === "windows" || p === "macos" || p === "linux") {
            cachedOS = p;
            return p;
        }
        cachedOS = "unknown";
        return "unknown";
    } catch {
        cachedOS = "unknown";
        return "unknown";
    }
}


export async function isLinux(): Promise<boolean> {
    const os = await getCurrentOS();
    return os === "linux";
}


export async function isWindows(): Promise<boolean> {
    const os = await getCurrentOS();
    return os === "windows";
}


export async function isMacOS(): Promise<boolean> {
    const os = await getCurrentOS();
    return os === "macos";
}


export async function shouldApplyBackdropBlur(): Promise<boolean> {
    const os = await getCurrentOS();
    return os !== "linux";
}
