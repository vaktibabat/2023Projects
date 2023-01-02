from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.backends import default_backend
import math

backend = default_backend()

def encrypt(plaintext, key):
    cipher = Cipher(algorithms.AES(key), modes.ECB(), backend=backend)
    encryptor = cipher.encryptor()

    enc_data = encryptor.update(plaintext) + encryptor.finalize()

    message = enc_data

    return message

def xor_str(s_1: str, s_2: str):
    res = bytearray()

    for i in range(0, len(s_1)):
        res.append(s_1[i] ^ s_2[i])
    return res


#def pad(bs, k):
    return bytes(str(bs) + str(hex((k - len(bs)))) * (k - len(bs)))

def cbc_encrypt(plaintext, key, iv):
#    pad(plaintext, math.ceil(len(plaintext) / 16))
    plaintext_chunks = [plaintext[i:i+16] for i in range(0, len(plaintext), 16)]
    
    c_1 = encrypt(xor_str(plaintext_chunks[0], iv), key)
    ciphertext_chunks = [c_1]

    for i in range(1, len(plaintext_chunks)):
        ciphertext_chunks.append(encrypt(xor_str(plaintext_chunks[i], ciphertext_chunks[i - 1]), key))
    
    print(ciphertext_chunks)

print(cbc_encrypt(b"YELLOW SUBMARINE", b"YELLOW SUBMARINE", b"0123456789abcdef"))
