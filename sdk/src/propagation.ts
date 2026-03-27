import { BPS } from "./constants";

export function computeDecayChain(decayFactors: number[]): number {
  let magnitude = 1.0;
  for (const decay of decayFactors) {
    magnitude *= decay / BPS;
  }
  return magnitude;
}

export function estimatePropagationLatency(edgeWeights: number[]): number {
  return edgeWeights.reduce((total, weight) => total + weight, 0);
}
