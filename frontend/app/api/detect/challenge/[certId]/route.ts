import { NextRequest, NextResponse } from "next/server";
import { randomUUID } from "crypto";

export async function POST(
  req: NextRequest,
  { params }: { params: Promise<{ certId: string }> }
) {
  const { certId } = await params;
  const body = await req.json().catch(() => null);

  if (!body?.challenger || !body?.reason) {
    return NextResponse.json(
      { error: "challenger and reason are required" },
      { status: 400 }
    );
  }

  if (!/^G[A-Z2-7]{55}$/.test(body.challenger)) {
    return NextResponse.json({ error: "invalid challenger Stellar key" }, { status: 400 });
  }

  // TODO: verify XLM stake, submit challenge to on-chain detection contract
  const challengeId = randomUUID();

  return NextResponse.json({
    challengeId,
    certId,
    challenger: body.challenger,
    reason: body.reason,
    status: "PENDING",
    timestamp: new Date().toISOString(),
    message: "Challenge submitted. Re-analysis will be triggered in a fresh TEE instance.",
  });
}
