// Singleton for Polkadot JS Extension Module
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

// Singleton for Polkadot API
let getPolkadotApi = (() => {
  let apiInstance = null;
  let initPromise = (async () => {
    const { ApiPromise, WsProvider } = await import(
      "https://cdn.jsdelivr.net/npm/@polkadot/api@10.9.1/+esm"
    );
    const provider = new WsProvider("ws://127.0.0.1:9944");
    apiInstance = await ApiPromise.create({ provider });
  })();

  return async () => {
    if (apiInstance == null) {
      await initPromise;
    }
    return apiInstance;
  };
})();

// Singleton for Polkadot Util
let getPolkadotUtil = (() => {
  let util = null;
  let initPromise = (async () => {
    util = await import(
      "https://cdn.jsdelivr.net/npm/@polkadot/util@12.4.1/+esm"
    );
  })();

  return async () => {
    if (util == null) {
      await initPromise;
    }
    return util;
  };
})();

// Singleton for Polkadot Contract API
let getPolkadotContractApi = (() => {
  let contractApi = null;
  let initPromise = (async () => {
    contractApi = await import(
      "https://cdn.jsdelivr.net/npm/@polkadot/api-contract@10.9.1/+esm"
    );
  })();

  return async () => {
    if (contractApi == null) {
      await initPromise;
    }
    return contractApi;
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
// Helper function to initialize the contract
async function initContract(contractAddress) {
  const { ContractPromise } = await getPolkadotContractApi();
  const api = await getPolkadotApi();
  const metadata = await loadContractMetadata();
  return new ContractPromise(api, metadata, contractAddress);
}

async function getGasLimit() {
  const { BN, BN_ONE } = await getPolkadotUtil();
  const MAX_CALL_WEIGHT = new BN(11344007255).isub(BN_ONE);
  const PROOFSIZE = new BN(131072);
  const api = await getPolkadotApi();
  return api?.registry.createType("WeightV2", {
    refTime: MAX_CALL_WEIGHT,
    proofSize: PROOFSIZE,
  });
}

async function queryContract(contractAddress, queryFunction, args) {
  console.log("queryContract", contractAddress, queryFunction, args);
  const contract = await initContract(contractAddress);
  const gasLimit = await getGasLimit();

  const { result, output } = await contract.query[queryFunction](
    contract.api.default,
    { gasLimit },
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

async function executeContractFunction(
  contractAddress,
  source,
  senderAddress,
  functionName,
  ...args
) {
  const contract = await initContract(contractAddress);
  const gasLimit = await getGasLimit();
  const extensionMod = await getPolkadotJsExtensionMod();
  const injector = await extensionMod.web3FromSource(source);

  const result = await contract.tx[functionName](
    {
      gasLimit,
    },
    ...args
  ).signAndSend(senderAddress, { signer: injector.signer });

  return result.toHuman();
}
