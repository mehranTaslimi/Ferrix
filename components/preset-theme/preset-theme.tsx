'use client';

import clsx from 'clsx';
import { Search, Check } from 'lucide-react';
import { useState, useRef, useEffect } from 'react';

import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';

import { Button } from '../ui/button';
import { Input } from '../ui/input';

import { presetCollection } from './preset-collection';
import { useThemePreset } from './preset-theme-context';

export function PresetThemes() {
  const { preset, tempPreset, addTempPreset, removeTempPreset, applyTempPreset } = useThemePreset();
  const [search, setSearch] = useState('');
  const [isOpen, setIsOpen] = useState(false);
  const activePresetRef = useRef<HTMLDivElement>(null);

  const filteredPresets = Object.entries(presetCollection).filter(([_, value]) =>
    value.label?.toLowerCase().includes(search.toLowerCase()),
  );

  useEffect(() => {
    if (isOpen) {
      const timer = setTimeout(() => {
        activePresetRef.current?.scrollIntoView({
          behavior: 'smooth',
          block: 'center',
        });
      }, 150);
      return () => clearTimeout(timer);
    }
  }, [isOpen, search]);

  return (
    <Dialog
      open={isOpen}
      onOpenChange={(open) => {
        setIsOpen(open);
        if (!open) removeTempPreset();
      }}
    >
      <form>
        <DialogTrigger asChild>
          <Button variant="outline">Preset</Button>
        </DialogTrigger>
        <DialogContent className="flex h-[500px] flex-col justify-between gap-4 sm:max-w-[425px]">
          <div>
            <DialogHeader>
              <DialogTitle>Change your preset</DialogTitle>
            </DialogHeader>
            <div className="mt-4 grid gap-4">
              <div className="focus-within:ring-ring flex items-center gap-2 rounded-lg border px-2 transition focus-within:ring-2">
                <Search className="text-muted-foreground h-4 w-4" />
                <Input
                  className="border-0 bg-transparent! px-0 focus-visible:ring-0"
                  id="search_presets"
                  name="search_presets"
                  value={search}
                  onChange={(e) => setSearch(e.target.value)}
                  placeholder="Search presets..."
                />
              </div>
              <div className="h-[320px] space-y-2 overflow-y-auto">
                {filteredPresets.map(([key, value]) => (
                  <div
                    key={key}
                    ref={(ref) => {
                      if (key === preset && ref) {
                        activePresetRef.current = ref;
                      }
                    }}
                    className={clsx(
                      'hover:bg-muted flex items-center justify-between rounded-lg p-2',
                      {
                        'bg-muted': tempPreset === key,
                      },
                    )}
                    onClick={() => addTempPreset(key)}
                    onKeyDown={(e) => {
                      if (e.key === 'Enter' || e.key === ' ') {
                        addTempPreset(key);
                      }
                    }}
                    role="button"
                    tabIndex={0}
                  >
                    <div className="flex items-center gap-4">
                      {getColorPalette(key)}
                      <span aria-label={value.label}>{value.label}</span>
                    </div>
                    {preset === key && <Check className="text-muted-foreground h-4 w-4" />}
                  </div>
                ))}
              </div>
            </div>
          </div>
          <DialogFooter>
            <DialogClose asChild>
              <Button variant="outline" onClick={removeTempPreset}>
                Cancel
              </Button>
            </DialogClose>
            <DialogClose asChild>
              <Button type="submit" onClick={() => applyTempPreset(tempPreset)}>
                Save changes
              </Button>
            </DialogClose>
          </DialogFooter>
        </DialogContent>
      </form>
    </Dialog>
  );
}

function getColorPalette(presetKey: string | undefined) {
  const themeKey = typeof window !== 'undefined' ? localStorage.getItem('theme') : 'light';

  if (!presetKey) {
    return null;
  }

  const preset = presetCollection[presetKey];
  const currentThemeStyles = themeKey === 'dark' ? preset.styles.dark : preset.styles.light;

  const colors = [
    currentThemeStyles['primary'],
    currentThemeStyles['secondary'],
    currentThemeStyles['accent'],
  ].filter(Boolean) as string[];

  return (
    <div className="flex items-center gap-2">
      {colors.map((color, index) => (
        <span
          key={index}
          className="h-2 w-2 rounded-full ring-1 ring-black/40 ring-offset-1"
          style={{ backgroundColor: color }}
        />
      ))}
    </div>
  );
}
