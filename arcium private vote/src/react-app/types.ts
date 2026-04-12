     1|export interface Proposal {
     2|  id: string;
     3|  title: string;
     4|  description: string;
     5|  options: string[];
     6|  status: 'active' | 'finalized';
     7|  voteCount: number;
     8|  votingEnd: number;
     9|  creator: string;
    10|  results?: Record<string, number>;
    11|}
    12|
    13|export type VoteStep = 'idle' | 'encrypting' | 'submitting' | 'confirmed';
    14|export type TabFilter = 'all' | 'active' | 'finalized';
    15|export type View = 'landing' | 'dashboard';
    16|