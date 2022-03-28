from .ibroker import IBroker

class RedisBroker(IBroker):

    def __init__(self) -> None:
        super().__init__()

    def connect(self) -> None:
        print("Redis connected")

    def load_policy(client: str, policy: str) -> None:
        return None