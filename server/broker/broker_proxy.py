from .implementations import IBroker, broker_factory, BrokerType

class BrokerProxy:
    """
    Bridge class to interface with Broker implemenations such as Redis and RabbitMQ
    """
    broker: IBroker

    def __init__(self) -> None:
        self.broker = broker_factory(BrokerType.REDIS)
        self.broker.connect()

    def load_policy(self, client: str, policy: str) -> None:
        """
        Loads a policy into the Broker

        Arguments:
        ----------
        * `client`: The client to apply the policy to
        * `policy`: The policy to load
        """
        self.broker.load_policy(client, policy)

    def get_logs(self) -> None:
        """
        Reads logs from the Broker
        """
        self.broker.get_logs()