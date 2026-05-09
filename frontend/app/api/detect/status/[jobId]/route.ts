import { NextRequest, NextResponse } from "next/server";

export async function GET(
  _req: NextRequest,
  { params }: { params: Promise<{ jobId: string }> }
) {
  const { jobId } = await params;

  if (!jobId) {
    return NextResponse.json({ error: "jobId is required" }, { status: 400 });
  }

  // TODO: query job status from DB / on-chain oracle contract
  return NextResponse.json({
    jobId,
    status: "QUEUED",
    message: "Job queued for TEE processing",
  });
}
