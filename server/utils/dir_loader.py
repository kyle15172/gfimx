from functools import lru_cache
import logging
import os

@lru_cache
def get_dir() -> str:
    try:
        policy_dir = os.environ["GFIMX_POLICY_DIR"]
    except KeyError:
        logging.info("No policy directory given. Using /etc/gfimx/policy")
        policy_dir = "/etc/gfimx/policy"

    return policy_dir