"use client";

import type { CreatorReputation } from "@stellarproof/shared/types";

interface Props {
  reputation: CreatorReputation;
}

export default function CreatorScore({ reputation }: Props) {
  const trustLevel =
    reputation.score >= 100
      ? { label: "⭐ Trusted Creator", color: "text-yellow-400" }
      : reputation.score >= 50
      ? { label: "✅ Verified", color: "text-green-400" }
      : reputation.score >= 0
      ? { label: "🔍 Unverified", color: "text-gray-400" }
      : { label: "⚠️ Flagged", color: "text-red-400" };

  return (
    <div className="rounded-xl border border-gray-700 bg-gray-900 p-6 space-y-4">
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-semibold text-gray-100">Creator Reputation</h2>
        <span className={`font-bold ${trustLevel.color}`}>{trustLevel.label}</span>
      </div>

      <div className="text-center">
        <span className="text-5xl font-bold text-yellow-400">{reputation.score}</span>
        <p className="text-sm text-gray-400 mt-1">Reputation Score</p>
      </div>

      <dl className="grid grid-cols-2 gap-x-4 gap-y-2 text-sm">
        <dt className="text-gray-400">Total Submissions</dt>
        <dd className="text-gray-100">{reputation.totalSubmissions}</dd>

        <dt className="text-gray-400">Verified Authentic</dt>
        <dd className="text-green-400">{reputation.verifiedAuthentic}</dd>

        <dt className="text-gray-400">Flagged Synthetic</dt>
        <dd className="text-red-400">{reputation.flaggedSynthetic}</dd>

        <dt className="text-gray-400">Disputed</dt>
        <dd className="text-yellow-400">{reputation.disputed}</dd>

        <dt className="text-gray-400">Stellar Key</dt>
        <dd className="text-gray-100 truncate font-mono text-xs">{reputation.stellarKey}</dd>

        <dt className="text-gray-400">Last Updated</dt>
        <dd className="text-gray-100">{new Date(reputation.lastUpdated).toLocaleDateString()}</dd>
      </dl>
    </div>
  );
}
