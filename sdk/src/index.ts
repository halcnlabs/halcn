export { HalcnClient } from "./client";
export {
  computeDecayChain,
  estimatePropagationLatency,
  confidenceIntervalWidth,
  canPropagate,
} from "./propagation";
export { PROGRAM_ID, SEEDS, BPS, DEFAULTS } from "./constants";
export { HalcnSDKError, parseErrorCode } from "./errors";
export { bpsToDecimal, decimalToBps, formatEta } from "./utils";
export type {
  SignalConfig,
  SignalRecord,
  PropagationNode,
  PropagationPathRecord,
  ImpactPredictionRecord,
  PreArrivalAlert,
  HalcnClientConfig,
} from "./types";
