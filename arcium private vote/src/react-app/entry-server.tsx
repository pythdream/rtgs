     1|
     2|import { renderToString } from 'react-dom/server';
     3|import { createElement, StrictMode } from 'react';
     4|import { StaticRouter } from 'react-router';
     5|import { Routes, Route } from 'react-router-dom';
     6|import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
     7|
     8|const queryClient = new QueryClient({
     9|	defaultOptions: {
    10|		queries: {
    11|			staleTime: 1000 * 60 * 5,
    12|			refetchOnWindowFocus: false,
    13|		},
    14|	},
    15|});
    16|
    17|function HomePage() {
    18|	return createElement(
    19|		'div',
    20|		{ className: 'flex items-center justify-center p-4 min-h-screen' },
    21|		createElement(
    22|			'div',
    23|			{ className: 'text-center' },
    24|			createElement(
    25|				'h1',
    26|				{ className: 'text-6xl font-bold bg-gradient-to-r from-purple-400 to-pink-600 bg-clip-text text-transparent' },
    27|				'Nullshot Beta'
    28|			)
    29|		)
    30|	);
    31|}
    32|
    33|function AppContent() {
    34|	return createElement(
    35|		'div',
    36|		{ className: 'min-h-screen bg-gradient-to-br from-gray-900 via-purple-900 to-gray-900 text-white' },
    37|		createElement(
    38|			Routes,
    39|			null,
    40|			createElement(Route, { path: '/', element: createElement(HomePage) })
    41|		)
    42|	);
    43|}
    44|
    45|export async function renderApp(pathname: string): Promise<string> {
    46|	const appHtml = renderToString(
    47|		createElement(
    48|			StrictMode,
    49|			null,
    50|			createElement(
    51|				QueryClientProvider,
    52|				{ client: queryClient },
    53|				createElement(
    54|					StaticRouter,
    55|					{ location: pathname },
    56|					createElement(AppContent)
    57|				)
    58|			)
    59|		)
    60|	);
    61|
    62|	return `<!DOCTYPE html>
    63|<html lang="en">
    64|  <head>
    65|    <meta charset="UTF-8" />
    66|    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    67|    <meta name="description" content="NullShot App - Built with React and Cloudflare Workers" />
    68|    <title>NullShot App</title>
    69|    <link rel="icon" type="image/svg+xml" href="/favicon.svg" />
    70|    <style>
    71|      /* Critical CSS - inlined for fast first paint */
    72|      :root {
    73|        color-scheme: dark;
    74|        --background: hsl(240 10% 4%);
    75|        --foreground: hsl(200 10% 95%);
    76|      }
    77|      * { margin: 0; padding: 0; box-sizing: border-box; }
    78|      body {
    79|        background-color: var(--background);
    80|        color: var(--foreground);
    81|        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    82|        min-height: 100vh;
    83|      }
    84|      /* Loading state before hydration */
    85|      .ssr-loading { opacity: 0.8; }
    86|    </style>
    87|    <!-- Preload the main bundle for faster hydration -->
    88|    <link rel="modulepreload" href="/assets/main.js" />
    89|  </head>
    90|  <body>
    91|    <div id="root" class="ssr-loading">${appHtml}</div>
    92|    <script type="module">
    93|      document.getElementById('root').classList.remove('ssr-loading');
    94|    </script>
    95|    <script type="module" src="/assets/main.js"></script>
    96|  </body>
    97|</html>`;
    98|}
    99|
   100|export async function renderAppToString(pathname: string = '/'): Promise<string> {
   101|	return renderToString(
   102|		createElement(
   103|			StrictMode,
   104|			null,
   105|			createElement(
   106|				QueryClientProvider,
   107|				{ client: queryClient },
   108|				createElement(
   109|					StaticRouter,
   110|					{ location: pathname },
   111|					createElement(AppContent)
   112|				)
   113|			)
   114|		)
   115|	);
   116|}
   117|