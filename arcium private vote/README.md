     1|# Nullshot Beta
     2|
     3|A modern React 19 template with Vite, Tailwind CSS v4, and Cloudflare Workers.
     4|
     5|## Features
     6|
     7|- **React 19** - Latest React with new features
     8|- **Vite** - Fast dev server with instant HMR
     9|- **Tailwind CSS v4** - Utility-first styling
    10|- **TypeScript** - Type-safe development
    11|- **Hono** - Lightweight API framework for Workers
    12|- **React Query** - Data fetching and caching
    13|
    14|## Local Development
    15|
    16|```bash
    17|# Install dependencies
    18|pnpm install
    19|
    20|# Start dev server
    21|pnpm dev
    22|```
    23|
    24|## File Structure
    25|
    26|```
    27|src/
    28|├── react-app/
    29|│   ├── app.tsx       # Main React component
    30|│   ├── main.tsx      # Client entry point
    31|│   └── globals.css   # Tailwind + global styles
    32|└── worker/
    33|    └── index.ts      # Hono API routes + asset serving
    34|```
    35|
    36|## API Routes
    37|
    38|Routes are defined in `/src/worker/index.ts`:
    39|
    40|- `GET /api/health` - Health check
    41|- `GET /api/` - API info
    42|- `POST /api/echo` - Echo request body
    43|- `GET /api/data` - Example data endpoint
    44|
    45|All other routes serve the React SPA.
    46|