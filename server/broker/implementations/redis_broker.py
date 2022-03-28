from .ibroker import IBroker
from redis import Redis

class RedisBroker(IBroker):

    redis: Redis

    def __init__(self) -> None:
        super().__init__()

    def connect(self) -> None:
        self.redis = Redis("localhost", 6379)

    def load_policy(self, client: str, policy: str) -> None:
        self.redis.set(f"{client}_policy", policy)