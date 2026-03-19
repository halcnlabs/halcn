import { PublicKey, Connection } from "@solana/web3.js";
import { Wallet } from "@coral-xyz/anchor";

export interface HalcnClientConfig {
  connection: Connection;
  wallet: Wallet;
  commitment?: "processed" | "confirmed" | "finalized";
}

export interface SignalConfig {
  source: string;
  threshold: number;
  windowMs: number;
}

export interface SignalRecord {
  publicKey: PublicKey;
  authority: PublicKey;
  sourceMarket: string;
  threshold: number;
  windowMs: number;
  detectedAt: number;
  consumed: boolean;
  signalIndex: number;
}

export interface PropagationNode {
  market: string;
  weight: number;
  decay: number;
}

export interface PropagationPathRecord {
  publicKey: PublicKey;
  signal: PublicKey;
  authority: PublicKey;
  pathNodes: string[];
  edgeWeights: number[];
  decayFactors: number[];
  totalLatencyMs: number;
  hopCount: number;
  computedAt: number;
}

export interface ImpactPredictionRecord {
  publicKey: PublicKey;
  propagationPath: PublicKey;
  authority: PublicKey;
  predictedMagnitudeBps: number;
  confidenceLowerBps: number;
  confidenceUpperBps: number;
  confidenceLevelBps: number;
  etaMs: number;
  predictedAt: number;
  alertEmitted: boolean;
  targetMarket: string;
}

export interface PreArrivalAlert {
  target: string;
  estimatedImpact: number;
  confidence: number;
  etaMs: number;
  signalKey: PublicKey;
  timestamp: number;
}
