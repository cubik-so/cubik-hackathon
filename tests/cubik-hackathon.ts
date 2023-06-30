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
      const [hackathon_account] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from('hackathon'),
          wallet.publicKey.toBuffer(),
          Buffer.from(JSON.stringify(0)),
        ],
        program.programId
      );
      const ix = await program.methods
        .hackathonInit(JSON.stringify(0))
        .accounts({
          authority: wallet.publicKey,
          hackathonAccount: hackathon_account,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      console.log(ix);
    } catch (error) {
      console.log(error);
    }
  });
  it('Is initialized!', async () => {
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
        '0',
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
