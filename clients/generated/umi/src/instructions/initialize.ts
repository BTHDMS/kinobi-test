/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  bytes,
  mapSerializer,
  struct,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type InitializeInstructionAccounts = {
  payer: Signer;
  pda: PublicKey | Pda;
  systemProgram?: PublicKey | Pda;
};

// Data.
export type InitializeInstructionData = { discriminator: Uint8Array };

export type InitializeInstructionDataArgs = {};

export function getInitializeInstructionDataSerializer(): Serializer<
  InitializeInstructionDataArgs,
  InitializeInstructionData
> {
  return mapSerializer<
    InitializeInstructionDataArgs,
    any,
    InitializeInstructionData
  >(
    struct<InitializeInstructionData>([['discriminator', bytes({ size: 8 })]], {
      description: 'InitializeInstructionData',
    }),
    (value) => ({
      ...value,
      discriminator: new Uint8Array([175, 175, 109, 31, 13, 152, 155, 237]),
    })
  ) as Serializer<InitializeInstructionDataArgs, InitializeInstructionData>;
}

// Instruction.
export function initialize(
  context: Pick<Context, 'programs'>,
  input: InitializeInstructionAccounts
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'kinobiTest',
    '3eSEGBd6d6C6imYBaeu9vDJqf4cCjxPE1JkX5TaagJCD'
  );

  // Accounts.
  const resolvedAccounts = {
    payer: {
      index: 0,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    pda: { index: 1, isWritable: true as boolean, value: input.pda ?? null },
    systemProgram: {
      index: 2,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Default values.
  if (!resolvedAccounts.systemProgram.value) {
    resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
      'systemProgram',
      '11111111111111111111111111111111'
    );
    resolvedAccounts.systemProgram.isWritable = false;
  }

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getInitializeInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}