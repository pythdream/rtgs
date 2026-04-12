     1|import { useState } from 'react';
     2|import { Plus, Search } from 'lucide-react';
     3|import type { Proposal, TabFilter } from '../types';
     4|import ProposalCard from './ProposalCard';
     5|
     6|interface ProposalListProps {
     7|  proposals: Proposal[];
     8|  votedProposals: Set<string>;
     9|  onVote: (proposal: Proposal) => void;
    10|  onViewResults: (proposal: Proposal) => void;
    11|  onCreateProposal: () => void;
    12|}
    13|
    14|export default function ProposalList({
    15|  proposals,
    16|  votedProposals,
    17|  onVote,
    18|  onViewResults,
    19|  onCreateProposal,
    20|}: ProposalListProps) {
    21|  const [activeTab, setActiveTab] = useState<TabFilter>('all');
    22|  const [searchQuery, setSearchQuery] = useState('');
    23|
    24|  const tabs: { id: TabFilter; label: string }[] = [
    25|    { id: 'all', label: 'All' },
    26|    { id: 'active', label: 'Active' },
    27|    { id: 'finalized', label: 'Finalized' },
    28|  ];
    29|
    30|  const filtered = proposals.filter((p) => {
    31|    const matchesTab =
    32|      activeTab === 'all' ||
    33|      (activeTab === 'active' && p.status === 'active') ||
    34|      (activeTab === 'finalized' && p.status === 'finalized');
    35|    const matchesSearch =
    36|      !searchQuery ||
    37|      p.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    38|      p.description.toLowerCase().includes(searchQuery.toLowerCase());
    39|    return matchesTab && matchesSearch;
    40|  });
    41|
    42|  const activeCount = proposals.filter((p) => p.status === 'active').length;
    43|  const finalizedCount = proposals.filter((p) => p.status === 'finalized').length;
    44|
    45|  return (
    46|    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    47|      {/* Section header */}
    48|      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-8">
    49|        <div>
    50|          <h2 className="text-2xl font-bold text-white">Governance Proposals</h2>
    51|          <p className="text-sm text-slate-400 mt-1">
    52|            {activeCount} active · {finalizedCount} finalized
    53|          </p>
    54|        </div>
    55|        <button
    56|          onClick={onCreateProposal}
    57|          className="inline-flex items-center gap-2 px-4 py-2.5 rounded-xl bg-primary hover:bg-primary/90 text-white font-semibold text-sm transition-all duration-200 purple-glow hover:scale-[1.02] self-start sm:self-auto"
    58|        >
    59|          <Plus className="w-4 h-4" />
    60|          Create Proposal
    61|        </button>
    62|      </div>
    63|
    64|      {/* Filters row */}
    65|      <div className="flex flex-col sm:flex-row gap-3 mb-6">
    66|        {/* Tab bar */}
    67|        <div className="flex items-center gap-1 p-1 rounded-xl glass border border-white/[0.06] self-start">
    68|          {tabs.map((tab) => (
    69|            <button
    70|              key={tab.id}
    71|              onClick={() => setActiveTab(tab.id)}
    72|              className={`px-4 py-1.5 rounded-lg text-sm font-medium transition-all duration-200 ${
    73|                activeTab === tab.id
    74|                  ? 'bg-primary text-white shadow-sm'
    75|                  : 'text-slate-400 hover:text-white'
    76|              }`}
    77|            >
    78|              {tab.label}
    79|              {tab.id === 'active' && activeCount > 0 && (
    80|                <span
    81|                  className={`ml-1.5 px-1.5 py-0.5 rounded-full text-xs ${
    82|                    activeTab === 'active'
    83|                      ? 'bg-white/20 text-white'
    84|                      : 'bg-green-500/20 text-green-400'
    85|                  }`}
    86|                >
    87|                  {activeCount}
    88|                </span>
    89|              )}
    90|            </button>
    91|          ))}
    92|        </div>
    93|
    94|        {/* Search */}
    95|        <div className="relative flex-1 max-w-xs">
    96|          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500" />
    97|          <input
    98|            type="text"
    99|            placeholder="Search proposals..."
   100|            value={searchQuery}
   101|            onChange={(e) => setSearchQuery(e.target.value)}
   102|            className="w-full pl-9 pr-4 py-2 rounded-xl glass border border-white/[0.06] text-sm text-white placeholder-slate-500 focus:outline-none focus:border-primary/40 transition-colors"
   103|          />
   104|        </div>
   105|      </div>
   106|
   107|      {/* Proposal grid */}
   108|      {filtered.length === 0 ? (
   109|        <div className="text-center py-20 glass rounded-2xl border border-white/[0.06]">
   110|          <div className="text-4xl mb-4">🗳️</div>
   111|          <p className="text-slate-400 font-medium">No proposals found</p>
   112|          <p className="text-slate-500 text-sm mt-1">
   113|            {searchQuery ? 'Try a different search term' : 'Be the first to create a proposal'}
   114|          </p>
   115|        </div>
   116|      ) : (
   117|        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-2 gap-5">
   118|          {filtered.map((proposal) => (
   119|            <ProposalCard
   120|              key={proposal.id}
   121|              proposal={proposal}
   122|              onVote={onVote}
   123|              onViewResults={onViewResults}
   124|              hasVoted={votedProposals.has(proposal.id)}
   125|            />
   126|          ))}
   127|        </div>
   128|      )}
   129|    </div>
   130|  );
   131|}
   132|