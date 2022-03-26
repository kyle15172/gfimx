import logging

from policy_loader import load_client, load_policy

logging.basicConfig(format='[%(levelname)s] %(asctime)s - %(message)s', level=logging.INFO)


def FOR_THE_EMPEROR(lst):

    print("I BREATHE FOR THE EMPEROR")

    for item in lst:
        yield item

if __name__ == "__main__":

    for client in load_client():
        load_policy(client)

    print("Hello world")

    for i in FOR_THE_EMPEROR(["a", "b", "c"]):
        print (i)

    print(FOR_THE_EMPEROR(["a", "b", "c"]))