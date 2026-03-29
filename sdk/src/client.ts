import { Connection, PublicKey } from "@solana/web3.js";
import { AnchorProvider, Wallet } from "@coral-xyz/anchor";
import type {
  SignalConfig, SignalRecord, PropagationNode, PropagationPathRecord,
  ImpactPredictionRecord, PreArrivalAlert, HalcnClientConfig,
} from "./types";
import { computeDecayChain } from "./propagation";
import { PROGRAM_ID, SEEDS, BPS, DEFAULTS } from "./constants";
import { decimalToBps } from "./utils";

export class HalcnClient {
  private connection: Connection;
  private wallet: Wallet;
  private provider: AnchorProvider;
  private listeners: Map<number, (alert: PreArrivalAlert) => void>;
  private nextListenerId: number;

  constructor(connection: Connection, wallet: Wallet);
  constructor(config: HalcnClientConfig);
  constructor(connOrConfig: Connection | HalcnClientConfig, wallet?: Wallet) {
    if (connOrConfig instanceof Connection) {
      this.connection = connOrConfig;
      this.wallet = wallet!;
    } else {
      this.connection = connOrConfig.connection;
      this.wallet = connOrConfig.wallet;
    }
    this.provider = new AnchorProvider(this.connection, this.wallet, {
      commitment: "confirmed",
    });
    this.listeners = new Map();
    this.nextListenerId = 0;
  }

  getSignalPDA(authority: PublicKey, sourceMarket: string): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [SEEDS.signal, authority.toBuffer(), Buffer.from(sourceMarket)],
      PROGRAM_ID,
    );
    return pda;
  }

  getPropagationPDA(signalKey: PublicKey): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [SEEDS.propagation, signalKey.toBuffer()],
      PROGRAM_ID,
    );
    return pda;
  }

  getPredictionPDA(propagationKey: PublicKey): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [SEEDS.prediction, propagationKey.toBuffer()],
      PROGRAM_ID,
    );
    return pda;
  }

  async detectSignal(config: SignalConfig): Promise<SignalRecord> {
    const authority = this.wallet.publicKey;
    const signalPDA = this.getSignalPDA(authority, config.source);
    return {
      publicKey: signalPDA, authority,
      sourceMarket: config.source,
      threshold: decimalToBps(config.threshold),
      windowMs: config.windowMs,
      detectedAt: Date.now(), consumed: false, signalIndex: 0,
    };
  }

  async getPropagationPath(signalKey: PublicKey, nodes: PropagationNode[] = []): Promise<PropagationPathRecord> {
    const authority = this.wallet.publicKey;
    const propPDA = this.getPropagationPDA(signalKey);
    const pathNodes = nodes.map((n) => n.market);
    const edgeWeights = nodes.slice(1).map((n) => decimalToBps(n.weight));
    const decayFactors = nodes.slice(1).map((n) => decimalToBps(n.decay));
    return {
      publicKey: propPDA, signal: signalKey, authority, pathNodes, edgeWeights, decayFactors,
      totalLatencyMs: edgeWeights.reduce((a, b) => a + b, 0),
      hopCount: Math.max(0, pathNodes.length - 1),
      computedAt: Date.now(),
    };
  }

  async predictImpact(propagationKey: PublicKey, confidenceBps = DEFAULTS.confidenceBps, timeHorizonMs = DEFAULTS.timeHorizonMs): Promise<ImpactPredictionRecord> {
    const authority = this.wallet.publicKey;
    const predPDA = this.getPredictionPDA(propagationKey);
    return {
      publicKey: predPDA, propagationPath: propagationKey, authority,
      predictedMagnitudeBps: 0, confidenceLowerBps: 0, confidenceUpperBps: 0,
      confidenceLevelBps: confidenceBps, etaMs: timeHorizonMs,
      predictedAt: Date.now(), alertEmitted: false, targetMarket: "",
    };
  }

  subscribeToAlerts(callback: (alert: PreArrivalAlert) => void): number {
    const id = this.nextListenerId++;
    this.listeners.set(id, callback);
    return id;
  }

  unsubscribeFromAlerts(listenerId: number): boolean {
    return this.listeners.delete(listenerId);
  }

  getActiveListenerCount(): number { return this.listeners.size; }
  getConnection(): Connection { return this.connection; }
  getWalletPublicKey(): PublicKey { return this.wallet.publicKey; }
}
