import { Card, CardContent } from "@/components/ui/card";
import { motion } from "framer-motion";

export function EmptyDownloadsIntro() {
    return (
        <Card className="overflow-hidden border-0 bg-transparent">
            <CardContent className="py-14">
                <div className="flex flex-col items-center text-center px-6">

                    <div className="relative mb-6">
                        <div className="h-16 w-16 rounded-2xl bg-white/5 ring-1 ring-white/10 backdrop-blur-sm grid place-items-center">
                            <img
                                src="/logo.png"
                                alt="Ferrix"
                                className="h-12 w-12 opacity-90"
                            />
                        </div>
                    </div>

                    <motion.h3
                        className="text-xl font-semibold tracking-tight"
                        initial={{ opacity: 0, y: 10 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ duration: 0.6, ease: "easeOut" }}
                    >
                        <motion.span
                            className="bg-clip-text text-transparent bg-gradient-to-r from-white to-white/60 inline-block"
                            initial={{ backgroundPositionX: "0%" }}
                            animate={{ backgroundPositionX: "100%" }}
                            transition={{
                                duration: 1.2,
                                ease: "easeInOut",
                                repeat: Infinity,
                                repeatType: "reverse"
                            }}
                            style={{
                                backgroundSize: "200% 100%",
                            }}
                        >
                            Meet Ferrix
                        </motion.span>
                    </motion.h3>


                    <p className="mt-2 max-w-prose text-sm text-muted-foreground">
                        A fast, reliable download manager built with Rust · smart retries, precise
                        control, and a clean desktop experience.
                    </p>

                    <div className="mt-5 flex flex-wrap items-center justify-center gap-2 text-[11px]">
                        <span className="rounded-full border bg-white/5 px-2.5 py-1 text-white/80">
                            Multi-chunk engine
                        </span>
                        <span className="rounded-full border bg-white/5 px-2.5 py-1 text-white/80">
                            Resume & verify
                        </span>
                        <span className="rounded-full border bg-white/5 px-2.5 py-1 text-white/80">
                            Smart backoff
                        </span>
                        <span className="rounded-full border bg-white/5 px-2.5 py-1 text-white/80">
                            Native notifications
                        </span>
                    </div>

                    <div className="mt-8 h-px w-24 bg-gradient-to-r from-transparent via-white/20 to-transparent" />

                    <p className="mt-3 text-xs text-muted-foreground">
                        Start a download to see live charts, speeds, and progress.
                    </p>
                </div>
            </CardContent>
        </Card>
    );
}
