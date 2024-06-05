import glob
import json
import os.path
import subprocess

def run_command(command: str) -> str:
    print(command)
    res = subprocess.run(command, capture_output=True, text=True ,shell=True)
    return res.stdout


with open("config.json", "r") as r:
    config = json.loads(r.read())


def save(contract_name: str, file_name: str, text: str):
    path = os.path.join(config['output-path'], contract_name)
    if not os.path.exists(path):
        os.makedirs(path)
    with open(os.path.join(path, file_name), "w") as w:
        w.write(text)


contracts: list[str] = [os.path.join(config['path'], str(contract), ".wasm") for contract in config['contracts']]
all_contracts: list[str] = glob.glob(os.path.join(config['path'], "*.wasm"))

if len(contracts) == 0:
    contracts = all_contracts

for contract in contracts:
    contract_name = contract.split('/')[-1].replace('.wasm', '')
    store_response = run_command(f"nibid tx wasm store {contract} \
    --from {config['wallet-address']} \
    --gas auto \
    --gas-adjustment 1.5 \
    --gas-prices 0.025unibi \
    --yes")
    save(contract_name, "store.json", store_response)

    store_response = json.loads(store_response)
    tx_hash = store_response['txhash']
    # tx_hash = "283CF4C9086853665AD261F43197A9EF13730D766D8E7BD522234642102A81DC"

    q_tx = run_command(f"nibid q tx {tx_hash}")
    save(contract_name, "txhash.json", q_tx)

    q_tx = json.loads(q_tx)
    save(contract_name, "code_id", q_tx['logs'][0]['events'][1]['attributes'][1]['value'])
