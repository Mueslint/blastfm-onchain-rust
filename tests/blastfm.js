const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("🚀 Starting test...")

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Blastfm;
  const baseAccount = anchor.web3.Keypair.generate();
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });
  console.log("📝 Your transaction signature", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Music Count', account.totalMusics.toString())

  // You'll need to now pass a Music link to the function! You'll also need to pass in the user submitting the Music!
  console.log('---------------------------------------')


  console.log('Adding a new music!')
  await program.rpc.addMusic("spotify_track_1", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  console.log('And an extra one ;)')

  await program.rpc.addMusic("spotify_track_2", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  // Call the account.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Music Count', account.totalMusics.toString())

  // Access music_list on the account!
  console.log('👀 Music List', account.musicList)


  console.log('---------------------------------------')


  console.log('Updating a music')
  await program.rpc.updateMusic("spotify_track_2", "spotify_track_3", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  
  // Call the account.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Music Count', account.totalMusics.toString())
    
  // Access music_list on the account!
  console.log('👀 Music List', account.musicList)


  console.log('---------------------------------------')


  console.log('Deleting a music')
  await program.rpc.deleteMusic("spotify_track_1", {
    accounts: {
      baseAccount: baseAccount.publicKey,
    },
  });
    
  // Call the account.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Music Count', account.totalMusics.toString())
      
  // Access music_list on the account!
  console.log('👀 Music List', account.musicList)


  console.log('---------------------------------------')

  
  console.log('Reseting all musics!')
  await program.rpc.resetAllMusic({
    accounts: {
      baseAccount: baseAccount.publicKey,
      // user: provider.wallet.publicKey,
    },
  });
    
  // Call the account.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 Music Count', account.totalMusics.toString())

  // Access music_list on the account!
  console.log('👀 Music List', account.musicList)
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();