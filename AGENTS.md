# AGENTS.md

## Cursor Cloud specific instructions

### Project Overview

This is **anydoor** — a minimal Node.js/Express static file server written in TypeScript. It has two entry points:

- `staticServer.ts` — Simple Express server on port 8080 with two hardcoded routes (`/` and `/test`).
- `staticServerNative.ts` — Static file server on port 3000 that serves files/directories from CWD (has an unresolved syntax error; see below).

### Running the Server

```bash
npm run start     # Runs staticServer.ts on port 8080
npm run dev       # Runs staticServerNative.ts on port 3000 (currently broken, see Known Issues)
```

Both scripts use `ts-node --transpile-only` because the code uses legacy `import * as X` patterns that fail strict type checking with modern TypeScript.

### Known Issues

- `staticServerNative.ts` has an empty `try {}` block (line 61-63) without `catch` or `finally`, which is a syntax error that prevents the file from being parsed. This must be fixed in the source code to make it runnable.
- The codebase uses `import * as express from 'express'` (legacy pattern). `tsconfig.json` has `esModuleInterop: false` to support this pattern at runtime.

### TypeScript Checking

```bash
npm run typecheck   # Runs tsc --noEmit (will report errors due to known issues above)
```

Type checking reports errors due to the legacy import pattern and the syntax error. Use `--transpile-only` for running code.

### Dependencies

- No lockfile convention established — uses npm (package-lock.json generated on install).
- Key runtime deps: `express`, `chalk` (v4, CJS).
- Key dev deps: `typescript`, `ts-node`, `tsx`, `@types/express`, `@types/node`.
