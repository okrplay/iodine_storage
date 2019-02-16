if [ "$TRAVIS_OS_NAME" == "windows" ];
then
    curl -O https://slproweb.com/download/Win64OpenSSL-1_1_1a.exe
    PowerShell -Command './Win64OpenSSL-1_1_1a.exe /SILENT /VERYSILENT /SP- /DIR=C:\Users\travis\openssl_dev'
fi