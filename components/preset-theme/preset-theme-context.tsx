'use client';

import { createContext, useContext, useState, useMemo, useLayoutEffect, useCallback } from 'react';

interface CustomThemeContextType {
  preset: string;
  setPreset: (preset: string) => void;
  tempPreset: string;
  removeTempPreset: () => void;
  addTempPreset: (preset: string) => void;
  applyTempPreset: (preset: string) => void;
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

export function PresetsThemeProvider({ children }: { children: React.ReactNode }) {
  const [preset, setPreset] = useState(() => {
    if (typeof window === 'undefined') {
      return 'default';
    }
    return localStorage.getItem('theme-preset') || 'default';
  });
  const [tempPreset, setTempPreset] = useState('');

  const handlePresetChange = useCallback((preset: string) => {
    const showDocument = () => {
      requestAnimationFrame(() => {
        document.documentElement.style.visibility = 'visible';
      });
    };

    const newLink = document.createElement('link');
    newLink.rel = 'stylesheet';
    newLink.href = `/themes/${preset}.css`;

    newLink.onload = () => {
      const oldLink = document.getElementById(styleTagId);
      if (oldLink) {
        oldLink.remove();
      }
      newLink.id = styleTagId;
      showDocument();
    };

    newLink.onerror = () => {
      newLink.remove();
      showDocument();
    };

    document.head.appendChild(newLink);

    localStorage.setItem('theme-preset', preset);
  }, []);

  const applyTempPreset = useCallback((preset: string) => {
    setPreset(preset);
  }, []);

  const addTempPreset = useCallback((preset: string) => {
    if (!preset) {
      return;
    }

    const existingLink = document.getElementById(tempStyleTagId);

    if (!existingLink) {
      const link = document.createElement('link');
      link.id = tempStyleTagId;
      link.rel = 'stylesheet';
      link.href = `/themes/${preset}.css`;
      document.head.appendChild(link);
    } else {
      existingLink.setAttribute('href', `/themes/${preset}.css`);
    }

    setTempPreset(preset);
  }, []);

  const removeTempPreset = useCallback(() => {
    const styleTag = document.getElementById(tempStyleTagId);

    if (styleTag) {
      styleTag.remove();
    }
    setTempPreset('');
  }, []);

  useLayoutEffect(() => {
    handlePresetChange(preset);
  }, [preset, handlePresetChange]);

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
