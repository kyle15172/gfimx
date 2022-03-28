import logging
from broker import BrokerProxy
from policy_loader import load_client, load_policy

logging.basicConfig(format='[%(levelname)s] %(asctime)s - %(message)s', level=logging.INFO)

if __name__ == "__main__":

    broker = BrokerProxy()

    for client in load_client():
        load_policy(client, broker)
