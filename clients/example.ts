import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { TransactionBuilderSendAndConfirmOptions, generateSigner, keypairIdentity, sol } from '@metaplex-foundation/umi';
import { publicKey as publicKeySerializer, string } from '@metaplex-foundation/umi/serializers';
import { getKinobiTestProgramId } from './generated/umi/src/programs/kinobiTest';
import { initialize, setData } from './generated/umi/src/instructions';

const umi = createUmi('http://127.0.0.1:8899', { commitment: 'processed' });
const creator = generateSigner(umi);
umi.use(keypairIdentity(creator));

const options: TransactionBuilderSendAndConfirmOptions = {
    confirm: { commitment: 'processed' }
};

const pda = umi.eddsa.findPda(getKinobiTestProgramId(umi), [
    string({ size: 'variable' }).serialize('example'),
    publicKeySerializer().serialize(creator.publicKey),
]);

async function logPda() {
    console.log(`PDA: ${pda.toString()}`);
}

async function airdropFunds() {
    try {
        await umi.rpc.airdrop(creator.publicKey, sol(100), options.confirm);
        console.log(`1. ✅ - Airdropped 100 SOL to the ${creator.publicKey.toString()}`);
    } catch (error) {
        console.error('1. ❌ - Error airdropping SOL to the wallet.', error);
    }
}

async function initializeAccount() {
    try {
        await initialize(umi, { pda, payer: creator }).sendAndConfirm(umi, options);
        console.log('2. ✅ - Initialized the account.');
    } catch (error) {
        console.error('2. ❌ - Error initializing the account.', error);
    }
}

async function setDataAccount(num: number, value: number) {
    try {
        await setData(umi, { authority: creator, pda, data: value }).sendAndConfirm(umi, options);
        console.log(`${num}. ✅ - Set data to ${value}.`);
    } catch (error) {
        console.error(num, '. ❌ - Error setting data.', error);
    }
}

async function main() {
    await logPda();
    await airdropFunds();
    await initializeAccount();
    await setDataAccount(3, 10);
    await setDataAccount(4, 20);
    await setDataAccount(5, 30);
    await setDataAccount(6, 40);
}

main().then(() => {
    console.log('🚀 - Done!');
}).catch((error) => {
    console.error('❌ - Error:', error);
});