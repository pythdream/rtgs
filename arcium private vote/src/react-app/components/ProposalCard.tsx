     1|import { Clock, Users, ChevronRight, CheckCircle, BarChart2 } from 'lucide-react';
     2|import type { Proposal } from '../types';
     3|
     4|interface ProposalCardProps {
     5|  proposal: Proposal;
     6|  onVote: (proposal: Proposal) => void;
     7|  onViewResults: (proposal: Proposal) => void;
     8|  hasVoted: boolean;
     9|}
    10|
    11|function formatTimeRemaining(ms: number): string {
    12|  if (ms <= 0) return 'Ended';
    13|  const days = Math.floor(ms / (1000 * 60 * 60 * 24));
    14|  const hours = Math.floor((ms % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
    15|  const minutes = Math.floor((ms % (1000 * 60 * 60)) / (1000 * 60));
    16|  if (days > 0) return `${days}d ${hours}h remaining`;
    17|  if (hours > 0) return `${hours}h ${minutes}m remaining`;
    18|  return `${minutes}m remaining`;
    19|}
    20|
    21|function MiniResultBar({ results, options }: { results: Record<string, number>; options: string[] }) {
    22|  const total = Object.values(results).reduce((a, b) => a + b, 0);
    23|  if (total === 0) return null;
    24|
    25|  const colors = ['bg-purple-500', 'bg-red-500', 'bg-slate-500'];
    26|
    27|  return (
    28|    <div className="mt-3">
    29|      <div className="flex h-1.5 rounded-full overflow-hidden gap-0.5">
    30|        {options.map((opt, i) => {
    31|          const pct = ((results[opt] ?? 0) / total) * 100;
    32|          return (
    33|            <div
    34|              key={opt}
    35|              className={`${colors[i] ?? 'bg-slate-600'} rounded-full transition-all duration-700`}
    36|              style={{ width: `${pct}%` }}
    37|            />
    38|          );
    39|        })}
    40|      </div>
    41|      <div className="flex gap-3 mt-1.5">
    42|        {options.map((opt, i) => {
    43|          const pct = Math.round(((results[opt] ?? 0) / total) * 100);
    44|          return (
    45|            <span key={opt} className="text-xs text-slate-500">
    46|              <span className={`inline-block w-1.5 h-1.5 rounded-full mr-1 ${colors[i] ?? 'bg-slate-600'}`} />
    47|              {opt} {pct}%
    48|            </span>
    49|          );
    50|        })}
    51|      </div>
    52|    </div>
    53|  );
    54|}
    55|
    56|export default function ProposalCard({ proposal, onVote, onViewResults, hasVoted }: ProposalCardProps) {
    57|  const timeRemaining = proposal.votingEnd - Date.now();
    58|  const isActive = proposal.status === 'active';
    59|
    60|  return (
    61|    <div className="glass glass-hover rounded-2xl p-6 flex flex-col gap-4 transition-all duration-300 group">
    62|      {/* Header */}
    63|      <div className="flex items-start justify-between gap-3">
    64|        <div className="flex-1 min-w-0">
    65|          <div className="flex items-center gap-2 mb-2">
    66|            {isActive ? (
    67|              <span className="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full bg-green-500/10 border border-green-500/20 text-xs font-medium text-green-400">
    68|                <span className="w-1.5 h-1.5 rounded-full bg-green-400 animate-pulse" />
    69|                Active
    70|              </span>
    71|            ) : (
    72|              <span className="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full bg-slate-500/10 border border-slate-500/20 text-xs font-medium text-slate-400">
    73|                <CheckCircle className="w-3 h-3" />
    74|                Results Published
    75|              </span>
    76|            )}
    77|            {hasVoted && (
    78|              <span className="inline-flex items-center gap-1 px-2 py-0.5 rounded-full bg-purple-500/10 border border-purple-500/20 text-xs text-purple-400">
    79|                <CheckCircle className="w-3 h-3" />
    80|                Voted
    81|              </span>
    82|            )}
    83|          </div>
    84|          <h3 className="text-white font-semibold text-base leading-snug group-hover:text-purple-300 transition-colors">
    85|            {proposal.title}
    86|          </h3>
    87|        </div>
    88|      </div>
    89|
    90|      {/* Description */}
    91|      <p className="text-sm text-slate-400 leading-relaxed line-clamp-2">
    92|        {proposal.description}
    93|      </p>
    94|
    95|      {/* Results mini bar (finalized) */}
    96|      {!isActive && proposal.results && (
    97|        <MiniResultBar results={proposal.results} options={proposal.options} />
    98|      )}
    99|
   100|      {/* Meta */}
   101|      <div className="flex items-center gap-4 text-xs text-slate-500">
   102|        <span className="flex items-center gap-1">
   103|          <Users className="w-3.5 h-3.5" />
   104|          {proposal.voteCount.toLocaleString()} votes
   105|        </span>
   106|        <span className="flex items-center gap-1">
   107|          <Clock className="w-3.5 h-3.5" />
   108|          {isActive ? formatTimeRemaining(timeRemaining) : 'Voting closed'}
   109|        </span>
   110|        <span className="font-mono truncate">by {proposal.creator}</span>
   111|      </div>
   112|
   113|      {/* Action */}
   114|      <div className="pt-1">
   115|        {isActive ? (
   116|          <button
   117|            onClick={() => onVote(proposal)}
   118|            disabled={hasVoted}
   119|            className={`w-full flex items-center justify-center gap-2 py-2.5 rounded-xl text-sm font-semibold transition-all duration-200 ${
   120|              hasVoted
   121|                ? 'bg-slate-800 text-slate-500 cursor-not-allowed'
   122|                : 'bg-purple-600 hover:bg-purple-500 text-white hover:scale-[1.02] purple-glow'
   123|            }`}
   124|          >
   125|            {hasVoted ? (
   126|              <>
   127|                <CheckCircle className="w-4 h-4" />
   128|                Vote Submitted Privately
   129|              </>
   130|            ) : (
   131|              <>
   132|                Vote Now
   133|                <ChevronRight className="w-4 h-4" />
   134|              </>
   135|            )}
   136|          </button>
   137|        ) : (
   138|          <button
   139|            onClick={() => onViewResults(proposal)}
   140|            className="w-full flex items-center justify-center gap-2 py-2.5 rounded-xl text-sm font-semibold bg-slate-800 hover:bg-slate-700 text-slate-300 hover:text-white transition-all duration-200 border border-slate-700/50"
   141|          >
   142|            <BarChart2 className="w-4 h-4" />
   143|            View Results
   144|          </button>
   145|        )}
   146|      </div>
   147|    </div>
   148|  );
   149|}
   150|