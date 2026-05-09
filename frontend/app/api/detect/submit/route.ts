import { NextRequest, NextResponse } from "next/server";
import { randomUUID } from "crypto";

export async function POST(req: NextRequest) {
  const form = await req.formData();
  const file = form.get("file") as File | null;
  const creator = form.get("creator") as string | null;

  if (!file || !creator) {
    return NextResponse.json({ error: "file and creator are required" }, { status: 400 });
  }

  if (!/^G[A-Z2-7]{55}$/.test(creator)) {
    return NextResponse.json({ error: "invalid Stellar public key" }, { status: 400 });
  }

  // Compute SHA-256 of the uploaded file
  const buffer = Buffer.from(await file.arrayBuffer());
  const hashBuffer = await crypto.subtle.digest("SHA-256", buffer);
  const contentHash = `sha256:${Buffer.from(hashBuffer).toString("hex")}`;

  const jobId = randomUUID();

  // TODO: persist job to DB and enqueue to TEE oracle worker
  return NextResponse.json({
    jobId,
    contentHash,
    creator,
    status: "QUEUED",
    createdAt: new Date().toISOString(),
  });
}
