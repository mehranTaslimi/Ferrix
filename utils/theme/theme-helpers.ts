import { ThemePreset } from "@/lib/validation/theme";

/**
 * Generates a CSS string of custom properties from a theme preset.
 * This string can be injected into a <style> tag to apply the theme.
 *
 * @param preset - The theme preset containing light and dark styles.
 * @returns A string of CSS variables.
 */
export function themePresetToCss(preset: ThemePreset) {
  const { light, dark } = preset.styles;

  const lightVars = Object.entries(light)
    .map(([key, value]) => `  --${key}: ${value};`)
    .join("\n");

  const darkVars = Object.entries(dark)
    .map(([key, value]) => `  --${key}: ${value};`)
    .join("\n");
  
  return `
    :root {
${lightVars}
    }
    .dark {
${darkVars}
    }
  `;
}

