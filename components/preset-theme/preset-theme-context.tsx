'use client';

import { type } from '@tauri-apps/plugin-os';
import {
  createContext,
  useContext,
  useState,
  useMemo,
  useLayoutEffect,
  useCallback,
  useRef,
} from 'react';

import { loadTheme, type ThemeName, themeNames } from './themes/index';

import type { ThemePreset } from '@/lib/validation';

interface CustomThemeContextType {
  preset: ThemeName;
  setPreset: (preset: ThemeName) => void;
  tempPreset: ThemeName | null;
  removeTempPreset: () => void;
  addTempPreset: (preset: ThemeName) => void;
  applyTempPreset: (preset: ThemeName) => void;
}

const CustomThemeContext = createContext<CustomThemeContextType | undefined>(undefined);

export function useThemePreset() {
  const context = useContext(CustomThemeContext);
  if (context === undefined) {
    throw new Error('useThemePreset must be used within a ThemeProvider');
  }
  return context;
}

const styleTagId = 'dynamic-theme-preset-style';
const tempStyleTagId = 'temp-theme-preset-style';

function applyCss(css: string, tagId: string) {
  let styleTag = document.getElementById(tagId) as HTMLStyleElement | null;

  if (!styleTag) {
    styleTag = document.createElement('style');
    styleTag.id = tagId;
    styleTag.textContent = css;
    document.head.appendChild(styleTag);
  } else {
    styleTag.textContent = css;
  }
}

export function PresetsThemeProvider({ children }: { children: React.ReactNode }) {
  const getLocalPreset = (): ThemeName => {
    if (typeof window === 'undefined') {
      return 'default';
    }
    const storedPreset = localStorage.getItem('theme-preset');
    if (storedPreset && themeNames.includes(storedPreset as ThemeName)) {
      return storedPreset as ThemeName;
    }
    return 'default';
  };
  const [preset, setPreset] = useState<ThemeName>(getLocalPreset);
  const [tempPreset, setTempPreset] = useState<ThemeName | null>(null);
  const latestRequest = useRef<string | null>(null);

  useLayoutEffect(() => {
    const currentPreset = preset;
    latestRequest.current = currentPreset;

    loadTheme(currentPreset)
      .then((theme) => {
        if (latestRequest.current === currentPreset) {
          const css = themePresetToCss(theme);
          applyCss(css, styleTagId);
        }
      })
      .catch((error) => {
        console.error(`Failed to load theme: ${currentPreset}`, error);
      })
      .finally(() => {
        if (latestRequest.current === currentPreset) {
          document.documentElement.style.visibility = 'visible';
        }
      });
  }, [preset]);

  const removeTempPreset = useCallback(() => {
    const styleTag = document.getElementById(tempStyleTagId);
    if (styleTag) {
      styleTag.remove();
    }
    setTempPreset(null);
  }, []);

  const applyTempPreset = useCallback(
    (newPreset: ThemeName) => {
      if (!newPreset) return;
      localStorage.setItem('theme-preset', newPreset);
      setPreset(newPreset);
      removeTempPreset();
    },
    [removeTempPreset],
  );

  const addTempPreset = useCallback(async (presetName: ThemeName) => {
    if (!presetName || presetName === tempPreset) return;
    setTempPreset(presetName);
    latestRequest.current = presetName;

    try {
      const theme = await loadTheme(presetName);
      if (latestRequest.current === presetName) {
        applyCss(themePresetToCss(theme), tempStyleTagId);
      }
    } catch (error) {
      console.error(`Failed to load temporary theme: ${presetName}`, error);
    }
  }, []);

  const value = useMemo(
    () => ({
      preset,
      setPreset,
      tempPreset,
      removeTempPreset,
      addTempPreset,
      applyTempPreset,
    }),
    [preset, tempPreset, removeTempPreset, addTempPreset, applyTempPreset],
  );

  return <CustomThemeContext.Provider value={value}>{children}</CustomThemeContext.Provider>;
}

export function themePresetToCss(preset: ThemePreset) {
  const { light, dark } = preset.styles;
  const isMacos = type() === 'macos';

  const lightVars = Object.entries(light)
    .map(([key, value]) => {
      if (key === 'sidebar' && isMacos) {
        return `  --${key}: transparent;`;
      }
      return `  --${key}: ${value};`;
    })
    .join('\n');

  const darkVars = Object.entries(dark)
    .map(([key, value]) => {
      if (key === 'sidebar' && isMacos) {
        return `  --${key}: transparent;`;
      }
      return `  --${key}: ${value};`;
    })
    .join('\n');

  return `
    :root {
${lightVars}
    }
    .dark {
${darkVars}
    }
  `;
}
