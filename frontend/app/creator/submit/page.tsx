"use client";

import { useState } from "react";

export default function SubmitPage() {
  const [stellarKey, setStellarKey] = useState("");
  const [file, setFile] = useState<File | null>(null);
  const [status, setStatus] = useState<"idle" | "submitting" | "submitted" | "error">("idle");
  const [jobId, setJobId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!file || !stellarKey) return;

    setStatus("submitting");
    setError(null);

    try {
      const form = new FormData();
      form.append("file", file);
      form.append("creator", stellarKey);

      const res = await fetch("/api/detect/submit", { method: "POST", body: form });
      if (!res.ok) throw new Error(await res.text());

      const data = await res.json();
      setJobId(data.jobId);
      setStatus("submitted");
    } catch (err) {
      setError(err instanceof Error ? err.message : "Submission failed");
      setStatus("error");
    }
  }

  return (
    <main className="max-w-xl mx-auto px-4 py-16">
      <h1 className="text-3xl font-bold text-yellow-400 mb-2">Submit Media</h1>
      <p className="text-gray-400 mb-8">
        Upload a video or image to register it on-chain and run deepfake analysis.
      </p>

      <form onSubmit={handleSubmit} className="space-y-5">
        <div>
          <label className="block text-sm text-gray-300 mb-1" htmlFor="stellarKey">
            Stellar Public Key
          </label>
          <input
            id="stellarKey"
            type="text"
            placeholder="G..."
            value={stellarKey}
            onChange={(e) => setStellarKey(e.target.value)}
            className="w-full bg-gray-800 border border-gray-600 rounded-lg px-3 py-2 text-gray-100 font-mono text-sm focus:outline-none focus:border-yellow-400"
            required
          />
        </div>

        <div>
          <label className="block text-sm text-gray-300 mb-1" htmlFor="mediaFile">
            Media File (image or video)
          </label>
          <input
            id="mediaFile"
            type="file"
            accept="image/*,video/*"
            onChange={(e) => setFile(e.target.files?.[0] ?? null)}
            className="w-full text-sm text-gray-400 file:mr-3 file:py-2 file:px-4 file:rounded-lg file:border-0 file:bg-yellow-400 file:text-gray-950 file:font-semibold hover:file:bg-yellow-300"
            required
          />
        </div>

        <button
          type="submit"
          disabled={status === "submitting"}
          className="w-full py-3 bg-yellow-400 text-gray-950 font-semibold rounded-lg hover:bg-yellow-300 disabled:opacity-50 transition-colors"
        >
          {status === "submitting" ? "Submitting…" : "Submit for Analysis"}
        </button>
      </form>

      {status === "submitted" && jobId && (
        <div className="mt-6 p-4 rounded-lg bg-green-900/40 border border-green-700 text-green-300">
          <p className="font-semibold">Submitted successfully!</p>
          <p className="text-sm mt-1">
            Job ID: <span className="font-mono">{jobId}</span>
          </p>
          <a
            href={`/api/detect/status/${jobId}`}
            className="text-sm underline mt-2 inline-block"
          >
            Check status →
          </a>
        </div>
      )}

      {status === "error" && error && (
        <div className="mt-6 p-4 rounded-lg bg-red-900/40 border border-red-700 text-red-300">
          {error}
        </div>
      )}
    </main>
  );
}
