from socketio import SimpleClient
from socketio.exceptions import TimeoutError
from dotenv import load_dotenv
from os import getenv, system


"""
    Python SocketIO client for testing purposes.
"""

def clear():
    system('cls')


def print_menu():
    print("1) Set LED | HIGH.")
    print("2) Set LED | LOW.")
    print("3) Send hello message.")
    print("4) Exit.")
    print("\nType option below (1 / 2 / 3 / 4).")


def connect(client: SimpleClient, host, port):
    client.connect(
        url = f'http://{host}:{port}',
        transports = ['websocket']
    )

    clear()
    print(f'Connected with sId {client.sid}.')
    print_menu()

    option = int(input("> "))

    if option == 1:
        print("Sending LED HIGH...")
        client.emit(
            event = 'server_high',
            data = ''
        )

    elif option == 2:
        print("Sending LED LOW...")
        client.emit(
            event = 'server_low',
            data = ''
        )

    elif option == 3:
        print("Sending event 'message' with data: 'Hello from client!'.")

        client.emit(
            event = "message",
            data = "Hello from client!"
        )

        print("Sent! Awaiting reply from server...")
        try:
            event = client.receive(timeout=5)
        except TimeoutError:
            print('Timed out waiting for reply.')
        else:
            print(f'Recieved event: {event}')
    elif option == 4:
        return
    else:
        print("Bad option.")

    input("[ENTER] to continue.")

def main():
    load_dotenv()

    host = getenv('HOST')
    port = getenv('PORT')

    with SimpleClient() as client:
        connect(client, host, port)


if __name__ == '__main__':
    main()