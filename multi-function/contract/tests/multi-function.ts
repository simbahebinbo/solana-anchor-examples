import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
// @ts-ignore
import {MultiFunction} from "../target/types/multi_function";

describe("multi-function", () => {
    // Configure the client to use the local cluster.
    let provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.MultiFunction as Program<MultiFunction>;
    const user = anchor.web3.Keypair.generate();

    it("Is all!", async () => {
        // Add your test here.
        let airdrop_tx = await provider.connection.requestAirdrop(
            user.publicKey,
            1000000000
        );
        const latestBlockHash = await provider.connection.getLatestBlockhash();
        await provider.connection.confirmTransaction({
            blockhash: latestBlockHash.blockhash,
            lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
            signature: airdrop_tx
        })


        try {
            // 构造 FunctionA 调用
            const txA = await program.methods
                .functionA(new anchor.BN(42))
                .accounts({
                    signer: user.publicKey,
                })
                .signers([user])
                .rpc();

            console.log("FunctionA transaction signature", txA);

            // 构造 FunctionB 调用
            const txB = await program.methods
                .functionB("hello")
                .accounts({
                    signer: user.publicKey,
                })
                .signers([user])
                .rpc();

            console.log("FunctionB transaction signature", txB);
        } catch (error) {
            console.log(error)
        }
    });
});
