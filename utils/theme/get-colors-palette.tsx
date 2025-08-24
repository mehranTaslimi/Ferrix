import { defaultPresets } from "./theme-presets";

export function getColorPalette(presetKey: string | undefined) {
  const themeKey =
    typeof window !== "undefined" ? localStorage.getItem("theme") : "light";

  if (!presetKey) {
    return null;
  }

  const preset = defaultPresets[presetKey];
  const currentThemeStyles = themeKey === "dark" ? preset.styles.dark : preset.styles.light;

  const colors = [
    currentThemeStyles["primary"],
    currentThemeStyles["secondary"],
    currentThemeStyles["accent"],
  ].filter(Boolean) as string[];

  return (
    <div className="flex items-center gap-1">
      {colors.map((color, index) => (
        <span
          key={index}
          className="h-2 w-2 rounded-full"
          style={{ backgroundColor: color }}
        />
      ))}
    </div>
  );
}