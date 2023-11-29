import { step } from "mocha-steps";
import {
	EVOLUTION_COLLECTION_ABI,
	GAS_LIMIT,
	GAS_PRICE,
	GENESIS_ACCOUNT,
	GENESIS_ACCOUNT_PRIVATE_KEY,
} from "./config";
import { createCollection, customRequest, describeWithExistingNode } from "./util";

// ATTENTION !!!
// Before running load test, make sure to:
// - update GENESIS_ACCOUNT with the address of the account that will be used to create the collection
// - update GENESIS_ACCOUNT_PRIVATE_KEY with the private key of the account that will be used to create the collection
// - be aware the number of transactions that will be sent is 6000
// - check rpc endpoint in config.ts => https://vscode.dev/github/freeverseio/laos/blob/test/load_testing/ownership-chain/e2e-tests/tests/util.ts#L42-L43
describeWithExistingNode("Frontier RPC (Load Test)", (context) => {

	step("thousands mints over a collection", async function () {
		this.timeout(700000);

		const nonce = await context.web3.eth.getTransactionCount(GENESIS_ACCOUNT);

		// check block gas limit
		const blockGasLimit = (await context.web3.eth.getBlock("latest")).gasLimit;
		console.log(`Block gas limit: ${blockGasLimit}`);

		console.log(`Creating collection...`);
		const result = await createCollection(context);
		console.log(`Collection created at: ${result.options.address}`);

		const collectionContract = new context.web3.eth.Contract(EVOLUTION_COLLECTION_ABI, result.options.address, {
			from: GENESIS_ACCOUNT,
			gasPrice: GAS_PRICE,
			gas: GAS_LIMIT,
		});
		context.web3.eth.accounts.wallet.add(GENESIS_ACCOUNT_PRIVATE_KEY);

		console.log(`Sending transactions...`);
		for (let i = 0; i < 6000; i++) {
			const slot = i;
			const to = GENESIS_ACCOUNT;
			const tokenURI = "ssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaassssssssssssssxxxxxxxxxxxxx";
			const estimateGas = await collectionContract.methods.mintWithExternalURI(to, slot, tokenURI).estimateGas({
				from: GENESIS_ACCOUNT,
			})
			console.log(`[${i}] Estimated gas: ${estimateGas} | collection: ${result.options.address}`); 
			collectionContract.methods.mintWithExternalURI(to, slot, tokenURI).send({
				from: GENESIS_ACCOUNT,
				gas: GAS_LIMIT,
				gasPrice: GAS_PRICE,
				nonce: nonce + i,
			})
		}

		// TXPOOL
		while (true) {
			let txpoolStatusAfter = await customRequest(context.web3, "txpool_status", []);
			console.log(`[POOL] Pending: ${parseInt(txpoolStatusAfter.result.pending, 16)}`);
			console.log(`[POOL] Queued: ${parseInt(txpoolStatusAfter.result.queued, 16)}`);
			await new Promise(resolve => setTimeout(resolve, 1000));
		}
	});
});

//////////////////////////////////////
/////// GET CALLDATA
//////////////////////////////////////
// const abi = contract.methods.createCollection(GENESIS_ACCOUNT).encodeABI();
// console.log(`ABI: ${abi}`);

// const tx = {
// 	to: CONTRACT_ADDRESS,
// 	gas: GAS_LIMIT,
// 	gasPrice: GAS_PRICE,
// 	data: abi,
// }

// context.web3.eth.accounts.signTransaction(tx, GENESIS_ACCOUNT_PRIVATE_KEY)
// 	.then((signed) => {
// 		context.web3.eth.sendSignedTransaction(signed.rawTransaction)
// 			.on('receipt', (receipt) => {
// 				console.log(`Transaction ${receipt.transactionHash} included in block ${receipt.blockNumber}`);
// 			})
// 			.on('error', (error) => {
// 				console.error(error);
// 			});
// 	});
//////////////////////////////////////
//////////////////////////////////////
//////////////////////////////////////