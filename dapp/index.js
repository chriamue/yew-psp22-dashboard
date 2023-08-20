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

async function fetchTotalSupply() {
  const { ApiPromise, WsProvider } = await import(
    "https://cdn.jsdelivr.net/npm/@polkadot/api@10.9.1/+esm"
  );
  const { ContractPromise } = await import(
    "https://cdn.jsdelivr.net/npm/@polkadot/api-contract@10.9.1/+esm"
  );

  const provider = new WsProvider("ws://127.0.0.1:9944");
  const api = await ApiPromise.create({ provider });

  const contractAddress = "5FbxgE9CZgib7p4oWi34Tx5vqLHsXKNGEWnfMn6pMT7VzwTx";
  const alice = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

  // Load metadata from the JSON file
  const metadata = await loadContractMetadata();

  const contract = new ContractPromise(api, metadata, contractAddress);

  console.log("contract:", contract.query);

  const gasLimit = 3000n * 1000000n;
  const storageDepositLimit = null;

  const totalSupply = await contract.query["psp22::totalSupply"](
    alice, 
    {
      gasLimit,
      storageDepositLimit,
    }
  );

  console.log("totalSupply:", totalSupply);

  if (!totalSupply.isOk) {
    throw new Error("Failed to fetch total supply", totalSupply.asErr);
  }

  console.log(`Total Supply: ${totalSupply.output.toNumber()}`);
}

fetchTotalSupply().catch(console.error);
