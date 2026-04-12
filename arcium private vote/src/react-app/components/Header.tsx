     1|import { useState } from 'react';
     2|import { Shield, Zap, Wallet, X } from 'lucide-react';
     3|
     4|interface HeaderProps {
     5|  onNavigate: (view: 'landing' | 'dashboard') => void;
     6|  currentView: string;
     7|}
     8|
     9|export default function Header({ onNavigate, currentView }: HeaderProps) {
    10|  const [walletConnected, setWalletConnected] = useState(false);
    11|  const [connecting, setConnecting] = useState(false);
    12|
    13|  const handleWalletConnect = () => {
    14|    if (walletConnected) {
    15|      setWalletConnected(false);
    16|      return;
    17|    }
    18|    setConnecting(true);
    19|    setTimeout(() => {
    20|      setConnecting(false);
    21|      setWalletConnected(true);
    22|    }, 1200);
    23|  };
    24|
    25|  return (
    26|    <header className="fixed top-0 left-0 right-0 z-50 glass border-b border-white/[0.06]">
    27|      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    28|        <div className="flex items-center justify-between h-16">
    29|          {/* Logo */}
    30|          <button
    31|            onClick={() => onNavigate('landing')}
    32|            className="flex items-center gap-2.5 group"
    33|          >
    34|            <div className="relative">
    35|              <div className="w-8 h-8 rounded-lg bg-primary/20 border border-primary/40 flex items-center justify-center group-hover:bg-primary/30 transition-colors">
    36|                <Shield className="w-4 h-4 text-primary" />
    37|              </div>
    38|              <div className="absolute -top-0.5 -right-0.5 w-2.5 h-2.5 rounded-full bg-cyan-400 border border-background animate-pulse" />
    39|            </div>
    40|            <div>
    41|              <span className="text-white font-bold text-lg tracking-tight">ArcVote</span>
    42|            </div>
    43|          </button>
    44|
    45|          {/* Center badge */}
    46|          <div className="hidden sm:flex items-center gap-1.5 px-3 py-1 rounded-full bg-primary/10 border border-primary/20">
    47|            <Zap className="w-3 h-3 text-cyan-400" />
    48|            <span className="text-xs text-cyan-400 font-medium">Powered by Arcium MPC</span>
    49|          </div>
    50|
    51|          {/* Right side */}
    52|          <div className="flex items-center gap-3">
    53|            {currentView !== 'landing' && (
    54|              <button
    55|                onClick={() => onNavigate('landing')}
    56|                className="hidden sm:block text-sm text-muted-foreground hover:text-white transition-colors"
    57|              >
    58|                Home
    59|              </button>
    60|            )}
    61|            {currentView !== 'dashboard' && (
    62|              <button
    63|                onClick={() => onNavigate('dashboard')}
    64|                className="hidden sm:block text-sm text-muted-foreground hover:text-white transition-colors"
    65|              >
    66|                Proposals
    67|              </button>
    68|            )}
    69|
    70|            <button
    71|              onClick={handleWalletConnect}
    72|              className={`flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm font-medium transition-all ${
    73|                walletConnected
    74|                  ? 'bg-green-500/10 border border-green-500/30 text-green-400 hover:bg-red-500/10 hover:border-red-500/30 hover:text-red-400'
    75|                  : 'bg-primary/20 border border-primary/40 text-primary hover:bg-primary/30'
    76|              }`}
    77|            >
    78|              {connecting ? (
    79|                <>
    80|                  <div className="w-3 h-3 border border-primary/60 border-t-primary rounded-full animate-spin" />
    81|                  <span>Connecting...</span>
    82|                </>
    83|              ) : walletConnected ? (
    84|                <>
    85|                  <div className="w-2 h-2 rounded-full bg-green-400" />
    86|                  <span className="font-mono text-xs">7xKp...3mNq</span>
    87|                  <X className="w-3 h-3 opacity-60" />
    88|                </>
    89|              ) : (
    90|                <>
    91|                  <Wallet className="w-3.5 h-3.5" />
    92|                  <span>Connect Wallet</span>
    93|                </>
    94|              )}
    95|            </button>
    96|          </div>
    97|        </div>
    98|      </div>
    99|    </header>
   100|  );
   101|}
   102|