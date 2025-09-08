import { type ThemePreset } from '@/lib/validation';

export const themeNames = [
  'amber-minimal',
  'amethyst-haze',
  'bold-tech',
  'bubblegum',
  'caffeine',
  'candyland',
  'catppuccin',
  'claude',
  'claymorphism',
  'clean-slate',
  'cosmic-night',
  'cyberpunk',
  'darkmatter',
  'doom-64',
  'elegant-luxury',
  'graphite',
  'kodama-grove',
  'midnight-bloom',
  'mocha-mousse',
  'modern-minimal',
  'mono',
  'nature',
  'neo-brutalism',
  'northern-lights',
  'notebook',
  'ocean-breeze',
  'pastel-dreams',
  'perpetuity',
  'quantum-rose',
  'retro-arcade',
  'soft-pop',
  'solar-dusk',
  'starry-night',
  'sunset-horizon',
  'supabase',
  't3-chat',
  'tangerine',
  'twitter',
  'vercel',
  'vintage-paper',
  'violet-bloom',
  'default',
] as const;

export type ThemeName = (typeof themeNames)[number];

export const loadTheme = (name: ThemeName): Promise<ThemePreset> => {
  switch (name) {
    case 'default':
      return import('./default').then((m) => m.defaultTheme);
    case 'amber-minimal':
      return import('./amber-minimal').then((m) => m.amberMinimal);
    case 'amethyst-haze':
      return import('./amethyst-haze').then((m) => m.amethystHaze);
    case 'bold-tech':
      return import('./bold-tech').then((m) => m.boldTech);
    case 'bubblegum':
      return import('./bubblegum').then((m) => m.bubblegum);
    case 'caffeine':
      return import('./caffeine').then((m) => m.caffeine);
    case 'candyland':
      return import('./candyland').then((m) => m.candyLand);
    case 'catppuccin':
      return import('./catppuccin').then((m) => m.catppuccin);
    case 'claude':
      return import('./claude').then((m) => m.claude);
    case 'claymorphism':
      return import('./claymorphism').then((m) => m.claymorphism);
    case 'clean-slate':
      return import('./clean-slate').then((m) => m.cleanSlate);
    case 'cosmic-night':
      return import('./cosmic-night').then((m) => m.cosmicNight);
    case 'cyberpunk':
      return import('./cyberpunk').then((m) => m.cyberpunk);
    case 'darkmatter':
      return import('./darkmatter').then((m) => m.darkMatter);
    case 'doom-64':
      return import('./doom-64').then((m) => m.doom64);
    case 'elegant-luxury':
      return import('./elegant-luxury').then((m) => m.elegantLuxury);
    case 'graphite':
      return import('./graphite').then((m) => m.graphite);
    case 'kodama-grove':
      return import('./kodama-grove').then((m) => m.kodamaGrove);
    case 'midnight-bloom':
      return import('./midnight-bloom').then((m) => m.midnightBloom);
    case 'mocha-mousse':
      return import('./mocha-mousse').then((m) => m.mochaMousse);
    case 'modern-minimal':
      return import('./modern-minimal').then((m) => m.modernMinimal);
    case 'mono':
      return import('./mono').then((m) => m.mono);
    case 'nature':
      return import('./nature').then((m) => m.nature);
    case 'neo-brutalism':
      return import('./neo-brutalism').then((m) => m.neoBrutalism);
    case 'northern-lights':
      return import('./northern-lights').then((m) => m.northernLights);
    case 'notebook':
      return import('./notebook').then((m) => m.notebook);
    case 'ocean-breeze':
      return import('./ocean-breeze').then((m) => m.oceanBreeze);
    case 'pastel-dreams':
      return import('./pastel-dreams').then((m) => m.pastelDreams);
    case 'perpetuity':
      return import('./perpetuity').then((m) => m.perpetuity);
    case 'quantum-rose':
      return import('./quantum-rose').then((m) => m.quantumRose);
    case 'retro-arcade':
      return import('./retro-arcade').then((m) => m.retroArcade);
    case 'soft-pop':
      return import('./soft-pop').then((m) => m.softPop);
    case 'solar-dusk':
      return import('./solar-dusk').then((m) => m.solarDusk);
    case 'starry-night':
      return import('./starry-night').then((m) => m.starryNight);
    case 'sunset-horizon':
      return import('./sunset-horizon').then((m) => m.sunsetHorizon);
    case 'supabase':
      return import('./supabase').then((m) => m.supabase);
    case 't3-chat':
      return import('./t3-chat').then((m) => m.t3Chat);
    case 'tangerine':
      return import('./tangerine').then((m) => m.tangerine);
    case 'twitter':
      return import('./twitter').then((m) => m.twitter);
    case 'vercel':
      return import('./vercel').then((m) => m.vercel);
    case 'vintage-paper':
      return import('./vintage-paper').then((m) => m.vintagePaper);
    case 'violet-bloom':
      return import('./violet-bloom').then((m) => m.violetBloom);
    default: {
      const exhaustiveCheck: never = name;
      throw new Error(`Unhandled theme: ${exhaustiveCheck}`);
    }
  }
};
