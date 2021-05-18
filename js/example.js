const {
    encodeInstructionData
} = require("./client.js");

async function createAccount(connection) {
    let transaction = new Transaction().add(
        new TransactionInstruction(
            keys: ,
            programId: ,
            data: encodeInstructionData({ initialize: {} })
        ),
    );

    await sendAndConfirmTransaction(
        connection,
        transaction,
        [],
        {
            commitment: "singleGossip",
            preflightCommitment: "singleGossip",
        };
}