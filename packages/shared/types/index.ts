// Detection certificate fields committed on-chain
export interface DetectionCertificate {
  certId: string;
  contentHash: string;         // sha256:...
  authenticityScore: number;   // 0–100
  confidence: number;          // 0.0–1.0
  verdict: Verdict;
  modelVersionHash: string;
  attestationProof: string;
  timestamp: string;           // ISO 8601
  creator: string;             // Stellar public key (G...)
  challengeStatus: ChallengeStatus;
  challengeDeadline: string;
}

// Media manifest submitted by creator before analysis
export interface DetectionManifest {
  contentHash: string;
  creator: string;
  timestamp: string;
  mediaType: string;
  metadata: MediaMetadata;
}

export interface MediaMetadata {
  durationSeconds?: number;
  resolution?: string;
  captureDevice?: string;
  aiModelClaim: string;
}

export interface CreatorReputation {
  stellarKey: string;
  score: number;
  totalSubmissions: number;
  verifiedAuthentic: number;
  flaggedSynthetic: number;
  disputed: number;
  lastUpdated: string;
}

export interface DetectionJob {
  jobId: string;
  contentHash: string;
  creator: string;
  status: JobStatus;
  createdAt: string;
  completedAt?: string;
  certId?: string;
}

export interface ChallengeRequest {
  certId: string;
  challenger: string;
  reason: string;
  stakeAmount: number; // XLM
  timestamp: string;
}

export type Verdict = "HUMAN_CREATED" | "AI_GENERATED" | "INCONCLUSIVE";
export type ChallengeStatus = "NONE" | "PENDING" | "RESOLVED_UPHELD" | "RESOLVED_OVERTURNED";
export type JobStatus = "QUEUED" | "PROCESSING" | "COMPLETE" | "FAILED";
