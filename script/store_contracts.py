from concurrent.futures import thread
import glob
import json
import os.path
import subprocess
from time import sleep

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


contracts: list[str] = [os.path.join(config['path'], f"{contract}.wasm") for contract in config['contracts']]
all_contracts: list[str] = glob.glob(os.path.join(config['path'], "*.wasm"))

if len(contracts) == 0:
    contracts = all_contracts

_from = config['wallet-address']
data_init = config['data-init']

for (i, contract) in enumerate(contracts):
    contract_name = contract.split('/')[-1].replace('.wasm', '')
    store_response = run_command(f"nibid tx wasm store {contract} \
    --from {_from} \
    --gas auto \
    --gas-adjustment 1.5 \
    --gas-prices 0.025unibi \
    --yes")
    save(contract_name, "store.json", store_response)

    store_response = json.loads(store_response)
    sleep(5)
    tx_hash = store_response['txhash']

    # tx_hash = "70A68683694E54F1FC8B851AA51714D2E15B469A8C600C18837D5348B8BC95A5"
    q_tx = run_command(f"nibid q tx {tx_hash}")
    save(contract_name, "txhash.json", q_tx)

    q_tx = json.loads(q_tx)
    code_id = q_tx['logs'][0]['events'][1]['attributes'][1]['value']
    txhash_init = json.loads(run_command(f"nibid tx wasm instantiate {code_id} \
        '{json.dumps(data_init[i])}' \
        --admin {_from} \
        --label {contract_name} \
        --from {_from} \
        --gas auto \
        --gas-adjustment 1.5 \
        --gas-prices 0.025unibi \
        --yes"))['txhash']
    sleep(5)
    # txhash_init = "9F49AAE881D83767903ACEB382AA4B19CAB044E6906B96A3FDD93DEF322C3702"
    contract_address = json.loads(run_command(f"nibid q tx {txhash_init}"))['logs'][0]['events'][1]['attributes'][0]['value']

    print({
        "code_id": code_id,
        "txhash_init": txhash_init,
        "contract_address": contract_address
    })

    save(contract_name, "env.json", json.dumps({
        "code_id": code_id,
        "txhash_init": txhash_init,
        "contract_address": contract_address
    }, indent=4))

