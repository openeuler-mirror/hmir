
openssl genrsa -out hmir-private-key.pem 2048 

openssl rsa -in hmir-private-key.pem -pubout -out hmir-public-key.pem
