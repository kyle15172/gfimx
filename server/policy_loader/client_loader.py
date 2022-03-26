"""
Reads a clients.toml file, containing client information such
as name, unique key and name of policy file.
"""

import logging
import toml
from pathlib import Path
from typing import Optional, Dict, Tuple

from utils import get_dir


def load_client() -> Tuple[str, Dict[str, str]]:
    logging.info("Loading clients.toml...")

    policy_dir: Optional[str]
    clients: Dict[str, str]

    policy_dir = get_dir()

    try:
        clients = toml.load(Path(policy_dir) / "clients.toml")
    except Exception as e:
        logging.error(f"Could not load clients.toml: {e}")
        exit(1)

    logging.info("clients.toml loaded! Loading client policies...")

    for k, v in clients.items():
        yield (k, v)
