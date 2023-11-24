import { expect } from "chai";
import { step } from "mocha-steps";
import Contract from "web3-eth-contract";
import {
	CONTRACT_ADDRESS,
	EVOLUTION_COLLECTION_ABI,
	EVOLUTION_COLLETION_FACTORY_ABI,
	GAS_LIMIT,
	GAS_PRICE,
	GENESIS_ACCOUNT,
	GENESIS_ACCOUNT_PRIVATE_KEY,
	REVERT_BYTECODE,
	SELECTOR_LOG_NEW_COLLECTION,
} from "./config";
import { createCollection, customRequest, describeWithExistingNode } from "./util";

describeWithExistingNode("Frontier RPC (Create Collection)", (context) => {
	let contract: Contract;
	// This is the contract that is created in the test
	let testCollectionContract: Contract;
	// This is the address of another contract that is created in the test
	let testCollectionAddress: string;

	step("when collection is created event is emitted", async function () {
		contract = new context.web3.eth.Contract(EVOLUTION_COLLETION_FACTORY_ABI, CONTRACT_ADDRESS, {
			from: GENESIS_ACCOUNT,
			gasPrice: GAS_PRICE,
			gas: GAS_LIMIT,
		});
		context.web3.eth.accounts.wallet.add(GENESIS_ACCOUNT_PRIVATE_KEY);
		const nonce = await context.web3.eth.getTransactionCount(GENESIS_ACCOUNT);

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

		// const blockGasLimit = (await context.web3.eth.getBlock("latest")).gasLimit;
		// console.log(`Block gas limit: ${blockGasLimit}`);
		// // await createCollection(context);
		// // console.log(collectionContract.options.address);
		// const collectionContract = new context.web3.eth.Contract(EVOLUTION_COLLECTION_ABI, "0xFffFFFFFFfFfFFFFfFFfFFFe0000000000000000", {
		// 	from: GENESIS_ACCOUNT,
		// 	gasPrice: GAS_PRICE,
		// 	gas: GAS_LIMIT,
		// });
		// context.web3.eth.accounts.wallet.add(GENESIS_ACCOUNT_PRIVATE_KEY);


		// console.log(`Sending transaction...`);
		// for (let i = 0; i < 6000; i++) {
		// 	// 	// await new Promise(resolve => setTimeout(resolve, 1));
		// 	const slot = i;
		// 	const to = GENESIS_ACCOUNT;
		// 	const tokenURI = "ssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaassssssssssssssxxxxxxxxxxxxx";
		// 	const estimateGas = await collectionContract.methods.mintWithExternalURI(to, slot, tokenURI).estimateGas({
		// 		from: GENESIS_ACCOUNT,
		// 	})
		// 	console.log(`Estimated gas: ${estimateGas}`); 22426
		// 	collectionContract.methods.mintWithExternalURI(to, slot, tokenURI).send({
		// 		from: GENESIS_ACCOUNT,
		// 		gas: GAS_LIMIT,
		// 		gasPrice: GAS_PRICE,
		// 		nonce: nonce + i,
		// 	})
		// }
		// TXPOOL
		while (true) {
			let txpoolStatusAfter = await customRequest(context.web3, "txpool_status", []);
			console.log(`Pending: ${parseInt(txpoolStatusAfter.result.pending, 16)}`);
			console.log(`queued: ${parseInt(txpoolStatusAfter.result.queued, 16)}`);
			await new Promise(resolve => setTimeout(resolve, 1000));
		}
	});
});
