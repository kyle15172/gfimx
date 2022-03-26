"""
Takes in a policy and loads it into the Broker.
"""

from asyncio.log import logger
from typing import Dict, Tuple


def load_policy(policy: Tuple[str, Dict[str, str]]):
    logger.info(f"Loading policy for client {policy[0]}")
    print(policy[1])