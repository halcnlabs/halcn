import { BPS } from "./constants";

export function bpsToDecimal(bps: number): number {
  return bps / BPS;
}

export function decimalToBps(decimal: number): number {
  return Math.round(decimal * BPS);
}

export function formatEta(etaMs: number): string {
  if (etaMs < 1000) return `${etaMs}ms`;
  return `${(etaMs / 1000).toFixed(1)}s`;
}

export function isValidMarketName(name: string, maxLen: number = 32): boolean {
  if (!name || name.length === 0 || name.length > maxLen) return false;
  return /^[\x20-\x7E]+$/.test(name);
}
