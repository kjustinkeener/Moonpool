// Cross-platform `build:web`: sets HELP_BASE (no `VAR=x cmd` on Windows shells) and
// builds into dist-web/. Pass HELP_BASE yourself to override the default.
import { spawnSync } from 'node:child_process';

const env = { ...process.env, HELP_BASE: process.env.HELP_BASE || '/software/moonpool/help/' };
const r = spawnSync('npx', ['astro', 'build'], { stdio: 'inherit', env, shell: true });
process.exit(r.status ?? 1);
