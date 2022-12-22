from base64 import b64decode
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.backends import default_backend

backend = default_backend()

def decrypt(ciphertext, key):
    cipher = Cipher(algorithms.AES(key), modes.ECB(), backend=backend)
    decryptor = cipher.decryptor()

    dec_data = decryptor.update(ciphertext) + decryptor.finalize()

    message = dec_data

    return message

print(decrypt(b64decode("""...Really long
"""), b"YELLOW SUBMARINE").decode())
