"""
Takes in a policy and loads it into the Broker.
"""

from asyncio.log import logger
from typing import Dict, Tuple
from pathlib import Path

from utils import get_dir

def load_policy(policy: Tuple[str, Dict[str, str]]):
    logger.info(f"Loading policy for client {policy[0]}")

    policy_dir = get_dir()
    
    try:
        with open(Path(policy_dir) / policy[1]['policy']) as pol:
            print(pol.read())
            logger.info(f"Policy for {policy[0]} loaded")
    except Exception as e:
        logger.error(f"Error in loading policy for {policy[0]}: {e}")
        exit(1)