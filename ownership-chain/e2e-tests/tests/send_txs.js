const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');

const BOB = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';


async function main() {
    const provider = new WsProvider('ws://127.0.0.0:9999');
    const api = await ApiPromise.create({ provider });

    const keyringEth = new Keyring({ type: 'ethereum' });
    const alice = keyringEth.addFromUri("0xb9d2ea9a615f3165812e8d44de0d24da9bbd164b65c4f0573e1ce2c8dbd9c8df");

    let nonce = await api.rpc.system.accountNextIndex(alice.address);
    
    for (let i = 0; i < 5000; i++) {
        const transfer = api.tx.balances.transferAllowDeath(BOB, 1);
        const hash = await transfer.signAndSend(alice, {nonce});
        nonce++;
    }

}

main().catch(console.error);
