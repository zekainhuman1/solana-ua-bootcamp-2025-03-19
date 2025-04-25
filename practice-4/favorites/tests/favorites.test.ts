import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Favorites } from "../target/types/favorites";
import { airdropIfRequired, getCustomErrorMessage } from "@solana-developers/helpers";
import { expect, describe, test } from '@jest/globals';
import { systemProgramErrors } from "./system-program-errors";

describe("favorites", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  it("Creates and updates favorites on the blockchain", async () => {
    const user = web3.Keypair.generate();
    const program = anchor.workspace.Favorites as Program<Favorites>;

    console.log(`User public key: ${user.publicKey}`);

    // Airdrop SOL to user
    await airdropIfRequired(
      anchor.getProvider().connection,
      user.publicKey,
      0.5 * web3.LAMPORTS_PER_SOL,
      1 * web3.LAMPORTS_PER_SOL
    );

    // Initial favorite data
    const initialFavoriteNumber = new anchor.BN(23);
    const initialFavoriteColor = "red";

    // 1. Create initial favorites
    let tx = await program.methods
      .setFavorites(initialFavoriteNumber, initialFavoriteColor)
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc();

    console.log(`Tx signature for setting favorites: ${tx}`);

    // Calculate the PDA account address for favorites
    const [favoritesPda, _favoritesBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), user.publicKey.toBuffer()],
      program.programId
    );

    // Ensure initial favorites are set correctly
    let dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.color).toEqual(initialFavoriteColor);
    expect(dataFromPda.number.toNumber()).toEqual(initialFavoriteNumber.toNumber());

    // Updated favorite data
    const updatedFavoriteNumber = new anchor.BN(42);
    const updatedFavoriteColor = "blue";

    // 2. Update the favorites
    tx = await program.methods
      .updateFavorites(updatedFavoriteNumber, updatedFavoriteColor)
      .accounts({
        user: user.publicKey,
      })
      .signers([user])
      .rpc();

    console.log(`Tx signature for updating favorites: ${tx}`);

    // Fetch updated favorites and validate
    dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.color).toEqual(updatedFavoriteColor);
    expect(dataFromPda.number.toNumber()).toEqual(updatedFavoriteNumber.toNumber());

    console.log("Test completed successfully");
  }, 60000);
});
