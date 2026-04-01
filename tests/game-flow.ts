import * as anchor from "@coral-xyz/anchor";
import { assert } from "chai";

describe("solana-homework", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  it("provider works", async () => {
    const wallet = provider.wallet;
    assert.ok(wallet.publicKey);
  });
});