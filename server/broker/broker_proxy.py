from .implementations import IBroker, broker_factory, BrokerType

class BrokerProxy:
    broker: IBroker

    def __init__(self) -> None:
        self.broker = broker_factory(BrokerType.REDIS)
        self.broker.connect()