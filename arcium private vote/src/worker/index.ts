     1|/**
     2| * Worker Entry Point
     3| *
     4| * Hono-based worker for handling API routes and serving static assets.
     5| */
     6|import { Hono } from 'hono';
     7|
     8|const app = new Hono<{ Bindings: Env }>();
     9|
    10|// ============================================================================
    11|// API Routes
    12|// ============================================================================
    13|
    14|app.get('/api/health', (c) => c.json({
    15|  status: 'ok',
    16|  timestamp: Date.now(),
    17|}));
    18|
    19|app.get('/api/', (c) => c.json({
    20|  name: 'Nullshot Beta',
    21|  version: '1.0.0',
    22|}));
    23|
    24|app.post('/api/echo', async (c) => {
    25|  const body = await c.req.json();
    26|  return c.json({ echo: body, timestamp: Date.now() });
    27|});
    28|
    29|app.get('/api/data', (c) => {
    30|  return c.json({
    31|    message: 'Hello from the API!',
    32|    data: [],
    33|  });
    34|});
    35|
    36|// ============================================================================
    37|// Static Assets
    38|// ============================================================================
    39|
    40|const MIME_TYPES: Record<string, string> = {
    41|  '.html': 'text/html; charset=utf-8',
    42|  '.js': 'application/javascript; charset=utf-8',
    43|  '.mjs': 'application/javascript; charset=utf-8',
    44|  '.css': 'text/css; charset=utf-8',
    45|  '.json': 'application/json; charset=utf-8',
    46|  '.svg': 'image/svg+xml',
    47|  '.png': 'image/png',
    48|  '.jpg': 'image/jpeg',
    49|  '.jpeg': 'image/jpeg',
    50|  '.gif': 'image/gif',
    51|  '.webp': 'image/webp',
    52|  '.ico': 'image/x-icon',
    53|  '.woff': 'font/woff',
    54|  '.woff2': 'font/woff2',
    55|  '.ttf': 'font/ttf',
    56|  '.txt': 'text/plain; charset=utf-8',
    57|  '.xml': 'application/xml; charset=utf-8',
    58|};
    59|
    60|function getMimeType(path: string): string {
    61|  const ext = path.match(/\.[^.]+$/)?.[0]?.toLowerCase() || '';
    62|  return MIME_TYPES[ext] || 'application/octet-stream';
    63|}
    64|
    65|async function serveAsset(request: Request, assets: Fetcher): Promise<Response> {
    66|  const response = await assets.fetch(request);
    67|  if (response.headers.get('content-type')) {
    68|    return response;
    69|  }
    70|  const url = new URL(request.url);
    71|  const headers = new Headers(response.headers);
    72|  headers.set('Content-Type', getMimeType(url.pathname));
    73|  return new Response(response.body, {
    74|    status: response.status,
    75|    statusText: response.statusText,
    76|    headers,
    77|  });
    78|}
    79|
    80|app.get('/assets/*', (c) => serveAsset(c.req.raw, c.env.ASSETS));
    81|app.get('/favicon.svg', (c) => serveAsset(c.req.raw, c.env.ASSETS));
    82|app.get('/robots.txt', (c) => serveAsset(c.req.raw, c.env.ASSETS));
    83|
    84|// SPA fallback
    85|app.get('*', (c) => serveAsset(c.req.raw, c.env.ASSETS));
    86|
    87|// ============================================================================
    88|// Error Handling
    89|// ============================================================================
    90|
    91|app.notFound((c) => c.json({ error: 'Not Found', path: c.req.path }, 404));
    92|
    93|app.onError((err, c) => {
    94|  console.error('[Worker] Error:', err);
    95|  return c.json({ error: err.name, message: err.message }, 500);
    96|});
    97|
    98|export default app;
    99|