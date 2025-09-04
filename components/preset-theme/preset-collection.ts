import type { ThemePreset } from '@/lib/validation';

export const presetCollection: Record<string, ThemePreset> = {
  default: {
    label: 'Default',
    styles: {
      light: {
        primary: 'oklch(0.205 0 0)',
        secondary: 'oklch(0.97 0 0)',
        accent: 'oklch(0.97 0 0)',
      },
      dark: {
        primary: 'oklch(0.985 0 0)',
        secondary: 'oklch(0.269 0 0)',
        accent: 'oklch(0.269 0 0)',
      },
    },
  },
  'modern-minimal': {
    label: 'Modern Minimal',
    styles: {
      light: {
        primary: '#3b82f6',
        secondary: '#f3f4f6',
        accent: '#e0f2fe',
      },
      dark: {
        primary: '#3b82f6',
        secondary: '#262626',
        accent: '#1e3a8a',
      },
    },
  },

  'violet-bloom': {
    label: 'Violet Bloom',
    createdAt: '2025-06-26',
    styles: {
      light: {
        primary: '#7033ff',
        secondary: '#edf0f4',
        accent: '#e2ebff',
      },
      dark: {
        primary: '#8c5cff',
        secondary: '#2a2c33',
        accent: '#1e293b',
      },
    },
  },

  't3-chat': {
    label: 'T3 Chat',
    createdAt: '2025-04-19',
    styles: {
      light: {
        primary: '#a84370',
        secondary: '#f1c4e6',
        accent: '#f1c4e6',
      },
      dark: {
        primary: '#a3004c',
        secondary: '#362d3d',
        accent: '#463753',
      },
    },
  },

  twitter: {
    label: 'Twitter',
    createdAt: '2025-04-24',
    styles: {
      light: {
        primary: '#1e9df1',
        secondary: '#0f1419',
        accent: '#E3ECF6',
      },
      dark: {
        primary: '#1c9cf0',
        secondary: '#f0f3f4',
        accent: '#061622',
      },
    },
  },

  'mocha-mousse': {
    label: 'Mocha Mousse',
    createdAt: '2025-04-24',
    styles: {
      light: {
        primary: '#A37764',
        secondary: '#BAAB92',
        accent: '#E4C7B8',
      },
      dark: {
        primary: '#C39E88',
        secondary: '#8A655A',
        accent: '#BAAB92',
      },
    },
  },

  bubblegum: {
    label: 'Bubblegum',
    createdAt: '2025-04-18',
    styles: {
      light: {
        primary: '#d04f99',
        secondary: '#8acfd1',
        accent: '#fbe2a7',
      },
      dark: {
        primary: '#fbe2a7',
        secondary: '#e4a2b1',
        accent: '#c67b96',
      },
    },
  },

  'amethyst-haze': {
    label: 'Amethyst Haze',
    createdAt: '2025-05-08',
    styles: {
      light: {
        primary: '#8a79ab',
        secondary: '#dfd9ec',
        accent: '#e6a5b8',
      },
      dark: {
        primary: '#a995c9',
        secondary: '#5a5370',
        accent: '#372e3f',
      },
    },
  },

  notebook: {
    label: 'Notebook',
    createdAt: '2025-05-10',
    styles: {
      light: {
        primary: '#606060',
        secondary: '#dedede',
        accent: '#f3eac8',
      },
      dark: {
        primary: '#b0b0b0',
        secondary: '#5a5a5a',
        accent: '#e0e0e0',
      },
    },
  },

  'doom-64': {
    label: 'Doom 64',
    createdAt: '2025-04-28',
    styles: {
      light: {
        primary: '#b71c1c',
        secondary: '#556b2f',
        accent: '#4682b4',
      },
      dark: {
        primary: '#e53935',
        secondary: '#689f38',
        accent: '#64b5f6',
      },
    },
  },

  catppuccin: {
    label: 'Catppuccin',
    createdAt: '2025-04-18',
    styles: {
      light: {
        primary: '#8839ef',
        secondary: '#ccd0da',
        accent: '#04a5e5',
      },
      dark: {
        primary: '#cba6f7',
        secondary: '#585b70',
        accent: '#89dceb',
      },
    },
  },

  graphite: {
    label: 'Graphite',
    createdAt: '2025-04-17',
    styles: {
      light: {
        primary: '#606060',
        secondary: '#e0e0e0',
        accent: '#c0c0c0',
      },
      dark: {
        primary: '#a0a0a0',
        secondary: '#303030',
        accent: '#404040',
      },
    },
  },

  perpetuity: {
    label: 'Perpetuity',
    createdAt: '2025-04-01',
    styles: {
      light: {
        primary: '#06858e',
        secondary: '#d9eaea',
        accent: '#c9e5e7',
      },
      dark: {
        primary: '#4de8e8',
        secondary: '#164955',
        accent: '#164955',
      },
    },
  },
  'kodama-grove': {
    label: 'Kodama Grove',
    styles: {
      light: {
        primary: '#8d9d4f',
        secondary: '#decea0',
        accent: '#dbc894',
      },
      dark: {
        primary: '#8a9f7b',
        secondary: '#5a5345',
        accent: '#a18f5c',
      },
    },
  },

  'cosmic-night': {
    label: 'Cosmic Night',
    createdAt: '2025-04-04',
    styles: {
      light: {
        primary: '#6e56cf',
        secondary: '#e4dfff',
        accent: '#d8e6ff',
      },
      dark: {
        primary: '#a48fff',
        secondary: '#2d2b55',
        accent: '#303060',
      },
    },
  },

  tangerine: {
    label: 'Tangerine',
    createdAt: '2025-04-09',
    styles: {
      light: {
        primary: '#e05d38',
        secondary: '#f3f4f6',
        accent: '#d6e4f0',
      },
      dark: {
        primary: '#e05d38',
        secondary: '#2a303e',
        accent: '#2a3656',
      },
    },
  },

  'quantum-rose': {
    label: 'Quantum Rose',
    createdAt: '2025-04-03',
    styles: {
      light: {
        primary: '#e6067a',
        secondary: '#ffd6ff',
        accent: '#ffc1e3',
      },
      dark: {
        primary: '#ff6bef',
        secondary: '#46204f',
        accent: '#5a1f5d',
      },
    },
  },

  nature: {
    label: 'Nature',
    styles: {
      light: {
        primary: '#2e7d32',
        secondary: '#e8f5e9',
        accent: '#c8e6c9',
      },
      dark: {
        primary: '#4caf50',
        secondary: '#3e4a3d',
        accent: '#388e3c',
      },
    },
  },

  'bold-tech': {
    label: 'Bold Tech',
    styles: {
      light: {
        primary: '#8b5cf6',
        secondary: '#f3f0ff',
        accent: '#dbeafe',
      },
      dark: {
        primary: '#8b5cf6',
        secondary: '#1e1b4b',
        accent: '#4338ca',
      },
    },
  },

  'elegant-luxury': {
    label: 'Elegant Luxury',
    styles: {
      light: {
        primary: '#9b2c2c',
        secondary: '#fdf2d6',
        accent: '#fef3c7',
      },
      dark: {
        primary: '#b91c1c',
        secondary: '#92400e',
        accent: '#b45309',
      },
    },
  },

  'amber-minimal': {
    label: 'Amber Minimal',
    createdAt: '2025-04-27',
    styles: {
      light: {
        primary: '#f59e0b',
        secondary: '#f3f4f6',
        accent: '#fffbeb',
      },
      dark: {
        primary: '#f59e0b',
        secondary: '#262626',
        accent: '#92400e',
      },
    },
  },

  supabase: {
    label: 'Supabase',
    createdAt: '2025-04-27',
    styles: {
      light: {
        primary: '#72e3ad',
        secondary: '#fdfdfd',
        accent: '#ededed',
      },
      dark: {
        primary: '#006239',
        secondary: '#242424',
        accent: '#313131',
      },
    },
  },

  'neo-brutalism': {
    label: 'Neo Brutalism',
    styles: {
      light: {
        primary: '#ff3333',
        secondary: '#ffff00',
        accent: '#0066ff',
      },
      dark: {
        primary: '#ff6666',
        secondary: '#ffff33',
        accent: '#3399ff',
      },
    },
  },

  'solar-dusk': {
    label: 'Solar Dusk',
    createdAt: '2025-04-12',
    styles: {
      light: {
        primary: '#B45309',
        secondary: '#E4C090',
        accent: '#f2daba',
      },
      dark: {
        primary: '#F97316',
        secondary: '#57534E',
        accent: '#1e4252',
      },
    },
  },

  claymorphism: {
    label: 'Claymorphism',
    styles: {
      light: {
        primary: '#6366f1',
        secondary: '#d6d3d1',
        accent: '#f3e5f5',
      },
      dark: {
        primary: '#818cf8',
        secondary: '#3a3633',
        accent: '#484441',
      },
    },
  },

  cyberpunk: {
    label: 'Cyberpunk',
    styles: {
      light: {
        primary: '#ff00c8',
        secondary: '#f0f0ff',
        accent: '#00ffcc',
      },
      dark: {
        primary: '#ff00c8',
        secondary: '#1e1e3f',
        accent: '#00ffcc',
      },
    },
  },
  'pastel-dreams': {
    label: 'Pastel Dreams',
    styles: {
      light: {
        primary: '#a78bfa',
        secondary: '#e9d8fd',
        accent: '#f3e5f5',
      },
      dark: {
        primary: '#c0aafd',
        secondary: '#3f324a',
        accent: '#4a3d5a',
      },
    },
  },

  'clean-slate': {
    label: 'Clean Slate',
    styles: {
      light: {
        primary: '#6366f1',
        secondary: '#e5e7eb',
        accent: '#e0e7ff',
      },
      dark: {
        primary: '#818cf8',
        secondary: '#2d3748',
        accent: '#374151',
      },
    },
  },

  caffeine: {
    label: 'Caffeine',
    styles: {
      light: {
        primary: '#644a40',
        secondary: '#ffdfb5',
        accent: '#e8e8e8',
      },
      dark: {
        primary: '#ffe0c2',
        secondary: '#393028',
        accent: '#2a2a2a',
      },
    },
  },
  'ocean-breeze': {
    label: 'Ocean Breeze',
    styles: {
      light: {
        primary: '#22c55e',
        secondary: '#e0f2fe',
        accent: '#d1fae5',
      },
      dark: {
        primary: '#34d399',
        secondary: '#2d3748',
        accent: '#374151',
      },
    },
  },
  'retro-arcade': {
    label: 'Retro Arcade',
    styles: {
      light: {
        primary: '#d33682',
        secondary: '#2aa198',
        accent: '#cb4b16',
      },
      dark: {
        primary: '#d33682',
        secondary: '#2aa198',
        accent: '#cb4b16',
      },
    },
  },

  'midnight-bloom': {
    label: 'Midnight Bloom',
    styles: {
      light: {
        primary: '#6c5ce7',
        secondary: '#a1c9f2',
        accent: '#8b9467',
      },
      dark: {
        primary: '#6c5ce7',
        secondary: '#4b0082',
        accent: '#6495ed',
      },
    },
  },
  candyland: {
    label: 'Candyland',
    styles: {
      light: {
        primary: '#ffc0cb',
        secondary: '#87ceeb',
        accent: '#ffff00',
      },
      dark: {
        primary: '#ff99cc',
        secondary: '#33cc33',
        accent: '#87ceeb',
      },
    },
  },
  'northern-lights': {
    label: 'Northern Lights',
    styles: {
      light: {
        primary: '#34a85a',
        secondary: '#6495ed',
        accent: '#66d9ef',
      },
      dark: {
        primary: '#34a85a',
        secondary: '#4682b4',
        accent: '#6495ed',
      },
    },
  },
  'vintage-paper': {
    label: 'Vintage Paper',
    styles: {
      light: {
        primary: '#a67c52',
        secondary: '#e2d8c3',
        accent: '#d4c8aa',
      },
      dark: {
        primary: '#c0a080',
        secondary: '#4a4039',
        accent: '#59493e',
      },
    },
  },
  'sunset-horizon': {
    label: 'Sunset Horizon',
    styles: {
      light: {
        primary: '#ff7e5f',
        secondary: '#ffedea',
        accent: '#feb47b',
      },
      dark: {
        primary: '#ff7e5f',
        secondary: '#463a41',
        accent: '#feb47b',
      },
    },
  },

  'starry-night': {
    label: 'Starry Night',
    createdAt: '2025-04-16',
    styles: {
      light: {
        primary: '#3a5ba0',
        secondary: '#f7c873',
        accent: '#6ea3c1',
      },
      dark: {
        primary: '#3a5ba0',
        secondary: '#ffe066',
        accent: '#bccdf0',
      },
    },
  },

  claude: {
    label: 'Claude',
    styles: {
      light: {
        primary: '#c96442',
        secondary: '#e9e6dc',
        accent: '#e9e6dc',
      },
      dark: {
        primary: '#d97757',
        secondary: '#faf9f5',
        accent: '#1a1915',
      },
    },
  },

  vercel: {
    label: 'Vercel',
    createdAt: '2025-04-13',
    styles: {
      light: {
        primary: 'oklch(0 0 0)',
        secondary: 'oklch(0.94 0 0)',
        accent: 'oklch(0.94 0 0)',
      },

      dark: {
        primary: 'oklch(1.00 0 0)',
        secondary: 'oklch(0.25 0 0)',
        accent: 'oklch(0.32 0 0)',
      },
    },
  },

  darkmatter: {
    label: 'Darkmatter',
    createdAt: '2025-08-23',
    styles: {
      light: {
        primary: '#d87943',
        secondary: '#527575',
        accent: '#eeeeee',
      },
      dark: {
        primary: '#e78a53',
        secondary: '#5f8787',
        accent: '#333333',
      },
    },
  },

  mono: {
    label: 'Mono',
    createdAt: '2025-04-20',
    styles: {
      light: {
        primary: '#737373',
        secondary: '#f5f5f5',
        accent: '#f5f5f5',
      },
      dark: {
        primary: '#737373',
        secondary: '#262626',
        accent: '#404040',
      },
    },
  },
  'soft-pop': {
    label: 'Soft Pop',
    createdAt: '2025-07-08',
    styles: {
      light: {
        primary: '#4f46e5',
        secondary: '#14b8a6',
        accent: '#f59e0b',
      },
      dark: {
        primary: '#818cf8',
        secondary: '#2dd4bf',
        accent: '#fcd34d',
      },
    },
  },
};
