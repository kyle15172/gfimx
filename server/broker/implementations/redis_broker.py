from .ibroker import IBroker
from redis import Redis

class RedisBroker(IBroker):

    redis: Redis
    log_state: int = 0

    def __init__(self) -> None:
        super().__init__()

    def connect(self) -> None:
        self.redis = Redis("localhost", 6379)

    def load_policy(self, client: str, policy: str) -> None:
        self.redis.set(f"{client}_policy", policy)

    def get_logs(self) -> None:

        log_len = self.redis.llen("test_log")
        if self.log_state == log_len:
            return

        end_pos = log_len - (self.log_state + 1)
        self.log_state = log_len
        logs = list(map(lambda x: x.decode('utf-8'), self.redis.lrange("test_log", 0, end_pos)))
        for log in logs:
            print(log)