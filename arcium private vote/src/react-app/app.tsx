     1|/**
     2| * App Component — Private DAO Voting powered by Arcium MPC
     3| */
     4|
     5|import { useState, useCallback } from 'react';
     6|import './globals.css';
     7|import Header from './components/Header';
     8|import LandingPage from './components/LandingPage';
     9|import ProposalList from './components/ProposalList';
    10|import VoteModal from './components/VoteModal';
    11|import ResultsModal from './components/ResultsModal';
    12|import { MOCK_PROPOSALS } from './mockData';
    13|import type { Proposal, View } from './types';
    14|
    15|// Toast notification component
    16|function Toast({ message, onDone }: { message: string; onDone: () => void }) {
    17|	return (
    18|		<div
    19|			className="fixed bottom-6 left-1/2 -translate-x-1/2 z-[100] flex items-center gap-3 px-5 py-3 rounded-2xl glass border border-green-500/30 bg-green-500/10 animate-fade-in-up shadow-xl"
    20|			onAnimationEnd={() => setTimeout(onDone, 3000)}
    21|		>
    22|			<span className="text-green-400 text-lg">✅</span>
    23|			<span className="text-white text-sm font-medium">{message}</span>
    24|		</div>
    25|	);
    26|}
    27|
    28|export default function App() {
    29|	const [view, setView] = useState<View>('landing');
    30|	const [proposals, setProposals] = useState<Proposal[]>(MOCK_PROPOSALS);
    31|	const [votedProposals, setVotedProposals] = useState<Set<string>>(new Set());
    32|	const [voteModalProposal, setVoteModalProposal] = useState<Proposal | null>(null);
    33|	const [resultsModalProposal, setResultsModalProposal] = useState<Proposal | null>(null);
    34|	const [toast, setToast] = useState<string | null>(null);
    35|
    36|	const handleNavigate = useCallback((v: View) => setView(v), []);
    37|
    38|	const handleViewProposals = useCallback(() => {
    39|		setView('dashboard');
    40|	}, []);
    41|
    42|	const handleVote = useCallback((proposal: Proposal) => {
    43|		setVoteModalProposal(proposal);
    44|	}, []);
    45|
    46|	const handleViewResults = useCallback((proposal: Proposal) => {
    47|		setResultsModalProposal(proposal);
    48|	}, []);
    49|
    50|	const handleVoteComplete = useCallback((proposalId: string) => {
    51|		setVotedProposals((prev) => new Set([...prev, proposalId]));
    52|		setProposals((prev) =>
    53|			prev.map((p) =>
    54|				p.id === proposalId ? { ...p, voteCount: p.voteCount + 1 } : p
    55|			)
    56|		);
    57|		setToast('Your encrypted vote has been submitted privately! 🔒');
    58|	}, []);
    59|
    60|	const handleCreateProposal = useCallback(() => {
    61|		setToast('Proposal creation coming soon — connect your wallet to submit.');
    62|	}, []);
    63|
    64|	return (
    65|		<div className="min-h-screen bg-background grid-bg text-foreground">
    66|			<Header onNavigate={handleNavigate} currentView={view} />
    67|
    68|			{/* Main content — padded for fixed header */}
    69|			<main className="pt-16">
    70|				{view === 'landing' ? (
    71|					<LandingPage onViewProposals={handleViewProposals} />
    72|				) : (
    73|					<ProposalList
    74|						proposals={proposals}
    75|						votedProposals={votedProposals}
    76|						onVote={handleVote}
    77|						onViewResults={handleViewResults}
    78|						onCreateProposal={handleCreateProposal}
    79|					/>
    80|				)}
    81|			</main>
    82|
    83|			{/* Vote Modal */}
    84|			{voteModalProposal && (
    85|				<VoteModal
    86|					proposal={voteModalProposal}
    87|					onClose={() => setVoteModalProposal(null)}
    88|					onVoteComplete={handleVoteComplete}
    89|				/>
    90|			)}
    91|
    92|			{/* Results Modal */}
    93|			{resultsModalProposal && (
    94|				<ResultsModal
    95|					proposal={resultsModalProposal}
    96|					onClose={() => setResultsModalProposal(null)}
    97|				/>
    98|			)}
    99|
   100|			{/* Toast */}
   101|			{toast && (
   102|				<Toast message={toast} onDone={() => setToast(null)} />
   103|			)}
   104|		</div>
   105|	);
   106|}
   107|