"""
Takes in a policy and loads it into the Broker.
"""

from asyncio.log import logger
from typing import Dict, Tuple
from pathlib import Path
from broker import BrokerProxy
from utils import validate_patterns
import re

from utils import get_dir

def load_policy(policy: Tuple[str, Dict[str, str]], broker: BrokerProxy) -> None:
    logger.info(f"Loading policy for client {policy[0]}")

    policy_dir = get_dir()
    
    try:
        with open(Path(policy_dir) / policy[1]['policy']) as pol:

            policy_str = pol.read()

            patterns = re.compile(r'patterns *= *\[(.+)\]').findall(policy_str)

            for pattern in patterns:
                validate_patterns(pattern)

            logger.info(f"Policy for {policy[0]} loaded")
    except Exception as e:
        logger.error(f"Error in loading policy for {policy[0]}: {e}")
        exit(1)