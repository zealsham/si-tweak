# Si-Tweak
si-Tweak is a silent  payment indexer that helps both full nodes and light clients detect silent payment index by providing precomputed tweak data, taproot outputs , and if given a mnemonic seed phrase will detect user silent payments and store them . This is the scalar of the hash of the inputs and sum of all public keys. 

# Dev timeline 
1.  version storing and returning just key tweaks 
2. version storing and returning key tweaks and taproot outputs 
3. version fetching blockchain data from a local node or via api 
4. version that respond both via rpc and api calls 
5. given a seed, version that can detect silent payments to the given seed and store them . 
6. multithreaded asynchronous version .