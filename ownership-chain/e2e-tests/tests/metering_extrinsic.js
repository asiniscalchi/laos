const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
    const provider = new WsProvider('ws://127.0.0.0:9999');
    const api = await ApiPromise.create({ provider });


    // let totalSize = 0;
    // let totalExtrinsics = 0;
    // const blockNumber = 251682; // Replace with the desired block number
    // const hash = await api.rpc.chain.getBlockHash(blockNumber);
    const block = await api.rpc.chain.getBlock();

    console.log(block.toHuman());
    block.block.extrinsics.forEach((tx) => {
        totalSize += tx.encodedLength;
        totalExtrinsics++;
    });

    console.log(api.tx)

    console.log(`Block number: ${block.block.header.number}`);

    console.log(`Total size of all transactions in the block: ${totalSize} bytes`);
    console.log(`Total number of transactions in the block: ${totalExtrinsics}`);



    const signer = new Wallet()
}

main().catch(console.error);
