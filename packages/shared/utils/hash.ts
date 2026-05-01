import { createHash } from "crypto";
import { readFile } from "fs/promises";

/** SHA-256 hash of a Buffer or string, returned as "sha256:<hex>" */
export function hashBuffer(data: Buffer | string): string {
  const hex = createHash("sha256")
    .update(data)
    .digest("hex");
  return `sha256:${hex}`;
}

/** SHA-256 hash of a file at the given path */
export async function hashFile(filePath: string): Promise<string> {
  const data = await readFile(filePath);
  return hashBuffer(data);
}

/** Verify a content hash string matches expected format */
export function isValidContentHash(hash: string): boolean {
  return /^sha256:[a-f0-9]{64}$/.test(hash);
}

/** Verify a Stellar public key (G... 56 chars) */
export function isValidStellarKey(key: string): boolean {
  return /^G[A-Z2-7]{55}$/.test(key);
}
