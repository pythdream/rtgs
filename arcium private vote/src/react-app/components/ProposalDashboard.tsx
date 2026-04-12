     1|import { useState } from 'react';
     2|import { Plus, Search } from 'lucide-react';
     3|import ProposalCard from './ProposalCard';
     4|import type { Proposal, TabFilter } from '../types';
     5|
     6|interface ProposalDashboardProps {
     7|  proposals: Proposal[];
     8|  votedProposals: Set<string>;
     9|  onVote: (proposal: Proposal) => void;
    10|  onViewResults: (proposal: Proposal) => void;
    11|  onCreateProposal: () => void;
    12|}
    13|
    14|export default function ProposalDashboard({
    15|  proposals,
    16|  votedProposals,
    17|  onVote,
    18|  onViewResults,
    19|  onCreateProposal,
    20|}: ProposalDashboardProps) {
    21|  const [tab, setTab] = useState<TabFilter>('all');
    22|  const [search, setSearch] = useState('');
    23|
    24|  const filtered = proposals.filter((p) => {
    25|    const matchesTab = tab === 'all' || p.status === tab;
    26|    const matchesSearch =
    27|      !search ||
    28|      p.title.toLowerCase().includes(search.toLowerCase()) ||
    29|      p.description.toLowerCase().includes(search.toLowerCase());
    30|    return matchesTab && matchesSearch;
    31|  });
    32|
    33|  const activeCount = proposals.filter((p) => p.status === 'active').length;
    34|  const finalizedCount = proposals.filter((p) => p.status === 'finalized').length;
    35|
    36|  const tabs: { key: TabFilter; label: string; count: number }[] = [
    37|    { key: 'all', label: 'All', count: proposals.length },
    38|    { key: 'active', label: 'Active', count: activeCount },
    39|    { key: 'finalized', label: 'Finalized', count: finalizedCount },
    40|  ];
    41|
    42|  return (
    43|    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    44|      {/* Page header */}
    45|      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-8">
    46|        <div>
    47|          <h1 className="text-2xl font-bold text-white">Governance Proposals</h1>
    48|          <p className="text-muted-foreground text-sm mt-1">
    49|            Vote privately — your choices are encrypted with Arcium MPC
    50|          </p>
    51|        </div>
    52|        <button
    53|          onClick={onCreateProposal}
    54|          className="flex items-center gap-2 px-4 py-2.5 rounded-xl bg-primary hover:bg-primary/90 text-white font-medium text-sm transition-all purple-glow hover:scale-105"
    55|        >
    56|          <Plus className="w-4 h-4" />
    57|          Create Proposal
    58|        </button>
    59|      </div>
    60|
    61|      {/* Filters */}
    62|      <div className="flex flex-col sm:flex-row gap-3 mb-6">
    63|        {/* Tab bar */}
    64|        <div className="flex items-center gap-1 p-1 rounded-xl glass border border-white/[0.06]">
    65|          {tabs.map((t) => (
    66|            <button
    67|              key={t.key}
    68|              onClick={() => setTab(t.key)}
    69|              className={`flex items-center gap-1.5 px-4 py-1.5 rounded-lg text-sm font-medium transition-all ${
    70|                tab === t.key
    71|                  ? 'bg-primary text-white'
    72|                  : 'text-muted-foreground hover:text-white'
    73|              }`}
    74|            >
    75|              {t.label}
    76|              <span
    77|                className={`text-xs px-1.5 py-0.5 rounded-full ${
    78|                  tab === t.key ? 'bg-white/20' : 'bg-white/5'
    79|                }`}
    80|              >
    81|                {t.count}
    82|              </span>
    83|            </button>
    84|          ))}
    85|        </div>
    86|
    87|        {/* Search */}
    88|        <div className="relative flex-1 sm:max-w-xs">
    89|          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
    90|          <input
    91|            type="text"
    92|            placeholder="Search proposals..."
    93|            value={search}
    94|            onChange={(e) => setSearch(e.target.value)}
    95|            className="w-full pl-9 pr-4 py-2 rounded-xl glass border border-white/[0.06] text-sm text-white placeholder:text-muted-foreground focus:outline-none focus:border-primary/40 transition-colors"
    96|          />
    97|        </div>
    98|      </div>
    99|
   100|      {/* Grid */}
   101|      {filtered.length === 0 ? (
   102|        <div className="text-center py-20 text-muted-foreground">
   103|          <p className="text-lg font-medium mb-2">No proposals found</p>
   104|          <p className="text-sm">Try adjusting your filters or create a new proposal.</p>
   105|        </div>
   106|      ) : (
   107|        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-2 gap-5">
   108|          {filtered.map((proposal) => (
   109|            <ProposalCard
   110|              key={proposal.id}
   111|              proposal={proposal}
   112|              onVote={onVote}
   113|              onViewResults={onViewResults}
   114|              hasVoted={votedProposals.has(proposal.id)}
   115|            />
   116|          ))}
   117|        </div>
   118|      )}
   119|    </div>
   120|  );
   121|}
   122|