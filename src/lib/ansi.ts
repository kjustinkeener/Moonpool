// 16-color ANSI palettes per theme id, applied to xterm so program output
// (colored text) matches the active theme. Keys are xterm ITheme color slots.
export type AnsiPalette = {
  black: string; red: string; green: string; yellow: string;
  blue: string; magenta: string; cyan: string; white: string;
  brightBlack: string; brightRed: string; brightGreen: string; brightYellow: string;
  brightBlue: string; brightMagenta: string; brightCyan: string; brightWhite: string;
};

export const ANSI: Record<string, AnsiPalette> = {
  // Neutral dark, GitHub-dark / VS Code dark style.
  dark: {
    black: "#484f58", red: "#ff7b72", green: "#3fb950", yellow: "#d29922",
    blue: "#58a6ff", magenta: "#bc8cff", cyan: "#39c5cf", white: "#b1bac4",
    brightBlack: "#6e7681", brightRed: "#ffa198", brightGreen: "#56d364", brightYellow: "#e3b341",
    brightBlue: "#79c0ff", brightMagenta: "#d2a8ff", brightCyan: "#56d4dd", brightWhite: "#ffffff",
  },
  // Readable light, GitHub-light style.
  light: {
    black: "#24292f", red: "#cf222e", green: "#1a7f37", yellow: "#9a6700",
    blue: "#0969da", magenta: "#8250df", cyan: "#1b7c83", white: "#6e7781",
    brightBlack: "#57606a", brightRed: "#a40e26", brightGreen: "#116329", brightYellow: "#7d4e00",
    brightBlue: "#0550ae", brightMagenta: "#6639ba", brightCyan: "#1b7c83", brightWhite: "#24292f",
  },
  // Violet-leaning dark.
  purple: {
    black: "#3a2f4d", red: "#ff7b9c", green: "#8ce0b0", yellow: "#e6c07b",
    blue: "#8a7bff", magenta: "#c792ea", cyan: "#7fd0e0", white: "#c8bfe0",
    brightBlack: "#6b5b8a", brightRed: "#ff9dbb", brightGreen: "#a8ecc6", brightYellow: "#f0d29a",
    brightBlue: "#a89bff", brightMagenta: "#dcaef5", brightCyan: "#a0e2ef", brightWhite: "#f2eeff",
  },
  // Deep blue/teal dark.
  ocean: {
    black: "#2a3f52", red: "#ff8a80", green: "#5fd0b0", yellow: "#e6c86b",
    blue: "#5fa8e6", magenta: "#a99be6", cyan: "#4fd0d8", white: "#b0c4d4",
    brightBlack: "#4f6a80", brightRed: "#ffab9e", brightGreen: "#84e6c9", brightYellow: "#f0d98a",
    brightBlue: "#82c0f0", brightMagenta: "#c2b5f0", brightCyan: "#79e2e8", brightWhite: "#e6f1f7",
  },
  // Green-on-black terminal.
  matrix: {
    black: "#0d2610", red: "#e06b5c", green: "#33ff66", yellow: "#b8e05c",
    blue: "#4fbf7f", magenta: "#7fdf8f", cyan: "#5fe6a0", white: "#9ce0a8",
    brightBlack: "#2f5c3a", brightRed: "#ff8c7a", brightGreen: "#66ff8f", brightYellow: "#d4f07a",
    brightBlue: "#78d99e", brightMagenta: "#a3efad", brightCyan: "#8cf5c0", brightWhite: "#d6ffdf",
  },
  // Warm amber dark.
  amber: {
    black: "#4a3524", red: "#ff8a5c", green: "#c0c05c", yellow: "#ffb638",
    blue: "#d19a5c", magenta: "#e6996b", cyan: "#d4b06b", white: "#e0c4a0",
    brightBlack: "#7a5c3a", brightRed: "#ffab7a", brightGreen: "#d6d67a", brightYellow: "#ffcf6b",
    brightBlue: "#e6b87a", brightMagenta: "#f0b58c", brightCyan: "#eacf8c", brightWhite: "#fff0d6",
  },
  // Pink/rose dark.
  rose: {
    black: "#4d2f38", red: "#ff6b8a", green: "#8cd0a0", yellow: "#e6bb7b",
    blue: "#c98ab0", magenta: "#f28fb0", cyan: "#d99ac4", white: "#e0c0cc",
    brightBlack: "#7a5060", brightRed: "#ff8fa8", brightGreen: "#a8e0bb", brightYellow: "#f0d29a",
    brightBlue: "#e0a8c4", brightMagenta: "#ffabc4", brightCyan: "#efb8da", brightWhite: "#ffe6ee",
  },
  // Nord (canonical aurora/frost).
  nord: {
    black: "#3b4252", red: "#bf616a", green: "#a3be8c", yellow: "#ebcb8b",
    blue: "#81a1c1", magenta: "#b48ead", cyan: "#88c0d0", white: "#e5e9f0",
    brightBlack: "#4c566a", brightRed: "#bf616a", brightGreen: "#a3be8c", brightYellow: "#ebcb8b",
    brightBlue: "#81a1c1", brightMagenta: "#b48ead", brightCyan: "#8fbcbb", brightWhite: "#eceff4",
  },
  // Dracula (canonical).
  dracula: {
    black: "#21222c", red: "#ff5555", green: "#50fa7b", yellow: "#f1fa8c",
    blue: "#bd93f9", magenta: "#ff79c6", cyan: "#8be9fd", white: "#f8f8f2",
    brightBlack: "#6272a4", brightRed: "#ff6e6e", brightGreen: "#69ff94", brightYellow: "#ffffa5",
    brightBlue: "#d6acff", brightMagenta: "#ff92df", brightCyan: "#a4ffff", brightWhite: "#ffffff",
  },
  // Gruvbox dark (canonical).
  gruvbox: {
    black: "#282828", red: "#cc241d", green: "#98971a", yellow: "#d79921",
    blue: "#458588", magenta: "#b16286", cyan: "#689d6a", white: "#a89984",
    brightBlack: "#928374", brightRed: "#fb4934", brightGreen: "#b8bb26", brightYellow: "#fabd2f",
    brightBlue: "#83a598", brightMagenta: "#d3869b", brightCyan: "#8ec07c", brightWhite: "#ebdbb2",
  },
  // Solarized dark (canonical).
  solarized: {
    black: "#073642", red: "#dc322f", green: "#859900", yellow: "#b58900",
    blue: "#268bd2", magenta: "#d33682", cyan: "#2aa198", white: "#eee8d5",
    brightBlack: "#002b36", brightRed: "#cb4b16", brightGreen: "#586e75", brightYellow: "#657b83",
    brightBlue: "#839496", brightMagenta: "#6c71c4", brightCyan: "#93a1a1", brightWhite: "#fdf6e3",
  },
  // Deep red dark.
  crimson: {
    black: "#4d2830", red: "#ff5c6b", green: "#9cc78a", yellow: "#e6b56b",
    blue: "#d17b8a", magenta: "#e67b8f", cyan: "#d18a9c", white: "#e0b8bf",
    brightBlack: "#7a4650", brightRed: "#ff7d8a", brightGreen: "#b6d9a3", brightYellow: "#f0cd8a",
    brightBlue: "#e6a3ad", brightMagenta: "#ffa3b0", brightCyan: "#e6abbb", brightWhite: "#ffe0e4",
  },
  // Fresh mint on LIGHT ground: colors kept dark enough to read.
  mint: {
    black: "#1f3d33", red: "#c0362e", green: "#0f8a5f", yellow: "#9a7200",
    blue: "#1f7a8c", magenta: "#8a3f9c", cyan: "#0e8a7a", white: "#527a6e",
    brightBlack: "#3a5c50", brightRed: "#a02820", brightGreen: "#0a6e4a", brightYellow: "#7d5c00",
    brightBlue: "#175f6e", brightMagenta: "#6e2f7d", brightCyan: "#0a6e60", brightWhite: "#1f3d33",
  },
  // Warm neutral paper, LIGHT ground.
  paper: {
    black: "#33302a", red: "#b03a2e", green: "#5a7d2a", yellow: "#96690f",
    blue: "#2e6b8a", magenta: "#8a4a7d", cyan: "#1f7a70", white: "#6e665a",
    brightBlack: "#524c40", brightRed: "#943024", brightGreen: "#476420", brightYellow: "#7d570a",
    brightBlue: "#25546e", brightMagenta: "#6e3a63", brightCyan: "#186259", brightWhite: "#33302a",
  },
  // Bright blue sky, LIGHT ground.
  sky: {
    black: "#1f3a4d", red: "#c0362e", green: "#2a7d4a", yellow: "#96690f",
    blue: "#1565c0", magenta: "#7d3f9c", cyan: "#0e7a8c", white: "#527088",
    brightBlack: "#3a566e", brightRed: "#a02820", brightGreen: "#20643a", brightYellow: "#7d570a",
    brightBlue: "#0d4f9c", brightMagenta: "#632f7d", brightCyan: "#0a6070", brightWhite: "#1f3a4d",
  },
  // Soft violet lavender, LIGHT ground.
  lavender: {
    black: "#352f4d", red: "#b83a54", green: "#3a7d5a", yellow: "#8a6a0f",
    blue: "#4a4fc0", magenta: "#8a3f9c", cyan: "#2a6a8c", white: "#6e6688",
    brightBlack: "#4f486e", brightRed: "#9c2840", brightGreen: "#2a6448", brightYellow: "#7d570a",
    brightBlue: "#3a3f9c", brightMagenta: "#6e2f7d", brightCyan: "#215470", brightWhite: "#352f4d",
  },
  // Muted green sage, LIGHT ground.
  sage: {
    black: "#2e3a2a", red: "#b03a2e", green: "#3a7d2e", yellow: "#8a6a0f",
    blue: "#2e6b7d", magenta: "#7d4a6e", cyan: "#1f7a5f", white: "#63705a",
    brightBlack: "#4a5642", brightRed: "#943024", brightGreen: "#2a6420", brightYellow: "#7d570a",
    brightBlue: "#255466", brightMagenta: "#633a57", brightCyan: "#186249", brightWhite: "#2e3a2a",
  },
};

// Resolve an ANSI palette for a resolved theme id (never "auto"); falls back to dark.
export function ansiFor(id: string): AnsiPalette {
  return ANSI[id] ?? ANSI.dark;
}
