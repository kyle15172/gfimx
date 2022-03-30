from abc import ABC, abstractmethod

class IBroker(ABC):
    """
    Abstract class exposing methods that all Broker implementations
    need to implement.
    """

    @abstractmethod
    def connect(self) -> None:
        """
        Instructs the Broker to connect to it's concrete service. I.e a Redis Broker connects to Redis
        """
        pass

    @abstractmethod
    def load_policy(self, client: str, policy: str) -> None:
        """
        Loads a policy into the Broker service.

        Arguments:
        ----------
        * `client`: The client to apply the policy to
        * `policy`: The policy to load
        """
        pass

    @abstractmethod
    def get_logs(self) -> None:
        """
        Reads logs from a Broker service
        """
        pass