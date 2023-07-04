import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CubikHackathon } from "../target/types/cubik_hackathon";
import * as spl from '@solana/spl-token';
describe('cubik-hackathon', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s'
  );
  const program = anchor.workspace.CubikHackathon as Program<CubikHackathon>;
  const provider = program.provider as anchor.AnchorProvider;
  const wallet = provider.wallet as anchor.Wallet;
  it('Is initialized!', async () => {
    try {
      // Add your test here.

      const counter = 1;
      const name = 'test';
      const symbol = 'test';
      const uri =
        'https://arweave.net/84624ygIr3NxlXkg6RXbLtrG14foXF33UisAv7YfGM8';
      const nftMint = anchor.web3.Keypair.generate();
      const [masterKey] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from('metadata'),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          nftMint.publicKey.toBuffer(),
          Buffer.from('edition'),
        ],
        TOKEN_METADATA_PROGRAM_ID
      );

      const [metadatakey] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from('metadata'),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          nftMint.publicKey.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
      );
      const [hackathon_account] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from('hackathon'),
          wallet.publicKey.toBuffer(),
          new anchor.BN(counter).toArrayLike(Buffer, 'le', 2),
        ],
        program.programId
      );
      console.log('hackathon_account', hackathon_account.toBase58());
      const [mint_account] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from('collection'),
          wallet.publicKey.toBuffer(),
          new anchor.BN(counter).toArrayLike(Buffer, 'le', 2),
        ],
        program.programId
      );
      console.log('mint_account', mint_account.toBase58());
      const collection_ata = spl.getAssociatedTokenAddressSync(
        nftMint.publicKey,
        hackathon_account,
        true
      );
      console.log('collection ata', collection_ata.toBase58());
      const ix = await program.methods
        .createCollection(counter, name, symbol, uri)
        .accounts({
          authority: wallet.publicKey,
          hackathonAccount: hackathon_account,
          associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
          masterEdition: masterKey,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          metadata: metadatakey,
          mplProgram: TOKEN_METADATA_PROGRAM_ID,
          mint: mint_account,
          tokenProgram: spl.TOKEN_PROGRAM_ID,
          collectionAta: collection_ata,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
    } catch (error) {
      console.log(error);
    }
  });
  it.skip('Is initialized!', async () => {
    const nftMint = anchor.web3.Keypair.generate();
    const [masterKey] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from('metadata'),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        nftMint.publicKey.toBuffer(),
        Buffer.from('edition'),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    const [metadatakey] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from('metadata'),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        nftMint.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );
    const [hackathon_account] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('hackathon'), wallet.publicKey.toBuffer(), Buffer.from('0')],
      program.programId
    );
    console.log('hackathon_account', hackathon_account.toBase58());
    const [participant_account] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from('participant'),
        hackathon_account.toBuffer(),
        wallet.publicKey.toBuffer(),
      ],
      program.programId
    );
    console.log('participant_account', participant_account.toBase58());
    const nft_ata = spl.getAssociatedTokenAddressSync(
      nftMint.publicKey,
      wallet.publicKey
    );
    const ix = await program.methods
      .crateParticipantNft(
        'test',
        'tes',
        'https://arweave.net/84624ygIr3NxlXkg6RXbLtrG14foXF33UisAv7YfGM8',
        new anchor.BN(0),
        wallet.publicKey
      )
      .accounts({
        associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
        authority: wallet.publicKey,
        hackathonAccount: hackathon_account,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        participantAccount: participant_account,
        powNftAta: nft_ata,
        metadata: metadatakey,
        masterEdition: masterKey,
        mint: nftMint.publicKey,
        mplProgram: TOKEN_METADATA_PROGRAM_ID,
      })
      .instruction();
  });
});
