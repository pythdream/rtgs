     1|import { useEffect, useState } from 'react';
     2|import { X, Shield, CheckCircle, Lock } from 'lucide-react';
     3|import type { Proposal } from '../types';
     4|
     5|interface ResultsModalProps {
     6|  proposal: Proposal;
     7|  onClose: () => void;
     8|}
     9|
    10|function getOptionColor(opt: string, index: number): string {
    11|  const lower = opt.toLowerCase();
    12|  if (lower === 'for' || lower === 'approve' || lower === 'accept' || lower === 'yes') return 'from-green-500 to-emerald-400';
    13|  if (lower === 'against' || lower === 'reject' || lower === 'no') return 'from-red-500 to-rose-400';
    14|  if (lower === 'abstain' || lower === 'modify') return 'from-slate-500 to-slate-400';
    15|  const colors = ['from-primary to-purple-400', 'from-cyan-500 to-cyan-400', 'from-yellow-500 to-yellow-400'];
    16|  return colors[index % colors.length];
    17|}
    18|
    19|export default function ResultsModal({ proposal, onClose }: ResultsModalProps) {
    20|  const [animated, setAnimated] = useState(false);
    21|
    22|  useEffect(() => {
    23|    const t = setTimeout(() => setAnimated(true), 100);
    24|    return () => clearTimeout(t);
    25|  }, []);
    26|
    27|  if (!proposal.results) return null;
    28|
    29|  const total = proposal.voteCount;
    30|  const winner = proposal.options.reduce((a, b) =>
    31|    (proposal.results![a] ?? 0) > (proposal.results![b] ?? 0) ? a : b
    32|  );
    33|
    34|  return (
    35|    <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
    36|      <div
    37|        className="absolute inset-0 bg-black/70 backdrop-blur-sm animate-fade-in"
    38|        onClick={onClose}
    39|      />
    40|
    41|      <div className="relative w-full max-w-lg glass rounded-2xl border border-white/10 animate-fade-in-up overflow-hidden">
    42|        {/* Header */}
    43|        <div className="flex items-start justify-between p-6 border-b border-white/[0.06]">
    44|          <div className="flex-1 pr-4">
    45|            <div className="flex items-center gap-2 mb-1">
    46|              <Shield className="w-4 h-4 text-cyan-400" />
    47|              <span className="text-xs text-cyan-400 font-medium">Results verified by Arcium MPC</span>
    48|            </div>
    49|            <h2 className="text-lg font-semibold text-white leading-snug">{proposal.title}</h2>
    50|          </div>
    51|          <button
    52|            onClick={onClose}
    53|            className="w-8 h-8 rounded-lg glass flex items-center justify-center text-muted-foreground hover:text-white transition-colors"
    54|          >
    55|            <X className="w-4 h-4" />
    56|          </button>
    57|        </div>
    58|
    59|        <div className="p-6 space-y-6">
    60|          <p className="text-muted-foreground text-sm leading-relaxed">{proposal.description}</p>
    61|
    62|          {/* Winner banner */}
    63|          <div className="flex items-center gap-3 p-3 rounded-xl bg-green-500/10 border border-green-500/20">
    64|            <CheckCircle className="w-5 h-5 text-green-400 flex-shrink-0" />
    65|            <div>
    66|              <p className="text-xs text-muted-foreground">Winning option</p>
    67|              <p className="text-green-400 font-semibold">{winner}</p>
    68|            </div>
    69|            <div className="ml-auto text-right">
    70|              <p className="text-xs text-muted-foreground">Total votes</p>
    71|              <p className="text-white font-semibold">{total.toLocaleString()}</p>
    72|            </div>
    73|          </div>
    74|
    75|          {/* Bar chart */}
    76|          <div className="space-y-4">
    77|            {proposal.options.map((opt, i) => {
    78|              const count = proposal.results![opt] ?? 0;
    79|              const pct = total > 0 ? Math.round((count / total) * 100) : 0;
    80|              const gradient = getOptionColor(opt, i);
    81|              return (
    82|                <div key={opt}>
    83|                  <div className="flex items-center justify-between mb-1.5">
    84|                    <span className="text-sm font-medium text-white">{opt}</span>
    85|                    <div className="flex items-center gap-3">
    86|                      <span className="text-sm text-muted-foreground">{count.toLocaleString()} votes</span>
    87|                      <span className="text-sm font-semibold text-white w-10 text-right">{pct}%</span>
    88|                    </div>
    89|                  </div>
    90|                  <div className="h-3 rounded-full bg-white/5 overflow-hidden">
    91|                    <div
    92|                      className={`h-full rounded-full bg-gradient-to-r ${gradient} transition-all duration-1000 ease-out`}
    93|                      style={{ width: animated ? `${pct}%` : '0%' }}
    94|                    />
    95|                  </div>
    96|                </div>
    97|              );
    98|            })}
    99|          </div>
   100|
   101|          {/* Privacy note */}
   102|          <div className="flex items-start gap-2 p-3 rounded-lg bg-primary/5 border border-primary/15">
   103|            <Lock className="w-3.5 h-3.5 text-primary mt-0.5 flex-shrink-0" />
   104|            <p className="text-xs text-muted-foreground leading-relaxed">
   105|              <span className="text-white font-medium">Privacy Note:</span> Individual votes were never revealed. Only this aggregate result was decrypted by Arcium's MPC network.
   106|            </p>
   107|          </div>
   108|
   109|          {/* Mock tx */}
   110|          <div className="flex items-center justify-between text-xs text-muted-foreground">
   111|            <span>Proof tx:</span>
   112|            <span className="font-mono text-cyan-400/70">4xKp...9mNq</span>
   113|          </div>
   114|        </div>
   115|      </div>
   116|    </div>
   117|  );
   118|}
   119|