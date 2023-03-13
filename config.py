
# pylint: disable=too-few-public-methods
import os


class Config:
    JWT_MAX_AGE_SECONDS = 600

    try:
        SECRET_KEY = os.environ["SECRET_KEY"]
        JWT_SECRET_KEY = os.environ["JWT_SECRET_KEY"]
        JWT_ALGORITHMS = os.environ['JWT_ALGORITHMS']

        REDIS_HOST = os.environ['REDIS_HOST']
        REDIS_PORT = os.environ['REDIS_PORT']
        REDIS_PASS = os.environ['REDIS_PASS']

    except KeyError as key:
        raise RuntimeError(f"Service configuration failed. {key} is unset") from key
