from .ibroker import IBroker
from enum import Enum
from . import redis_broker


class BrokerType(Enum):
    REDIS = 1

def broker_factory(broker_type: BrokerType) -> IBroker:
    if broker_type == BrokerType.REDIS:
        return redis_broker.RedisBroker()
    else:
        raise Exception(f"BrokerType {broker_type} not found!")