     1|import { useState, useEffect } from 'react';
     2|import { X, Lock, Link, CheckCircle, Shield } from 'lucide-react';
     3|import type { Proposal, VoteStep } from '../types';
     4|
     5|interface VoteModalProps {
     6|  proposal: Proposal;
     7|  onClose: () => void;
     8|  onVoteComplete: (proposalId: string) => void;
     9|}
    10|
    11|const VOTE_STEPS: { step: VoteStep; icon: React.ReactNode; label: string; duration: number }[] = [
    12|  {
    13|    step: 'encrypting',
    14|    icon: <Lock className="w-8 h-8 text-purple-400 animate-spin-slow" />,
    15|    label: 'Encrypting your vote with Arcium MPC...',
    16|    duration: 1500,
    17|  },
    18|  {
    19|    step: 'submitting',
    20|    icon: <Link className="w-8 h-8 text-cyan-400 animate-chain-pulse" />,
    21|    label: 'Submitting to Solana...',
    22|    duration: 1500,
    23|  },
    24|  {
    25|    step: 'confirmed',
    26|    icon: <CheckCircle className="w-8 h-8 text-green-400 animate-check-pop" />,
    27|    label: 'Vote confirmed! Your choice remains private.',
    28|    duration: 1000,
    29|  },
    30|];
    31|
    32|export default function VoteModal({ proposal, onClose, onVoteComplete }: VoteModalProps) {
    33|  const [selectedOption, setSelectedOption] = useState<string | null>(null);
    34|  const [voteStep, setVoteStep] = useState<VoteStep>('idle');
    35|  const [currentStepIndex, setCurrentStepIndex] = useState(0);
    36|
    37|  const isAnimating = voteStep !== 'idle';
    38|
    39|  const handleCastVote = () => {
    40|    if (!selectedOption) return;
    41|    setVoteStep('encrypting');
    42|    setCurrentStepIndex(0);
    43|  };
    44|
    45|  useEffect(() => {
    46|    if (voteStep === 'idle') return;
    47|
    48|    const stepDef = VOTE_STEPS[currentStepIndex];
    49|    if (!stepDef) return;
    50|
    51|    const timer = setTimeout(() => {
    52|      if (currentStepIndex < VOTE_STEPS.length - 1) {
    53|        const nextStep = VOTE_STEPS[currentStepIndex + 1];
    54|        setCurrentStepIndex(currentStepIndex + 1);
    55|        setVoteStep(nextStep.step);
    56|      } else {
    57|        // Done — complete
    58|        setTimeout(() => {
    59|          onVoteComplete(proposal.id);
    60|          onClose();
    61|        }, 600);
    62|      }
    63|    }, stepDef.duration);
    64|
    65|    return () => clearTimeout(timer);
    66|  }, [voteStep, currentStepIndex, proposal.id, onVoteComplete, onClose]);
    67|
    68|  const optionColors: Record<number, { base: string; selected: string }> = {
    69|    0: {
    70|      base: 'border-slate-700/50 hover:border-green-500/40 hover:bg-green-500/5',
    71|      selected: 'border-green-500/60 bg-green-500/10 shadow-[0_0_20px_rgba(34,197,94,0.15)]',
    72|    },
    73|    1: {
    74|      base: 'border-slate-700/50 hover:border-red-500/40 hover:bg-red-500/5',
    75|      selected: 'border-red-500/60 bg-red-500/10 shadow-[0_0_20px_rgba(239,68,68,0.15)]',
    76|    },
    77|    2: {
    78|      base: 'border-slate-700/50 hover:border-slate-500/40 hover:bg-slate-500/5',
    79|      selected: 'border-slate-400/60 bg-slate-500/10 shadow-[0_0_20px_rgba(148,163,184,0.1)]',
    80|    },
    81|  };
    82|
    83|  const optionIcons = ['✅', '❌', '⚪'];
    84|
    85|  const currentStep = VOTE_STEPS[currentStepIndex];
    86|
    87|  return (
    88|    <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
    89|      {/* Backdrop */}
    90|      <div
    91|        className="absolute inset-0 bg-black/70 backdrop-blur-sm animate-fade-in"
    92|        onClick={!isAnimating ? onClose : undefined}
    93|      />
    94|
    95|      {/* Modal */}
    96|      <div className="relative w-full max-w-lg glass rounded-2xl border border-white/10 animate-fade-in-up overflow-hidden">
    97|        {/* Header */}
    98|        <div className="flex items-start justify-between p-6 border-b border-white/[0.06]">
    99|          <div className="flex-1 pr-4">
   100|            <div className="flex items-center gap-2 mb-1">
   101|              <Shield className="w-4 h-4 text-purple-400" />
   102|              <span className="text-xs text-purple-400 font-medium">MPC Protected Vote</span>
   103|            </div>
   104|            <h2 className="text-lg font-bold text-white leading-snug">{proposal.title}</h2>
   105|          </div>
   106|          {!isAnimating && (
   107|            <button
   108|              onClick={onClose}
   109|              className="flex-shrink-0 w-8 h-8 rounded-lg bg-white/5 hover:bg-white/10 flex items-center justify-center transition-colors"
   110|            >
   111|              <X className="w-4 h-4 text-slate-400" />
   112|            </button>
   113|          )}
   114|        </div>
   115|
   116|        <div className="p-6">
   117|          {!isAnimating ? (
   118|            <>
   119|              {/* Description */}
   120|              <p className="text-sm text-slate-400 mb-6 leading-relaxed">{proposal.description}</p>
   121|
   122|              {/* Options */}
   123|              <div className="space-y-3 mb-6">
   124|                <p className="text-xs font-medium text-slate-500 uppercase tracking-wider">Select your vote</p>
   125|                {proposal.options.map((option, i) => {
   126|                  const colors = optionColors[i] ?? optionColors[2];
   127|                  const isSelected = selectedOption === option;
   128|                  return (
   129|                    <button
   130|                      key={option}
   131|                      onClick={() => setSelectedOption(option)}
   132|                      className={`w-full flex items-center gap-4 p-4 rounded-xl border transition-all duration-200 text-left ${
   133|                        isSelected ? colors.selected : `glass ${colors.base}`
   134|                      }`}
   135|                    >
   136|                      <span className="text-2xl">{optionIcons[i] ?? '⚪'}</span>
   137|                      <div>
   138|                        <div className="text-white font-semibold">{option}</div>
   139|                        {isSelected && (
   140|                          <div className="text-xs text-slate-400 mt-0.5">Selected</div>
   141|                        )}
   142|                      </div>
   143|                      {isSelected && (
   144|                        <div className="ml-auto w-5 h-5 rounded-full bg-purple-500 flex items-center justify-center">
   145|                          <CheckCircle className="w-3.5 h-3.5 text-white" />
   146|                        </div>
   147|                      )}
   148|                    </button>
   149|                  );
   150|                })}
   151|              </div>
   152|
   153|              {/* Privacy notice */}
   154|              <div className="flex items-start gap-2 p-3 rounded-lg bg-purple-500/5 border border-purple-500/15 mb-6">
   155|                <Lock className="w-4 h-4 text-purple-400 flex-shrink-0 mt-0.5" />
   156|                <p className="text-xs text-slate-400 leading-relaxed">
   157|                  Your vote is encrypted client-side using X25519 before transmission. Individual votes are never revealed — only the final tally is decrypted by Arcium's MPC network.
   158|                </p>
   159|              </div>
   160|
   161|              {/* Cast Vote button */}
   162|              <button
   163|                onClick={handleCastVote}
   164|                disabled={!selectedOption}
   165|                className={`w-full py-3.5 rounded-xl font-semibold text-sm transition-all duration-200 ${
   166|                  selectedOption
   167|                    ? 'bg-purple-600 hover:bg-purple-500 text-white purple-glow hover:scale-[1.02]'
   168|                    : 'bg-slate-800 text-slate-500 cursor-not-allowed'
   169|                }`}
   170|              >
   171|                {selectedOption ? `Cast Vote: ${selectedOption}` : 'Select an option to vote'}
   172|              </button>
   173|            </>
   174|          ) : (
   175|            /* Animation flow */
   176|            <div className="py-8 flex flex-col items-center text-center">
   177|              {/* Progress dots */}
   178|              <div className="flex gap-2 mb-8">
   179|                {VOTE_STEPS.map((s, i) => (
   180|                  <div
   181|                    key={s.step}
   182|                    className={`h-1.5 rounded-full transition-all duration-500 ${
   183|                      i < currentStepIndex
   184|                        ? 'w-8 bg-green-500'
   185|                        : i === currentStepIndex
   186|                        ? 'w-8 bg-purple-500'
   187|                        : 'w-4 bg-slate-700'
   188|                    }`}
   189|                  />
   190|                ))}
   191|              </div>
   192|
   193|              {/* Icon */}
   194|              <div className="w-20 h-20 rounded-2xl bg-white/5 border border-white/10 flex items-center justify-center mb-6">
   195|                {currentStep?.icon}
   196|              </div>
   197|
   198|              {/* Step label */}
   199|              <p className="text-white font-semibold text-lg mb-2">
   200|                {currentStep?.label}
   201|              </p>
   202|
   203|              {/* Step number */}
   204|              <p className="text-xs text-slate-500">
   205|                Step {currentStepIndex + 1} of {VOTE_STEPS.length}
   206|              </p>
   207|
   208|              {/* Arcium branding */}
   209|              <div className="mt-8 flex items-center gap-2 px-4 py-2 rounded-full bg-purple-500/10 border border-purple-500/20">
   210|                <Shield className="w-3.5 h-3.5 text-purple-400" />
   211|                <span className="text-xs text-purple-400">Arcium MPC Network</span>
   212|              </div>
   213|            </div>
   214|          )}
   215|        </div>
   216|      </div>
   217|    </div>
   218|  );
   219|}
   220|