from abc import ABC, abstractmethod

class IBroker(ABC):

    @abstractmethod
    def connect(self) -> None:
        pass

    @abstractmethod
    def load_policy(self, client: str, policy: str) -> None:
        pass