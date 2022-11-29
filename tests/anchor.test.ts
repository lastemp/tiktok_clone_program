// No imports needed: web3, anchor, pg and more are globally available

const utf8 = anchor.utils.bytes.utf8

const defaultAccounts = {
  //tokenProgram: TOKEN_PROGRAM_ID,
  //clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
  clock: web3.SYSVAR_CLOCK_PUBKEY,
  //systemProgram: SystemProgram.programId,
  systemProgram: web3.SystemProgram.programId,
  //systemProgram: pg.program.programId,//.program.programId
  //systemProgram: pg.program.programId,
  // rent: anchor.web3.SYSVAR_RENT_PUBKEY,
}

// Generate keypair for the new account
//const user = new web3.Keypair();

// Configure the client to use the local cluster.
//anchor.setProvider(provider)
//const program = anchor.workspace.TikTokClone as Program<TikTokClone>
//let creatorKey = provider.wallet.publicKey
let creatorKey = pg.wallet.publicKey
//let stateSigner //= pg.wallet.publicKey
//let videoSigner //= pg.wallet.publicKey

describe('tiktok-test', () => {
  const [stateSigner, _] = web3.PublicKey
      .findProgramAddressSync(
        [
          anchor.utils.bytes.utf8.encode("state")
        ],
        pg.program.programId
      );
	  
  it('Setup Platform', async () => {
          
    await pg.program.methods
      .setupPlatform()
      .accounts({
        signer: pg.wallet.publicKey,
        state: stateSigner,
      })
      .rpc();
    
    const stateInfo = await pg.program.account.stateAccount.fetch(stateSigner);
      assert(
        stateInfo.signer.toString() === creatorKey.toString(),
        'State Creator is Invalid',
      )

  });
  

  it('SignUp User', async () => {
      const [signUpUserPDA, _] = await web3.PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("user"),
          pg.wallet.publicKey.toBuffer()
        ],
        pg.program.programId
      );

      const name: string = "firstuser";
      const profileUrl: string = "https://firstuser.com";
      await pg.program.methods
      .signUpUser(name,profileUrl)
      .accounts({
        user: signUpUserPDA,
        signer: pg.wallet.publicKey,
        ...defaultAccounts
      })
      .rpc();  
    
      const userInfo = await pg.program.account.userAccount.fetch(signUpUserPDA)
      assert(userInfo.signer.toString() === signUpUserPDA.toString(), "User is Invalid");
	  
      console.log("userInfo.signer data is:", userInfo.signer.toString());  
  });

  it('Upload First Video', async () => {
      const stateInfo = await pg.program.account.stateAccount.fetch(stateSigner);
      if (stateInfo.videoCount > 0) {
        return;
      }
      const [videoAccountPDA, _] = await web3.PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("video"),
          //stateInfo.videoCount.toBuffer("be", 8)
          new BN(stateInfo.videoCount).toArrayLike(Buffer, "be", 8)
          //anchor.utils.bytes.utf8.encode(stateInfo.videoCount.toString),
        ],
        pg.program.programId
      );

      const description: string = "this is the first video";
      const videoUrl: string = "https://firstvideo.com";
      const name: string = "firstuser";
      const profileUrl: string = "https://firstuser.com";
      await pg.program.methods
      .uploadVideo(description,videoUrl,name,profileUrl)
      .accounts({
        state: stateSigner,
        video: videoAccountPDA,
        signer: pg.wallet.publicKey,
        ...defaultAccounts
      })
      .rpc();  
    
      const videoInfo = await pg.program.account.videoAccount.fetch(videoAccountPDA)
      assert(videoInfo.signer.toString() === videoAccountPDA.toString(), "Video is Invalid");
	  
      console.log("videoInfo.signer data is:", videoInfo.signer.toString());  
  });
  
  it('Upload Second Video', async () => {
      const stateInfo = await pg.program.account.stateAccount.fetch(stateSigner);
      if (stateInfo.videoCount > 0) {
        return;
      }
      const [videoAccountPDA, _] = await web3.PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("video"),
          new BN(stateInfo.videoCount).toArrayLike(Buffer, "be", 8)
        ],
        pg.program.programId
      );

      const description: string = "this is the second video";
      const videoUrl: string = "https://secondvideo.com";
      const name: string = "seconduser";
      const profileUrl: string = "https://seconduser.com";
      await pg.program.methods
      .uploadVideo(description,videoUrl,name,profileUrl)
      .accounts({
        state: stateSigner,
        video: videoAccountPDA,
        signer: pg.wallet.publicKey,
        ...defaultAccounts
      })
      .rpc();  
    
      const videoInfo = await pg.program.account.videoAccount.fetch(videoAccountPDA)
      assert(videoInfo.signer.toString() === videoAccountPDA.toString(), "Second Video is Invalid");
	  
      console.log("videoInfo.signer data is:", videoInfo.signer.toString());  
  });

  it('Create First Comment', async () => {
      const [videoAccountPDA] = await web3.PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("video"),
          new BN(0).toArrayLike(Buffer, "be", 8)
        ],
        pg.program.programId
      );

      const videoInfo = await pg.program.account.videoAccount.fetch(videoAccountPDA);

      const [commentAccountPDA] = await web3.PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("comment"),
          new BN(videoInfo.index).toArrayLike(Buffer, "be", 8),
          new BN(videoInfo.commentCount).toArrayLike(Buffer, "be", 8)
        ],
        pg.program.programId
      );

      const text: string = "this is the first comment";
      const name: string = "firstcommenter";
      const profileUrl: string = "https://firstcommenter.com";
      await pg.program.methods
      .createComment(text,name,profileUrl)
      .accounts({
        video: videoAccountPDA,
        comment: commentAccountPDA,
        signer: pg.wallet.publicKey,
        ...defaultAccounts
      })
      .rpc();  
    
      const commentInfo = await pg.program.account.commentAccount.fetch(commentAccountPDA)
      assert(commentInfo.signer.toString() === commentAccountPDA.toString(), "Comment is Invalid");
	  
      console.log("commentInfo.signer data is:", commentInfo.signer.toString());  
  });

  it('Like Video', async () => {
      const [videoAccountPDA] = await web3.PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("video"),
          new BN(0).toArrayLike(Buffer, "be", 8)
        ],
        pg.program.programId
      );
      //const fifthUser = new web3.Keypair();

      await pg.program.methods
      .likeVideo()
      .accounts({
        video: videoAccountPDA,
        signer: pg.wallet.publicKey,
        ...defaultAccounts
      })
      .rpc();  
    
      const likeVideoInfo = await pg.program.account.videoAccount.fetch(videoAccountPDA)
      assert(likeVideoInfo.signer.toString() === videoAccountPDA.toString(), "like Video is Invalid");
	  
      console.log("likeVideoInfo.signer data is:", likeVideoInfo.signer.toString());  
  });
  

  it('Second user liked Video', async () => {
      const [videoAccountPDA] = await web3.PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("video"),
          new BN(0).toArrayLike(Buffer, "be", 8)
        ],
        pg.program.programId
      );
      
      const secondUser = new web3.Keypair();

      await pg.program.methods
      .likeVideo()
      .accounts({
        video: videoAccountPDA,
        signer: secondUser.publicKey,
        ...defaultAccounts
      })
      .signers([secondUser])
      .rpc();
    
      const likeVideoInfo = await pg.program.account.videoAccount.fetch(videoAccountPDA)
      assert(likeVideoInfo.signer.toString() === videoAccountPDA.toString(), "like Video is Invalid");
	  
      console.log("likeVideoInfo.signer data is:", likeVideoInfo.signer.toString());  
  });
  

  it('Third user liked Video', async () => {
      const [videoAccountPDA] = await web3.PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("video"),
          new BN(0).toArrayLike(Buffer, "be", 8)
        ],
        pg.program.programId
      );
      
      const thirdUser = new web3.Keypair();

      await pg.program.methods
      .likeVideo()
      .accounts({
        video: videoAccountPDA,
        signer: thirdUser.publicKey,
        ...defaultAccounts
      })
      .signers([thirdUser])
      .rpc();
    
      const likeVideoInfo = await pg.program.account.videoAccount.fetch(videoAccountPDA)
      assert(likeVideoInfo.signer.toString() === videoAccountPDA.toString(), "like Video is Invalid");
	  
      console.log("likeVideoInfo.signer data is:", likeVideoInfo.signer.toString());  
  });
  

  it('Follow One Another', async () => {
      const firstUser = new web3.Keypair();
      const [signUpUserPDA, _] = await web3.PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("user"),
          pg.wallet.publicKey.toBuffer()
        ],
        pg.program.programId
      );

      await pg.program.methods
      .followOneAnother()
      .accounts({
        user: signUpUserPDA,
        signer: firstUser.publicKey,
        ...defaultAccounts
      })
      .signers([firstUser])
      .rpc();
    
      const followOneAnotherInfo = await pg.program.account.userAccount.fetch(signUpUserPDA)
      assert(followOneAnotherInfo.signer.toString() === signUpUserPDA.toString(), "following One Another is Invalidated");
	  
      //console.log("followOneAnotherInfo.signer data is:", followOneAnotherInfo.signer.toString());  
  });
  
})