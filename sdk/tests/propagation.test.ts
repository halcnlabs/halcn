import {
  computeDecayChain, estimatePropagationLatency,
  confidenceIntervalWidth, canPropagate, criticalHopCount,
} from "../src/propagation";

describe("computeDecayChain", () => {
  test("empty chain returns 1.0", () => { expect(computeDecayChain([])).toBe(1.0); });
  test("full retention returns 1.0", () => { expect(computeDecayChain([10_000, 10_000])).toBe(1.0); });
  test("50% decay per hop across two hops", () => {
    expect(computeDecayChain([5_000, 5_000])).toBeCloseTo(0.25, 4);
  });
  test("single hop 90% retention", () => {
    expect(computeDecayChain([9_000])).toBeCloseTo(0.9, 4);
  });
  test("three hops with varying decay", () => {
    expect(computeDecayChain([8_000, 9_000, 7_000])).toBeCloseTo(0.504, 3);
  });
});

describe("estimatePropagationLatency", () => {
  test("sums all edge weights", () => { expect(estimatePropagationLatency([100, 200, 300])).toBe(600); });
  test("empty weights returns 0", () => { expect(estimatePropagationLatency([])).toBe(0); });
  test("single weight returns itself", () => { expect(estimatePropagationLatency([42])).toBe(42); });
});

describe("confidenceIntervalWidth", () => {
  test("95% confidence on 1000 bps", () => { expect(confidenceIntervalWidth(1000, 9500)).toBe(50); });
  test("99% confidence narrows interval", () => { expect(confidenceIntervalWidth(1000, 9900)).toBe(10); });
  test("50% confidence widens interval", () => { expect(confidenceIntervalWidth(1000, 5000)).toBe(500); });
});

describe("canPropagate", () => {
  test("strong signal propagates", () => { expect(canPropagate(1.0, 3, 9000, 0.5)).toBe(true); });
  test("weak signal fails", () => { expect(canPropagate(0.1, 5, 5000, 0.5)).toBe(false); });
  test("zero hops always works", () => { expect(canPropagate(0.01, 0, 5000, 0.001)).toBe(true); });
});

describe("criticalHopCount", () => {
  test("returns max hops before threshold", () => { expect(criticalHopCount(1.0, 5000, 0.1)).toBe(3); });
  test("no decay returns Infinity", () => { expect(criticalHopCount(1.0, 10000, 0.1)).toBe(Infinity); });
  test("zero decay returns 0", () => { expect(criticalHopCount(1.0, 0, 0.1)).toBe(0); });
});
