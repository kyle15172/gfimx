"""
Reads a clients.toml file, containing client information such
as name, unique key and name of policy file.
"""

import logging
import os
import toml
from pathlib import Path
from typing import Optional, Dict, Tuple


def load_client() -> Tuple[str, Dict[str, str]]:
    logging.info("Loading clients.toml...")

    policy_dir: Optional[str]
    clients: Dict[str, str]

    try:
        policy_dir = os.environ["GFIMX_POLICY_DIR"]
    except KeyError:
        logging.info("No policy directory given. Using /etc/gfimx/policy")
        policy_dir = "/etc/gfimx/policy"

    try:
        clients = toml.load(Path(policy_dir) / "clients.toml")
    except Exception as e:
        logging.error(f"Could not load clients.toml: {e}")
        exit(1)

    logging.info("Clients loaded")

    for k, v in clients.items():
        yield (k, v)
