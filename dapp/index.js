let getPolkadotJsExtensionMod = (() => {
  let mod = null;

  let initPromise = (async () => {
    mod = await import(
      "https://cdn.jsdelivr.net/npm/@polkadot/extension-dapp@0.46.5/+esm"
    );
  })();

  return async () => {
    if (mod == null) {
      await initPromise;
    }
    return mod;
  };
})();

async function getAccounts() {
  const extensionMod = await getPolkadotJsExtensionMod();
  await extensionMod.web3Enable("Subxt Example App");
  const allAccounts = await extensionMod.web3Accounts();
  const accountObjects = allAccounts.map((account) => ({
    name: account.meta.name,
    source: account.meta.source,
    ty: account.type,
    address: account.address,
  }));
  console.log(accountObjects);
  return JSON.stringify(accountObjects);
}

async function signPayload(payloadAsStr, source, address) {
  let payload = JSON.parse(payloadAsStr);
  const extensionMod = await getPolkadotJsExtensionMod();
  const injector = await extensionMod.web3FromSource(source);
  const signPayload = injector?.signer?.signPayload;
  if (!!signPayload) {
    const { signature } = await signPayload(payload);
    console.log("signature js:", signature);
    return signature;
  } else {
    throw "The extension's injector does not have a `signPayload` function on its `signer`";
  }
}

// Load the contract metadata from a JSON file
async function loadContractMetadata() {
  const response = await fetch("y_psp22_token.json");

  if (!response.ok) {
    throw new Error("Failed to fetch the contract metadata");
  }

  return await response.json();
}
async function initApi() {
  const { ApiPromise, WsProvider } = await import(
    "https://cdn.jsdelivr.net/npm/@polkadot/api@10.9.1/+esm"
  );

  const provider = new WsProvider("ws://127.0.0.1:9944");
  const api = await ApiPromise.create({ provider });


  return { api };
}

async function queryContract(contractAddress, queryFunction, ...args) {
  const { ContractPromise } = await import(
    "https://cdn.jsdelivr.net/npm/@polkadot/api-contract@10.9.1/+esm"
  );

  const { api } = await initApi();

  const metadata = await loadContractMetadata();
  const contract = new ContractPromise(api, metadata, contractAddress);

  const { BN, BN_ONE } = await import(
    "https://cdn.jsdelivr.net/npm/@polkadot/util@12.4.1/+esm"
  );

  const MAX_CALL_WEIGHT = new BN(5_000_000_000_000).isub(BN_ONE);
  const PROOFSIZE = new BN(1_000_000);

  const gasLimit = api?.registry.createType("WeightV2", {
    refTime: MAX_CALL_WEIGHT,
    proofSize: PROOFSIZE,
  });

  const storageDepositLimit = null;

  const { gasRequired, storageDeposit, result, output } = await contract.query[queryFunction](
    api.default,
    { gasLimit, storageDepositLimit },
    ...args
  );

  if (result.isOk) {
    console.log(`Success ${queryFunction}`, output.toHuman());
  } else {
    console.error("Error", result.asErr);
    throw new Error(result.asErr);
  }

  return output.toHuman().Ok;
}

async function fetchTotalSupply(contractAddress) {
  return await queryContract(contractAddress, "psp22::totalSupply");
}

async function fetchBalance(contractAddress, accountAddress) {
  return await queryContract(contractAddress, "psp22::balanceOf", accountAddress);
}

fetchTotalSupply("5FbxgE9CZgib7p4oWi34Tx5vqLHsXKNGEWnfMn6pMT7VzwTx").catch(
  console.error
);

fetchBalance("5FbxgE9CZgib7p4oWi34Tx5vqLHsXKNGEWnfMn6pMT7VzwTx", "5CAKT2tGVrvdu6b4HdQTds57nVNKi6TnjfiMyqPJQG9RQraY").catch(
  console.error
);
