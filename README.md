Title: Basic File Encryption Utility

Description: This mini project is a simple command-line interface (CLI) application that provides basic file encryption and decryption using a symmetric-key algorithm. A basic file encryption utility that supports encryption and decryption using the AES256-CBC mode with PKCS7 padding. You can use it by providing the input file, output

## build
```
$ cargo build --release
$ ./target/release/basic_file_encryption --help
```

This basic file encryption utility that supports encryption and decryption using the AES256-CBC mode with PKCS7 padding. You can use it by providing the input file, output file, and the key. Here's an example of how to use the command line tool for encryption and decryption:

### Encryption:
```
$ ./target/release/basic_file_encryption encrypt -i input.txt -o encrypted.bin -k 00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff
```

### Decryption:
```
$ ./target/release/basic_file_encryption decrypt -i encrypted.bin -o decrypted.txt -k 00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff
```

Just replace input.txt, encrypted.bin, decrypted.txt, and the key with your actual file paths and encryption key. The key must be a 64-character hex string (representing a 256-bit key).

However, this project does not include features such as key management, authentication, or secure random number generation. Those will be implemented in further projects
