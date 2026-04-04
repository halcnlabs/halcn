import { Keypair, PublicKey } from "@solana/web3.js";
import { PROGRAM_ID, SEEDS } from "../src/constants";

describe("PDA derivation", () => {
  test("signal PDA is deterministic", () => {
    const authority = Keypair.generate().publicKey;
    const market = "SOL/USDC";
    const [pda1] = PublicKey.findProgramAddressSync(
      [SEEDS.signal, authority.toBuffer(), Buffer.from(market)], PROGRAM_ID,
    );
    const [pda2] = PublicKey.findProgramAddressSync(
      [SEEDS.signal, authority.toBuffer(), Buffer.from(market)], PROGRAM_ID,
    );
    expect(pda1.equals(pda2)).toBe(true);
  });

  test("different markets yield different PDAs", () => {
    const authority = Keypair.generate().publicKey;
    const [pda1] = PublicKey.findProgramAddressSync(
      [SEEDS.signal, authority.toBuffer(), Buffer.from("SOL/USDC")], PROGRAM_ID,
    );
    const [pda2] = PublicKey.findProgramAddressSync(
      [SEEDS.signal, authority.toBuffer(), Buffer.from("ETH/USDC")], PROGRAM_ID,
    );
    expect(pda1.equals(pda2)).toBe(false);
  });

  test("different authorities yield different PDAs", () => {
    const auth1 = Keypair.generate().publicKey;
    const auth2 = Keypair.generate().publicKey;
    const [pda1] = PublicKey.findProgramAddressSync(
      [SEEDS.signal, auth1.toBuffer(), Buffer.from("SOL/USDC")], PROGRAM_ID,
    );
    const [pda2] = PublicKey.findProgramAddressSync(
      [SEEDS.signal, auth2.toBuffer(), Buffer.from("SOL/USDC")], PROGRAM_ID,
    );
    expect(pda1.equals(pda2)).toBe(false);
  });

  test("propagation PDA derives from signal key", () => {
    const signalKey = Keypair.generate().publicKey;
    const [pda] = PublicKey.findProgramAddressSync(
      [SEEDS.propagation, signalKey.toBuffer()], PROGRAM_ID,
    );
    expect(pda).toBeInstanceOf(PublicKey);
  });

  test("prediction PDA derives from propagation key", () => {
    const propKey = Keypair.generate().publicKey;
    const [pda] = PublicKey.findProgramAddressSync(
      [SEEDS.prediction, propKey.toBuffer()], PROGRAM_ID,
    );
    expect(pda).toBeInstanceOf(PublicKey);
  });
});
