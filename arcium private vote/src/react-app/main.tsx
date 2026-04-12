     1|/**
     2| * Client Entry Point
     3| *
     4| * This file initializes the React application.
     5| * The App component contains React Router and React Query setup.
     6| */
     7|
     8|import { StrictMode } from 'react';
     9|import { createRoot } from 'react-dom/client';
    10|import App from './app';
    11|import './globals.css';
    12|
    13|const rootElement = document.getElementById('root');
    14|
    15|if (!rootElement) {
    16|	throw new Error('Root element not found');
    17|}
    18|
    19|createRoot(rootElement).render(
    20|	<StrictMode>
    21|		<App />
    22|	</StrictMode>
    23|);
    24|