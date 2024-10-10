# sshuttle_tunnel.py
import ssl
import socket

def connect(host, port):
    context = ssl.create_default_context()
    with socket.create_connection((host, port)) as sock:
        with context.wrap_socket(sock, server_hostname=host) as ssock:
            # Use the TLS/SSL tunnel to communicate with the server
            # Send a test message
            test_message = b"Hello, server!"
            ssock.sendall(test_message)
            # Receive a response from the server
            response = ssock.recv(1024)
            return response