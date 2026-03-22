const ERROR_CODES: Record<number, string> = {
  6000: "ThresholdOutOfRange",
  6001: "WindowOutOfRange",
  6002: "PathTooLong",
  6003: "PathWeightMismatch",
  6004: "PathDecayMismatch",
  6005: "SignalAlreadyConsumed",
  6006: "ConfidenceOutOfRange",
  6007: "TimeHorizonTooLarge",
  6008: "MarketNameTooLong",
  6009: "EmptyPath",
  6010: "DecayFactorInvalid",
  6011: "DecayFactorTooLow",
  6012: "Unauthorized",
  6013: "MathOverflow",
  6014: "InvalidVersion",
};

export class HalcnSDKError extends Error {
  public readonly code: number;
  public readonly errorName: string;

  constructor(code: number, message?: string) {
    const name = ERROR_CODES[code] || "UnknownError";
    super(message || name);
    this.code = code;
    this.errorName = name;
    this.name = "HalcnSDKError";
  }
}

export function parseErrorCode(code: number): string {
  return ERROR_CODES[code] || `Unknown error code: ${code}`;
}
