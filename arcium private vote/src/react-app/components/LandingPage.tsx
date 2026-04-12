     1|import { Lock, CheckCircle, Shield, ArrowRight, Zap, Eye, Link } from 'lucide-react';
     2|
     3|interface LandingPageProps {
     4|  onViewProposals: () => void;
     5|}
     6|
     7|const HOW_IT_WORKS = [
     8|  {
     9|    step: 1,
    10|    icon: Lock,
    11|    title: 'You Cast Your Vote',
    12|    description: 'Select your choice. It\'s encrypted client-side using X25519 before leaving your browser — no one sees your raw vote.',
    13|    color: 'text-purple-400',
    14|    bg: 'bg-purple-500/10',
    15|    border: 'border-purple-500/20',
    16|  },
    17|  {
    18|    step: 2,
    19|    icon: Link,
    20|    title: 'Encrypted Vote → Solana',
    21|    description: 'Your encrypted vote is submitted on-chain and queued for Arcium\'s MPC cluster to process.',
    22|    color: 'text-cyan-400',
    23|    bg: 'bg-cyan-500/10',
    24|    border: 'border-cyan-500/20',
    25|  },
    26|  {
    27|    step: 3,
    28|    icon: Eye,
    29|    title: 'MPC Nodes Process Blindly',
    30|    description: 'Arcium\'s distributed MPC nodes compute the tally without any single node seeing individual votes.',
    31|    color: 'text-blue-400',
    32|    bg: 'bg-blue-500/10',
    33|    border: 'border-blue-500/20',
    34|  },
    35|  {
    36|    step: 4,
    37|    icon: Zap,
    38|    title: 'Tally Decrypted & Verified',
    39|    description: 'After voting ends, only the final aggregate result is decrypted. A cryptographic proof is published on-chain.',
    40|    color: 'text-green-400',
    41|    bg: 'bg-green-500/10',
    42|    border: 'border-green-500/20',
    43|  },
    44|  {
    45|    step: 5,
    46|    icon: CheckCircle,
    47|    title: 'Results Published',
    48|    description: 'The verified tally is available to all. Individual votes remain private forever — not even DAO operators can see them.',
    49|    color: 'text-emerald-400',
    50|    bg: 'bg-emerald-500/10',
    51|    border: 'border-emerald-500/20',
    52|  },
    53|];
    54|
    55|const FEATURES = [
    56|  {
    57|    icon: '🔒',
    58|    title: 'Votes Stay Private',
    59|    description: 'Individual votes are encrypted with MPC, never revealed to anyone — not even the DAO operators.',
    60|  },
    61|  {
    62|    icon: '✅',
    63|    title: 'Results Are Verified',
    64|    description: 'Cryptographic proofs confirm the tally is mathematically correct without exposing individual choices.',
    65|  },
    66|  {
    67|    icon: '🛡️',
    68|    title: 'No Double Voting',
    69|    description: 'On-chain records prevent voting twice while keeping your identity and choice completely private.',
    70|  },
    71|];
    72|
    73|export default function LandingPage({ onViewProposals }: LandingPageProps) {
    74|  return (
    75|    <div className="min-h-screen">
    76|      {/* Hero Section */}
    77|      <section className="relative pt-32 pb-24 px-4 overflow-hidden">
    78|        {/* Background glow */}
    79|        <div className="absolute inset-0 pointer-events-none">
    80|          <div className="absolute top-1/4 left-1/2 -translate-x-1/2 w-[600px] h-[600px] rounded-full bg-purple-600/10 blur-[120px]" />
    81|          <div className="absolute top-1/3 left-1/3 w-[300px] h-[300px] rounded-full bg-cyan-500/8 blur-[80px]" />
    82|        </div>
    83|
    84|        <div className="relative max-w-4xl mx-auto text-center">
    85|          {/* Badge */}
    86|          <div className="inline-flex items-center gap-2 px-4 py-1.5 rounded-full bg-purple-500/10 border border-purple-500/20 mb-8 animate-fade-in">
    87|            <div className="w-2 h-2 rounded-full bg-cyan-400 animate-pulse" />
    88|            <span className="text-sm text-cyan-400 font-medium">Powered by Arcium MPC on Solana</span>
    89|          </div>
    90|
    91|          {/* Title */}
    92|          <div className="flex items-center justify-center gap-4 mb-6 animate-fade-in-up">
    93|            <div className="w-14 h-14 rounded-2xl bg-purple-500/20 border border-purple-500/30 flex items-center justify-center animate-float">
    94|              <Lock className="w-7 h-7 text-purple-400" />
    95|            </div>
    96|            <h1 className="text-5xl sm:text-6xl lg:text-7xl font-bold tracking-tight">
    97|              <span className="text-white">Private</span>{' '}
    98|              <span className="bg-gradient-to-r from-purple-400 to-cyan-400 bg-clip-text text-transparent">
    99|                DAO Voting
   100|              </span>
   101|            </h1>
   102|          </div>
   103|
   104|          {/* Tagline */}
   105|          <p className="text-xl sm:text-2xl text-slate-400 mb-4 max-w-2xl mx-auto animate-fade-in-up">
   106|            Your vote is encrypted. Your voice is heard.
   107|          </p>
   108|          <p className="text-base text-slate-500 mb-12 max-w-xl mx-auto animate-fade-in-up">
   109|            Traditional DAO voting is public — your choices can be seen, copied, or coerced.
   110|            ArcVote uses Arcium's Multi-Party Computation to keep every vote private while
   111|            ensuring results are mathematically verifiable.
   112|          </p>
   113|
   114|          {/* CTA */}
   115|          <button
   116|            onClick={onViewProposals}
   117|            className="inline-flex items-center gap-2 px-8 py-4 rounded-xl bg-purple-600 hover:bg-purple-500 text-white font-semibold text-lg transition-all duration-200 purple-glow hover:scale-105 animate-fade-in-up"
   118|          >
   119|            View Proposals
   120|            <ArrowRight className="w-5 h-5" />
   121|          </button>
   122|        </div>
   123|      </section>
   124|
   125|      {/* Feature Cards */}
   126|      <section className="py-16 px-4">
   127|        <div className="max-w-5xl mx-auto">
   128|          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
   129|            {FEATURES.map((feature, i) => (
   130|              <div
   131|                key={i}
   132|                className="glass glass-hover rounded-2xl p-6 text-center transition-all duration-300"
   133|              >
   134|                <div className="text-4xl mb-4">{feature.icon}</div>
   135|                <h3 className="text-lg font-semibold text-white mb-2">{feature.title}</h3>
   136|                <p className="text-sm text-slate-400 leading-relaxed">{feature.description}</p>
   137|              </div>
   138|            ))}
   139|          </div>
   140|        </div>
   141|      </section>
   142|
   143|      {/* How It Works */}
   144|      <section className="py-16 px-4">
   145|        <div className="max-w-3xl mx-auto">
   146|          <div className="text-center mb-12">
   147|            <h2 className="text-3xl font-bold text-white mb-3">How Your Vote Stays Private</h2>
   148|            <p className="text-slate-400">
   149|              Arcium's MPC network ensures no single party ever sees your vote
   150|            </p>
   151|          </div>
   152|
   153|          <div className="relative">
   154|            {/* Vertical line */}
   155|            <div className="absolute left-6 top-8 bottom-8 w-px bg-gradient-to-b from-purple-500/50 via-cyan-500/30 to-transparent hidden sm:block" />
   156|
   157|            <div className="space-y-6">
   158|              {HOW_IT_WORKS.map((item) => {
   159|                const Icon = item.icon;
   160|                return (
   161|                  <div key={item.step} className="flex gap-4 sm:gap-6">
   162|                    <div className={`flex-shrink-0 w-12 h-12 rounded-xl ${item.bg} border ${item.border} flex items-center justify-center z-10`}>
   163|                      <Icon className={`w-5 h-5 ${item.color}`} />
   164|                    </div>
   165|                    <div className="glass rounded-xl p-4 flex-1">
   166|                      <div className="flex items-center gap-2 mb-1">
   167|                        <span className="text-xs font-mono text-slate-500">Step {item.step}</span>
   168|                      </div>
   169|                      <h4 className="text-white font-semibold mb-1">{item.title}</h4>
   170|                      <p className="text-sm text-slate-400 leading-relaxed">{item.description}</p>
   171|                    </div>
   172|                  </div>
   173|                );
   174|              })}
   175|            </div>
   176|          </div>
   177|        </div>
   178|      </section>
   179|
   180|      {/* Bottom CTA */}
   181|      <section className="py-16 px-4">
   182|        <div className="max-w-2xl mx-auto text-center glass rounded-3xl p-12 border border-purple-500/20">
   183|          <div className="w-16 h-16 rounded-2xl bg-purple-500/20 border border-purple-500/30 flex items-center justify-center mx-auto mb-6">
   184|            <Shield className="w-8 h-8 text-purple-400" />
   185|          </div>
   186|          <h2 className="text-2xl font-bold text-white mb-3">Ready to vote privately?</h2>
   187|          <p className="text-slate-400 mb-8">
   188|            Browse active proposals and cast your encrypted vote. No one will ever know how you voted.
   189|          </p>
   190|          <button
   191|            onClick={onViewProposals}
   192|            className="inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-purple-600 hover:bg-purple-500 text-white font-semibold transition-all duration-200 hover:scale-105"
   193|          >
   194|            View Active Proposals
   195|            <ArrowRight className="w-4 h-4" />
   196|          </button>
   197|        </div>
   198|      </section>
   199|    </div>
   200|  );
   201|}
   202|