import { PublicKey } from "@solana/web3.js";

export const PROGRAM_ID = new PublicKey("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

export const SEEDS = {
  signal: Buffer.from("signal"),
  propagation: Buffer.from("propagation"),
  prediction: Buffer.from("prediction"),
  protocol: Buffer.from("protocol"),
} as const;

export const BPS = 10_000;

export const DEFAULTS = {
  windowMs: 5_000,
  confidenceBps: 9_500,
  timeHorizonMs: 60_000,
  maxPathHops: 8,
  maxMarketLen: 32,
} as const;
