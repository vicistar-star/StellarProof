export default function HomePage() {
  return (
    <main className="flex flex-col items-center justify-center min-h-screen px-4 text-center">
      <h1 className="text-5xl font-bold text-yellow-400 mb-4">⭐ StellarProof</h1>
      <p className="text-xl text-gray-300 max-w-xl mb-8">
        The anti-deepfake truth layer for the Stellar ecosystem. Register
        original media on-chain and expose AI-generated content.
      </p>
      <a
        href="/creator/submit"
        className="px-6 py-3 bg-yellow-400 text-gray-950 font-semibold rounded-lg hover:bg-yellow-300 transition-colors"
      >
        Submit Media for Analysis
      </a>
    </main>
  );
}
