from Crypto.Cipher import AES
from Crypto.Util.Padding import pad, unpad

key = b'Daddy4lan\0\0\0\0\0\0\0'
cipher = AES.new(key, AES.MODE_CBC, iv=b"\x12\xad\xf8\x8f\xb3\x10\x02\x92.\x98\t@\x02&\xd2T")

with open("stage2",'rb') as fin:
    with open("stage2_decrypted.zip",'wb') as fout:
        data = fin.read();
        out = unpad(cipher.decrypt(data), AES.block_size)
        fout.write(out);
