import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NotesProgram } from "../target/types/notes_program";
import { PublicKey } from "@solana/web3.js";

describe("notes-pda", () => {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.notesProgram as Program<NotesProgram>;
  const walletkey = provider.wallet.publicKey;
  const PROGRAM_ID = program.programId;

  const [notesPda, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("note"), walletkey.toBuffer()],
    PROGRAM_ID
  );

  
  it("updating Note", async () => {
    const tx = await program.methods.updateNote("I think I like Renuka!...").accounts({
      noteAccount: notesPda,
      user: walletkey,
    }).rpc();

    console.log("Update signature", tx);

    const account = await program.account.noteAccount.fetch(notesPda);
    console.log("Updated Note:", account.content);
  });
  
  it("delete note", async () => {
    const tx = await program.methods.deleteNote().accounts({
      noteAccount: notesPda,
      user: walletkey,
    }).rpc();

    console.log("Delete signature", tx);

    const account = await program.account.noteAccount.fetch(notesPda);
    console.log("Updated Cleared:", account.content);
  });
});
