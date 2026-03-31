import { BPS } from "./constants";

export function computeDecayChain(decayFactors: number[]): number {
  let magnitude = 1.0;
  for (const decay of decayFactors) magnitude *= decay / BPS;
  return magnitude;
}

export function estimatePropagationLatency(edgeWeights: number[]): number {
  return edgeWeights.reduce((total, weight) => total + weight, 0);
}

export function confidenceIntervalWidth(magnitudeBps: number, confidenceBps: number): number {
  return ((BPS - confidenceBps) * magnitudeBps) / BPS;
}

export function canPropagate(initialStrength: number, hops: number, minDecayPerHop: number, threshold: number): boolean {
  return initialStrength * Math.pow(minDecayPerHop / BPS, hops) >= threshold;
}

export function criticalHopCount(initialStrength: number, decayPerHop: number, threshold: number): number {
  if (decayPerHop >= BPS) return Infinity;
  if (decayPerHop <= 0) return 0;
  return Math.max(0, Math.floor(Math.log(threshold / initialStrength) / Math.log(decayPerHop / BPS)));
}
