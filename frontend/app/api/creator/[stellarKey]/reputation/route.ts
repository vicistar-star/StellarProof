import { NextRequest, NextResponse } from "next/server";

export async function GET(
  _req: NextRequest,
  { params }: { params: Promise<{ stellarKey: string }> }
) {
  const { stellarKey } = await params;

  if (!/^G[A-Z2-7]{55}$/.test(stellarKey)) {
    return NextResponse.json({ error: "invalid Stellar public key" }, { status: 400 });
  }

  // TODO: query on-chain reputation contract via Stellar RPC
  return NextResponse.json({
    stellarKey,
    score: 0,
    totalSubmissions: 0,
    verifiedAuthentic: 0,
    flaggedSynthetic: 0,
    disputed: 0,
    lastUpdated: new Date().toISOString(),
  });
}
