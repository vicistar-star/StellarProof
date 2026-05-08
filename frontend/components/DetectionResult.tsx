"use client";

import type { DetectionCertificate } from "@stellarproof/shared/types";

interface Props {
  certificate: DetectionCertificate;
}

const VERDICT_LABEL: Record<string, { label: string; color: string }> = {
  HUMAN_CREATED: { label: "✅ Human Created", color: "text-green-400" },
  AI_GENERATED: { label: "🤖 AI Generated", color: "text-red-400" },
  INCONCLUSIVE: { label: "⚠️ Inconclusive", color: "text-yellow-400" },
};

export default function DetectionResult({ certificate }: Props) {
  const { label, color } = VERDICT_LABEL[certificate.verdict] ?? VERDICT_LABEL.INCONCLUSIVE;

  return (
    <div className="rounded-xl border border-gray-700 bg-gray-900 p-6 space-y-4">
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-semibold text-gray-100">Detection Result</h2>
        <span className={`text-xl font-bold ${color}`}>{label}</span>
      </div>

      {/* Authenticity score bar */}
      <div>
        <div className="flex justify-between text-sm text-gray-400 mb-1">
          <span>Authenticity Score</span>
          <span>{certificate.authenticityScore}/100</span>
        </div>
        <div className="w-full bg-gray-700 rounded-full h-2">
          <div
            className="bg-yellow-400 h-2 rounded-full transition-all"
            style={{ width: `${certificate.authenticityScore}%` }}
          />
        </div>
      </div>

      <dl className="grid grid-cols-2 gap-x-4 gap-y-2 text-sm">
        <dt className="text-gray-400">Confidence</dt>
        <dd className="text-gray-100">{(certificate.confidence * 100).toFixed(1)}%</dd>

        <dt className="text-gray-400">Certificate ID</dt>
        <dd className="text-gray-100 truncate font-mono text-xs">{certificate.certId}</dd>

        <dt className="text-gray-400">Content Hash</dt>
        <dd className="text-gray-100 truncate font-mono text-xs">{certificate.contentHash}</dd>

        <dt className="text-gray-400">Model Hash</dt>
        <dd className="text-gray-100 truncate font-mono text-xs">{certificate.modelVersionHash}</dd>

        <dt className="text-gray-400">Timestamp</dt>
        <dd className="text-gray-100">{new Date(certificate.timestamp).toLocaleString()}</dd>

        <dt className="text-gray-400">Challenge Status</dt>
        <dd className="text-gray-100">{certificate.challengeStatus}</dd>
      </dl>
    </div>
  );
}
