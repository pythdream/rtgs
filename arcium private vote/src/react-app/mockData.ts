     1|import type { Proposal } from './types';
     2|
     3|export const MOCK_PROPOSALS: Proposal[] = [
     4|  {
     5|    id: '1',
     6|    title: 'Treasury Allocation Q1 2025',
     7|    description:
     8|      'Allocate 500,000 USDC from the DAO treasury to fund protocol development, security audits, and community grants for Q1 2025.',
     9|    options: ['Approve', 'Reject', 'Abstain'],
    10|    status: 'active',
    11|    voteCount: 847,
    12|    votingEnd: Date.now() + 2 * 24 * 60 * 60 * 1000,
    13|    creator: '7xKX...9mPq',
    14|  },
    15|  {
    16|    id: '2',
    17|    title: 'Protocol Upgrade v2.4',
    18|    description:
    19|      'Implement the proposed smart contract upgrades including gas optimizations, new staking mechanics, and cross-chain bridge support.',
    20|    options: ['For', 'Against', 'Abstain'],
    21|    status: 'active',
    22|    voteCount: 1203,
    23|    votingEnd: Date.now() + 5 * 24 * 60 * 60 * 1000,
    24|    creator: '3pRt...7kLm',
    25|  },
    26|  {
    27|    id: '3',
    28|    title: 'Community Fund Expansion',
    29|    description:
    30|      'Expand the community grants program by 200% to support ecosystem builders, hackathon prizes, and educational content creators.',
    31|    options: ['For', 'Against', 'Abstain'],
    32|    status: 'finalized',
    33|    voteCount: 2341,
    34|    votingEnd: Date.now() - 3 * 24 * 60 * 60 * 1000,
    35|    creator: '9mNx...2pQr',
    36|    results: { For: 1456, Against: 612, Abstain: 273 },
    37|  },
    38|  {
    39|    id: '4',
    40|    title: 'Governance Parameter Update',
    41|    description:
    42|      'Update quorum requirements from 10% to 15% of total supply and extend voting periods from 3 days to 5 days for major proposals.',
    43|    options: ['Accept', 'Reject', 'Modify'],
    44|    status: 'finalized',
    45|    voteCount: 1876,
    46|    votingEnd: Date.now() - 7 * 24 * 60 * 60 * 1000,
    47|    creator: '5kLp...8nWs',
    48|    results: { Accept: 987, Reject: 445, Modify: 444 },
    49|  },
    50|];
    51|