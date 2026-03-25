import { Connection, PublicKey } from "@solana/web3.js";
import { AnchorProvider, Wallet } from "@coral-xyz/anchor";
import type { SignalConfig, SignalRecord, HalcnClientConfig } from "./types";
import { PROGRAM_ID, SEEDS, BPS } from "./constants";
import { decimalToBps } from "./utils";

export class HalcnClient {
  private connection: Connection;
  private wallet: Wallet;
  private provider: AnchorProvider;

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
  }

  getSignalPDA(authority: PublicKey, sourceMarket: string): PublicKey {
    const [pda] = PublicKey.findProgramAddressSync(
      [SEEDS.signal, authority.toBuffer(), Buffer.from(sourceMarket)],
      PROGRAM_ID,
    );
    return pda;
  }

  async detectSignal(config: SignalConfig): Promise<SignalRecord> {
    const authority = this.wallet.publicKey;
    const signalPDA = this.getSignalPDA(authority, config.source);
    return {
      publicKey: signalPDA,
      authority,
      sourceMarket: config.source,
      threshold: decimalToBps(config.threshold),
      windowMs: config.windowMs,
      detectedAt: Date.now(),
      consumed: false,
      signalIndex: 0,
    };
  }
}
